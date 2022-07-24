use std::collections::HashMap;
use std::error::Error;
use std::thread;
use std::time::Duration;

use num_enum::TryFromPrimitive;
use object::{File, Object, ObjectSection, ObjectSymbol, ReadCache, RelocationTarget};
use portmidi::PortMidi;

use crate::elf::ElfFile;
use crate::instruction_code::unlinked::{Opcode, Operand, Register};
use crate::midi::{get_midi_output_port, MidiMessage, MidiOutput, Tone};
use crate::vm::StepRes::{DelUniverse, NewUniverse};

#[repr(u8)]
#[derive(Clone, Debug, Copy, PartialEq, Eq, PartialOrd, Ord, TryFromPrimitive)]
pub enum Note {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
}

type Address = u64;

#[derive(Clone, Debug, Copy)]
pub enum Value {
    Number(i64),
    Note(Note),
}

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (Value::Number(a), Value::Number(b)) => a.partial_cmp(&b),
            (Value::Note(a), Value::Note(b)) => a.partial_cmp(&b),
            _ => None,
        }
    }
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Value::Number(a), Value::Number(b)) => a == b,
            (Value::Note(a), Value::Note(b)) => a == b,
            _ => false,
        }
    }
}

impl From<Note> for Value {
    fn from(n: Note) -> Self {
        Self::Note(n)
    }
}

impl From<i64> for Value {
    fn from(i: i64) -> Self {
        Self::Number(i)
    }
}

#[derive(Clone)]
struct Universe {
    memory: HashMap<Address, Value>,
    num_registers: [i64; 20],
    note_registers: [Note; 8],
    time: (u64, u64),
    notes_to_play: HashMap<u64, Vec<(Tone, u8)>>,
    current_beat: u64,
    instruments: HashMap<u8, u8>,

    wait_time: i64,
}

impl Universe {
    pub fn new() -> Self {
        Self {
            memory: Default::default(),
            num_registers: [0; 20],
            note_registers: [Note::A; 8],
            time: (4, 4),
            notes_to_play: HashMap::new(),
            current_beat: 1,
            instruments: HashMap::new(),
            wait_time: 1
        }
    }

    pub fn write_register(&mut self, reg: Register, value: impl Into<Value>) {
        match reg {
            Register::Br
            | Register::Bt
            | Register::Pc
            | Register::Sp
            | Register::Ir
            | Register::Oc
            | Register::AA
            | Register::AB
            | Register::AC
            | Register::AD
            | Register::AE
            | Register::AF
            | Register::AG
            | Register::R0
            | Register::R1
            | Register::R2
            | Register::R3
            | Register::R4
            | Register::R5
            | Register::R6
            | Register::R7 => {
                let value = convert_number(value);
                self.num_registers[reg as usize] = value;
            }
            Register::N0
            | Register::N1
            | Register::N2
            | Register::N3
            | Register::N4
            | Register::N5
            | Register::N6
            | Register::N7 => {
                let value = convert_note(value);
                self.note_registers[(reg as u8 - Register::N0 as u8) as usize] = value;
            }
        }
    }

    pub fn read_register(&self, reg: Register) -> Value {
        match reg {
            Register::Br
            | Register::Bt
            | Register::Pc
            | Register::Sp
            | Register::Ir
            | Register::Oc
            | Register::AA
            | Register::AB
            | Register::AC
            | Register::AD
            | Register::AE
            | Register::AF
            | Register::AG
            | Register::R0
            | Register::R1
            | Register::R2
            | Register::R3
            | Register::R4
            | Register::R5
            | Register::R6
            | Register::R7 => Value::Number(self.num_registers[reg as usize]),
            Register::N0
            | Register::N1
            | Register::N2
            | Register::N3
            | Register::N4
            | Register::N5
            | Register::N6
            | Register::N7 => {
                Value::Note(self.note_registers[(reg as u8 - Register::N0 as u8) as usize])
            }
        }
    }

    pub fn write(&mut self, address: Address, value: impl Into<Value>) {
        self.memory.insert(address, value.into());
    }

    pub fn read(&self, address: Address) -> Value {
        self.memory
            .get(&address)
            .cloned()
            .unwrap_or(Value::Number(0))
    }

    fn push(&mut self, v: impl Into<Value>) {
        let sp = convert_number(self.read_register(Register::Sp));
        self.write_register(Register::Sp, sp - 1);

        self.write(sp as u64, v);
    }

    fn pop(&mut self) -> Value {
        let sp = convert_number(self.read_register(Register::Sp));
        self.write_register(Register::Sp, sp + 1);

        self.read((sp + 1) as u64)
    }

    fn next_instr_value(&mut self) -> u8 {
        let pc = self.read_register(Register::Pc);
        let pc_num = convert_number(pc);
        self.write_register(Register::Pc, pc_num + 1);

        let value = self.read(pc_num as u64);
        convert_number(value) as u8
    }

    fn decode_opcode(&self, code: u8) -> Result<(Opcode, usize), Box<dyn Error>> {
        let opcode = Opcode::try_from(code)?;
        Ok((opcode, opcode.num_operands()))
    }

    fn next_opcode(&mut self) -> Result<(Opcode, usize), Box<dyn Error>> {
        let v = self.next_instr_value();
        self.decode_opcode(v)
    }

    fn next_operand_spec(&mut self) -> u8 {
        self.next_instr_value()
    }

    fn next_immediate(&mut self) -> Operand {
        Operand::Immediate(i64::from_le_bytes([(); 8].map(|_| self.next_instr_value())))
    }

    fn next_register(&mut self) -> Result<Operand, Box<dyn Error>> {
        Ok(Operand::Register(Register::try_from(
            self.next_instr_value(),
        )?))
    }

    fn next_note(&mut self) -> Result<Operand, Box<dyn Error>> {
        Ok(Operand::Note(Note::try_from(self.next_instr_value())?))
    }

    fn execute<'midi>(&mut self, opcode: Opcode, operands: Vec<Operand>, output: &mut MidiOutput<'midi>) -> Result<StepRes, Box<dyn Error>> {
        println!("executing opcode {:?}", opcode);
        match opcode {
            Opcode::Call => {
                let pc = self.read_register(Register::Pc);
                self.push(pc);

                let v = self.get_value(&operands[0]);
                self.write_register(Register::Pc, v);
            }
            Opcode::Ret => {
                let old_pc = self.pop();
                self.write_register(Register::Pc, old_pc);
            }
            Opcode::Jmp => {
                let v = self.get_value(&operands[0]);
                self.write_register(Register::Pc, v);
            }
            Opcode::Bpm => {
                return Ok(StepRes::Bpm(convert_number(self.get_value(&operands[0])) as u64));
            }
            Opcode::Time => {
                let v1 = self.get_value(&operands[0]);
                let v2 = self.get_value(&operands[1]);
                self.time = (
                    convert_number(v1) as u64,
                    convert_number(v2) as u64,
                );
            }
            Opcode::Wait => {
                let v = convert_number(self.get_value(&operands[0]));
                let old = self.read_register(Register::Bt);
                let new = convert_number(old) + v;
                self.write_register(Register::Bt, new);
                if let Some(notes) = self.notes_to_play.remove(&self.current_beat) {
                    println!("stopping notes: {:?}", notes);
                    for (note, track) in notes {
                        output.send_message(MidiMessage::note_off(track, note));
                    }
                }
                self.current_beat += 1;

                return Ok(StepRes::Wait(v as usize))
            }
            Opcode::Play => {
                let note: Tone = self.convert_tone(convert_note(self.get_value(&operands[0])));
                let length = convert_number(self.get_value(&operands[1])) as u64;
                let ir = convert_number(self.read_register(Register::Ir)) as u64;
                if ir != 0 {
                    let track = convert_number(self.read(ir)) as u8;
                    let instrument = convert_number(self.read(ir + 1)) as u8;
                    if let None = self.instruments.get(&(track as u8)) {
                        output
                            .send_message(MidiMessage::change_instrument(track, instrument));
                        self.instruments.insert(track as u8, instrument as u8);
                    }
                    output
                        .send_message(MidiMessage::note_on(track, note, 100));
                    self.notes_to_play
                        .entry(self.current_beat + length - 1)
                        .or_insert(Vec::new())
                        .push((note, track));
                }
            }
            Opcode::Push => {
                let v = self.get_value(&operands[0]);
                self.push(v)
            }
            Opcode::Pop => {
                let r = self.get_register(&operands[0]);
                let v = self.pop();
                self.write_register(r, v);
            }
            Opcode::Add => {
                let r = self.get_register(&operands[0]);
                let v1 = self.get_value(&operands[1]);
                let v2 = self.get_value(&operands[2]);

                self.write_register(r, convert_number(v1) + convert_number(v2));
            }
            Opcode::Sub => {
                let r = self.get_register(&operands[0]);
                let v1 = self.get_value(&operands[1]);
                let v2 = self.get_value(&operands[2]);

                self.write_register(r, convert_number(v1) - convert_number(v2));
            }
            Opcode::Jg => {
                let v1 = convert_number(self.get_value(&operands[0]));
                let v2 = convert_number(self.get_value(&operands[1]));

                if v1 > v2 {
                    let v = self.get_value(&operands[2]);
                    self.write_register(Register::Pc, v);
                }
            }
            Opcode::Jl => {
                let v1 = convert_number(self.get_value(&operands[0]));
                let v2 = convert_number(self.get_value(&operands[1]));

                if v1 < v2 {
                    let v = self.get_value(&operands[2]);
                    self.write_register(Register::Pc, v);
                }
            }
            Opcode::Jge => {
                let v1 = convert_number(self.get_value(&operands[0]));
                let v2 = convert_number(self.get_value(&operands[1]));

                if v1 >= v2 {
                    let v = self.get_value(&operands[2]);
                    self.write_register(Register::Pc, v);
                }
            }
            Opcode::Jle => {
                let v1 = convert_number(self.get_value(&operands[0]));
                let v2 = convert_number(self.get_value(&operands[1]));

                if v1 <= v2 {
                    let v = self.get_value(&operands[2]);
                    self.write_register(Register::Pc, v);
                }
            }
            Opcode::Jne => {
                let v1 = convert_number(self.get_value(&operands[0]));
                let v2 = convert_number(self.get_value(&operands[1]));

                if v1 != v2 {
                    let v = self.get_value(&operands[2]);
                    self.write_register(Register::Pc, v);
                }
            }
            Opcode::Jeq => {
                let v1 = convert_number(self.get_value(&operands[0]));
                let v2 = convert_number(self.get_value(&operands[1]));

                if v1 == v2 {
                    let v = self.get_value(&operands[2]);
                    self.write_register(Register::Pc, v);
                }
            }
            Opcode::St => {
                let v = self.get_value(&operands[0]);
                let addr = self.get_value(&operands[1]);

                self.write(convert_number(addr) as u64, v);
            }
            Opcode::Ld => {
                let r = self.get_register(&operands[0]);
                let addr = self.get_value(&operands[1]);

                let v = self.read(convert_number(addr) as u64);

                self.write_register(r, v);
            }
            Opcode::Stop => {
                return Ok(StepRes::Stop);
            }
            Opcode::CallAsync => {
                let addr = self.get_value(&operands[0]);
                return Ok(NewUniverse(convert_number(addr) as u64))
            }
            Opcode::RetAsync => {
                return Ok(DelUniverse)
            }
        }

        Ok(StepRes::Continue)
    }


    fn get_value(&self, op: &Operand) -> Value {
        match op {
            Operand::Immediate(i) => Value::Number(*i),
            Operand::Register(r) => self.read_register(*r),
            Operand::Note(n) => Value::Note(*n),
            Operand::Label(_) => {
                unreachable!("labels should have been replaced with immediates by relocation")
            }
        }
    }

    fn get_register(&self, op: &Operand) -> Register {
        match op {
            Operand::Immediate(_) => unreachable!(),
            Operand::Register(r) => *r,
            Operand::Note(_) => unreachable!(),
            Operand::Label(_) => {
                unreachable!("labels should have been replaced with immediates by relocation")
            }
        }
    }
    fn convert_tone(&self, n: Note) -> Tone {
        let t = Tone::from(n);
        let octave = convert_number(self.read_register(Register::Oc));
        let octave_adjusted = t.add(octave * 12);

        match n {
            Note::A => octave_adjusted.add(convert_number(self.read_register(Register::AA))),
            Note::B => octave_adjusted.add(convert_number(self.read_register(Register::AB))),
            Note::C => octave_adjusted.add(convert_number(self.read_register(Register::AC))),
            Note::D => octave_adjusted.add(convert_number(self.read_register(Register::AD))),
            Note::E => octave_adjusted.add(convert_number(self.read_register(Register::AE))),
            Note::F => octave_adjusted.add(convert_number(self.read_register(Register::AF))),
            Note::G => octave_adjusted.add(convert_number(self.read_register(Register::AG))),
        }
    }

    pub fn step<'midi>(&mut self, output: &mut MidiOutput<'midi>) -> Result<StepRes, Box<dyn Error>> {
        let (opcode, num_operands) = self.next_opcode()?;

        let mut operands = Vec::with_capacity(num_operands);

        if num_operands > 0 {
            let spec = self.next_operand_spec();

            for i in 0..num_operands {
                let spec_idx = (num_operands - 1) - i;

                let op_tp = (spec >> (spec_idx * 2)) & 0b11;

                operands.push(match op_tp {
                    0b00 => self.next_immediate(),
                    0b01 => self.next_note()?,
                    0b10 => self.next_immediate(),
                    0b11 => self.next_register()?,
                    _ => unreachable!(),
                });
            }
        }

        self.execute(opcode, operands, output)
    }
}

enum StepRes {
    Bpm(u64),
    Wait(usize),
    Stop,
    Continue,
    NewUniverse(u64),
    DelUniverse,
}

pub struct Cpu<'midi> {
    universes: Vec<Universe>,
    output: MidiOutput<'midi>,
    bpm: u64,
}

impl<'a> Cpu<'a> {
    pub fn new(ctx: &'a PortMidi) -> Self {
        Self {
            universes: vec![],
            output: get_midi_output_port(ctx),
            bpm: 120,
        }
    }

    pub fn link(&mut self, files: &mut [ElfFile]) -> Result<(), Box<dyn Error>> {
        let mut universe = Universe::new();
        universe.write(0, 0);

        // skip one page
        let mut offset = 1024;

        let mut offset_symbols = HashMap::new();
        let mut section_offsets = HashMap::new();
        let mut caches = Vec::new();

        for (f_idx, i) in files.into_iter().enumerate() {
            let cache = ReadCache::new(&mut i.f);
            let rf = File::parse(&cache)?;

            for sym in rf.symbols() {
                let name = sym.name()?;
                if name.trim() == "" {
                    continue;
                }

                if sym.is_undefined() {
                    continue;
                }

                let s_offs = sym.address();
                let s_sec = sym
                    .section()
                    .index()
                    .ok_or(format!("symbol {name} not defined in a section"))?;

                offset_symbols
                    .entry((f_idx, s_sec))
                    .or_insert_with(HashMap::new)
                    .entry(s_offs)
                    .or_insert_with(Vec::new)
                    .push(name.to_string());
            }

            // println!("{offset_symbols:?}");

            for sec in rf.sections() {
                let sec_start_offset = offset;
                section_offsets.insert((f_idx, sec.index()), sec_start_offset);

                for &b in sec.data()? {
                    universe.write(offset, b as i64);
                    offset += 1;
                }
            }

            caches.push(cache);
        }
        // println!("{section_offsets:?}");

        let mut symbol_addresses = HashMap::new();
        for ((f_idx, s_idx), symbols) in offset_symbols {
            for (offset, symbol_names) in symbols {
                for symbol in symbol_names {
                    let sec_off = section_offsets
                        .get(&(f_idx, s_idx))
                        .expect("section exists");
                    if symbol_addresses
                        .insert(symbol.clone(), sec_off + offset)
                        .is_some()
                    {
                        return Err(format!("duplicate symbol: {}", symbol).into());
                    }
                }
            }
        }
        // println!("{symbol_addresses:?}");

        for (f_idx, cache) in caches.into_iter().enumerate() {
            let rf = File::parse(&cache)?;

            for sec in rf.sections() {
                for (offset, reloc) in sec.relocations() {
                    let sec_off = section_offsets
                        .get(&(f_idx, sec.index()))
                        .expect("section exists");

                    if let RelocationTarget::Symbol(s_idx) = reloc.target() {
                        let sym = rf.symbol_by_index(s_idx)?;
                        let name = sym.name()?;
                        let offset = offset + sec_off;
                        let value = symbol_addresses
                            .get(name)
                            .ok_or(format!("undefined reference to symbol {}", name))?;
                        // println!("{offset} {:?} {value}", name);

                        for (ctr, i) in value.to_le_bytes().into_iter().enumerate() {
                            universe.write(offset + ctr as u64, i as i64);
                        }
                    }
                }
            }
        }

        // set entry point
        universe.write_register(
            Register::Pc,
            *symbol_addresses
                .get("main")
                .ok_or("no main entry point label found")? as i64,
        );

        self.universes.push(universe);

        Ok(())
    }

    pub fn step(mut self) -> Result<(Self, bool), Box<dyn Error>> {
        let mut new_bpm = self.bpm;
        let mut new_universes = Vec::new();
        let mut del_universes = Vec::new();

        if self.universes.len() == 0 {
            return Ok((self, false));
        }

        for (idx, i) in self.universes.iter_mut().enumerate() {
            i.wait_time -= 1;
            if i.wait_time > 0 {
                continue;
            }

            loop {
                match i.step(&mut self.output)? {
                    StepRes::Wait(time) => {
                        i.wait_time = time as i64;
                        break
                    },
                    StepRes::Stop => return Ok((self, false)),
                    StepRes::Continue => {}
                    StepRes::Bpm(b) => {
                        new_bpm = b;
                    }
                    StepRes::NewUniverse(addr) => {
                        let mut universe = i.clone();
                        universe.write_register(Register::Pc, addr as i64);
                        new_universes.push(universe);
                    },
                    StepRes::DelUniverse => {
                        del_universes.push(idx);
                    }
                }
            }
        }

        thread::sleep(Duration::from_millis(1000 * 60 / self.bpm));
        self.bpm = new_bpm;

        self.universes = self.universes
            .into_iter()
            .enumerate()
            .filter(|(idx, _)| !del_universes.contains(idx))
            .map(|(_, i)| i)
            .collect();
        self.universes.extend(new_universes);

        Ok((self, true))
    }
}

fn convert_note(value: impl Into<Value>) -> Note {
    match value.into() {
        Value::Note(i) => i,
        Value::Number(n) => Note::try_from((n % 7) as u8).unwrap(),
    }
}
fn convert_number(value: impl Into<Value>) -> i64 {
    match value.into() {
        Value::Number(i) => i,
        Value::Note(n) => (n as u8 % 7) as i64,
    }
}

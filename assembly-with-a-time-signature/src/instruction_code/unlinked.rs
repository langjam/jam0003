use std::collections::HashMap;
use crate::instruction_code::Relocation;
use crate::vm::Note;
use num_enum::TryFromPrimitive;

#[repr(u8)]
#[derive(Debug, Copy, Clone)]
#[derive(TryFromPrimitive)]
pub enum Opcode {
    CallAsync,
    RetAsync,
    Call,
    Ret,
    Jmp,
    Bpm,
    Time,
    Wait,
    Play,
    Push,
    Pop,
    Add,
    Sub,
    Jg,
    Jl,
    Jge,
    Jle,
    Jne,
    Jeq,
    St,
    Ld,
    Stop,
}

impl Opcode {
    pub fn num_operands(&self) -> usize {
        match self {
            Opcode::Call => 1,
            Opcode::Ret => 0,
            Opcode::Jmp => 1,
            Opcode::Bpm => 1,
            Opcode::Time => 2,
            Opcode::Wait => 1,
            Opcode::Play => 2,
            Opcode::Push => 1,
            Opcode::Pop => 1,
            Opcode::Add => 3,
            Opcode::Sub => 3,
            Opcode::Jg => 3,
            Opcode::Jl => 3,
            Opcode::Jge => 3,
            Opcode::Jle => 3,
            Opcode::Jne => 3,
            Opcode::Jeq => 3,
            Opcode::St => 2,
            Opcode::Ld => 2,
            Opcode::Stop => 0,
            Opcode::CallAsync => 1,
            Opcode::RetAsync => 0,
        }
    }
}

#[derive(Debug, Copy, Clone)]
#[repr(u8)]
#[derive(TryFromPrimitive)]
pub enum Register {
    Br = 0,
    Bt,
    Pc,
    Sp,
    Ir,
    Oc,
    AA,
    AB,
    AC,
    AD,
    AE,
    AF,
    AG,
    R0,
    R1,
    R2,
    R3,
    R4,
    R5,
    R6,
    R7,
    N0,
    N1,
    N2,
    N3,
    N4,
    N5,
    N6,
    N7,
}

pub type Label = String;

#[derive(Debug)]
pub enum Operand {
    Immediate(i64),
    Register(Register),
    Note(Note),
    Label(Label)
}

impl Operand {
    pub(crate) fn serialize(self, current_offset: u64) -> (Vec<u8>, Vec<Relocation>) {
        match self {
            Operand::Immediate(i) => (i.to_le_bytes().to_vec(), vec![]),
            Operand::Note(n) => (vec![n as u8], vec![]),
            Operand::Label(l) => (0u64.to_le_bytes().to_vec(), vec![Relocation {
                offset: current_offset,
                label: l
            }]),
            Operand::Register(r) => (vec![r as u8], vec![])
        }
    }
}

#[derive(Debug)]
pub enum UnlinkedInstruction {
    Normal {
        opcode: Opcode,
        operands: Vec<Operand>
    },
    Label(Label),
    Define(Vec<u8>),
}

#[derive(Default, Debug)]
pub struct UnlinkedSection {
    pub instr: Vec<UnlinkedInstruction>
}

#[derive(Debug)]
pub struct UnlinkedInstructionStream {
    pub sections: HashMap<String, UnlinkedSection>
}

impl UnlinkedInstructionStream {

}

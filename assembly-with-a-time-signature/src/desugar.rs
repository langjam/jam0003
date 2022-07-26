use std::collections::HashMap;
use lwb_parser::codegen_prelude::AstNode;
use lwb_parser::language::Language;

use crate::instruction_code::unlinked::{Opcode, Operand, Register, UnlinkedInstruction, UnlinkedInstructionStream, UnlinkedSection};
use crate::parser::{Asm, ast};
use crate::parser::ast::{Instruction, Label, LabelOrInstr, RegisterName};
use crate::vm::Note;


pub fn desugar_lbl<M>(label: ast::Label<M>) -> UnlinkedInstruction {
    match label {
        Label::Global(_, lbl) => UnlinkedInstruction::Label(lbl.1.as_str().to_string()),
        Label::Local(_, _) => todo!()
    }
}

pub fn desugar_reg_as_op<M>(r: &ast::Register<M>) -> Operand {
    Operand::Register(desugar_reg(r))
}

pub fn accidental_reg<M>(n: &ast::Note<M>) -> Register {
    match n.1.as_str() {
        "a" => Register::AA,
        "b" => Register::AB,
        "c" => Register::AC,
        "d" => Register::AD,
        "e" => Register::AE,
        "f" => Register::AF,
        "g" => Register::AG,
        _ => unreachable!()
    }

}

pub fn desugar_reg<M>(r: &ast::Register<M>) -> Register {
    match &r.1 {
        RegisterName::Br(_) => Register::Br,
        RegisterName::Bt(_) => Register::Bt,
        RegisterName::Pc(_) => Register::Pc,
        RegisterName::Sp(_) => Register::Sp,
        RegisterName::Ir(_) => Register::Ir,
        RegisterName::Oc(_) => Register::Oc,
        RegisterName::Accidental(_, n) => accidental_reg(&n),
        RegisterName::Num(_, i) => match i.as_str() {
            "0" => Register::R0,
            "1" => Register::R1,
            "2" => Register::R2,
            "3" => Register::R3,
            "4" => Register::R4,
            "5" => Register::R5,
            "6" => Register::R6,
            "7" => Register::R7,
            _ => unreachable!()
        }
        RegisterName::Note(_, i) => match i.as_str() {
            "0" => Register::N0,
            "1" => Register::N1,
            "2" => Register::N2,
            "3" => Register::N3,
            "4" => Register::N4,
            "5" => Register::N5,
            "6" => Register::N6,
            "7" => Register::N7,
            _ => unreachable!()
        }
    }
}

pub fn desugar_op<M>(op: ast::Operand<M>) -> Operand {
    match op {
        ast::Operand::LabelRef(_, a) => Operand::Label(a.1.as_str().to_string()),
        ast::Operand::Int(_, i) => Operand::Immediate(i.1.as_str().parse().expect("integer")),
        ast::Operand::Register(_, r) => Operand::Register(desugar_reg(&r)),
        ast::Operand::Note(_, n) => Operand::Note(match n.1.as_str() {
            "a" => Note::A,
            "b" => Note::B,
            "c" => Note::C,
            "d" => Note::D,
            "e" => Note::E,
            "f" => Note::F,
            "g" => Note::G,
            _ => unreachable!()
        })
    }
}

fn desugar_instr<M>(instr: ast::Instruction<M>, state: &mut DesugarState) -> UnlinkedInstruction {
    match instr {
        Instruction::Call(_, a) => UnlinkedInstruction::Normal { opcode: Opcode::Call, operands: vec![desugar_op(a)] },
        Instruction::Ret(_) => UnlinkedInstruction::Normal {opcode: Opcode::Ret, operands: vec![]},
        Instruction::RetAsync(_) => UnlinkedInstruction::Normal {opcode: Opcode::RetAsync, operands: vec![]},
        Instruction::Jmp(_, a) => UnlinkedInstruction::Normal {opcode: Opcode::Jmp, operands: vec![desugar_op(a)]},
        Instruction::Bpm(_, a) => UnlinkedInstruction::Normal {opcode: Opcode::Bpm, operands: vec![desugar_op(a)]},
        Instruction::Time(_, a, b) => UnlinkedInstruction::Normal {opcode: Opcode::Time, operands: vec![desugar_op(a), desugar_op(b)]},
        Instruction::Beat(_) => UnlinkedInstruction::Normal {opcode: Opcode::Wait, operands: vec![Operand::Immediate(1)]},
        Instruction::Wait(_, a) => UnlinkedInstruction::Normal {opcode: Opcode::Wait, operands: vec![desugar_op(a)]},
        Instruction::PlayOne(_, a) => UnlinkedInstruction::Normal {opcode: Opcode::Play, operands: vec![desugar_op(a), Operand::Immediate(1)]},
        Instruction::Play(_, a, b) => UnlinkedInstruction::Normal {opcode: Opcode::Play, operands: vec![desugar_op(a), desugar_op(b)]},
        Instruction::Flat(_, a) => UnlinkedInstruction::Normal {opcode: Opcode::Sub, operands: vec![Operand::Register(accidental_reg(&a)), Operand::Register(accidental_reg(&a)), Operand::Immediate(1)]},
        Instruction::Sharp(_, a) => UnlinkedInstruction::Normal {opcode: Opcode::Add, operands: vec![Operand::Register(accidental_reg(&a)), Operand::Register(accidental_reg(&a)), Operand::Immediate(1)]},
        Instruction::Push(_, a) => UnlinkedInstruction::Normal {opcode: Opcode::Push, operands: vec![desugar_op(a)]},
        Instruction::Pop(_, a) => UnlinkedInstruction::Normal {opcode: Opcode::Pop, operands: vec![desugar_reg_as_op(&a)]},
        Instruction::Add(_, a, b, c) => UnlinkedInstruction::Normal {opcode: Opcode::Add, operands: vec![desugar_reg_as_op(&a), desugar_op(b), desugar_op(c)]},
        Instruction::Sub(_, a, b, c) => UnlinkedInstruction::Normal {opcode: Opcode::Sub, operands: vec![desugar_reg_as_op(&a), desugar_op(b), desugar_op(c)]},
        Instruction::Inc(_, a) => UnlinkedInstruction::Normal {opcode: Opcode::Add, operands: vec![desugar_reg_as_op(&a), desugar_reg_as_op(&a), Operand::Immediate(1)]},
        Instruction::Dec(_, a) => UnlinkedInstruction::Normal {opcode: Opcode::Sub, operands: vec![desugar_reg_as_op(&a), desugar_reg_as_op(&a), Operand::Immediate(1)]},
        Instruction::Mov(_, a, b) => UnlinkedInstruction::Normal {opcode: Opcode::Add, operands: vec![desugar_reg_as_op(&a), Operand::Immediate(0), desugar_op(b)]},
        Instruction::St(_, a, b) => UnlinkedInstruction::Normal {opcode: Opcode::St, operands: vec![desugar_op(a), desugar_op(b)]},
        Instruction::Ld(_, a, b) => UnlinkedInstruction::Normal {opcode: Opcode::Ld, operands: vec![desugar_reg_as_op(&a), desugar_op(b)]},
        Instruction::Define(_, a) => {
            let track_id = state.new_track() as u8;
            let instrument: u8 = a.into();

            UnlinkedInstruction::Define(vec![track_id, instrument])
        },
        Instruction::Lir(_, a) => UnlinkedInstruction::Normal {opcode: Opcode::Add, operands: vec![Operand::Register(Register::Ir), Operand::Immediate(0), desugar_op(a)]},
        Instruction::Stop(_) => UnlinkedInstruction::Normal {opcode: Opcode::Stop, operands: vec![]},
        Instruction::Jg(_, a, b, c) => UnlinkedInstruction::Normal {opcode: Opcode::Jg, operands: vec![desugar_op(a), desugar_op(b), desugar_op(c)]},
        Instruction::Jl(_, a, b, c) => UnlinkedInstruction::Normal {opcode: Opcode::Jl, operands: vec![desugar_op(a), desugar_op(b), desugar_op(c)]},
        Instruction::Jge(_, a, b, c) => UnlinkedInstruction::Normal {opcode: Opcode::Jge, operands: vec![desugar_op(a), desugar_op(b), desugar_op(c)]},
        Instruction::Jle(_, a, b, c) => UnlinkedInstruction::Normal {opcode: Opcode::Jle, operands: vec![desugar_op(a), desugar_op(b), desugar_op(c)]},
        Instruction::Jeq(_, a, b, c) => UnlinkedInstruction::Normal {opcode: Opcode::Jeq, operands: vec![desugar_op(a), desugar_op(b), desugar_op(c)]},
        Instruction::Je(_, a, b, c) => UnlinkedInstruction::Normal {opcode: Opcode::Jeq, operands: vec![desugar_op(a), desugar_op(b), desugar_op(c)]},
        Instruction::Jne(_, a, b, c) => UnlinkedInstruction::Normal {opcode: Opcode::Jne, operands: vec![desugar_op(a), desugar_op(b), desugar_op(c)]},
        Instruction::CallAsync(_, a) => UnlinkedInstruction::Normal { opcode: Opcode::CallAsync, operands: vec![desugar_op(a)] },
    }
}

struct DesugarState {
    tracks: usize
}

impl DesugarState {
    pub fn new() -> Self {
        Self {
            tracks: 0
        }
    }

    pub fn new_track(&mut self) -> usize {
        let track = self.tracks;
        self.tracks += 1;

        track
    }
}

pub fn desugar_ast(ast: <Asm as Language>::Ast) -> UnlinkedInstructionStream {
    let mut desugar_state = DesugarState::new();

    let mut sections = HashMap::new();
    let mut current_section = "song".to_string();
    let mut instructions = Vec::new();

    for i in ast.1 {
        match i {
            LabelOrInstr::Label(_, lbl) => {
                instructions.push(desugar_lbl(lbl));
            }
            LabelOrInstr::Instruction(_, instr) => {
                instructions.push(desugar_instr(instr, &mut desugar_state));
            }
            LabelOrInstr::Combined(_, label, instr) => {
                instructions.push(desugar_lbl(label));
                instructions.push(desugar_instr(instr, &mut desugar_state));
            }
            LabelOrInstr::Section(_, name) => {
                sections.entry(current_section)
                    .or_insert_with(|| UnlinkedSection::default())
                    .instr
                    .extend(instructions);

                current_section = name.1.as_str().to_string();
                instructions = Vec::new();
            }
            LabelOrInstr::Newline(_) => {}
        }
    }

    if instructions.len() > 0 {
        sections.entry(current_section)
            .or_insert_with(|| UnlinkedSection::default())
            .instr
            .extend(instructions)
    }

    UnlinkedInstructionStream {
        sections
    }
}


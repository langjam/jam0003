#![allow(overflowing_literals)]

use std::collections::HashMap;

use crate::parser::{IRData, Value, IR};
use crate::utils::*;

pub type Instructions = Vec<u8>;

pub enum Op {
    NoOp = 0,
    Move = 1,
    Store = 2,
    Add = 3,
    Subtract = 4,
    Multiply = 5,
    Divide = 6,
    Stbg = 7,
    Stps = 8,
    Stcl = 9,
    Strd = 10,
    Cmp = 11,
    Jmp = 12,
    Jeq = 13,
    Jne = 14,
    Jlt = 15,
    Jgt = 16,
    Jle = 17,
    Jge = 18,
    Rect = 19,
    Line = 20,
    Elps = 21,
    Vert = 22,
    Pgon = 23,
}

impl TryFrom<u8> for Op {
    type Error = String;

    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        use Op::*;
        let op = match value {
            0 => NoOp,
            1 => Move,
            2 => Store,
            3 => Add,
            4 => Subtract,
            5 => Multiply,
            6 => Divide,
            7 => Stbg,
            8 => Stps,
            9 => Stcl,
            10 => Strd,
            11 => Cmp,
            12 => Jmp,
            13 => Jeq,
            14 => Jne,
            15 => Jlt,
            16 => Jgt,
            17 => Jle,
            18 => Jge,
            19 => Rect,
            20 => Line,
            21 => Elps,
            22 => Vert,
            23 => Pgon,
            _ => return Err(format!("Invalid Op: {}", value)),
        };

        Ok(op)
    }
}

pub fn compile(ir: Vec<IR>) -> Result<Instructions> {
    let mut c = Compiler::new();

    for i in ir {
        use IRData::*;
        match i.data {
            DefineLabel(label) => {
                let label_dst = c.instructions.len();

                {
                    let mut i = 0;
                    while i < c.unresolved_jumps.len() {
                        let uj = &c.unresolved_jumps[i];
                        if uj.label == label {
                            let jump = (label_dst - uj.instructions_index) as i16;
                            c.instructions[uj.instructions_index] = (jump & 0x00FF) as u8;
                            c.instructions[uj.instructions_index + 1] =
                                ((jump & 0xFF00) >> 8) as u8;

                            c.unresolved_jumps.remove(i);
                        } else {
                            i += 1;
                        }
                    }
                }

                if c.labels.insert(label.clone(), label_dst).is_some() {
                    return Err(Error::new(
                        i.location,
                        format!("Redeclaration of label `{}`.", label),
                    ));
                }
            }
            Move(dx, dy) => {
                c.emit_op(Op::Move);
                c.emit_value(dx);
                c.emit_value(dy);
            }
            Store(reg, value) => {
                c.emit_op(Op::Store);
                c.emit_byte(reg);
                c.emit_value(value);
            }
            Add(reg, a, b) => {
                c.emit_op(Op::Add);
                c.emit_byte(reg);
                c.emit_value(a);
                c.emit_value(b);
            }
            Subtract(reg, a, b) => {
                c.emit_op(Op::Subtract);
                c.emit_byte(reg);
                c.emit_value(a);
                c.emit_value(b);
            }
            Multiply(reg, a, b) => {
                c.emit_op(Op::Multiply);
                c.emit_byte(reg);
                c.emit_value(a);
                c.emit_value(b);
            }
            Divide(reg, a, b) => {
                c.emit_op(Op::Divide);
                c.emit_byte(reg);
                c.emit_value(a);
                c.emit_value(b);
            }
            Stbg(r, g, b) => {
                c.emit_op(Op::Stbg);
                c.emit_value(r);
                c.emit_value(g);
                c.emit_value(b);
            }
            Stps(x, y) => {
                c.emit_op(Op::Stps);
                c.emit_value(x);
                c.emit_value(y);
            }
            Stcl(r, g, b) => {
                c.emit_op(Op::Stcl);
                c.emit_value(r);
                c.emit_value(g);
                c.emit_value(b);
            }
            Strd(radius) => {
                c.emit_op(Op::Strd);
                c.emit_value(radius);
            }
            Cmp(a, b) => {
                c.emit_op(Op::Cmp);
                c.emit_value(a);
                c.emit_value(b);
            }
            Jmp(label) => {
                c.emit_jump(Op::Jmp, label);
            }
            Jeq(label) => {
                c.emit_jump(Op::Jeq, label);
            }
            Jne(label) => {
                c.emit_jump(Op::Jne, label);
            }
            Jlt(label) => {
                c.emit_jump(Op::Jlt, label);
            }
            Jgt(label) => {
                c.emit_jump(Op::Jgt, label);
            }
            Jle(label) => {
                c.emit_jump(Op::Jle, label);
            }
            Jge(label) => {
                c.emit_jump(Op::Jge, label);
            }
            Rect(w, h) => {
                c.emit_op(Op::Rect);
                c.emit_value(w);
                c.emit_value(h);
            }
            Line(w, h) => {
                c.emit_op(Op::Line);
                c.emit_value(w);
                c.emit_value(h);
            }
            Elps(w, h) => {
                c.emit_op(Op::Elps);
                c.emit_value(w);
                c.emit_value(h);
            }
            Vert(x, y) => {
                c.emit_op(Op::Vert);
                c.emit_value(x);
                c.emit_value(y);
            }
            Pgon => {
                c.emit_op(Op::Pgon);
            }
        }
    }

    Ok(c.instructions)
}

struct Compiler {
    labels: HashMap<String, usize>,
    unresolved_jumps: Vec<UnresolvedJump>,
    instructions: Instructions,
}

impl Compiler {
    fn new() -> Self {
        Self {
            labels: Default::default(),
            unresolved_jumps: Default::default(),
            instructions: Default::default(),
        }
    }

    fn emit_op(&mut self, op: Op) {
        let op = op as u8;
        self.instructions.push(op);
    }

    fn emit_byte(&mut self, byte: u8) {
        self.instructions.push(byte);
    }

    fn emit_int(&mut self, value: i16) {
        let bytes = value as u16;
        let byte1 = (bytes & 0x00FF) as u8;
        let byte2 = ((bytes & 0xFF00) >> 8) as u8;

        self.emit_byte(byte1);
        self.emit_byte(byte2);
    }

    fn emit_value(&mut self, value: Value) {
        if value.from_reg {
            self.emit_byte(1);
            self.emit_byte(value.value as u8);
        } else {
            self.emit_byte(0);
            self.emit_int(value.value);
        }
    }

    fn emit_jump(&mut self, jump_op: Op, label: String) {
        self.emit_op(jump_op);
        if let Some(&dst) = self.labels.get(&label) {
            let jump = (dst.wrapping_sub(self.instructions.len())) as i16;
            self.emit_int(jump)
        } else {
            let jmp_dst_idx = self.instructions.len();
            let unresolved = UnresolvedJump {
                label,
                instructions_index: jmp_dst_idx,
            };
            self.unresolved_jumps.push(unresolved);
            self.emit_int(0xBAAD);
        }
    }
}

struct UnresolvedJump {
    label: String,
    instructions_index: usize,
}

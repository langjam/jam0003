use std::collections::HashMap;

use crate::parser::{Value, IR};
use crate::utils::*;

pub type Instructions = Vec<u8>;

pub enum Op {
    NoOp = 0,
    Move = 1,
    Store = 2,
    Add = 3,
    Stbg = 4,
    Stps = 5,
    Stcl = 6,
    Strd = 7,
    Rect = 8,
    Line = 9,
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
            4 => Stbg,
            5 => Stps,
            6 => Stcl,
            7 => Strd,
            8 => Rect,
            9 => Line,
            _ => return Err(format!("Invalid Op: {}", value)),
        };

        Ok(op)
    }
}

pub fn compile(ir: Vec<IR>) -> Result<Instructions> {
    let mut c = Compiler::new();

    for i in ir {
        use IR::*;
        match i {
            DefineLabel(label) => {
                if c.labels
                    .insert(label.clone(), c.instructions.len())
                    .is_some()
                {
                    return Err(Error::new(
                        CodeLocation::new(0, 0),
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
}

struct UnresolvedJump {
    label: String,
    instructions_index: usize,
}

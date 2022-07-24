use crate::value::Value;

#[derive(Debug, Clone)]
pub enum Op {
    Nop,

    Constant(Value),
    Pop,
    Dup,

    Add,
    Sub,
    Mul,
    Div,

    Print,

    Lt,
    Gt,
    Eq,

    Not,
    And,
    Or,

    Label(u8),
    Jmp(u8),
    JmpIf(u8),
}

impl Op {
    pub fn from_op_id(byte: u8) -> Option<Self> {
        match byte {
            0 => Some(Self::Nop),
            2 => Some(Self::Pop),
            3 => Some(Self::Add),
            4 => Some(Self::Sub),
            5 => Some(Self::Mul),
            6 => Some(Self::Div),
            7 => Some(Self::Print),
            11 => Some(Self::Lt),
            12 => Some(Self::Gt),
            13 => Some(Self::Eq),
            14 => Some(Self::Not),
            15 => Some(Self::Or),
            16 => Some(Self::And),
            17 => Some(Self::Dup),
            _ => None,
        }
    }

    pub fn op_id(&self) -> u8 {
        match self {
            Op::Nop => 0,
            Op::Constant(_) => 1,
            Op::Pop => 2,
            Op::Add => 3,
            Op::Sub => 4,
            Op::Mul => 5,
            Op::Div => 6,
            Op::Print => 7,
            Op::Label(_) => 8,
            Op::Jmp(_) => 9,
            Op::JmpIf(_) => 10,
            Op::Lt => 11,
            Op::Gt => 12,
            Op::Eq => 13,
            Op::Not => 14,
            Op::And => 15,
            Op::Or => 16,
            Op::Dup => 17,
        }
    }
}

pub fn encode(ops: &[Op]) -> Vec<u8> {
    let mut result = Vec::new();

    for op in ops {
        result.push(op.op_id());

        match op {
            Op::Constant(value) => {
                result.push(value.type_id());
                match value {
                    Value::Number(number) => {
                        for byte in number.to_le_bytes() {
                            result.push(byte)
                        }
                    }
                    Value::String(string) => {
                        for byte in (string.len() as u64).to_le_bytes() {
                            result.push(byte)
                        }
                        for byte in string.bytes() {
                            result.push(byte)
                        }
                    }
                    Value::Bool(boolean) => {
                        result.push(if *boolean { 1 } else { 0 });
                    }
                }
            }
            Op::Label(id) | Op::Jmp(id) | Op::JmpIf(id) => result.push(*id),
            Op::Pop
            | Op::Nop
            | Op::Add
            | Op::Sub
            | Op::Mul
            | Op::Div
            | Op::Print
            | Op::Lt
            | Op::Gt
            | Op::Eq
            | Op::Not
            | Op::And
            | Op::Or
            | Op::Dup => {}
        }
    }

    result
}

pub fn decode(bytes: &[u8]) -> Vec<Op> {
    let mut result = Vec::<Op>::new();

    let mut cursor = 0;
    while cursor < bytes.len() {
        match bytes[cursor] {
            1 => {
                cursor += 1;
                match bytes[cursor] {
                    0 => {
                        cursor += 1;
                        result.push(Op::Constant(Value::Number(f64::from_le_bytes([
                            bytes[cursor],
                            bytes[cursor + 1],
                            bytes[cursor + 2],
                            bytes[cursor + 3],
                            bytes[cursor + 4],
                            bytes[cursor + 5],
                            bytes[cursor + 6],
                            bytes[cursor + 7],
                        ]))));
                        cursor += 8;
                    }
                    1 => {
                        cursor += 1;
                        let length = u64::from_le_bytes([
                            bytes[cursor],
                            bytes[cursor + 1],
                            bytes[cursor + 2],
                            bytes[cursor + 3],
                            bytes[cursor + 4],
                            bytes[cursor + 5],
                            bytes[cursor + 6],
                            bytes[cursor + 7],
                        ]) as usize;
                        cursor += 8;

                        let mut byte_string = vec![0u8; length];
                        for i in 0..length {
                            byte_string[i] = bytes[cursor + i];
                        }
                        cursor += length;

                        let string = String::from_utf8(byte_string).unwrap();
                        result.push(Op::Constant(Value::String(string)));
                    }
                    2 => {
                        cursor += 1;
                        let boolean = if bytes[cursor] == 1 { true } else { false };
                        result.push(Op::Constant(Value::Bool(boolean)));
                    }
                    _ => todo!(),
                };
            }
            8 | 9 | 10 => {
                cursor += 1;

                result.push(match bytes[cursor - 1] {
                    8 => Op::Label(bytes[cursor]),
                    9 => Op::Jmp(bytes[cursor]),
                    10 => Op::JmpIf(bytes[cursor]),
                    _ => unreachable!(),
                });

                cursor += 1;
            }
            _ => {
                result.push(Op::from_op_id(bytes[cursor]).unwrap());
                cursor += 1;
            }
        }
    }

    result
}

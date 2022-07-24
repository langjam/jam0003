use std::collections::HashMap;

lazy_static! {
    static ref INSTRUCTIONS: HashMap<[InstructionMatch; 9], InstructionType> = HashMap::from([
        (
            [
                InstructionMatch::Exact('*'),
                InstructionMatch::Exact('*'),
                InstructionMatch::Exact('*'),
                InstructionMatch::Exact('*'),
                InstructionMatch::Any,
                InstructionMatch::Exact('*'),
                InstructionMatch::Exact('*'),
                InstructionMatch::Exact('*'),
                InstructionMatch::Exact('*')
            ],
            InstructionType::Value
        ),
        (
            [
                InstructionMatch::Exact('@'),
                InstructionMatch::Exact('@'),
                InstructionMatch::Exact('@'),
                InstructionMatch::Exact('@'),
                InstructionMatch::Any,
                InstructionMatch::Exact('@'),
                InstructionMatch::Exact('@'),
                InstructionMatch::Exact('@'),
                InstructionMatch::Exact('@')
            ],
            InstructionType::Address
        ),
        (
            [
                InstructionMatch::Exact(' '),
                InstructionMatch::Exact('>'),
                InstructionMatch::Exact(' '),
                InstructionMatch::Exact('-'),
                InstructionMatch::Exact('-'),
                InstructionMatch::Exact('>'),
                InstructionMatch::Exact(' '),
                InstructionMatch::Exact('>'),
                InstructionMatch::Exact(' ')
            ],
            InstructionType::Load
        ),
        (
            [
                InstructionMatch::Exact(' '),
                InstructionMatch::Exact('<'),
                InstructionMatch::Exact(' '),
                InstructionMatch::Exact('<'),
                InstructionMatch::Exact('-'),
                InstructionMatch::Exact('-'),
                InstructionMatch::Exact(' '),
                InstructionMatch::Exact('<'),
                InstructionMatch::Exact(' ')
            ],
            InstructionType::Store
        ),
        (
            [
                InstructionMatch::Exact('*'),
                InstructionMatch::Exact(' '),
                InstructionMatch::Exact('*'),
                InstructionMatch::Exact(' '),
                InstructionMatch::Exact('*'),
                InstructionMatch::Exact(' '),
                InstructionMatch::Exact('*'),
                InstructionMatch::Exact(' '),
                InstructionMatch::Exact('*')
            ],
            InstructionType::Mul
        ),
        (
            [
                InstructionMatch::Exact(' '),
                InstructionMatch::Exact(' '),
                InstructionMatch::Exact('/'),
                InstructionMatch::Exact(' '),
                InstructionMatch::Exact('/'),
                InstructionMatch::Exact(' '),
                InstructionMatch::Exact('/'),
                InstructionMatch::Exact(' '),
                InstructionMatch::Exact(' ')
            ],
            InstructionType::Div
        ),
        (
            [
                InstructionMatch::Exact(' '),
                InstructionMatch::Exact('+'),
                InstructionMatch::Exact(' '),
                InstructionMatch::Exact('+'),
                InstructionMatch::Exact('+'),
                InstructionMatch::Exact('+'),
                InstructionMatch::Exact(' '),
                InstructionMatch::Exact('+'),
                InstructionMatch::Exact(' ')
            ],
            InstructionType::Add
        ),
        (
            [
                InstructionMatch::Exact(' '),
                InstructionMatch::Exact(' '),
                InstructionMatch::Exact(' '),
                InstructionMatch::Exact('-'),
                InstructionMatch::Exact('-'),
                InstructionMatch::Exact('-'),
                InstructionMatch::Exact(' '),
                InstructionMatch::Exact(' '),
                InstructionMatch::Exact(' ')
            ],
            InstructionType::Sub
        ),
        (
            [
                InstructionMatch::Exact('*'),
                InstructionMatch::Exact('-'),
                InstructionMatch::Exact('>'),
                InstructionMatch::Exact('*'),
                InstructionMatch::Exact('-'),
                InstructionMatch::Exact('>'),
                InstructionMatch::Exact('*'),
                InstructionMatch::Exact('-'),
                InstructionMatch::Exact('>')
            ],
            InstructionType::Put
        ),
        (
            [
                InstructionMatch::Exact('|'),
                InstructionMatch::Exact('-'),
                InstructionMatch::Exact('|'),
                InstructionMatch::Exact('|'),
                InstructionMatch::Any,
                InstructionMatch::Exact('|'),
                InstructionMatch::Exact('|'),
                InstructionMatch::Exact('-'),
                InstructionMatch::Exact('|')
            ],
            InstructionType::Label
        ),
        (
            [
                InstructionMatch::Exact('@'),
                InstructionMatch::Exact('-'),
                InstructionMatch::Exact('>'),
                InstructionMatch::Exact('@'),
                InstructionMatch::Any,
                InstructionMatch::Exact('>'),
                InstructionMatch::Exact('@'),
                InstructionMatch::Exact('-'),
                InstructionMatch::Exact('>')
            ],
            InstructionType::GotoEq
        ),
        (
            [
                InstructionMatch::Exact('@'),
                InstructionMatch::Exact('-'),
                InstructionMatch::Exact('>'),
                InstructionMatch::Exact('!'),
                InstructionMatch::Any,
                InstructionMatch::Exact('>'),
                InstructionMatch::Exact('@'),
                InstructionMatch::Exact('-'),
                InstructionMatch::Exact('>')
            ],
            InstructionType::GotoNeq
        )
    ]);
}
pub fn lex(input: &String) -> Vec<Vec<Token>> {
    // awful i know
    let lines = input.lines();

    let mut chunks = vec![];
    let mut buffer = vec![];
    for l in lines {
        if !l.is_empty() && !l.contains(";") {
            buffer.push(l);
            if buffer.len() == 3 {
                chunks.push(buffer.clone());
                buffer.clear();
            }
        }
    }

    let mut instructions = vec![];
    for c in chunks {
        let mut tokens = vec![];
        if c.len() % 3 == 0 {
            let mut i = 0;
            while i * 3 < c[0].len() {
                let mut buffer = vec![];
                for y in 0..3 {
                    for x in 0..3 {
                        buffer.push(c[y].chars().nth(x + i * 3));
                    }
                }
                tokens.push(match_instruction(buffer));
                i += 1;
            }
        } else {
            panic!()
        }
        instructions.push(tokens);
    }

    instructions
}
fn match_instruction(input: Vec<Option<char>>) -> Token {
    'outer: for (pattern, instruction) in INSTRUCTIONS.iter() {
        for (i, opt) in input.iter().enumerate() {
            let char = match opt {
                Some(c) => *c,
                None => ' ',
            };
            match &pattern[i] {
                InstructionMatch::Exact(c) => {
                    if char != *c {
                        continue 'outer;
                    }
                }
                InstructionMatch::Any => (),
            }
        }

        match instruction {
            InstructionType::Address => {
                return Token::Value(Value::Address(
                    u8::from_str_radix(&input[4].unwrap().to_string(), 16).unwrap(),
                ));
            }
            InstructionType::Value => {
                return Token::Value(Value::Value(
                    u8::from_str_radix(&input[4].unwrap().to_string(), 16).unwrap(),
                ));
            }
            InstructionType::GotoEq => {
                return Token::GotoEq(
                    u8::from_str_radix(&input[4].unwrap().to_string(), 16).unwrap(),
                );
            }

            InstructionType::GotoNeq => {
                return Token::GotoNeq(
                    u8::from_str_radix(&input[4].unwrap().to_string(), 16).unwrap(),
                );
            }
            InstructionType::Label => {
                return Token::Label(
                    u8::from_str_radix(&input[4].unwrap().to_string(), 16).unwrap(),
                );
            }

            _ => {
                return Token::Call(instruction.clone());
            }
        }
    }

    panic!()
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum InstructionMatch {
    Exact(char),
    Any,
}
#[derive(Debug, Clone)]
pub enum InstructionType {
    Address,
    Value,
    Load,
    Store,
    Label,
    GotoEq,
    GotoNeq,

    Put,
    Mul,
    Add,
    Sub,
    Div,
}

#[derive(Debug, Clone)]
pub enum Token {
    Value(Value),
    Label(u8),
    GotoEq(u8),
    GotoNeq(u8),
    Call(InstructionType),
}

#[derive(Debug, Clone)]
pub enum Value {
    Value(u8),
    Address(u8),
}

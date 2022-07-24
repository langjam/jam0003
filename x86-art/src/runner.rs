use std::collections::{HashMap, VecDeque};

use crate::lexer::{InstructionType, Token, Value};

pub fn run(lines: &Vec<Vec<Token>>) {
    let mut index = 0;

    let mut memory = [0; 16];

    let mut register: VecDeque<u8> = VecDeque::new();

    let mut labels = HashMap::new();
    while index < lines.len() {
        let line = &lines[index];
        let instruction = &line[0];
        match instruction {
            Token::Value(_) => {
                panic!()
            }
            Token::Label(l) => {
                labels.insert(*l, index);
            }
            Token::GotoEq(g) => {
                let val = match &line[1] {
                    Token::Value(v) => match v {
                        Value::Value(i) => *i,
                        Value::Address(i) => memory[*i as usize],
                    },
                    _ => panic!(),
                };
                if val == register[0] {
                    index = *g as usize;
                }
            }
            Token::GotoNeq(g) => {
                let val = match &line[1] {
                    Token::Value(v) => match v {
                        Value::Value(i) => *i,
                        Value::Address(i) => memory[*i as usize],
                    },
                    _ => panic!(),
                };
                if val != register[0] {
                    index = *g as usize;
                }
            }
            Token::Call(c) => match c {
                InstructionType::Value | InstructionType::Address => panic!(),
                InstructionType::Load | InstructionType::Store => {
                    let val = match &line[1] {
                        Token::Value(v) => match v {
                            Value::Value(i) => *i,
                            Value::Address(i) => memory[*i as usize],
                        },
                        _ => panic!(),
                    };
                    match c {
                        InstructionType::Load => register.push_front(val),
                        InstructionType::Store => {
                            memory[val as usize] = register.pop_front().unwrap();
                        }
                        _ => unreachable!(),
                    }
                }
                InstructionType::Put => {
                    println!("{}", register[0]);
                    register.pop_front();
                }
                InstructionType::Mul => {
                    let a = register[0];
                    let b = register[1];
                    register.clear();
                    register.push_front(a * b);
                }
                InstructionType::Div => {
                    let a = register[0];
                    let b = register[1];
                    register.clear();
                    register.push_front(a / b);
                }
                InstructionType::Add => {
                    let a = register[0];
                    let b = register[1];
                    register.clear();
                    register.push_front(a + b);
                }
                InstructionType::Sub => {
                    let a = register[0];
                    let b = register[1];
                    register.clear();
                    register.push_front(a - b);
                }
                _ => panic!(),
            },
        }

        index += 1;
    }
}

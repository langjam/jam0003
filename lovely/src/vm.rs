use std::collections::HashMap;

use crate::{bytecode::Op, value::Value};

struct Executor {
    stack: Vec<Value>,
    ops: Vec<Op>,
    labels: HashMap<u8, usize>,
    cursor: usize,
}

impl Executor {
    fn new(ops: &[Op]) -> Self {
        let mut labels = HashMap::new();

        let ops = ops.to_vec();
        for (i, op) in ops.iter().enumerate() {
            if let Op::Label(id) = op {
                labels.insert(*id, i);
            }
        }

        Self {
            stack: Vec::new(),
            ops,
            labels,
            cursor: 0,
        }
    }

    fn execute_op(&mut self) {
        match &self.ops[self.cursor] {
            Op::Nop => {}
            Op::Constant(value) => self.stack.push(value.clone()),
            Op::Pop => {
                self.stack.pop();
            }
            Op::Dup => {
                if let Some(last) = self.stack.last() {
                    self.stack.push(last.clone())
                }
            }
            Op::Print => {
                let value = self.stack.pop();
                if let Some(value) = value {
                    println!("{}", value.to_string())
                }
            }
            Op::Add => {
                let second = self.stack.pop().unwrap();
                let first = self.stack.pop().unwrap();

                let result = match (&first, &second) {
                    (Value::Number(number1), Value::Number(number2)) => {
                        Value::Number(number1 + number2)
                    }
                    (_, Value::String(string)) => {
                        Value::String(format!("{}{}", first.to_string(), string))
                    }
                    (Value::String(string), _) => {
                        Value::String(format!("{}{}", string, second.to_string()))
                    }
                    _ => panic!(""),
                };

                self.stack.push(result);
            }
            Op::Sub => {
                let second = self.stack.pop().unwrap();
                let first = self.stack.pop().unwrap();

                let result = match (first, second) {
                    (Value::Number(number1), Value::Number(number2)) => {
                        Value::Number(number1 - number2)
                    }
                    _ => panic!(""),
                };

                self.stack.push(result);
            }
            Op::Mul => {
                let second = self.stack.pop().unwrap();
                let first = self.stack.pop().unwrap();

                let result = match (first, second) {
                    (Value::Number(number1), Value::Number(number2)) => {
                        Value::Number(number1 * number2)
                    }
                    (Value::String(string), Value::Number(number)) => {
                        let num_repeated = number.abs().floor() as u64;
                        let strings = vec![string; num_repeated as usize];
                        Value::String(strings.join(""))
                    }
                    _ => panic!(""),
                };

                self.stack.push(result);
            }
            Op::Div => {
                let second = self.stack.pop().unwrap();
                let first = self.stack.pop().unwrap();

                let result = match (first, second) {
                    (Value::Number(number1), Value::Number(number2)) => {
                        Value::Number(number1 / number2)
                    }
                    _ => panic!(""),
                };

                self.stack.push(result);
            }

            Op::Lt => {
                let second = self.stack.pop().unwrap();
                let first = self.stack.pop().unwrap();

                let result = match (first, second) {
                    (Value::Number(number1), Value::Number(number2)) => {
                        Value::Bool(number1 < number2)
                    }
                    _ => panic!(""),
                };

                self.stack.push(result);
            }
            Op::Gt => {
                let second = self.stack.pop().unwrap();
                let first = self.stack.pop().unwrap();

                let result = match (first, second) {
                    (Value::Number(number1), Value::Number(number2)) => {
                        Value::Bool(number1 > number2)
                    }
                    _ => panic!(""),
                };

                self.stack.push(result);
            }
            Op::Eq => {
                let second = self.stack.pop().unwrap();
                let first = self.stack.pop().unwrap();
                let result = Value::Bool(first.eq(&second));
                self.stack.push(result);
            }
            Op::Not => {
                if let Some(Value::Bool(boolean)) = self.stack.pop() {
                    self.stack.push(Value::Bool(!boolean));
                }
            }
            Op::And => {
                let second = self.stack.pop().unwrap();
                let first = self.stack.pop().unwrap();

                let result = match (first, second) {
                    (Value::Bool(bool1), Value::Bool(bool2)) => Value::Bool(bool1 && bool2),
                    _ => panic!(""),
                };

                self.stack.push(result);
            }
            Op::Or => {
                let second = self.stack.pop().unwrap();
                let first = self.stack.pop().unwrap();

                let result = match (first, second) {
                    (Value::Bool(bool1), Value::Bool(bool2)) => Value::Bool(bool1 || bool2),
                    _ => panic!(""),
                };

                self.stack.push(result);
            }

            Op::Jmp(id) => {
                if let Some(idx) = self.labels.get(&id) {
                    self.cursor = *idx;
                    return;
                }
            }
            Op::JmpIf(id) => {
                if let Some(value) = self.stack.pop() {
                    if let Some(idx) = self.labels.get(&id) {
                        if value.truthy() {
                            self.cursor = *idx;
                            return;
                        }
                    }
                }
            }
            Op::Label(_) => {}
        }

        self.cursor += 1;
    }

    fn execute(&mut self) -> Option<Value> {
        while self.cursor < self.ops.len() {
            self.execute_op();
        }

        self.stack.pop()
    }
}

pub fn execute(ops: &[Op]) -> Option<Value> {
    let mut executor = Executor::new(ops);
    executor.execute()
}

use crate::{
    syntax::{Builtin, Machine, Program, Statement, Stream, Value},
    Definition,
};
use std::cell::RefCell;
use std::collections::HashMap;
use std::io::stdin;

use anyhow::{anyhow, bail, Context, Result};
use tracing::info;

/// BAM! execution engine.
pub struct Factory {
    machines: RefCell<HashMap<String, Machine>>,
    streams: RefCell<HashMap<String, Stream>>,
}

impl Factory {
    /// Create a factory given the abstract syntax tree.
    pub fn new(program: Program) -> Self {
        Factory {
            machines: RefCell::new(
                program
                    .machines
                    .into_iter()
                    .map(|m| (m.name.clone(), Machine::Defined(m.body, m.result)))
                    .collect(),
            ),
            streams: RefCell::new(HashMap::new()),
        }
    }

    // Add a named machine to the factory from a definition.
    pub fn bind_definition(&self, name: String, machine: Definition) {
        // TODO: handle Borrow errors.
        self.machines
            .borrow_mut()
            .insert(name, Machine::Defined(machine.body, machine.result));
    }

    /// Perform one step of builtin machine evaluation.
    pub fn run_builtin_machine(&self, builtin: &Builtin, value: Value) -> Result<Value> {
        match builtin {
            Builtin::Add => {
                let (lhs, rhs) = value.to_pair();
                let lhs = lhs.to_num();
                let rhs = rhs.to_num();
                Ok(Value::Num(lhs + rhs))
            }
            Builtin::Sub => {
                let (lhs, rhs) = value.to_pair();
                let lhs = lhs.to_num();
                let rhs = rhs.to_num();
                Ok(Value::Num(lhs - rhs))
            }
            Builtin::Mul => {
                let (lhs, rhs) = value.to_pair();
                let lhs = lhs.to_num();
                let rhs = rhs.to_num();
                Ok(Value::Num(lhs * rhs))
            }
            Builtin::Div => {
                let (lhs, rhs) = value.to_pair();
                let lhs = lhs.to_num();
                let rhs = rhs.to_num();
                if lhs == 0_f64 {
                    bail!("Division by zero")
                } else {
                    Ok(Value::Num(lhs / rhs))
                }
            }
            Builtin::Mod => {
                let (lhs, rhs) = value.to_pair();
                let lhs = lhs.to_num();
                let rhs = rhs.to_num();
                Ok(Value::Num(lhs % rhs))
            }
            Builtin::Pow => {
                let (lhs, rhs) = value.to_pair();
                let lhs = lhs.to_num();
                let rhs = rhs.to_num();
                Ok(Value::Num(lhs.powf(rhs)))
            }
            Builtin::Sqrt => {
                let num = value.to_num();
                Ok(Value::Num(f64::sqrt(num)))
            }
            Builtin::Gt => {
                let (lhs, rhs) = value.to_pair();
                let lhs = lhs.to_num();
                let rhs = rhs.to_num();
                Ok(Value::Bool(lhs > rhs))
            }
            Builtin::Lt => {
                let (lhs, rhs) = value.to_pair();
                let lhs = lhs.to_num();
                let rhs = rhs.to_num();
                Ok(Value::Bool(lhs < rhs))
            }
            Builtin::Eq => {
                let (lhs, rhs) = value.to_pair();
                Ok(Value::Bool(lhs == rhs))
            }
            Builtin::And => {
                let (lhs, rhs) = value.to_pair();
                let lhs = lhs.to_bool();
                let rhs = rhs.to_bool();
                Ok(Value::Bool(lhs && rhs))
            }
            Builtin::Or => {
                let (lhs, rhs) = value.to_pair();
                let lhs = lhs.to_bool();
                let rhs = rhs.to_bool();
                Ok(Value::Bool(lhs || rhs))
            }
            Builtin::Not => Ok(Value::Bool(!value.to_bool())),
            Builtin::Dup2 => Ok(Value::Tuple(vec![value.clone(), value])),
            Builtin::Dup3 => Ok(Value::Tuple(vec![value.clone(), value.clone(), value])),
            Builtin::Print => {
                println!("{}", &value);
                Ok(value)
            }
            Builtin::Read => {
                let mut buf = String::new();
                stdin().read_line(&mut buf);
                Ok(Value::Str(buf))
            }
        }
    }

    /// Run a machine statement, corresponds to one step in the REPL.
    pub fn run_statement(&self, stmt: &mut Statement) -> Result<Option<Value>> {
        match stmt {
            Statement::Let(names, stream) => {
                if names.len() == 1 {
                    self.streams
                        .try_borrow_mut()
                        .map(|mut ss| ss.insert(names.get(0).unwrap().clone(), stream.clone()))
                        .with_context(|| format!("Unable to access streams"))?;
                } else {
                    for (index, name) in names.iter().enumerate() {
                        let stream = Stream::Unzip(Box::new(stream.clone()), index);
                        self.streams
                            .try_borrow_mut()
                            .map(|mut ss| ss.insert(name.clone(), stream))
                            .with_context(|| format!("Unable to access streams"))?;
                    }
                }
                Ok(None)
            }
            Statement::Consume(stream) => Ok(Some(self.advance_stream(stream)?)),
        }
    }

    /// Perform one step of user-supplied machine evaluation.
    pub fn run_defined_machine(
        &self,
        body: &mut [Statement],
        result: &mut Stream,
        value: Value,
    ) -> Result<Value> {
        self.streams
            .borrow_mut()
            .insert("input".to_string(), Stream::Const(value));

        for mut stmt in body {
            self.run_statement(stmt);
        }

        self.advance_stream(result)
    }

    /// Perform one step of a machine's evaluation.
    pub fn run_machine(&self, machine: &mut Machine, value: Value) -> Result<Value> {
        match machine {
            Machine::Var(var) => {
                let mut machines = self.machines.borrow_mut();

                info!("[EVAL] about to pull machine `{}` from the factory.", var);
                info!("[EVAL] factory machines: {:#?}", machines);

                self.run_machine(
                    machines
                        .get_mut(var)
                        .ok_or(anyhow!("Undefined fantastic machine: {}", var))?,
                    value,
                )
            }
            Machine::Builtin(builtin) => self.run_builtin_machine(builtin, value),
            Machine::Defined(body, result) => self.run_defined_machine(body, result, value),
        }
    }

    /// Get the next element from the stream.
    pub fn advance_stream(&self, stream: &mut Stream) -> Result<Value> {
        match stream {
            Stream::Var(var) => {
                let mut streams = self
                    .streams
                    .try_borrow_mut()
                    .with_context(|| format!("Unable to access streams"))?;

                info!("[EVAL] about to pull stream `{}` from the factory.", var);
                info!("[EVAL] factory streams: {:#?}", streams);

                let stream = streams
                    .get_mut(var)
                    .ok_or(anyhow!("Undefined fantastic stream: {}", var))?;

                self.advance_stream(stream)
            }
            Stream::Const(value) => Ok(value.clone()),
            Stream::Pipe(stream, machine) => {
                let value = self.advance_stream(stream)?;
                self.run_machine(machine, value)
            }
            Stream::Zip(streams) => streams
                .iter_mut()
                .map(|s| self.advance_stream(s))
                .collect::<Result<Vec<_>>>()
                .map(Value::Tuple),
            Stream::Unzip(stream, index) => {
                if let Value::Tuple(values) = self.advance_stream(stream)? {
                    values
                        .get(*index)
                        .cloned()
                        .ok_or(bail!("bad index in unzip: {}", *index))
                } else {
                    bail!("called unzip on non-tupled stream: {:?}", stream)
                }
            }
            Stream::Limit(stream, limit) => {
                if *limit == 0 {
                    Ok(Value::Null)
                } else {
                    *limit -= 1;
                    self.advance_stream(stream)
                }
            }
            Stream::Cond(cond_stream, then_stream, else_stream) => {
                if let Value::Bool(cond) = self.advance_stream(cond_stream)? {
                    if cond {
                        self.advance_stream(then_stream)
                    } else {
                        self.advance_stream(else_stream)
                    }
                } else {
                    bail!("non-bool value in conditional")
                }
            }
        }
    }
}

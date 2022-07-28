use crate::{
    syntax::{Builtin, Machine, Program, Statement, Stream, Value},
    Definition,
};
use std::collections::HashMap;
use std::collections::VecDeque;
use std::io::stdin;
use std::rc::Rc;
use std::{borrow::Borrow, cell::RefCell};

use anyhow::{anyhow, bail, Context, Result};
use tracing::info;

/// Execution context of a single machine.
/// Holds local variables.
#[derive(Debug)]
pub struct Task {
    locals: HashMap<String, Stream>,
}

impl Task {
    /// Create a new, empty Task.
    pub fn new(input: &mut Stream) -> Self {
        let mut locals = HashMap::new();

        input.share();
        locals.insert(String::from("input"), input.clone());

        Task { locals }
    }

    /// Bind a stream in the local environment.
    pub fn bind(&mut self, name: String, stream: Stream) {
        self.locals.insert(name, stream);
    }

    /// Resolve a stream from the local environmnt.
    pub fn resolve(&self, name: &str) -> Result<&Stream> {
        self.locals
            .get(name)
            .ok_or(anyhow!("Undefined fantastic (local) stream"))
    }
}

/// BAM! execution engine.
pub struct Factory {
    machines: HashMap<String, Machine>,
    tasks: RefCell<Vec<Task>>,
    current: RefCell<Option<usize>>,
}

impl Factory {
    /// Create a factory given the abstract syntax tree.
    pub fn new(program: Program) -> Self {
        Factory {
            machines: program
                .machines
                .into_iter()
                .map(|m| (m.name.clone(), Machine::Defined(m.body, m.result)))
                .collect(),
            tasks: RefCell::new(Vec::new()),
            current: RefCell::new(None),
        }
    }

    /// Add a named machine to the factory from a definition.
    pub fn bind_definition(&mut self, name: String, machine: Definition) {
        self.machines
            .insert(name, Machine::Defined(machine.body, machine.result));
    }

    /// Get the next element from the stream.
    pub fn advance_stream(&self, stream: &mut Stream) -> Result<Value> {
        match stream {
            Stream::Local(stream, index) => {
                info!(
                    "[EVAL] Advancing Local: '{:#?} at task = {:#?}'",
                    stream, index
                );

                let current = *self.current.borrow();
                *self.current.borrow_mut() = Some(*index);

                let val = self.advance_stream(stream)?;

                *self.current.borrow_mut() = current;

                Ok(val)
            }
            Stream::Var(var) => {
                info!("[EVAL] Advancing Var: '{:#?}'", var);

                // INVARIANT: shared streams like 'input' are always behind Share

                let mut tasks = self.tasks.borrow();
                let current = *self.current.borrow();
                let task = tasks.get(current.unwrap()).ok_or(anyhow!(
                    "Stack pointer {:?} out of bound {:#?}",
                    current,
                    tasks
                ))?;

                match task.resolve(var)? {
                    Stream::Share(s) => {
                        // let mut s = s.try_borrow_mut().with_context(|| {
                        //     format!("Cannot mutate shared stream `{}` because Rust.", var)
                        // })?;
                        let s = unsafe { s.as_ptr().as_mut().unwrap() };
                        self.advance_stream(s)
                    }
                    other => bail!(
                        "Cannot reference non-shared stream: {} = `{:#?}`",
                        var,
                        other
                    ),
                }
            }
            Stream::Const(val) => {
                info!("[EVAL] Advancing Const: '{:#?}'", val);

                Ok(val.clone())
            }
            Stream::Pipe(stream, machine) => {
                info!("[EVAL] Advancing Pipe: {:#?} -> {:#?}", stream, &machine);

                stream.local(self.current.borrow().unwrap());
                stream.share();
                self.run_machine(*machine.clone(), stream)
            }
            Stream::Zip(streams) => {
                info!("[EVAL] Advancing Zip: {:#?}", streams);

                streams
                    .iter_mut()
                    .map(|s| self.advance_stream(s))
                    .collect::<Result<Vec<_>>>()
                    .map(Value::Tuple)
            }
            Stream::Cond(cond_stream, then_stream, else_stream) => {
                info!(
                    "[EVAL] Advancing Cond: if {:#?} then {:#?} else {:#?}",
                    cond_stream, then_stream, else_stream
                );

                let val = self.advance_stream(cond_stream)?;
                info!("[EVAL] Cond recieved {:?} => {}", cond_stream, val.clone());
                if let Value::Bool(cond) = val {
                    if cond {
                        self.advance_stream(then_stream)
                    } else {
                        self.advance_stream(else_stream)
                    }
                } else {
                    bail!("Non-bool value in conditional")
                }
            }
            Stream::Take(stream, limit) => {
                info!("[EVAL] Advancing Take: {:#?}[{limit}]", stream);

                if *limit == 0 {
                    Ok(Value::Null)
                } else {
                    *limit -= 1;
                    self.advance_stream(stream)
                }
            }
            Stream::Peek(stream) => {
                info!("[EVAL] Advancing Peek: {:#?}", stream);

                // INVARIANT: Peek always has a hold behind it, evantually.
                match &mut **stream {
                    // We already created a cache at this level, use it.
                    Stream::Hold(protected_stream, cache) => match cache.back().cloned() {
                        None => {
                            let val = self.advance_stream(protected_stream)?;
                            cache.push_front(val.clone());
                            Ok(val)
                        }
                        Some(val) => Ok(val),
                    },
                    stream => {
                        let val = self.advance_stream(stream)?;
                        let mut cache = VecDeque::new();
                        cache.push_front(val.clone());

                        let old_stream = std::mem::take(stream);
                        /* Nothing should access the stream here, it's Null */
                        let new_stream = Stream::Hold(Box::new(old_stream), cache);
                        std::mem::replace(stream, new_stream);

                        Ok(val)
                    }
                }
            }
            Stream::Proj(stream, index) => {
                info!("Advancing Proj: {:#?}, at {}", stream, index);

                if let Value::Tuple(values) = self.advance_stream(stream)? {
                    info!("Unwrapping {:#?} in Unzip, at {}", values, index);

                    match values.get(*index) {
                        Some(value) => Ok(value.clone()),
                        None => bail!("Bad index in unzip: {}", *index),
                    }
                    // values -- WTF???
                    //     .get(index)
                    //     .ok_or(bail!("Bad index in unzip: {}", index))
                    //     .cloned()
                } else {
                    bail!("Called unzip on non-tupled stream: {:?}", stream)
                }
            }
            Stream::Hold(stream, cache) => {
                info!("Advancing Hold: {:#?} | {:?}", stream, cache);

                // INVARIANT: A Hold with no Peek protecting it invalidates its cache.
                match cache.back().cloned() {
                    None => self.advance_stream(stream),
                    Some(val) => Ok(val),
                }
            }
            Stream::Share(stream) => {
                info!("Advancing Share: {:#?}", stream);

                let mut stream = stream
                    .try_borrow_mut()
                    .context(format!("Could not mutate shared stream"))?;
                self.advance_stream(&mut stream)
            }
        }
    }

    /// Perform one step of a machine's evaluation.
    pub fn run_machine(&self, machine: Machine, input: &mut Stream) -> Result<Value> {
        let val = match machine {
            Machine::Var(var) => self.run_machine(
                self.machines
                    .get(&var)
                    .cloned()
                    .ok_or(anyhow!("Undefined fantastic machine: {}", var))?,
                input,
            ),
            Machine::Builtin(builtin) => self.run_builtin_machine(builtin, input),
            Machine::Defined(body, result) => self.run_defined_machine(body, result, input),
        }?;

        info!("[EVAL] Machine returned: {:#?}", val);
        Ok(val)
    }

    /// Perform one step of builtin machine evaluation.
    pub fn run_builtin_machine(&self, builtin: Builtin, input: &mut Stream) -> Result<Value> {
        info!("Running builtin {:#?} with input = {:#?}", builtin, input);

        let value = self.advance_stream(input)?;
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

    /// Perform one step of user-supplied machine evaluation.
    pub fn run_defined_machine(
        &self,
        mut body: Vec<Statement>,
        mut result: Stream,
        input: &mut Stream,
    ) -> Result<Value> {
        info!(
            "Running machine ({:#?}, {:#?}) with input = {:#?}",
            body, result, input
        );

        let mut task = Task::new();

        // INVARIANT: running a machine always stored its locals on top of the stack.
        task.bind(String::from("input"), input.clone());
        self.tasks.borrow_mut().push(task);
        self.current.borrow_mut().as_mut().map(|x| *x = *x + 1);

        for mut stmt in &mut body {
            match stmt {
                Statement::Let(names, stream) => {
                    let mut tasks = self.tasks.borrow_mut();
                    let index = tasks.len() - 1;
                    let task = tasks.last_mut().ok_or(anyhow!("Empty task stack"))?;

                    // Now to the tricky part.
                    if names.len() == 1 {
                        stream.local(index);
                        stream.share();
                        task.bind(names.get(0).unwrap().clone(), stream.clone());
                    } else {
                        for (index, name) in names.iter().enumerate() {
                            let mut stream = Stream::Proj(Box::new(stream.clone()), index);
                            stream.local(index);
                            stream.share();
                            task.bind(name.clone(), stream);
                        }
                    }
                }
                Statement::Consume(stream) => {
                    let stream = if stream.is_input() {
                        &mut *input
                    } else {
                        stream
                    };
                    self.advance_stream(stream)?;
                }
            }
        }

        let val = self.advance_stream(&mut result);
        self.tasks.borrow_mut().pop();
        self.current.borrow_mut().as_mut().map(|x| *x = *x - 1);
        val
    }
}

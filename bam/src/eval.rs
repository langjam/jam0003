use crate::syntax::{Builtin, Machine, Program, Statement, Stream, Value};
use std::cell::RefCell;
use std::collections::HashMap;

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

    /// Perform one step of builtin machine evaluation.
    pub fn run_builtin_machine(&self, builtin: &Builtin, value: Value) -> Value {
        match builtin {
            Builtin::Add => {
                let (lhs, rhs) = value.to_pair().expect("Error: Add takes two streams.");
                let lhs = lhs
                    .to_num()
                    .expect("Error: First stream to Add should contain Num elements");
                let rhs = rhs
                    .to_num()
                    .expect("Error: Second stream to Add should contain Num elements");
                Value::Num(lhs + rhs)
            }
            Builtin::Mul => {
                let (lhs, rhs) = value.to_pair().expect("Error: Mul takes two streams.");
                let lhs = lhs
                    .to_num()
                    .expect("Error: First stream to Mul should contain Num elements");
                let rhs = rhs
                    .to_num()
                    .expect("Error: Second stream to Mul should contain Num elements");
                Value::Num(lhs * rhs)
            }
            Builtin::Mod => {
                let (lhs, rhs) = value.to_pair().expect("Error: Mod takes two streams.");
                let lhs = lhs
                    .to_num()
                    .expect("Error: First stream to Mod should contain Num elements");
                let rhs = rhs
                    .to_num()
                    .expect("Error: Second stream to Mod should contain Num elements");
                Value::Num(lhs % rhs)
            }
            Builtin::Pow => {
                let (lhs, rhs) = value.to_pair().expect("Error: Pow takes two streams.");
                let lhs = lhs
                    .to_num()
                    .expect("Error: First stream to Pow should contain Num elements");
                let rhs = rhs
                    .to_num()
                    .expect("Error: Second stream to Pow should contain Num elements");
                Value::Num(lhs.powf(rhs))
            }
            Builtin::Sqrt => {
                let num = value.to_num().expect("Error: Sqrt expects one Num stream.");
                Value::Num(f64::sqrt(num))
            }
            Builtin::Gte => {
                let (lhs, rhs) = value.to_pair().expect("Error: Gte takes two streams.");
                let lhs = lhs
                    .to_num()
                    .expect("Error: First stream to Gte should contain Num elements");
                let rhs = rhs
                    .to_num()
                    .expect("Error: Second stream to Gte should contain Num elements");
                Value::Bool(lhs >= rhs)
            }
            Builtin::Lt => {
                let (lhs, rhs) = value.to_pair().expect("Error: Lt takes two streams.");
                let lhs = lhs
                    .to_num()
                    .expect("Error: First stream to Lt should contain Num elements");
                let rhs = rhs
                    .to_num()
                    .expect("Error: Second stream to Lt should contain Num elements");
                Value::Bool(lhs < rhs)
            }
            Builtin::Eq => {
                let (lhs, rhs) = value.to_pair().expect("Error: Eq takes two streams.");
                Value::Bool(lhs == rhs)
            }
            Builtin::Dup2 => Value::Tuple(vec![value.clone(), value]),
            Builtin::Dup3 => Value::Tuple(vec![value.clone(), value.clone(), value]),
            Builtin::Print => {
                println!("{:?}", &value);
                value
            }
        }
    }

    /// Perform one step of user-supplied machine evaluation.
    pub fn run_defined_machine(
        &self,
        body: &[Statement],
        result: &mut Stream,
        value: Value,
    ) -> Value {
        self.streams
            .borrow_mut()
            .insert("input".to_string(), Stream::Const(value));

        for stmt in body {
            match stmt {
                Statement::Let(names, stream) => {
                    for (index, name) in names.iter().enumerate() {
                        let stream = Stream::Unzip(Box::new(stream.clone()), index);
                        self.streams.borrow_mut().insert(name.clone(), stream);
                    }
                }
                Statement::Consume(stream) => {
                    self.advance_stream(&mut stream.clone());
                }
            }
        }

        self.advance_stream(result)
    }

    /// Perform one step of a machine's evaluation.
    pub fn run_machine(&self, machine: &Machine, value: Value) -> Value {
        match machine {
            Machine::Var(var) => {
                let machines = self.machines.borrow();
                self.run_machine(machines.get(var).expect("Error: undefined stream."), value)
            }
            Machine::Builtin(builtin) => self.run_builtin_machine(builtin, value),
            Machine::Defined(body, result) => {
                self.run_defined_machine(body, &mut result.clone(), value)
            }
        }
    }

    /// Get the next element from the stream.
    pub fn advance_stream(&self, stream: &mut Stream) -> Value {
        match stream {
            Stream::Var(var) => self
                .streams
                .borrow()
                .get(var)
                .map(|s| self.advance_stream(&mut s.clone()))
                .expect("Error: undefined stream."),
            Stream::Const(value) => value.clone(),
            Stream::Pipe(stream, machine) => {
                let value = self.advance_stream(&mut stream.clone());
                self.run_machine(machine, value)
            }
            Stream::Zip(streams) => {
                Value::Tuple(streams.iter_mut().map(|s| self.advance_stream(s)).collect())
            }
            Stream::Unzip(stream, index) => {
                if let Value::Tuple(values) = self.advance_stream(stream) {
                    values
                        .get(*index)
                        .expect("Fatal: bad index in unzip.")
                        .clone()
                } else {
                    panic!("Error: unzip on non-tupled stream.")
                }
            }
            Stream::Limit(stream, limit) => {
                if *limit == 0 {
                    Value::Null
                } else {
                    *limit -= 1;
                    self.advance_stream(stream)
                }
            }
            Stream::Cond(cond_stream, then_stream, else_stream) => {
                if let Value::Bool(cond) = self.advance_stream(cond_stream) {
                    if cond {
                        self.advance_stream(then_stream)
                    } else {
                        self.advance_stream(else_stream)
                    }
                } else {
                    panic!("Error: non-bool in conditional")
                }
            }
        }
    }
}

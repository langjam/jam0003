//! BAM! bytecode compiler.

use std::collections::{HashMap, VecDeque};

use crate::{syntax::Value, vm::SharedStream, Builtin, Machine, Program, Statement, Stream};

/// Local handle to a [`Flow`].
#[derive(Copy, Clone)]
pub struct Handle(u32);

/// SSA-like symbol holding a [`Value`].
#[derive(Copy, Clone)]
pub struct Symbol(u32);

/// Global handle to a [`Unit`].
#[derive(Copy, Clone)]
pub struct Number(u32);

/// Compiled version of a stream.
pub enum Flow {
    /// Always return the same value. Boring!
    Const(Value),
    /// Stream of tuples.
    Zip(Vec<Handle>),
    /// Consume from the underlying stream a limited number of times.
    Take(Handle, usize),
    /// Consume from the underlying stream without changing it.
    Peek(Handle),
    /// Run a unit with a stream.
    Pipe(Handle, Number),
    /// Builtin operations.
    Exec(Handle, Builtin),
    /// Projection into a tuple value.
    Proj(Handle, usize), // Value -> Value
    /// Conditional stream.
    Cond(Handle, Handle, Handle),
}

/// Compiled version of a machine.
pub struct Unit(Vec<Instr>);

impl Unit {
    pub fn new() -> Self {
        Self(Vec::new())
    }
}

/// Machine instructions.
pub enum Instr {
    /// Create a shared stream in the current task
    /// and bind it to the given symbol.
    Make(Handle, Flow), // () -> Stream
    /// Consume a value from this stream and drop it (?).
    Next(Handle), // Stream -> Value
    /// Return the given handle.
    Eval(Handle),
}

/// Renaming utility.
pub struct Renamer {
    map: HashMap<String, u32>,
    count: u32,
}

impl Renamer {
    /// Create a renamer.
    fn new() -> Self {
        Self {
            map: HashMap::new(),
            count: 0,
        }
    }

    /// Bind a name in the renamer.
    fn bind(&mut self, name: String) -> u32 {
        match self.map.get(&name).cloned() {
            None => {
                self.count += 1;
                self.map.insert(name, self.count);
                self.count
            }
            Some(sym) => sym,
        }
    }

    /// Get a fresh new symbol.
    fn fresh(&mut self) -> u32 {
        let sym = self.count;
        self.count += 1;
        sym
    }
}

/// Compiler context, and also compiled code representation.
pub struct Code {
    /// Collected units, correspond to machines.
    units: Vec<Unit>,
    /// Symbol renamer.
    symbols: Renamer,
    /// Handle renamer.
    handles: Renamer,
    /// Number renamer.
    numbers: Renamer,
}

impl Code {
    /// Create a compiler.
    pub fn new() -> Self {
        Code {
            units: Vec::new(),
            symbols: Renamer::new(),
            handles: Renamer::new(),
            numbers: Renamer::new(),
        }
    }

    /// Fresh number.
    pub fn number(&mut self) -> Number {
        Number(self.numbers.fresh())
    }

    /// Fresh handle.
    pub fn handle(&mut self) -> Handle {
        Handle(self.handles.fresh())
    }

    /// Fresh symbol.
    pub fn symbol(&mut self) -> Symbol {
        Symbol(self.symbols.fresh())
    }

    /// Transform a syntax tree into a mostly flat representation.
    pub fn transform(&mut self, program: Program) {
        for machine in program.machines {
            self.transform_machine(machine.body, machine.result);
        }
    }

    /// Transform a machine into a unit.
    pub fn transform_machine(&mut self, body: Vec<Statement>, result: Stream) {
        self.units.push(Unit::new());

        for stmt in body {
            self.transform_statement(stmt);
        }

        let handle = self.transform_stream(result);
        self.add_instr(Instr::Eval(handle))
    }

    /// Transform a stream to a flow, pipes get
    pub fn transform_statement(&mut self, stmt: Statement) {
        match stmt {
            // let x, y = stream;
            Statement::Let(mut names, stream) => {
                if names.len() == 1 {
                    let name = names.pop().unwrap();
                    let handle = Handle(self.handles.bind(name));
                    self.transform_stream_with(stream, handle);
                } else {
                    let tuple = self.handle();
                    self.transform_stream_with(stream, tuple);
                    for (index, name) in names.into_iter().enumerate() {
                        let handle = Handle(self.handles.bind(name));
                        self.add_instr(Instr::Make(handle, Flow::Proj(tuple, index)))
                    }
                }
            }
            Statement::Consume(stream) => {
                let handle = self.transform_stream(stream);
                self.add_instr(Instr::Next(handle))
            }
        }
    }

    /// Transform a stream to a flow and give it the supplied handle.
    pub fn transform_stream_with(&mut self, stream: Stream, handle: Handle) {
        match stream {
            Stream::Var(var) => {}
            Stream::Const(val) => {
                self.add_instr(Instr::Make(handle, Flow::Const(val)));
            }
            Stream::Pipe(stream, machine) => {
                let input = self.transform_stream(*stream);
                let flow = match *machine {
                    Machine::Var(var) => Flow::Pipe(input, Number(self.numbers.bind(var))),
                    Machine::Builtin(builtin) => Flow::Exec(input, builtin),
                };

                self.add_instr(Instr::Make(handle, flow));
            }
            Stream::Zip(streams) => {
                let zipped = streams
                    .into_iter()
                    .map(|s| self.transform_stream(s))
                    .collect::<Vec<_>>();

                self.add_instr(Instr::Make(handle, Flow::Zip(zipped)));
            }
            Stream::Cond(cond, then, otherwise) => {
                let cond = self.transform_stream(*cond);
                let then = self.transform_stream(*then);
                let otherwise = self.transform_stream(*otherwise);

                self.add_instr(Instr::Make(handle, Flow::Cond(cond, then, otherwise)))
            }
            Stream::Take(stream, count) => {
                let inner = self.transform_stream(*stream);

                self.add_instr(Instr::Make(handle, Flow::Take(inner, count)))
            }
            Stream::Peek(stream) => {
                let inner = self.transform_stream(*stream);

                self.add_instr(Instr::Make(handle, Flow::Peek(inner)))
            }
        };
    }

    /// Helper function for [`transform_stream_with`].
    pub fn transform_stream(&mut self, stream: Stream) -> Handle {
        let handle = self.handle();
        self.transform_stream_with(stream, handle);
        handle
    }

    /// Add an instruction in the currently compiled unit.
    pub fn add_instr(&mut self, instr: Instr) {
        // TODO(fuzzypixelz): handle errors.
        self.units
            .last_mut()
            .expect("no units in `add_instr`")
            .0
            .push(Instr::Make(Handle(0), Flow::Const(Value::Null)))
    }
}

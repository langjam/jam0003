//! BAM! bytecode compiler.

use std::collections::{HashMap, VecDeque};

use crate::{syntax::Value, vm::SharedStream, Builtin, Program, Statement, Stream};

/// Local handle to a [`FlatStream`].
pub struct Symbol(u32);

/// Compiled version of a stream.
pub enum Flow {
    /// Always return the same value. Boring!
    Const(Value),
    /// Consume from the underlying stream a limited number of times.
    Take(Symbol, u32),
    /// Consume from the underlying stream without changing it.
    Peek(Symbol),
    /// Adds a cache to some stream.
    Cache(Symbol, VecDeque<Value>),
}

/// Compiled version of a machine.
pub enum Unit {
    Instrs(Vec<Instr>),
    Builtin(Builtin),
}

/// Machine instructions.
pub enum Instr {
    /// Create a shared stream in the current task
    /// and bind it to the given symbol.
    Make(Symbol, Flow),
    /// Consume a value from this stream and return it.
    /// INVARIANT: the last element is always of this variant.
    Next(Symbol),
    /// Run a unit with a tuple of streams,
    /// and return a value from it.
    /// FIXME: does the unit need a name?
    Exec(Vec<Symbol>, String),
}

/// Compiled program.
pub struct Code(HashMap<String, Unit>);

/// Compiler context.
pub struct Compiler {
    /// Collected units, correspond to machines.
    units: HashMap<String, Unit>,
    /// Symbol counter, resets to zero after each machine compilation.
    symbols: u32,
}

impl Compiler {
    /// Create a compiler.
    pub fn new() -> Self {
        Compiler {
            units: HashMap::new(),
            symbols: 0,
        }
    }

    /// Transform a syntax tree into a mostly flat representation.
    pub fn transform(&mut self, program: Program) -> Code {
        Code(
            program
                .machines
                .into_iter()
                .map(|m| (m.name, self.transform_machine(m.body, m.result)))
                .collect::<HashMap<_, _>>(),
        )
    }

    /// Transform a machine into a unit.
    pub fn transform_machine(&mut self, body: Vec<Statement>, result: Stream) -> Unit {
        todo!()
    }
}

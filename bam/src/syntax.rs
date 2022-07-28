use core::fmt;
use std::{
    cell::{Cell, RefCell},
    collections::VecDeque,
    ptr::NonNull,
    rc::Rc,
};

#[derive(Debug, Clone, PartialEq)]
pub struct Program {
    pub machines: Vec<Definition>,
}
#[derive(Debug, Clone, PartialEq)]
pub struct Definition {
    pub name: String,
    pub body: Vec<Statement>,
    pub result: Stream,
}
#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    Let(Vec<String>, Stream), // let x, y = s;
    Consume(Stream),          // s;
}
// Id = input
// Five = 5

#[derive(Debug, Clone, PartialEq)]
pub enum Stream {
    Var(String),                                 // x
    Const(Value),                                // v
    Pipe(Box<Stream>, Box<Machine>),             // s -> m
    Zip(Vec<Stream>),                            // s₁ , .. , sₙ
    Cond(Box<Stream>, Box<Stream>, Box<Stream>), // s₁ ? s₂ : s₃
    Take(Box<Stream>, usize),                    // s{n}
    /// Read a value from the underlying stream
    /// without consuming it.
    Peek(Box<Stream>), // !s

    /* Only generated during evaluation. */
    /// Contains the original stream to unzip,
    /// and the index with which to project.
    Proj(Box<Stream>, usize), // let x, y = s

    /// A stream that caches returned values.
    Hold(Box<Stream>, VecDeque<Value>),

    /// A stream that can be sneakily mutated.
    Share(Rc<RefCell<Stream>>),

    /// A Stream plus an index to the task it was created in.
    Local(Box<Stream>, usize),
}

impl Default for Stream {
    fn default() -> Self {
        Stream::Const(Value::Null)
    }
}

impl Stream {
    /// Is this the input stream?
    pub fn is_input(&self) -> bool {
        match self {
            Stream::Var(var) if var.eq("input") => true,
            _ => false,
        }
    }

    /// Transforms a stream in place to be `Share`.
    pub fn share(&mut self) {
        if let Self::Share(_) = self {
            return;
        }
        let old_stream = std::mem::take(self);
        /* Nothing should/would access the stream here, it's Null */
        let new_stream = Self::Share(Rc::new(RefCell::new(old_stream)));
        std::mem::replace(self, new_stream);
    }

    /// Transforms a stream in place to be `Local`.
    pub fn local(&mut self, index: usize) {
        let old_stream = std::mem::take(self);
        /* Nothing should/would access the stream here, it's Null */
        let new_stream = Self::Local(Box::new(old_stream), index);
        std::mem::replace(self, new_stream);
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Machine {
    Var(String),
    Builtin(Builtin),
    /// Only generated during evaluation.
    Defined(Vec<Statement>, Stream),
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum Builtin {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Pow,
    Sqrt,
    Gt,
    Lt,
    Eq,
    And,
    Or,
    Not,
    Dup2,
    Dup3,
    Print,
    Read,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    /// All streams are infinite.
    /// When a stream is empty is keeps returning Null.
    Null,
    Num(f64),
    Str(String),
    Bool(bool),
    Tuple(Vec<Value>),
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Value::*;
        match self {
            Null => write!(f, "null"),
            Num(float) => write!(f, "{}", float),
            Str(string) => write!(f, "{}", string),
            Bool(boolean) => write!(f, "{}", boolean),
            Tuple(values) => write!(
                f,
                "({})",
                values
                    .iter()
                    .map(|s| format!("{s}"))
                    .reduce(|acc, val| acc + val.as_str())
                    .unwrap_or_else(|| "".to_string())
            ),
        }
    }
}

impl Value {
    /// Try to transform into a tuple.
    pub fn to_tuple(self) -> Vec<Value> {
        match self {
            Value::Tuple(t) => t,
            other => panic!("Fatal: expected Tuple in Value, found {other}"),
        }
    }

    /// Try to transform into a pair.
    pub fn to_pair(self) -> (Value, Value) {
        match self {
            Value::Tuple(mut t) if t.len() == 2 => {
                let rhs = t.pop().unwrap();
                let lhs = t.pop().unwrap();
                (lhs, rhs)
            }
            other => panic!("Fatal: expected Pair in Value, found {other}"),
        }
    }

    /// Try to transform into a number.
    pub fn to_num(self) -> f64 {
        match self {
            Value::Num(f) => f,
            other => panic!("Fatal: expected Num in Value, found {other}"),
        }
    }

    /// Try to transform into a boolean.
    pub fn to_bool(self) -> bool {
        match self {
            Value::Bool(b) => b,
            other => panic!("Fatal: expected Bool in Value, found {other}"),
        }
    }
}

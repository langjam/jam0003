#![allow(unused)]

mod ast;
mod error;
mod lexer;
mod parser;
mod util;

pub use ast::{Ast, Expression, Operator, Value};
pub use lexer::{LexerBuilder, StandardToken, Token};

use chumsky::{
    prelude::{end, filter, Simple},
    Parser,
};

pub type Span = std::ops::Range<usize>;
pub type Spanned<T> = (T, Span);

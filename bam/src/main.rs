#![allow(unused)]
use chumsky::Parser;
use tracing::{info, level_filters, Level};

mod eval;
mod lexer;
mod parser;
mod syntax;
mod types;
mod util;

pub use lexer::{LexerBuilder, Token, KEYWORD_MAP};

use crate::{
    parser::ParserBuilder,
    syntax::{Machine, Stream, Value},
};

pub type Span = std::ops::Range<usize>;
pub type Spanned<T> = (T, Span);

// lazy_static! {
//     static ref PROGRAMS: HashMap<ProgramName, &'static str> = hashmap![];
// }

static HELLO_PROGRAM_PATH: &'static str = "../examples/hello.bam";

fn main() {
    tracing_subscriber::fmt::init();

    run_temp()
}

fn run(path: &str) {
    let lexer = LexerBuilder::build();
    let parser = ParserBuilder::build();

    let source = include_str!("../examples/hello.bam").to_owned();
    info!("[SOURCE]: {source}\n");

    info!("[LEXER] begin");
    let tokens = lexer.parse(source).unwrap();
    info!("[LEXER] end");

    let tokens: Vec<Token> = tokens.into_iter().map(|(token, _)| token).collect();
    info!("[TOKENS]: {tokens:#?}\n");

    let program = parser.parse(tokens).unwrap();
    info!("[PROGRAM]: {program:#?}\n");

    match types::check(&program) {
        Ok(()) => (),
        Err(err) => println!("Type Error: {err:#?}"),
    }

    let factory = eval::Factory::new(program);
    let result = factory.advance_stream(&mut Stream::Pipe(
        Stream::Const(Value::Num(42f64)).into(),
        Machine::Var("AddOne".to_string()).into(),
    ));

    info!("[RESULT]: {result:#?}");
}

fn run_temp() {
    let decl_program = vec![
        Token::Let,
        Token::Ident("n".to_owned()),
        Token::Equals,
        Token::FloatLit("1.2".to_owned()),
    ];

    let hello_program = vec![
        Token::Machine,
        Token::Ident("hello".to_owned()),
        Token::StringLit("hello".to_owned()),
        Token::Pipe,
        Token::Ident("Print".to_owned()),
    ];

    // let parser = parser::parser();
    // info!("decl: {:#?}", parser.parse(decl_program).unwrap());
    // info!("hello: {:#?}", parser.parse(hello_program).unwrap());
}

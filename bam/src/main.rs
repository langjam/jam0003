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

pub use crate::{
    parser::ParserBuilder,
    syntax::{Builtin, Definition, Machine, Program, Statement, Stream, Value},
};

pub type Span = std::ops::Range<usize>;
pub type Spanned<T> = (T, Span);

fn main() {
    // tracing_subscriber::fmt::init();
}

fn run(path: &str) {
    let lexer = LexerBuilder::build();
    let parser = ParserBuilder::build();

    let source = include_str!("../examples/hello.bam").to_owned();
    info!("[SOURCE]: {source}\n");

    info!("[LEXER] begin");
    let tokens = lexer.parse(source).unwrap();
    info!("[LEXER] end");

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_hello_world_from_tokens() {
        // machine Main {
        //     let hello = "Hello, BAM!";
        //     hello{1} -> Print
        // };

        let tokens = vec![
            Token::Machine,
            Token::Ident("Main".to_string()),
            Token::Lbrace,
            Token::Let,
            Token::Ident("hello".to_string()),
            Token::Equals,
            Token::StringLit("Hello, BAM!".to_string()),
            Token::Semicolon,
            Token::Ident("hello".to_string()),
            Token::Lbrace,
            Token::IntLit("1".to_string()),
            Token::Rbrace,
            Token::Pipe,
            Token::Ident("Print".to_string()),
            Token::Rbrace,
        ];

        let result = ParserBuilder::build()
            .parse(tokens)
            .map_err(|errs| {
                errs.iter().for_each(|e| eprintln!("Error(parse): {:?}", e));
                errs
            })
            .unwrap();

        use Builtin::*;
        use Statement::*;
        use Stream::*;
        use Value::*;
        assert_eq!(
            result,
            Program {
                machines: vec![Definition {
                    name: "Main".to_string(),
                    body: vec![Let(
                        vec!["hello".to_string()],
                        Const(Str("Hello, BAM!".to_string()))
                    )],
                    result: Pipe(
                        Box::new(Limit(Box::new(Stream::Var("hello".to_string())), 1)),
                        Box::new(Machine::Builtin(Print))
                    )
                }]
            }
        );
    }

    #[test]
    fn parse_hello_world_from_source() {
        let source = "machine Main {
            let hello = \"Hello, BAM!\";
            hello{1} -> Print
        }";

        let tokens = LexerBuilder::build()
            .parse(source)
            .map_err(|errs| {
                errs.iter().for_each(|e| eprintln!("Error(lex): {:?}", e));
                errs
            })
            .unwrap();

        info!("[TOKENS]: {:#?}", &tokens);

        let result = ParserBuilder::build()
            .parse(tokens)
            .map_err(|errs| {
                errs.iter().for_each(|e| eprintln!("Error(parse): {:?}", e));
                errs
            })
            .unwrap();

        use Builtin::*;
        use Statement::*;
        use Stream::*;
        use Value::*;
        assert_eq!(
            result,
            Program {
                machines: vec![Definition {
                    name: "Main".to_string(),
                    body: vec![Let(
                        vec!["hello".to_string()],
                        Const(Str("Hello, BAM!".to_string()))
                    )],
                    result: Pipe(
                        Box::new(Limit(Box::new(Stream::Var("hello".to_string())), 1)),
                        Box::new(Machine::Builtin(Print))
                    )
                }]
            }
        );
    }
}

#![allow(unused)]
use chumsky::Parser;
use clap;
use tracing::{info, level_filters, Level};

mod eval;
mod lexer;
mod parser;
mod syntax;
mod types;
mod util;

pub use lexer::{LexerBuilder, Token, KEYWORD_MAP};
use types::GlobalTypeEnv;

pub use crate::{
    parser::ParserBuilder,
    syntax::{Builtin, Definition, Machine, Program, Statement, Stream, Value},
};

pub type Span = std::ops::Range<usize>;
pub type Spanned<T> = (T, Span);

#[derive(clap::Parser, Debug)]
#[clap(version, about, long_about = None)]
struct Args {
    #[clap(help = "The source file to execute")]
    source: Option<String>,
    #[clap(long, help = "Emable tracing")]
    trace: bool,
}

fn main() {
    let args: Args = clap::Parser::parse();

    if args.trace {
        tracing_subscriber::fmt::init();
    }

    match args.source {
        None => run_repl(),
        Some(source) => run(&source),
    }
}

fn run_repl() {
    let mut rl = rustyline::Editor::<()>::new().unwrap();
    let lexer = LexerBuilder::build();
    let parser = ParserBuilder::build();

    let mut type_env = GlobalTypeEnv::new();
    loop {
        match rl.readline("\x1b[36m*> \x1b[0m") {
            Err(rustyline::error::ReadlineError::Eof) => break,
            Err(err) => panic!("ReadlineError: {err}"),
            Ok(line) => {
                let tokens = lexer.parse(line).unwrap();

                // TODO: Currently, this parses the entire program.
                // Ideally, we should parse a single machine definition or a stream expression.
                match parser.parse(tokens) {
                    Err(errors) => {
                        println!(
                            "\x1b[31mParse Errors:\n{}\x1b[0m",
                            errors.into_iter().map(|err| { err.to_string() }).collect::<Vec<_>>().join("\n")
                        )
                    }
                    Ok(program) => {
                        // This should really be a single machine
                        for machine in &program.machines {
                            match types::check_machine_def(&mut type_env, machine) {
                                Err(err) => {
                                    println!("\x1b[31mType Error: {err}")
                                }
                                Ok(_) => ()
                            }
                        }
                    }
                }
            }
        }
    }
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

    loop {
        let result = factory.advance_stream(&mut Stream::Pipe(
            Stream::Const(Value::Num(42f64)).into(),
            Machine::Var("Main".to_string()).into(),
        ));

        info!("[STEP]: {result:#?}");
    }
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

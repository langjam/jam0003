#![allow(unused)]
use ansi_term::{Colour, Style};
use anyhow::{anyhow, bail, Context, Result};
use chumsky::Parser;
use clap;
use eval::Factory;
use rustyline::{error::ReadlineError, Editor};
use tracing::{info, level_filters, Level};

mod util;
mod lexer;
mod parser;
mod syntax;
mod types;
mod compiler;
// mod eval;
mod vm;

pub use lexer::{LexerBuilder, Token, KEYWORD_MAP};
use types::GlobalTypeEnv;

pub use crate::{
    parser::ParserBuilder,
    syntax::{Builtin, Definition, Machine, Program, Statement, Stream, Value},
};

pub type Span = std::ops::Range<usize>;
pub type Spanned<T> = (T, Span);

const BAM: &str = r#"
         ▄▄▄▄    ▄▄▄       ███▄ ▄███▓ ▐██▌ 
        ▓█████▄ ▒████▄    ▓██▒▀█▀ ██▒ ▐██▌ 
        ▒██▒ ▄██▒██  ▀█▄  ▓██    ▓██░ ▐██▌ 
        ▒██░█▀  ░██▄▄▄▄██ ▒██    ▒██  ▓██▒ 
        ░▓█  ▀█▓ ▓█   ▓██▒▒██▒   ░██▒ ▒▄▄  
        ░▒▓███▀▒ ▒▒   ▓▒█░░ ▒░   ░  ░ ░▀▀▒ 
        ▒░▒   ░   ▒   ▒▒ ░░  ░      ░ ░  ░ 
         ░    ░   ░   ▒   ░      ░       ░ 
         ░            ░  ░       ░    ░    
              ░                            
        "Beautifully Assembled Machines"
"#;

const HELP: &str = r#"
> Type a stream expression and advance through it with Enter
> Use the :d command to define a machine
> Exit all modes, including the REPL, with Ctrl-D
> Happy streaming!
"#;

#[derive(clap::Parser, Debug)]
#[clap(version, about, long_about = None)]
struct Args {
    #[clap(help = "The source file to execute")]
    source: Option<String>,
    #[clap(long, help = "Enable tracing")]
    trace: bool,
    #[clap(long, help = "Don't run the REPL")]
    no_repl: bool,
}

fn main() -> Result<()> {
    let args: Args = clap::Parser::parse();

    if args.trace {
        tracing_subscriber::fmt().with_max_level(Level::INFO).init();
    }

    run_repl(args.source)
}

/// REPL mode.
enum Mode {
    /// If in this mode, accept statements, so things like:
    /// - `let x, y = stream -> Machine`
    /// - `null -> Read -> ToNum -> Sqrt`
    Statement,
    /// If in this mode, accept machine-declarations.
    /// Stores the current line number.
    Definiton(usize),
    /// If in this mode, we're stepping through a stream.
    Streaming(Statement),
}

fn run_repl(filename: Option<String>) -> Result<()> {
    println!("{}", Colour::Purple.bold().paint(BAM));
    println!("{}", Colour::White.bold().paint(HELP));

    let mut rl = Editor::<()>::new().unwrap();

    let mut type_env = GlobalTypeEnv::new();

    let lexer = LexerBuilder::build();
    let (program_parser, statement_parser) = ParserBuilder::build();

    let mut factory = Factory::new(Program { machines: vec![] });

    let mut load = |factory: &mut Factory, source: String| {
        let tokens = lexer.parse(source).unwrap();
        match program_parser.parse(tokens) {
            Err(errors) => {
                let errors = errors
                    .into_iter()
                    .map(|err| err.to_string())
                    .collect::<Vec<_>>()
                    .join("\n");

                eprintln!(
                    "{}",
                    Colour::Red.bold().paint(format!("ParseErrors: {}", errors))
                );
            }
            Ok(program) => {
                // This should really be a single machine
                for machine in program.machines {
                    match types::check_machine_def(&mut type_env, &machine) {
                        Err(err) => {
                            eprintln!("{}", Colour::Red.bold().paint(format!("TypeError: {err}")))
                        }
                        Ok(()) => factory.bind_definition(machine.name.clone(), machine),
                    }
                }
            }
        }
    };

    if let Some(filename) = filename {
        let source = std::fs::read_to_string(&filename)
            .with_context(|| format!("Could not load file `{}`", filename))?;
        load(&mut factory, source);
    }

    let mut mode = Mode::Statement;
    let mut definition_buf = String::new();

    let prompt = |mode: &mut Mode| match mode {
        Mode::Statement => Colour::Blue.bold().paint("bam> ").to_string(),
        Mode::Streaming(_) => Colour::Purple.bold().paint("BAM!> ").to_string(),
        Mode::Definiton(l) => {
            *l += 1;
            Colour::White.bold().paint(format!("{l} | ")).to_string()
        }
    };

    rl.load_history(".history");
    loop {
        let current = rl.readline(&prompt(&mut mode));
        info!("[REPL] handling line: {:?}", &current);
        match current {
            Ok(line) if line.trim().is_empty() => {
                if let Mode::Streaming(ref mut stream) = mode {
                    if let Statement::Consume(s) = stream {
                        match factory.advance_stream(s) {
                            Ok(value) => println!("{}", value),
                            Err(err) => {
                                eprintln!("{}", Colour::Red.italic().paint(format!("{err}")))
                            }
                        };
                    } else {
                        eprintln!(
                            "{}",
                            Colour::Red
                                .italic()
                                .paint("Cannot run let-statements, for now ...")
                        )
                    }
                }
            }
            Ok(line) if line.starts_with(':') => match line.as_str() {
                ":d" | ":define" => {
                    mode = Mode::Definiton(0);
                }
                _ => eprintln!(
                    "{}",
                    Colour::Red
                        .bold()
                        .paint(format!("Unknown command: `{}`", &line[1..]))
                ),
            },
            Ok(line) if matches!(mode, Mode::Definiton(_)) => {
                definition_buf.push_str(&line);
            }
            Ok(line) if matches!(mode, Mode::Statement) => {
                rl.add_history_entry(line.as_str());

                let tokens = lexer.parse(line).unwrap();
                match statement_parser.parse(tokens) {
                    Err(errors) => eprintln!(
                        "{}\n{}",
                        Colour::Red.bold().paint("ParseError:"),
                        errors
                            .into_iter()
                            .map(|err| { err.to_string() })
                            .collect::<Vec<_>>()
                            .join("\n")
                    ),
                    Ok(stream) => mode = Mode::Streaming(stream),
                }
            }
            Err(ReadlineError::Eof) if matches!(mode, Mode::Streaming(_)) => mode = Mode::Statement,
            Err(ReadlineError::Eof) if matches!(mode, Mode::Definiton(_)) => {
                let lines = definition_buf.drain(..).collect::<String>();
                rl.add_history_entry(lines.as_str());
                load(&mut factory, lines);
                mode = Mode::Statement
            }
            Err(ReadlineError::Interrupted) => {
                // Ctrl+C shouldn't crash the REPL
            }
            Err(ReadlineError::Eof) => {
                println!("{}", Style::new().italic().paint("Exiting ..."));
                rl.save_history(".history").is_ok();
                break;
            }
            error => {
                rl.save_history(".history").is_ok();
                error
                    .map(|_| ())
                    .with_context(|| format!("Unexpected error while reading input"))?;
            }
        }
    }

    rl.save_history(".history");
    Ok(())
}

fn run(path: &str) {
    let lexer = LexerBuilder::build();
    let parser = ParserBuilder::build().0;

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

    let mut factory = eval::Factory::new(program);

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
            .0
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
                        Box::new(Take(Box::new(Stream::Var("hello".to_string())), 1)),
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
            .0
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
                        Box::new(Take(Box::new(Stream::Var("hello".to_string())), 1)),
                        Box::new(Machine::Builtin(Print))
                    )
                }]
            }
        );
    }
}

use crate::{
    syntax::{Builtin, Definition, Machine, Program, Statement, Stream, Value},
    Token,
};
use chumsky::{prelude::*, primitive::FilterMap};
use std::ops::Range;
use tracing::info;

pub struct ParserBuilder;

impl ParserBuilder {
    #[inline]
    pub fn build() -> (
        impl Parser<Token, Program, Error = Simple<Token>>,
        impl Parser<Token, Statement, Error = Simple<Token>>,
    ) {
        let stream: Recursive<Token, Stream, _> = recursive(|stream| {
            let stream_leaf = Self::ident()
                .map(|name| Stream::Var(name)) // x
                .or(just(Token::Null).to(Stream::Const(Value::Null))) // null
                .or(Self::float().map(|f| Stream::Const(Value::Num(f)))) // n
                .or(Self::string().map(|s| Stream::Const(Value::Str(s)))) // s
                .or(just(Token::Lparen)
                    .then(stream.clone())
                    .then_ignore(just(Token::Rparen))
                    .map(|(_, s)| s))
                .boxed(); // ( s )

            let limit = stream_leaf
                .clone()
                .then_ignore(just(Token::Lbrace))
                .then(Self::int())
                .then_ignore(just(Token::Rbrace))
                .map(|(stream, count)| Stream::Take(Box::new(stream), count));

            let peek = just(Token::Bang)
                .ignore_then(stream_leaf.clone())
                .map(|stream| Stream::Peek(Box::new(stream)));

            let stream_limit = peek.or(limit.or(stream_leaf.clone())).boxed();

            let stream_zip = stream_limit
                .clone()
                .separated_by(just(Token::Comma))
                .at_least(1)
                .map(|streams| {
                    if streams.len() == 1 {
                        streams[0].clone()
                    } else {
                        Stream::Zip(streams)
                    }
                });

            let cond_ = stream_zip
                .clone()
                .then_ignore(just(Token::QuestionMark))
                .then(stream.clone())
                .then_ignore(just(Token::Colon))
                .then(stream.clone())
                .map(|((pred, then), else_)| {
                    Stream::Cond(Box::new(pred), Box::new(then), Box::new(else_))
                });

            let stream_cond = cond_.or(stream_zip);

            let pipe = stream_cond
                .clone()
                .then_ignore(just(Token::Pipe))
                .then(Self::machine().separated_by(just(Token::Pipe)))
                .map(|(stream, machines)| {
                    machines.into_iter().fold(stream, |stream, machine| {
                        Stream::Pipe(Box::new(stream), Box::new(machine))
                    })
                });

            pipe.or(stream_cond)
        });
        let let_ = just(Token::Let)
            .then(Self::ident().separated_by(just(Token::Comma)))
            .then_ignore(just(Token::Equals))
            .then(stream.clone())
            .map(|((_, vars), stream)| Statement::Let(vars, stream));

        let consume_stream = stream.clone().map(|s| Statement::Consume(s));

        let statement = let_.or(consume_stream).boxed();

        let machine_def = just(Token::Machine)
            .then(Self::ident())
            .then_ignore(just(Token::Lbrace))
            .then(
                statement
                    .clone()
                    .then_ignore(just(Token::Semicolon))
                    .repeated(),
            )
            .then(stream.clone())
            .then_ignore(just(Token::Rbrace))
            .map(|(((_, name), body), result)| Definition { name, body, result });

        let program = machine_def
            .repeated()
            .then_ignore(end())
            .map(|machines| Program { machines });

        (program, statement)
    }

    #[inline]
    fn ident() -> impl Parser<Token, String, Error = Simple<Token>> {
        filter_map(|span: Range<usize>, tok| {
            info!(
                "[PARSER] <{}..{}> parsing identifier: {tok}",
                span.start, span.end
            );

            match tok {
                Token::Ident(ident) => Ok(ident),
                _ => Err(Simple::custom(span, "Expected an identifier")),
            }
        })
    }

    #[inline]
    fn float() -> impl Parser<Token, f64, Error = Simple<Token>> {
        filter_map(|span: Range<usize>, tok| {
            info!(
                "[PARSER] <{}..{}> parsing float: {tok}",
                span.start, span.end
            );

            match tok {
                Token::FloatLit(str) => Ok(str.parse::<f64>().unwrap()),
                Token::IntLit(str) => Ok(str.parse::<f64>().unwrap()),
                _ => Err(Simple::custom(span, "Expected a number literal")),
            }
        })
    }

    #[inline]
    fn int() -> impl Parser<Token, usize, Error = Simple<Token>> {
        filter_map(|span: Range<usize>, tok| {
            info!(
                "[PARSER] <{}..{}> parsing integer: {tok}",
                span.start, span.end
            );

            match tok {
                Token::IntLit(i) => Ok(i.parse::<usize>().unwrap()),
                _ => Err(Simple::custom(span, "Expected a positive integer")),
            }
        })
    }

    #[inline]
    fn string() -> impl Parser<Token, String, Error = Simple<Token>> {
        filter_map(|span: Range<usize>, tok| {
            info!(
                "[PARSER] <{}..{}> parsing string: {tok}",
                span.start, span.end
            );

            match tok {
                Token::StringLit(str) => Ok(str),
                _ => Err(Simple::custom(span, "Expected a string literal")),
            }
        })
    }

    #[inline]
    fn machine() -> impl Parser<Token, Machine, Error = Simple<Token>> {
        filter_map(|span: Range<usize>, tok| {
            info!(
                "[PARSER] <{}..{}> parsing machine: {tok}",
                span.start, span.end
            );

            use Builtin::*;
            match tok {
                Token::Ident(ident) => Ok(match ident.as_str() {
                    "Add" => Machine::Builtin(Add),
                    "Sub" => Machine::Builtin(Sub),
                    "Mul" => Machine::Builtin(Mul),
                    "Div" => Machine::Builtin(Div),
                    "Mod" => Machine::Builtin(Mod),
                    "Pow" => Machine::Builtin(Pow),
                    "Sqrt" => Machine::Builtin(Sqrt),
                    "Gt" => Machine::Builtin(Gt),
                    "Lt" => Machine::Builtin(Lt),
                    "Eq" => Machine::Builtin(Eq),
                    "And" => Machine::Builtin(And),
                    "Or" => Machine::Builtin(Or),
                    "Not" => Machine::Builtin(Not),
                    "Dup2" => Machine::Builtin(Dup2),
                    "Dup3" => Machine::Builtin(Dup3),
                    "Print" => Machine::Builtin(Print),
                    "Read" => Machine::Builtin(Read),
                    _ => Machine::Var(ident),
                }),
                _ => Err(Simple::custom(span, "Expected a machine")),
            }
        })
    }
}

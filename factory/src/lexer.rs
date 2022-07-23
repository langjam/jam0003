use crate::Span;
use chumsky::{
    primitive, recovery,
    text::{self, TextParser},
    Parser,
};
use std::fmt::Display;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum StandardToken {
    Component,
    Pipe,
    Machine,
    Factory,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Token {
    Boolean(bool),
    Number(String),
    String(String),
    Operator(String),
    Identifier(String),
    Control(char),
    Standard(StandardToken),
    If,
    Else,
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let message = match self {
            Token::Boolean(value) => format!("{value}"),
            Token::Number(value)
            | Token::String(value)
            | Token::Operator(value)
            | Token::Identifier(value) => value.to_owned(),
            Token::Control(value) => value.to_string(),
            Token::Standard(value) => match value {
                StandardToken::Component => "component".to_owned(),
                StandardToken::Pipe => "pipe".to_owned(),
                StandardToken::Machine => "machine".to_owned(),
                StandardToken::Factory => "factory".to_owned(),
            },
            Token::If => "if".to_owned(),
            Token::Else => "else".to_owned(),
        };

        write!(f, "{message}")
    }
}

pub struct LexerBuilder;

impl LexerBuilder {
    pub fn build() -> impl Parser<char, Vec<(Token, Span)>, Error = chumsky::error::Simple<char>> {
        let token = Self::number()
            .or(Self::string())
            .or(Self::operator())
            .or(Self::control())
            .or(Self::identifier())
            .recover_with(recovery::skip_then_retry_until([]));

        let comment = primitive::just("//")
            .then(primitive::take_until(primitive::just("\n")))
            .padded();

        token
            .map_with_span(|token, span| (token, span))
            .padded_by(comment.repeated())
            .padded()
            .repeated()
    }

    fn number() -> impl Parser<char, Token, Error = chumsky::error::Simple<char>> {
        text::int(10)
            .chain::<char, _, _>(
                primitive::just('.')
                    .chain(text::digits(10))
                    .or_not()
                    .flatten(),
            )
            .collect::<String>()
            .map(Token::Number)
    }

    fn string() -> impl Parser<char, Token, Error = chumsky::error::Simple<char>> {
        primitive::just('"')
            .ignore_then(primitive::filter(|c: &char| *c != '"').repeated())
            .then_ignore(primitive::just('"'))
            .collect::<String>()
            .map(Token::String)
    }

    fn operator() -> impl Parser<char, Token, Error = chumsky::error::Simple<char>> {
        primitive::one_of("+-*/%!=><:")
            .repeated()
            .at_least(1)
            .collect::<String>()
            .map(Token::Operator)
    }

    fn control() -> impl Parser<char, Token, Error = chumsky::error::Simple<char>> {
        primitive::one_of("()[]{},").map(|c: char| Token::Control(c))
    }

    fn identifier() -> impl Parser<char, Token, Error = chumsky::error::Simple<char>> {
        text::ident().map(|ident: String| match ident.as_str() {
            "component" => Token::Standard(StandardToken::Component),
            "pipe" => Token::Standard(StandardToken::Pipe),
            "machine" => Token::Standard(StandardToken::Machine),
            "factory" => Token::Standard(StandardToken::Factory),
            "if" => Token::If,
            "else" => Token::Else,
            "true" => Token::Boolean(true),
            "false" => Token::Boolean(false),
            _ => Token::Identifier(ident),
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn basic() {
        let source = r#"
component test {
    name: String,
    value: Number,
}
        "#;

        let lexer = LexerBuilder::build();
        let tokens = lexer.parse(source).unwrap();

        println!("{tokens:?}");
    }
}

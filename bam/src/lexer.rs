use crate::{hashmap, Span};
use chumsky::{
    error::Simple,
    prelude::*,
    primitive::*,
    recovery, select,
    text::{self, ident, int, keyword, TextParser},
    Parser,
};
use lazy_static::lazy_static;
use std::{collections::HashMap, fmt::Display};

lazy_static! {
    pub static ref KEYWORD_MAP: HashMap<String, Token> = hashmap!(<String, Token> [
        "=" => Token::Equals,
        "{" => Token::Lbrace,
        "}" => Token::Rbrace,
        "(" => Token::Lparen,
        ")" => Token::Rparen,
        "," => Token::Comma,
        "?" => Token::QuestionMark,
        ":" => Token::Colon,
        ";" => Token::Semicolon,
        "!" => Token::Bang,
        "->" => Token::Pipe,
        "let" => Token::Let,
        "machine" => Token::Machine,
        "null" => Token::Null
    ]);
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum Token {
    Ident(String),
    FloatLit(String),
    IntLit(String),
    StringLit(String),
    Let,
    Equals,
    Lbrace,
    Rbrace,
    Lparen,
    Rparen,
    Comma,
    QuestionMark,
    Colon,
    Semicolon,
    Pipe,
    Machine,
    Null,
    Bang
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let message = match self {
            Self::Ident(value) => format!("Ident: {value}"),
            Self::FloatLit(value) => format!("Float: {value}"),
            Self::IntLit(value) => format!("Int: {value}"),
            Self::StringLit(value) => format!("Str: {value}"),
            value => KEYWORD_MAP
                .iter()
                .find_map(|(key, val)| {
                    if val == value {
                        Some(key.clone())
                    } else {
                        None
                    }
                })
                .unwrap(),
        };

        write!(f, "{message}")
    }
}

pub struct LexerBuilder;

impl LexerBuilder {
    #[inline]
    pub fn build() -> impl Parser<char, Vec<Token>, Error = Simple<char>> {
        let comment = just("//")
            .ignore_then(take_until(just("\n")).ignored())
            .padded()
            .repeated()
            .ignored();

        choice((
            Self::operator(),
            Self::keyword(),
            Self::number(),
            Self::string(),
            Self::identifier(),
        ))
        .padded()
        .padded_by(comment)
        .repeated()
        .then_ignore(end())
    }

    #[inline]
    fn number() -> impl Parser<char, Token, Error = Simple<char>> {
        let radix = 10;

        text::int(radix)
            .chain::<char, _, _>(just('.').chain(text::digits(radix)).or_not().flatten())
            .collect::<String>()
            .map(|value| match value.contains('.') {
                true => Token::FloatLit(value),
                false => Token::IntLit(value),
            })
    }

    #[inline]
    fn string() -> impl Parser<char, Token, Error = Simple<char>> {
        just('"')
            .ignore_then(filter(|c: &char| *c != '"').repeated())
            .then_ignore(just('"'))
            .collect::<String>()
            .map(|value| Token::StringLit(value))
    }

    #[inline]
    fn operator() -> impl Parser<char, Token, Error = Simple<char>> {
        choice((
            just("=").to(Token::Equals),
            just("{").to(Token::Lbrace),
            just("}").to(Token::Rbrace),
            just("(").to(Token::Lparen),
            just(")").to(Token::Rparen),
            just(",").to(Token::Comma),
            just("?").to(Token::QuestionMark),
            just(":").to(Token::Colon),
            just(";").to(Token::Semicolon),
            just("->").to(Token::Pipe),
            just("!").to(Token::Bang),
        ))
    }

    #[inline]
    fn keyword() -> impl Parser<char, Token, Error = Simple<char>> {
        choice((
            keyword("machine").to(Token::Machine),
            keyword("let").to(Token::Let),
            keyword("null").to(Token::Null),
        ))
    }

    #[inline]
    fn identifier() -> impl Parser<char, Token, Error = Simple<char>> {
        ident().map(Token::Ident)
    }
}

use crate::{hashmap, Span};
use chumsky::{
    error::Simple,
    primitive, recovery, select,
    text::{self, TextParser},
    Parser,
};
use lazy_static::lazy_static;
use std::{collections::HashMap, fmt::Display};
use tracing::info;

lazy_static! {
    pub static ref KEYWORD_MAP: HashMap<String, Token> = hashmap!(<String, Token> [
        "=" => Token::Equals,
        "{" => Token::Lbrace,
        "}" => Token::Rbrace,
        "[" => Token::Lbracket,
        "]" => Token::Rbracket,
        "(" => Token::Lparen,
        ")" => Token::Rparen,
        "," => Token::Comma,
        "?" => Token::QuestionMark,
        ":" => Token::Colon,
        ";" => Token::Semicolon,
        "->" => Token::Pipe,
        "let" =>  Token::Let,
        "component" =>  Token::Component,
        "machine" => Token::Machine,
        "factory" => Token::Factory
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
    Lbracket,
    Rbracket,
    Lparen,
    Rparen,
    Comma,
    QuestionMark,
    Colon,
    Semicolon,
    Component,
    Pipe,
    Machine,
    Factory,
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
    pub fn build() -> impl Parser<char, Vec<(Token, Span)>, Error = Simple<char>> {
        let token = Self::number()
            .or(Self::string())
            .or(Self::identifier())
            .recover_with(recovery::skip_then_retry_until([]));

        let comment = primitive::just("//")
            .then(primitive::take_until(primitive::just('\n')))
            .padded();

        token
            .map_with_span(|token, span| (token, span))
            .padded_by(comment.repeated())
            .padded()
            .repeated()
    }

    #[inline]
    fn number() -> impl Parser<char, Token, Error = Simple<char>> {
        let radix = 10;

        text::int(radix)
            .chain::<char, _, _>(
                primitive::just('.')
                    .chain(text::digits(radix))
                    .or_not()
                    .flatten(),
            )
            .collect::<String>()
            .map(|value| {
                info!("[LEXER] parsing number: {value}");

                match value.contains('.') {
                    true => Token::FloatLit(value),
                    false => Token::IntLit(value),
                }
            })
    }

    #[inline]
    fn string() -> impl Parser<char, Token, Error = Simple<char>> {
        primitive::just('"')
            .ignore_then(primitive::filter(|c: &char| *c != '"').repeated())
            .then_ignore(primitive::just('"'))
            .collect::<String>()
            .map(|value| {
                info!("[LEXER] parsing string: {value}");

                Token::StringLit(value)
            })
    }

    #[inline]
    fn identifier() -> impl Parser<char, Token, Error = Simple<char>> {
        text::ident().map(|ident: String| {
            info!("[LEXER] parsing identifier: {ident}");

            match ident.as_str() {
                "=" => Token::Equals,
                "{" => Token::Lbrace,
                "}" => Token::Rbrace,
                "[" => Token::Lbracket,
                "]" => Token::Rbracket,
                "(" => Token::Lparen,
                ")" => Token::Rparen,
                "," => Token::Comma,
                "?" => Token::QuestionMark,
                ":" => Token::Colon,
                ";" => Token::Semicolon,
                "->" => Token::Pipe,
                "let" => Token::Let,
                "component" => Token::Component,
                "machine" => Token::Machine,
                "factory" => Token::Factory,
                _ => Token::Ident(ident),
            }
        })
        // primitive::just("machine")
        //     .to(Token::Machine)
        //     .or(primitive::just("->")
        //         .to(Token::Pipe)
        //         .or(primitive::just("=").to(Token::Equals))
        //         .or(
        //             text::ident().map(|ident: String| match KEYWORD_MAP.get(&ident) {
        //                 Some(token) => token.to_owned(),
        //                 None => Token::Ident(ident),
        //             }),
        //         ))
    }
}

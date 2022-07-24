use anyhow::{anyhow, bail, Result};
use ariadne::{Label, Report, ReportKind, Source};
use combine::{
    between, choice,
    easy::{self, Error, Info},
    eof, many, many1,
    parser::{
        char::{alpha_num, digit, newline, space, tab},
        combinator::spanned,
    },
    satisfy, skip_many,
    stream::span::{self, Span},
    token, ParseError, Parser, Stream,
};

use crate::Tok;
fn tok<Input>() -> impl Parser<Input, Output = Tok>
where
    Input: Stream<Token = char>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
{
    let whitespace = || choice!(space(), tab(), newline()).map(|_| ());
    let ws = || skip_many(whitespace());

    let tid = || {
        many1(choice!(
            alpha_num(),
            token('+'),
            token('.'),
            token('-'),
            token('?'),
            token('<'),
            token('>')
        ))
    };
    let tstring = || between(token('"'), token('"'), many(satisfy(|s| s != '"'))).map(Tok::String);
    let tint = || many1(digit()).map(|x: String| Tok::Int(x.parse().unwrap())); // TODO perhaps don't unwrap
    let tptr = || (token('#'), tid()).map(|(_, id)| Tok::Ptr(id));
    let tpoint = || (token('@'), tid()).map(|(_, id)| Tok::Point(id));
    let tcall = || token('!').map(|_| Tok::Call);
    let tend = || token('~').map(|_| Tok::End);
    // let tstar = || token('*').map(|_| Tok::Star);

    (
        ws(),
        choice!(
            tpoint(),
            tptr(),
            tcall(),
            tend(),
            tint(),
            tstring()
        ),
        ws(),
    )
        .map(|(_, tok, _)| tok)
}

pub fn raw_parser<Input, Q>() -> impl Parser<Input, Output = Vec<Tok>>
where
    Input: Stream<Token = char, Position = Span<Q>>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position>,
    Q: Ord + Clone,
{
    (many(spanned(tok())), eof()).map(|(v, _)| v)
}

pub fn parse_str(buf: &str) -> Result<Vec<Tok>> {
    let result = raw_parser().parse(
        // This type took like 20 minutes±5 to write. Not fun.
        span::Stream::<_, easy::Errors<char, &str, span::Span<_>>>::from(buf),
    );
    match result {
        Ok((toks, _stream)) => Ok(toks),
        Err(raw) => {
            // raw.errors' first element is usually
            // the actual error, and any following elements
            // are suggestions.
            let main_err = raw.errors.first().unwrap();
            let pos = raw.position.map(|p| p.translate_position(buf));

            let mut expected = vec![];
            for e in &raw.errors[1..] {
                match e {
                    Error::Expected(Info::Static(i)) => expected.push(i),
                    _ => {}
                };
            }

            let mut note = "expected ".to_string();
            for (i, e) in expected.iter().enumerate() {
                if expected.len() - 1 == i {
                    note.push_str("or ");
                }
                // TODO: ariadne seems to strip colors
                // so we can't color this! ):
                note.push_str(e);
                if expected.len() - 1 != i {
                    note.push_str(", ");
                }
            }

            let mut err_buf = Vec::with_capacity(4096);
            let start = Report::build(ReportKind::Error, (), main_err.position());
            match main_err {
                Error::Other(really_unexpected) => bail!(really_unexpected.to_string()),
                Error::Unexpected(unexpected) => {
                    start.with_message(format!("unexpected {unexpected}"))
                }
                er => start.with_message(format!("<fallback error formatter> {er:?}")),
            }
            .with_label(Label::new(pos.start..pos.end).with_message(note))
            .finish()
            .write(Source::from(buf), &mut err_buf)?;
            Err(anyhow!(String::from_utf8(err_buf)?))
        }
    }
}

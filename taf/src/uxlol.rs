use std::{cell::RefCell, rc::Weak, borrow::{Borrow, Cow}};

use colored::Colorize;
use combine::{stream::span, easy, Parser, ParseError};
use lazy_static::__Deref;
use rustyline::{
    completion::{Completer},
    highlight::Highlighter,
    hint::Hinter,
    validate::Validator,
    Helper,
};
use tafokr::{Machine, raw_parser};
pub struct RustylineExt {
    weak_machine: Weak<RefCell<Machine>>,
}

impl RustylineExt {
    pub fn new(weak_machine: Weak<RefCell<Machine>>) -> Self { Self { weak_machine } }
}

impl Helper for RustylineExt {}
impl Highlighter for RustylineExt {
    fn highlight<'l>(&self, line: &'l str, _cursor_pos: usize) -> Cow<'l, str> {
        let rc = self.weak_machine.upgrade().unwrap();
        let _machine = rc.deref().borrow();
        // heresy is leaking..
        let parse = raw_parser().parse(
            span::Stream::<_, easy::Errors<char, &str, span::Span<_>>>::from(line),
        );
        // dbg!(&line, &parsed);
        let colored = if let Err(er) = parse {
            let pos = er.position.map(|p| p.translate_position(line));
            // let end = (pos.end + 1).min(line.len() - 1);
            let end = line.len();
            format!("{}{}{}", &line[..pos.start], &line[pos.start..end].red(), &line[end..])
        } else {
            line.to_string()
        };
        Cow::Owned(colored)
    }
    fn highlight_char(&self, _line: &str, _cursor_pos: usize) -> bool {
        true
    }
}
impl Validator for RustylineExt {}
impl Hinter for RustylineExt {
    type Hint = String;
}
impl Completer for RustylineExt {
    type Candidate = String;
    // TODO
}

use crate::{
    evaluator,
    utils::{CodeLocation, Error, Result},
};

#[derive(Debug)]
struct Token {
    location: CodeLocation,
    data: TokenData,
}

#[derive(Debug, PartialEq, Eq)]
enum TokenData {
    Label(String),

    // Values
    Int(i16),
    Reg(u8),

    // Separators
    Comma,
    Colon,

    // Instructions
    Move,
    Store,
    Add,
    Subtract,
    Multiply,
    Divide,
    Stbg,
    Stps,
    Stcl,
    Strd,
    Cmp,
    Jmp,
    Jeq,
    Jne,
    Jlt,
    Jgt,
    Jle,
    Jge,
    Rect,
    Line,
    Elps,
    Vert,
    Pgon,
}

struct Tokenizer {
    source: &'static str,
    index: usize,
    line: usize,
    col: usize,
}

impl Tokenizer {
    fn new(source: &'static str) -> Self {
        Self {
            source,
            index: 0,
            line: 1,
            col: 1,
        }
    }

    fn has_more(&self) -> bool {
        self.index < self.source.len()
    }

    fn advance(&mut self) {
        if self.index == self.source.len() {
            return;
        }

        self.index += 1;
        self.col += 1;

        let c = self.source.as_bytes()[self.index - 1] as char;
        if c == '\n' {
            self.line += 1;
            self.col = 1;
        }
    }

    fn peek_char(&self, n: usize) -> Option<char> {
        let idx = self.index + n;
        if idx >= self.source.len() {
            None
        } else {
            Some(self.source.as_bytes()[idx] as char)
        }
    }

    fn next(&mut self) -> Result<Option<Token>> {
        self.skip_whitespace();

        let c = if let Some(c) = self.peek_char(0) {
            c
        } else {
            return Ok(None);
        };

        let token: Token;
        if c == ',' {
            token = Token {
                location: CodeLocation::new(self.line, self.col),
                data: TokenData::Comma,
            };
            self.advance();
        } else if c == ':' {
            token = Token {
                location: CodeLocation::new(self.line, self.col),
                data: TokenData::Colon,
            };
            self.advance();
        } else if c.is_ascii_digit()
            || (c == '-' && self.peek_char(1).filter(char::is_ascii_digit).is_some())
        {
            token = self.tokenize_integer()?;
        } else if c.is_ascii_alphabetic() {
            token = self.tokenize_symbol();
        } else {
            return Err(Error::new(
                CodeLocation::new(self.line, self.col),
                format!("What the frick is this!!! `{}`", c),
            ));
        }

        Ok(Some(token))
    }

    fn skip_whitespace(&mut self) {
        loop {
            match self.peek_char(0) {
                Some(';') => {
                    while self.peek_char(0).filter(|&c| c != '\n').is_some() && self.has_more() {
                        self.advance();
                    }
                }
                Some(c) if c.is_ascii_whitespace() => self.advance(),
                None => break,
                _ => break,
            }
        }
    }

    fn tokenize_integer(&mut self) -> Result<Token> {
        let c0 = self.col;

        let mut word = String::new();

        if self.peek_char(0).expect("Expected a character") == '-' {
            word.push('-');
            self.advance();
        }

        while self.peek_char(0).filter(char::is_ascii_digit).is_some() && self.has_more() {
            word.push(self.peek_char(0).expect("Expected a digit character."));
            self.advance();
        }

        let int: i16 = word.parse().or(Err(Error::new(
            CodeLocation::new(self.line, c0),
            "Integer literal too large!",
        )))?;

        Ok(Token {
            location: CodeLocation::new(self.line, c0),
            data: TokenData::Int(int),
        })
    }

    fn tokenize_symbol(&mut self) -> Token {
        let c0 = self.col;

        let mut word = String::new();
        while self
            .peek_char(0)
            .filter(char::is_ascii_alphanumeric)
            .is_some()
            && self.has_more()
        {
            word.push(
                self.peek_char(0)
                    .expect("Expected an alphanumeric character."),
            );
            self.advance();
        }

        if word.len() > 1
            && word.starts_with('r')
            && word.chars().skip(1).all(|c| c.is_ascii_digit())
        {
            let reg_num_str = word
                .get(1..)
                .expect("Expected numeric part of register identifier.");
            let reg_num = reg_num_str.parse().expect("Expected all ascii digits.");

            Token {
                location: CodeLocation::new(self.line, c0),
                data: TokenData::Reg(reg_num),
            }
        } else {
            let data = match word.as_str() {
                "move" => TokenData::Move,
                "store" => TokenData::Store,
                "add" => TokenData::Add,
                "sub" => TokenData::Subtract,
                "mul" => TokenData::Multiply,
                "div" => TokenData::Divide,
                "stbg" => TokenData::Stbg,
                "stps" => TokenData::Stps,
                "stcl" => TokenData::Stcl,
                "strd" => TokenData::Strd,
                "cmp" => TokenData::Cmp,
                "jmp" => TokenData::Jmp,
                "jeq" => TokenData::Jeq,
                "jne" => TokenData::Jne,
                "jlt" => TokenData::Jlt,
                "jgt" => TokenData::Jgt,
                "jle" => TokenData::Jle,
                "jge" => TokenData::Jge,
                "rect" => TokenData::Rect,
                "line" => TokenData::Line,
                "elps" => TokenData::Elps,
                "vert" => TokenData::Vert,
                "pgon" => TokenData::Pgon,
                _ => TokenData::Label(word),
            };

            Token {
                location: CodeLocation::new(self.line, c0),
                data,
            }
        }
    }
}

#[derive(Debug)]
pub struct Value {
    pub from_reg: bool,
    pub value: i16,
}

#[derive(Debug)]
pub struct IR {
    pub location: CodeLocation,
    pub data: IRData,
}

impl IR {
    fn new(location: CodeLocation, data: IRData) -> Self {
        Self { location, data }
    }
}

#[derive(Debug)]
pub enum IRData {
    DefineLabel(String),
    Move(Value, Value),
    Store(u8, Value),
    Add(u8, Value, Value),
    Subtract(u8, Value, Value),
    Multiply(u8, Value, Value),
    Divide(u8, Value, Value),
    Stbg(Value, Value, Value),
    Stps(Value, Value),
    Stcl(Value, Value, Value),
    Strd(Value),
    Cmp(Value, Value),
    Jmp(String),
    Jeq(String),
    Jne(String),
    Jlt(String),
    Jgt(String),
    Jle(String),
    Jge(String),
    Rect(Value, Value),
    Line(Value, Value),
    Elps(Value, Value),
    Vert(Value, Value),
    Pgon,
}

pub fn parse(source: &'static str) -> Result<Vec<IR>> {
    let mut t = Tokenizer::new(source);

    let mut ir = vec![];

    while let Some(token) = t.next()? {
        use TokenData::*;
        match token.data {
            Label(label) => {
                eat(&mut t, TokenData::Colon, "Expected a `:` after label.")?;
                ir.push(IR::new(token.location, IRData::DefineLabel(label)));
            }
            Int(_) => {
                return Err(Error::new(
                    token.location,
                    "Didn't expect this random integer.",
                ))
            }
            Reg(_) => {
                return Err(Error::new(
                    token.location,
                    "Didn't expect this random register name.",
                ))
            }
            Comma => {
                return Err(Error::new(
                    token.location,
                    "Didn't expect this random comma.",
                ))
            }
            Colon => {
                return Err(Error::new(
                    token.location,
                    "Didn't expect this random colon.",
                ))
            }
            Move => {
                let dx = parse_value(&mut t)?;
                eat(&mut t, TokenData::Comma, "Expected a `,`.")?;

                let dy = parse_value(&mut t)?;

                ir.push(IR::new(token.location, IRData::Move(dx, dy)));
            }
            Store => {
                let reg = parse_register_name(&mut t)?;
                eat(&mut t, TokenData::Comma, "Expected a `,`.")?;
                let value = parse_value(&mut t)?;
                ir.push(IR::new(token.location, IRData::Store(reg, value)));
            }
            Add => {
                let reg = parse_register_name(&mut t)?;
                eat(&mut t, TokenData::Comma, "Expected a `,`.")?;

                let a = parse_value(&mut t)?;
                eat(&mut t, TokenData::Comma, "Expected a `,`.")?;

                let b = parse_value(&mut t)?;

                ir.push(IR::new(token.location, IRData::Add(reg, a, b)));
            }
            Subtract => {
                let reg = parse_register_name(&mut t)?;
                eat(&mut t, TokenData::Comma, "Expected a `,`.")?;

                let a = parse_value(&mut t)?;
                eat(&mut t, TokenData::Comma, "Expected a `,`.")?;

                let b = parse_value(&mut t)?;

                ir.push(IR::new(token.location, IRData::Subtract(reg, a, b)));
            }
            Multiply => {
                let reg = parse_register_name(&mut t)?;
                eat(&mut t, TokenData::Comma, "Expected a `,`.")?;

                let a = parse_value(&mut t)?;
                eat(&mut t, TokenData::Comma, "Expected a `,`.")?;

                let b = parse_value(&mut t)?;

                ir.push(IR::new(token.location, IRData::Multiply(reg, a, b)));
            }
            Divide => {
                let reg = parse_register_name(&mut t)?;
                eat(&mut t, TokenData::Comma, "Expected a `,`.")?;

                let a = parse_value(&mut t)?;
                eat(&mut t, TokenData::Comma, "Expected a `,`.")?;

                let b = parse_value(&mut t)?;

                ir.push(IR::new(token.location, IRData::Divide(reg, a, b)));
            }
            Stbg => {
                let r = parse_value(&mut t)?;
                eat(&mut t, TokenData::Comma, "Expected a `,`.")?;

                let g = parse_value(&mut t)?;
                eat(&mut t, TokenData::Comma, "Expected a `,`.")?;

                let b = parse_value(&mut t)?;

                ir.push(IR::new(token.location, IRData::Stbg(r, g, b)));
            }
            Stps => {
                let x = parse_value(&mut t)?;
                eat(&mut t, TokenData::Comma, "Expected a `,`.")?;

                let y = parse_value(&mut t)?;

                ir.push(IR::new(token.location, IRData::Stps(x, y)));
            }
            Stcl => {
                let r = parse_value(&mut t)?;
                eat(&mut t, TokenData::Comma, "Expected a `,`.")?;

                let g = parse_value(&mut t)?;
                eat(&mut t, TokenData::Comma, "Expected a `,`.")?;

                let b = parse_value(&mut t)?;

                ir.push(IR::new(token.location, IRData::Stcl(r, g, b)));
            }
            Strd => {
                let r = parse_value(&mut t)?;
                ir.push(IR::new(token.location, IRData::Strd(r)));
            }
            Cmp => {
                let a = parse_value(&mut t)?;
                eat(&mut t, TokenData::Comma, "Expected a `,`.")?;

                let b = parse_value(&mut t)?;

                ir.push(IR::new(token.location, IRData::Cmp(a, b)));
            }
            Jmp => {
                let dst = parse_label_name(&mut t)?;
                ir.push(IR::new(token.location, IRData::Jmp(dst)));
            }
            Jeq => {
                let dst = parse_label_name(&mut t)?;
                ir.push(IR::new(token.location, IRData::Jeq(dst)));
            }
            Jne => {
                let dst = parse_label_name(&mut t)?;
                ir.push(IR::new(token.location, IRData::Jne(dst)));
            }
            Jlt => {
                let dst = parse_label_name(&mut t)?;
                ir.push(IR::new(token.location, IRData::Jlt(dst)));
            }
            Jgt => {
                let dst = parse_label_name(&mut t)?;
                ir.push(IR::new(token.location, IRData::Jgt(dst)));
            }
            Jle => {
                let dst = parse_label_name(&mut t)?;
                ir.push(IR::new(token.location, IRData::Jle(dst)));
            }
            Jge => {
                let dst = parse_label_name(&mut t)?;
                ir.push(IR::new(token.location, IRData::Jge(dst)));
            }
            Rect => {
                let w = parse_value(&mut t)?;
                eat(&mut t, TokenData::Comma, "Expected a `,`.")?;

                let h = parse_value(&mut t)?;

                ir.push(IR::new(token.location, IRData::Rect(w, h)));
            }
            Line => {
                let w = parse_value(&mut t)?;
                eat(&mut t, TokenData::Comma, "Expected a `,`.")?;

                let h = parse_value(&mut t)?;

                ir.push(IR::new(token.location, IRData::Line(w, h)));
            }
            Elps => {
                let w = parse_value(&mut t)?;
                eat(&mut t, TokenData::Comma, "Expected a `,`.")?;

                let h = parse_value(&mut t)?;

                ir.push(IR::new(token.location, IRData::Elps(w, h)));
            }
            Vert => {
                let x = parse_value(&mut t)?;
                eat(&mut t, TokenData::Comma, "Expected a `,`.")?;

                let y = parse_value(&mut t)?;

                ir.push(IR::new(token.location, IRData::Vert(x, y)));
            }
            Pgon => {
                ir.push(IR::new(token.location, IRData::Pgon));
            }
        }
    }

    Ok(ir)
}

fn eat(t: &mut Tokenizer, expected: TokenData, err: impl Into<String>) -> Result<()> {
    let opt_token = t.next()?;
    let (data, location) = opt_token
        .map(|t| (Some(t.data), t.location))
        .unwrap_or((None, CodeLocation::new(t.line, t.col)));

    data.filter(|d| *d == expected)
        .ok_or(Error::new(location, err.into()))?;

    Ok(())
}

fn parse_register_name(t: &mut Tokenizer) -> Result<u8> {
    let opt_token = t.next()?;
    let (data, location) = opt_token
        .map(|t| (Some(t.data), t.location))
        .unwrap_or((None, CodeLocation::new(t.line, t.col)));

    let reg = data
        .map(|d| {
            if let TokenData::Reg(reg) = d {
                Some(reg)
            } else {
                None
            }
        })
        .flatten()
        .ok_or(Error::new(location, "Expected a register name."))?;

    if reg as usize >= evaluator::NUM_REGISTERS {
        return Err(Error::new(location, "Invalid register name! r0-r15."));
    }

    Ok(reg)
}

fn parse_value(t: &mut Tokenizer) -> Result<Value> {
    let opt_token = t.next()?;
    let (data, location) = opt_token
        .map(|t| (Some(t.data), t.location))
        .unwrap_or((None, CodeLocation::new(t.line, t.col)));

    let value = data
        .map(|d| match d {
            TokenData::Int(int) => Some(Value {
                from_reg: false,
                value: int as i16,
            }),
            TokenData::Reg(reg) => Some(Value {
                from_reg: true,
                value: reg as i16,
            }),
            _ => None,
        })
        .flatten()
        .ok_or(Error::new(
            location,
            "Expected either an integer value or a register name.",
        ))?;

    Ok(value)
}

fn parse_label_name(t: &mut Tokenizer) -> Result<String> {
    let opt_token = t.next()?;
    let (data, location) = opt_token
        .map(|t| (Some(t.data), t.location))
        .unwrap_or((None, CodeLocation::new(t.line, t.col)));

    let label = data
        .map(|d| match d {
            TokenData::Label(l) => Some(l),
            _ => None,
        })
        .flatten()
        .ok_or(Error::new(location, "Expected a label name."))?;

    Ok(label)
}

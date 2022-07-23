#[derive(Debug)]
pub struct CodeLocation {
    line: usize,
    c0: usize,
    c1: usize,
}

impl CodeLocation {
    pub fn new(line: usize, c0: usize, c1: usize) -> Self {
        Self { line, c0, c1 }
    }
}

#[derive(Debug)]
struct Token {
    location: CodeLocation,
    data: TokenData,
}

#[derive(Debug)]
enum TokenData {
    Label(String),

    // Values
    Int(i16),
    Reg(usize),

    // Separators
    Comma,
    Colon,

    // Instructions
    Stbg,
    Stps,
    Stcl,
    Rect,
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

    fn next_char(&mut self) -> Option<char> {
        let c = self.peek_char(0)?;
        self.advance();

        Some(c)
    }

    fn next(&mut self) -> Result<Option<Token>, String> {
        self.skip_whitespace();

        let c = if let Some(c) = self.peek_char(0) {
            c
        } else {
            return Ok(None);
        };

        let token: Token;
        if c == ',' {
            token = Token {
                location: CodeLocation::new(self.line, self.col, self.col),
                data: TokenData::Comma,
            };
            self.advance();
        } else if c == ':' {
            token = Token {
                location: CodeLocation::new(self.line, self.col, self.col),
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
            return Err(format!("What the frick is this!!! `{}`", c));
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

    fn tokenize_integer(&mut self) -> Result<Token, String> {
        let c0 = self.col;

        let mut word = String::new();
        while self.peek_char(0).filter(char::is_ascii_digit).is_some() && self.has_more() {
            word.push(self.peek_char(0).expect("Expected a digit character."));
            self.advance();
        }

        let int: i16 = word
            .parse()
            .or(Err("Integer literal too large!".to_string()))?;

        Ok(Token {
            location: CodeLocation::new(self.line, c0, self.col),
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
                location: CodeLocation::new(self.line, c0, self.col),
                data: TokenData::Reg(reg_num),
            }
        } else {
            let data = match word.as_str() {
                "stbg" => TokenData::Stbg,
                "stps" => TokenData::Stps,
                "stcl" => TokenData::Stcl,
                "rect" => TokenData::Rect,
                _ => TokenData::Label(word),
            };

            Token {
                location: CodeLocation::new(self.line, c0, self.col),
                data,
            }
        }
    }
}

pub fn parse(source: &'static str) -> Result<(), String> {
    let mut t = Tokenizer::new(source);

    while let Some(token) = t.next()? {
        println!("{:?}", token);
    }

    Ok(())
}

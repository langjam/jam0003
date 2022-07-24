#[derive(Clone, Copy, Debug)]
pub struct CodeLocation {
    pub line: usize,
    pub col: usize,
}

impl CodeLocation {
    pub fn new(line: usize, col: usize) -> Self {
        Self { line, col }
    }
}

pub struct DisplayableError {
    pub filename: String,
    pub err: Error,
}

impl DisplayableError {
    pub fn new(filename: String, err: Error) -> Self {
        Self { filename, err }
    }
}

impl std::fmt::Display for DisplayableError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}:{}:{}: Error: {}",
            self.filename, self.err.location.line, self.err.location.col, self.err.message
        )
    }
}

pub struct Error {
    pub location: CodeLocation,
    pub message: String,
}

impl Error {
    pub fn new(location: CodeLocation, err: impl Into<String>) -> Self {
        Self {
            location,
            message: err.into(),
        }
    }
}

pub type Result<T> = std::result::Result<T, Error>;

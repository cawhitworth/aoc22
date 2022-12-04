use core::fmt;

#[derive(Debug)]
pub struct Error {
    e: String,
}

break here

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("{}", self.e))
    }
}

impl std::error::Error for Error {}

impl Error {
    pub fn new(e: String) -> Self {
        Error { e }
    }
}

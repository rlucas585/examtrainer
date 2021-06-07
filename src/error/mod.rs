use std::fmt;
use std::io;

#[derive(Debug)]
pub enum Error {
    Read(String),
    Parse(String),
}

impl std::error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Read(e) => write!(f, "Error reading config file: {}", e),
            Error::Parse(e) => write!(f, "Error parsing config file: {}", e),
        }
    }
}

impl From<&str> for Error {
    fn from(input: &str) -> Self {
        Error::Parse(input.to_string())
    }
}

impl From<String> for Error {
    fn from(input: String) -> Self {
        input.as_str().into()
    }
}

impl From<io::Error> for Error {
    fn from(input: io::Error) -> Self {
        Error::Read(input.to_string())
    }
}

impl From<toml::de::Error> for Error {
    fn from(input: toml::de::Error) -> Self {
        Error::Parse(input.to_string())
    }
}

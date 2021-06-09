use std::fmt;
use std::io;

use crate::config::ConfigError;
use crate::question::QuestionError;

#[derive(Debug)]
pub enum Error {
    Config(ConfigError),
    IO(io::Error),
    Question(QuestionError),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Config(config_error) => write!(f, "Config Error: {}", config_error),
            Self::IO(io_e) => write!(f, "IO Error: {}", io_e),
            Self::Question(q_e) => write!(f, "Question Error: {}", q_e),
        }
    }
}

impl From<io::Error> for Error {
    fn from(input: io::Error) -> Error {
        Error::IO(input)
    }
}

impl From<ConfigError> for Error {
    fn from(input: ConfigError) -> Error {
        Error::Config(input)
    }
}

impl From<QuestionError> for Error {
    fn from(input: QuestionError) -> Error {
        match input {
            QuestionError::IO(io_e) => Error::IO(io_e),
            _ => Error::Question(input),
        }
    }
}

impl std::error::Error for Error {}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn config_errors() {
        assert_eq!(
            ConfigError::NoHomeDirectory.to_string(),
            "Home directory could not be found"
        );
        assert_eq!(
            ConfigError::NoConfigDirectory.to_string(),
            ".config/ directory was not found in $HOME"
        );
        assert_eq!(
            Error::Config(ConfigError::NoConfigDirectory).to_string(),
            "Config Error: .config/ directory was not found in $HOME"
        );
    }
}
// TODO Use this for Config Documentation

// A valid configuration file must contain the following:
// * `submit_directory` - The directory to submit exercises when sitting an exam
// * `module_directory `

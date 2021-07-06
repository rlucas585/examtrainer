use std::fmt;
use std::io;
use std::sync::PoisonError;

use crate::config::ConfigError;
use crate::exam::ExamError;
use crate::question::QuestionError;

#[derive(Debug)]
pub enum Error {
    Config(ConfigError),
    IO(io::Error),
    Question(QuestionError),
    General(String),
    Exam(ExamError),
    ConcurrencyError,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Config(config_error) => write!(f, "Config Error: {}", config_error),
            Self::IO(io_e) => write!(f, "IO Error: {}", io_e),
            Self::Question(q_e) => write!(f, "Question Error: {}", q_e),
            Self::General(e) => write!(f, "Error: {}", e),
            Self::Exam(e_e) => write!(f, "Exam Error: {}", e_e),
            Self::ConcurrencyError => write!(f, "Threading Error"),
        }
    }
}

impl From<io::Error> for Error {
    fn from(input: io::Error) -> Error {
        Error::IO(input)
    }
}

impl<T> From<PoisonError<T>> for Error {
    fn from(_: PoisonError<T>) -> Error {
        Error::ConcurrencyError
    }
}

impl From<ConfigError> for Error {
    fn from(input: ConfigError) -> Error {
        match input {
            ConfigError::IO(io_e) => Error::IO(io_e),
            _ => Error::Config(input),
        }
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

impl From<ExamError> for Error {
    fn from(input: ExamError) -> Self {
        match input {
            ExamError::IO(io_e) => Self::IO(io_e),
            _ => Self::Exam(input),
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

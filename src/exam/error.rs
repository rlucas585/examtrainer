use std::fmt;
use std::io;

/// `LevelError` denotes errors when reading/parsing the `levels` section of an `Exam` .toml file
#[derive(Debug)]
pub enum LevelError {
    InvalidType(String),
    NoQuestions,
    NoPoints,
    MissingQuestion(String),
}

impl fmt::Display for LevelError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::InvalidType(i) => write!(f, "Invalid level type: {}", i),
            Self::NoQuestions => write!(f, "Each level must have at least one question name"),
            Self::NoPoints => write!(f, "Each level must have at least points value"),
            Self::MissingQuestion(q) => write!(f, "Question \'{}\' missing", q),
        }
    }
}

impl std::error::Error for LevelError {}

/// `ExamError` denotes errors that can occur when reading/parsing an `Exam` .toml file
#[derive(Debug)]
pub enum ExamError {
    InvalidTime,
    InvalidGrade,
    InvalidLevel(usize, LevelError),
    NoLevels,
    NotToml,
    InvalidExamFile(toml_parse::de::Error),
    IO(io::Error),
}

impl fmt::Display for ExamError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::InvalidTime => write!(f, "Invalid time value"),
            Self::InvalidGrade => write!(f, "Invalid grades value"),
            Self::InvalidLevel(index, e) => write!(f, "Level {} error: {}", index, e),
            Self::NoLevels => write!(f, "An exam must have at least one level"),
            Self::NotToml => write!(f, "File in exam directory is not a toml file"),
            Self::InvalidExamFile(toml_e) => write!(f, "Error parsing exam: {}", toml_e),
            Self::IO(io_e) => write!(f, "IO Error: {}", io_e),
        }
    }
}

impl From<io::Error> for ExamError {
    fn from(input: io::Error) -> Self {
        Self::IO(input)
    }
}

impl From<toml_parse::de::Error> for ExamError {
    fn from(input: toml_parse::de::Error) -> Self {
        Self::InvalidExamFile(input)
    }
}

impl std::error::Error for ExamError {}

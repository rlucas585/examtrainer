use std::fmt;
use std::io;
use std::sync::{MutexGuard, PoisonError};

/// `QuestionError` denotes errors that can occur when reading/parsing a `Question` .toml file.
#[derive(Debug)]
pub enum QuestionError {
    InvalidTestType(String),
    InvalidSubmissionType(String),
    InvalidTestFile(toml_parse::de::Error),
    MismatchedQuestion(String, String), // TODO: use Enum instead of String in future
    NoSubject,
    NoStdout,
    NoStderr,
    MissingKey(MissingKeys),
    InvalidFramework(String),
    MultipleConfigs,
    NoConfig,
    DuplicateQuestion(String),
    IO(io::Error),
    Mutex,
}

impl fmt::Display for QuestionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::InvalidTestType(i) => write!(f, "Invalid test type: {}", i),
            Self::InvalidSubmissionType(s) => write!(f, "Invalid submission type: {}", s),
            Self::InvalidTestFile(toml_e) => write!(f, "Error parsing question: {}", toml_e),
            Self::MismatchedQuestion(sub, test) => write!(
                f,
                "Submission type {} cannot be used together with Question type {}",
                sub, test
            ),
            Self::NoSubject => write!(f, "Subject directory not found in Question"),
            Self::NoStdout => write!(f, "Expected Stdout file does not exist"),
            Self::NoStderr => write!(f, "Expected Stderr file does not exist"),
            Self::MissingKey(e) => write!(f, "Missing key: {}", e),
            Self::InvalidFramework(frame) => write!(f, "Invalid framework: {}", frame),
            Self::MultipleConfigs => {
                write!(f, "Multiple .toml files were found in Question directory")
            }
            Self::NoConfig => write!(f, "No config file found in Question directory"),
            Self::DuplicateQuestion(name) => write!(
                f,
                "The question {} appeared twice, second instance ignored",
                name
            ),
            Self::IO(io_e) => write!(f, "IO Error: {}", io_e),
            Self::Mutex => write!(f, "Poisoned Mutex error"),
        }
    }
}

impl From<io::Error> for QuestionError {
    fn from(input: io::Error) -> Self {
        Self::IO(input)
    }
}

impl From<toml_parse::de::Error> for QuestionError {
    fn from(input: toml_parse::de::Error) -> Self {
        Self::InvalidTestFile(input)
    }
}

impl From<MissingKeys> for QuestionError {
    fn from(input: MissingKeys) -> Self {
        Self::MissingKey(input)
    }
}

impl<T> From<PoisonError<MutexGuard<'_, T>>> for QuestionError {
    fn from(_: PoisonError<MutexGuard<T>>) -> Self {
        Self::Mutex
    }
}

impl std::error::Error for QuestionError {}

#[derive(Debug)]
pub enum MissingKeys {
    Exec,
    UnitTest,
    Sources,
    CompiledTogether,
    SubExec,
    SubSources,
}

impl fmt::Display for MissingKeys {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Exec => write!(
                f,
                "'executable' type question must contain the following keys:\n- binary\n- args",
            ),
            Self::UnitTest => write!(
                f,
                "'unit-test' type question must contain the following keys:\n- sources\n- compiler",
            ),
            Self::Sources => write!(
                f,
                "'sources' type question must contain the following keys:\n- sources\n- compiler",
            ),
            Self::CompiledTogether => write!(
                f,
                "'expected-output' type question must contain the following keys:
- sources\n- compiler\n- expected_stdout\n- expected_stderr\n- args",
            ),
            Self::SubExec => write!(
                f,
                "'executable' type submission must contain the following key:\n- binary",
            ),
            Self::SubSources => write!(
                f,
                "'sources' type submission must contain the following keys:\n- sources\n- compiler",
            ),
        }
    }
}

impl std::error::Error for MissingKeys {}

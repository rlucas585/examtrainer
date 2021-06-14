use std::fmt;
use std::io;

/// `ConfigError` denotes errors that can occur due to reading/parsing an examtrainer `Config`
/// .toml file.
#[derive(Debug)]
pub enum ConfigError {
    NoHomeDirectory,
    NoConfigDirectory,
    NoExamTrainerDirectory,
    ConfigFileNotFound,
    InvalidConfigFile(toml_parse::de::Error),
    NoQuestionDirectory(String),
    NoExamDirectory(String),
    NoSubjectDirectory(String),
    IO(io::Error),
    InvalidFramework,
    InvalidFrameworkDir(String),
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::NoHomeDirectory => write!(f, "Home directory could not be found"),
            Self::NoConfigDirectory => write!(f, ".config/ directory was not found in $HOME"),
            Self::NoExamTrainerDirectory => {
                write!(f, "Could not find examtrainer/ directory in $HOME/.config/")
            }
            Self::ConfigFileNotFound => write!(f, "Unable to find/open config file"),
            Self::InvalidConfigFile(toml_e) => write!(f, "Error parsing config: {}", toml_e),
            Self::NoQuestionDirectory(d) => {
                write!(f, "Question directory '{}' could not be found", d)
            }
            Self::NoExamDirectory(d) => write!(f, "Exam directory '{}' could not be found", d),
            Self::NoSubjectDirectory(d) => {
                write!(f, "Subject directory '{}' could not be found", d)
            }
            Self::IO(io_e) => write!(f, "IO Error: {}", io_e),
            Self::InvalidFramework => write!(f, "Framework values must start with '-l' or '-L'"),
            Self::InvalidFrameworkDir(dir) => {
                write!(f, "Invalid framework directory in Config: {}", dir)
            }
        }
    }
}

impl From<io::Error> for ConfigError {
    fn from(input: io::Error) -> Self {
        Self::IO(input)
    }
}

impl From<toml_parse::de::Error> for ConfigError {
    fn from(input: toml_parse::de::Error) -> Self {
        ConfigError::InvalidConfigFile(input)
    }
}

impl std::error::Error for ConfigError {}

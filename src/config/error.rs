use std::fmt;

/// `ConfigError` denotes errors that can occur due to reading/parsing an examtrainer `Config`
/// .toml file.
#[derive(Debug)]
pub enum ConfigError {
    NoHomeDirectory,
    NoConfigDirectory,
    NoExamTrainerDirectory,
    InvalidConfigFile(toml::de::Error),
    NoQuestionDirectory(String),
    NoExamDirectory(String),
    NoSubjectDirectory(String),
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::NoHomeDirectory => write!(f, "Home directory could not be found"),
            Self::NoConfigDirectory => write!(f, ".config/ directory was not found in $HOME"),
            Self::NoExamTrainerDirectory => {
                write!(f, "Could not find examtrainer/ directory in $HOME/.config/")
            }
            Self::InvalidConfigFile(toml_e) => write!(f, "Error parsing config: {}", toml_e),
            Self::NoQuestionDirectory(d) => {
                write!(f, "Question directory '{}' could not be found", d)
            }
            Self::NoExamDirectory(d) => write!(f, "Exam directory '{}' could not be found", d),
            Self::NoSubjectDirectory(d) => {
                write!(f, "Subject directory '{}' could not be found", d)
            }
        }
    }
}

impl From<toml::de::Error> for ConfigError {
    fn from(input: toml::de::Error) -> ConfigError {
        ConfigError::InvalidConfigFile(input)
    }
}

impl std::error::Error for ConfigError {}

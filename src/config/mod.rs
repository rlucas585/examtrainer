//! Configuration settings for [`examtrainer`]
//
//! A valid configuration file must contain the following:
//! * `submit_directory` - The directory to submit exercises when sitting an exam
//! * `question_directory` - Directory where questions are located
//! * `exam_directory` - Directory where exams are located
//! * `subject_directory` - Directory where the subjects of assignments will be sent
//!
//! Below is a valid config file:
//! ```toml
//! [directories]
//! submit_directory = "/home/rlucas/rendu"
//! question_directory = "/home/rlucas/.config/examtrainer/questions"
//! exam_directory = "/home/rlucas/.config/examtrainer/exams"
//! subject_directory = "/home/rlucas/subjects"
//! ```

pub mod error;
mod frameworks;
mod toml;

pub use error::ConfigError;

use frameworks::FrameworkManager;

#[derive(Debug)]
struct Directories {
    submit_directory: String,
    question_directory: String,
    exam_directory: String,
    subject_directory: String,
}

impl From<toml::Directories> for Directories {
    fn from(input: toml::Directories) -> Self {
        Self {
            submit_directory: input.submit_directory,
            question_directory: input.question_directory,
            exam_directory: input.exam_directory,
            subject_directory: input.subject_directory,
        }
    }
}

#[derive(Debug)]
pub struct Config {
    directories: Directories,
    frameworks: FrameworkManager,
}

impl Config {
    /// Parse config file to create a [`Config`] for `examtrainer`
    ///
    /// [`Config::new`] will search for a config.toml file inside of `$HOME/.config/examtrainer/`.
    /// If it is not found, or either of the two required directories within (exams and questions)
    /// are not found, the user will be prompted to create them.
    ///
    /// This function returns either `Ok([`Config`])`, or `Err([`ConfigError`])`.
    pub fn new() -> Result<Self, ConfigError> {
        let config_toml = toml::Config::new()?;
        Self::new_internal(config_toml)
    }

    pub fn new_from(config_path: &str) -> Result<Self, ConfigError> {
        let config_toml = toml::Config::new_from(config_path)?;
        Self::new_internal(config_toml)
    }

    fn new_internal(config_toml: toml::Config) -> Result<Self, ConfigError> {
        let directories = config_toml.directories.into();
        let frameworks = FrameworkManager::new(config_toml.frameworks)?;
        Ok(Self {
            directories,
            frameworks,
        })
    }

    pub fn submit_dir(&self) -> &str {
        &self.directories.submit_directory
    }
    pub fn question_dir(&self) -> &str {
        &self.directories.question_directory
    }
    pub fn exam_dir(&self) -> &str {
        &self.directories.exam_directory
    }
    pub fn subject_directory(&self) -> &str {
        &self.directories.subject_directory
    }
    pub fn get_framework(&self, name: &str) -> Option<&Vec<String>> {
        self.frameworks.get(name)
    }
}

// Config is difficult to test, as parts are interactive (asking the user if they want to create
// directories, etc.)
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn initialize_config() -> Result<(), ConfigError> {
        let config = Config::new_from("tst/resources/config_1.toml")?;
        assert_eq!(config.directories.submit_directory, "/home/rlucas/rendu");
        assert_eq!(
            config.directories.question_directory,
            "/home/rlucas/.config/examtrainer/questions"
        );
        assert_eq!(
            config.directories.exam_directory,
            "/home/rlucas/.config/examtrainer/exams"
        );
        assert_eq!(
            config.directories.subject_directory,
            "/home/rlucas/subjects"
        );
        Ok(())
    }

    #[test]
    fn initialize_test_config() -> Result<(), ConfigError> {
        let config = Config::new_from("tst/resources/test_config1.toml")?;
        assert_eq!(
            config.directories.submit_directory,
            "tst/resources/rendu_test"
        );
        assert_eq!(
            config.directories.question_directory,
            "tst/resources/questions"
        );
        assert_eq!(config.directories.exam_directory, "tst/resources/exams");
        assert_eq!(
            config.directories.subject_directory,
            "tst/resources/subjects"
        );
        Ok(())
    }

    #[test]
    #[ignore] // Ignore as framework path should be system dependent
    fn framework_test() -> Result<(), ConfigError> {
        let config = Config::new_from("tst/resources/test_config2.toml")?;
        let gtest_flags = config.get_framework("gtest");
        assert!(gtest_flags.is_some());
        let gtest_flags = gtest_flags.unwrap();
        assert_eq!(
            *gtest_flags,
            vec![
                "-lgtest".to_string(),
                "-lpthread".to_string(),
                "-L/mnt/hard_drive/usr/lib/googletest/build/lib".to_string()
            ]
        );
        Ok(())
    }

    #[test]
    #[ignore]
    fn invalid_framework_test() -> Result<(), ConfigError> {
        let config_res = Config::new_from("tst/resources/invalid_framework_config1.toml");
        assert!(config_res.is_err());
        match config_res.unwrap_err() {
            ConfigError::InvalidFrameworkDir(s) => assert_eq!(s, "/home/rlucas/googletest"),
            _ => panic!("Incorrect enum result for invalid framework_test"),
        }
        Ok(())
    }

    #[test]
    fn invalid_framework_test_no_prefix() -> Result<(), ConfigError> {
        let config_res = Config::new_from("tst/resources/invalid_framework_config2.toml");
        assert!(config_res.is_err());
        match config_res.unwrap_err() {
            ConfigError::InvalidFramework => Ok(()),
            _ => panic!("Incorrect enum result for invalid framework_test"),
        }
    }
}

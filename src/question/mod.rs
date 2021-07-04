mod binary_runner;
mod compiler;
pub mod database;
pub mod error;
mod submission;
pub mod test;
mod toml;
mod trace;

pub use binary_runner::{run_binary_with_args, BinaryResult};
pub use database::QuestionDB;
pub use error::QuestionError;
pub use trace::Trace;

use crate::config::Config;
use crate::question::error::MissingKeys;
use crate::question::test::TestResult;
use crate::utils::Range;
use colored::*;
use std::fmt;
use std::fs::DirEntry;
use std::path::Path;
use submission::Submission;
use test::Test;

#[derive(Debug)]
pub struct QuestionDirs {
    pub submit_directory: String,
    pub question_directory: String,
    pub subject_directory: String,
}

#[derive(Debug)]
pub struct Question {
    name: String,
    description: Option<String>,
    difficulty: u32,
    directories: QuestionDirs,
    submission: Submission,
    test: Test,
}

impl Question {
    pub fn build_from_toml(
        config: &Config,
        toml: toml::Question,
        dir_path: &str,
    ) -> Result<Self, QuestionError> {
        Question::check_type_validity(&toml)?;
        let name = toml.info.name;
        let submit_directory = format!("{}/{}", config.submit_dir(), name);
        let question_directory = dir_path.to_string();
        let subject_directory =
            Self::validate_subject_directory(&question_directory, &toml.test.subject)?;

        let test: Test = Test::build_from_toml(toml.test, dir_path)?;
        test.invalid_framework(config)
            .map_err(QuestionError::InvalidFramework)?;

        let submission: Submission = Submission::build_from_toml(toml.submission)?;

        Ok(Self {
            name,
            description: toml.info.description,
            difficulty: toml.info.difficulty,
            directories: QuestionDirs {
                submit_directory,
                question_directory,
                subject_directory,
            },
            submission,
            test,
        })
    }

    pub fn directories(&self) -> &QuestionDirs {
        &self.directories
    }

    pub fn build_from_dir_entry(
        config: &Config,
        question_dir: &DirEntry,
    ) -> Result<Self, QuestionError> {
        let dir_path = question_dir.path();
        let mut question_opt = None;
        let files = std::fs::read_dir(&dir_path)?;
        for file in files {
            let file = file?;
            if let Some(extension) = file.path().extension() {
                if extension == "toml" {
                    if question_opt.is_some() {
                        return Err(QuestionError::MultipleConfigs);
                    }
                    let buffer = std::fs::read_to_string(file.path())?;
                    let toml: toml::Question = toml_parse::from_str(&buffer)?;
                    let question =
                        Question::build_from_toml(&config, toml, &dir_path.to_str().unwrap())?;
                    question_opt = Some(question);
                }
            }
        }
        question_opt.ok_or(QuestionError::NoConfig)
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn difficulty(&self) -> u32 {
        self.difficulty
    }

    pub fn has_difficulty_in_range(&self, range: &Range) -> bool {
        range.contains(self.difficulty)
    }

    fn validate_subject_directory(
        question_dir: &str,
        subject_dir: &str,
    ) -> Result<String, QuestionError> {
        let subject_directory = format!("{}/{}", question_dir, subject_dir);
        let path = Path::new(&subject_directory);
        match path.exists() {
            true => (),
            false => return Err(QuestionError::NoSubject),
        }
        match path.is_dir() {
            true => Ok(subject_directory),
            false => Err(QuestionError::NoSubject),
        }
    }

    /// Check that the submission type and test type are valid together - a submission type of
    /// 'executable' is incompatible with a test type of 'unit-test' for example.
    ///
    /// Returns a Result<(), QuestionError>, where [`QuestionError`] will be the
    /// [`QuestionError::MismatchedQuestion`] variant.
    ///
    /// If additional types are added in future, then additional validation may be required here.
    fn check_type_validity(toml: &toml::Question) -> Result<(), QuestionError> {
        match &toml.test.test_type[..] {
            "executable" => {
                if toml.submission.submission_type == "sources"
                    && toml.submission.compiler.is_none()
                {
                    Err(QuestionError::MissingKey(MissingKeys::SubSources))
                } else {
                    Ok(())
                }
            }
            "unit-test" => {
                if toml.submission.submission_type != "sources" {
                    Err(QuestionError::MismatchedQuestion(
                        toml.test.test_type.clone(),
                        toml.submission.submission_type.clone(),
                    ))
                } else {
                    Ok(())
                }
            }
            "sources" => Ok(()),
            "expected-output" => Ok(()),
            invalid => Err(QuestionError::InvalidTestType(invalid.into())),
        }
    }

    pub fn grade(&self, config: &Config) -> Result<TestResult, QuestionError> {
        self.test.run(&self.submission, &self.directories, config)
    }
}

impl fmt::Display for Question {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(description) = &self.description {
            write!(f, "{} - {}", format!("{}", self.name).green(), description)
        } else {
            write!(f, "{}", format!("{}", self.name).green())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::Error;
    use crate::question::test::TestError;
    use crate::question::QuestionDB;
    use std::fs;
    #[test]
    fn question_no_subject() -> Result<(), Error> {
        let config = Config::new_from("tst/resources/test_config1.toml")?;
        let dir_path = format!("{}/{}", config.question_dir(), "no_sub_question");
        let file = format!("{}/{}", dir_path, "hello_world.toml");
        let buffer = fs::read_to_string(file)?;
        let toml: toml::Question =
            toml_parse::from_str(&buffer).map_err(|e| Error::Question(e.into()))?;
        let question_res = Question::build_from_toml(&config, toml, &dir_path);
        assert!(question_res.is_err());
        assert!(matches!(
            question_res.unwrap_err(),
            QuestionError::NoSubject
        ));
        Ok(())
    }

    #[test]
    fn build_valid_question() -> Result<(), Error> {
        let config = Config::new_from("tst/resources/test_config1.toml")?;
        let dir_path = format!("{}/{}", config.question_dir(), "hello_world");
        let file = format!("{}/{}", dir_path, "hello_world.toml");
        let buffer = fs::read_to_string(file)?;
        let toml: toml::Question =
            toml_parse::from_str(&buffer).map_err(|e| Error::Question(e.into()))?;
        let question_res = Question::build_from_toml(&config, toml, &dir_path);
        assert!(question_res.is_ok());
        let question = question_res?;
        assert_eq!(question.name(), "hello_world");
        assert_eq!(
            question.directories().submit_directory,
            "tst/resources/rendu_test/hello_world"
        );
        assert_eq!(
            question.directories().question_directory,
            "tst/resources/questions/hello_world"
        );
        assert_eq!(
            question.directories().subject_directory,
            "tst/resources/questions/hello_world/hello_world.subject"
        );
        assert!(matches!(question.test, Test::CompiledTogether(_)));
        assert!(question.has_difficulty_in_range(&Range::new(0, 4)?));
        assert!(!question.has_difficulty_in_range(&Range::new(5, 10)?));
        Ok(())
    }

    #[test]
    fn build_from_dir_entry() -> Result<(), Error> {
        let config = Config::new_from("tst/resources/test_config1.toml")?;
        let question_dirs = std::fs::read_dir(&config.question_dir())?;
        let mut dir_entry_opt: Option<DirEntry> = None;
        for dir in question_dirs.into_iter() {
            if let Ok(dir) = dir {
                if dir_entry_opt.is_none() && dir.path().to_str().unwrap().contains("hello_world") {
                    dir_entry_opt = Some(dir);
                }
            }
        }
        assert!(dir_entry_opt.is_some());
        let dir_entry = dir_entry_opt.unwrap();
        let question_res = Question::build_from_dir_entry(&config, &dir_entry);
        assert!(question_res.is_ok());
        let question = question_res?;
        assert_eq!(
            question.directories().submit_directory,
            "tst/resources/rendu_test/hello_world"
        );
        assert_eq!(
            question.directories().question_directory,
            "tst/resources/questions/hello_world"
        );
        assert_eq!(
            question.directories().subject_directory,
            "tst/resources/questions/hello_world/hello_world.subject"
        );
        assert!(matches!(question.test, Test::CompiledTogether(_)));
        Ok(())
    }

    #[test]
    fn question_invalid_sources() -> Result<(), Error> {
        let config = Config::new_from("tst/resources/test_config1.toml")?;
        let dir_path = format!("{}/{}", config.question_dir(), "Z_no_compiler_countdown");
        let file = format!("{}/{}", dir_path, "ft_countdown.toml");
        let buffer = fs::read_to_string(file)?;
        let toml: toml::Question =
            toml_parse::from_str(&buffer).map_err(|e| Error::Question(e.into()))?;
        let question_res = Question::build_from_toml(&config, toml, &dir_path);
        assert!(question_res.is_err());
        assert!(matches!(
            question_res.unwrap_err(),
            QuestionError::MissingKey(_)
        ));
        Ok(())
    }

    #[test]
    fn question_wrong_answer() -> Result<(), Error> {
        let config = Config::new_from("tst/resources/test_config1.toml")?;
        let question_database = QuestionDB::new(&config)?;
        let question = question_database.get_question_by_name("Z_failed_countdown");
        assert!(question.is_some());
        let question = question.unwrap();
        assert_eq!(question.difficulty(), 1);
        let test_result = question.grade(&config)?;
        let error = match test_result {
            TestResult::Passed => panic!("Test should have failed"),
            TestResult::Failed(error) => error,
        };
        let trace = match error {
            TestError::DoesNotCompile(e) => {
                panic!("This test case should compile correctly, but: {}", e)
            }
            TestError::IncorrectOutput(trace) => trace,
            TestError::Timeout => panic!("This test case should pass, but it timed out!"),
            _ => panic!("TestError should have IncorrectOutput type"),
        };
        assert_eq!(
            trace.to_string(),
            format!(
                "{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}",
                "Failure with args: \n",
                "Expected Output:\n",
                "Exit Code: 0\n",
                "Stdout: 9876543210\n",
                "\n",
                "Stderr: \n",
                "Actual Output:\n",
                "Exit Code: 0\n",
                "Stdout: 987543210\n",
                "\n",
                "Stderr: \n",
                "Failure with args: I'll, be, ignored, \n",
                "Expected Output:\n",
                "Exit Code: 0\n",
                "Stdout: 9876543210\n",
                "\n",
                "Stderr: \n",
                "Actual Output:\n",
                "Exit Code: 0\n",
                "Stdout: 987543210\n",
                "\n",
                "Stderr: \n",
            )
        );
        Ok(())
    }

    #[test]
    #[ignore]
    fn question_timeout() -> Result<(), Error> {
        let config = Config::new_from("tst/resources/test_config1.toml")?;
        let question_database = QuestionDB::new(&config)?;
        let question = question_database.get_question_by_name("Z_countdown_timeout");
        assert!(question.is_some());
        let question = question.unwrap();
        let test_result = question.grade(&config)?;
        let error = match test_result {
            TestResult::Passed => panic!("Test should have failed"),
            TestResult::Failed(error) => error,
        };
        assert!(matches!(error, TestError::Timeout));
        Ok(())
    }

    #[test]
    fn invalid_framework() -> Result<(), Error> {
        let config = Config::new_from("tst/resources/test_config2.toml")?;
        let dir_path = format!("{}/{}", config.question_dir(), "Z_invalid_framework_strlen");
        let file = format!("{}/{}", dir_path, "ft_strlen.toml");
        let buffer = fs::read_to_string(file)?;
        let toml: toml::Question =
            toml_parse::from_str(&buffer).map_err(|e| Error::Question(e.into()))?;
        let question_res = Question::build_from_toml(&config, toml, &dir_path);
        assert!(question_res.is_err());
        match question_res.unwrap_err() {
            QuestionError::InvalidFramework(s) => assert_eq!(s, "catch2"),
            _ => panic!("Wrong error type for invalid framework test"),
        }
        Ok(())
    }

    #[test]
    fn question_correct_answer_unit_test() -> Result<(), Error> {
        let config = Config::new_from("tst/resources/test_config2.toml")?;
        let question_database = QuestionDB::new(&config)?;
        let question = question_database.get_question_by_name("ft_strlen");
        assert!(question.is_some());
        let question = question.unwrap();
        let test_result = question.grade(&config)?;
        match test_result {
            TestResult::Passed => Ok(()),
            TestResult::Failed(error) => {
                println!("{}", error);
                panic!("This test should have passed")
            }
        }
    }

    #[test]
    fn question_incorrect_answer_unit_test() -> Result<(), Error> {
        let config = Config::new_from("tst/resources/test_config2.toml")?;
        let question_database = QuestionDB::new(&config)?;
        let question = question_database.get_question_by_name("Z_incorrect_strlen");
        assert!(question.is_some());
        let question = question.unwrap();
        let test_result = question.grade(&config)?;
        match test_result {
            TestResult::Passed => panic!("This test should have failed"),
            TestResult::Failed(error) => match error {
                TestError::FailedUnitTest(_) => Ok(()),
                _ => panic!("This test should fail with unit test error"),
            },
        }
    }

    #[test]
    fn question_correct_answer_sources() -> Result<(), Error> {
        let config = Config::new_from("tst/resources/test_config2.toml")?;
        let question_database = QuestionDB::new(&config)?;
        let question = question_database.get_question_by_name("ft_countdown_sources");
        assert!(question.is_some());
        let question = question.unwrap();
        let test_result = question.grade(&config)?;
        match test_result {
            TestResult::Passed => Ok(()),
            _ => panic!("Test should have passed"),
        }
    }

    #[test]
    fn question_incorrect_answer_sources() -> Result<(), Error> {
        let config = Config::new_from("tst/resources/test_config2.toml")?;
        let question_database = QuestionDB::new(&config)?;
        let question = question_database.get_question_by_name("Z_countdown_sources_wrong");
        assert!(question.is_some());
        let question = question.unwrap();
        let test_result = question.grade(&config)?;
        match test_result {
            TestResult::Passed => panic!("Test should have failed"),
            TestResult::Failed(error) => match error {
                TestError::IncorrectOutput(_) => Ok(()),
                _ => panic!("This test should fail with incorrect output error"),
            },
        }
    }

    #[test]
    fn question_correct_expected_output() -> Result<(), Error> {
        let config = Config::new_from("tst/resources/test_config2.toml")?;
        let question_database = QuestionDB::new(&config)?;
        let question = question_database.get_question_by_name("aff_a");
        assert!(question.is_some());
        let question = question.unwrap();
        let test_result = question.grade(&config)?;
        match test_result {
            TestResult::Passed => Ok(()),
            TestResult::Failed(_) => panic!("This test should have passed"),
        }
    }

    #[test]
    fn question_incorrect_expected_output() -> Result<(), Error> {
        let config = Config::new_from("tst/resources/test_config2.toml")?;
        let question_database = QuestionDB::new(&config)?;
        let question = question_database.get_question_by_name("Z_incorrect_aff_a");
        assert!(question.is_some());
        let question = question.unwrap();
        let test_result = question.grade(&config)?;
        match test_result {
            TestResult::Passed => panic!("Test should have failed"),
            TestResult::Failed(error) => match error {
                TestError::IncorrectOutput(_) => Ok(()),
                _ => panic!("This test should fail with incorrect output error"),
            },
        }
    }
}

#[cfg(test)]
mod display {
    use super::*;
    use crate::Error;

    #[test]
    fn question_display() -> Result<(), Error> {
        let config = Config::new_from("tst/resources/test_config2.toml")?;
        let question_database = QuestionDB::new(&config)?;
        let question = question_database.get_question_by_name("ft_countdown_sources");

        assert!(question.is_some());
        let question = question.unwrap();
        println!("{}", question);
        Ok(())
    }
}

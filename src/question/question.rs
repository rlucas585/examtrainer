use crate::config::Config;
use crate::question::error::MissingKeys;
use crate::question::{self, QuestionError, Submission, Test};
use crate::utils::Range;
use std::fs::DirEntry;
use std::path::Path;

#[derive(Debug)]
pub struct QuestionDirs {
    pub submit_directory: String,
    pub question_directory: String,
    pub subject_directory: String,
}

#[derive(Debug)]
pub struct Question {
    name: String,
    difficulty: u32,
    directories: QuestionDirs,
    submission: Submission,
    test: Test,
}

impl Question {
    pub fn build_from_toml(
        config: &Config,
        toml: question::toml::Question,
        dir_path: &str,
    ) -> Result<Self, QuestionError> {
        Question::check_type_validity(&toml)?;
        let name = toml.info.name;
        let submit_directory = format!("{}/{}", config.directories.submit_directory, name);
        let question_directory = dir_path.to_string();
        let subject_directory =
            Self::validate_subject_directory(&question_directory, &toml.test.subject)?;
        let test: Test = Test::build_from_toml(toml.test, dir_path)?;
        let submission: Submission = Submission::build_from_toml(toml.submission)?;
        Ok(Self {
            name,
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
                    let toml: question::toml::Question = toml::from_str(&buffer)?;
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
    fn check_type_validity(toml: &question::toml::Question) -> Result<(), QuestionError> {
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

    // pub fn grade(&self) -> Result<TestResult, QuestionError> {
    //
    // }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::Error;
    use std::fs;
    #[test]
    fn question_no_subject() -> Result<(), Error> {
        let config = Config::new_from("tst/resources/test_config1.toml")?;
        let dir_path = format!(
            "{}/{}",
            config.directories.question_directory, "no_sub_question"
        );
        let file = format!("{}/{}", dir_path, "hello_world.toml");
        let buffer = fs::read_to_string(file)?;
        let toml: question::toml::Question =
            toml::from_str(&buffer).map_err(|e| Error::Question(e.into()))?;
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
        let dir_path = format!(
            "{}/{}",
            config.directories.question_directory, "hello_world"
        );
        let file = format!("{}/{}", dir_path, "hello_world.toml");
        let buffer = fs::read_to_string(file)?;
        let toml: question::toml::Question =
            toml::from_str(&buffer).map_err(|e| Error::Question(e.into()))?;
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
        let question_dirs = std::fs::read_dir(&config.directories.question_directory)?;
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
        let dir_path = format!(
            "{}/{}",
            config.directories.question_directory, "Z_no_compiler_countdown"
        );
        let file = format!("{}/{}", dir_path, "ft_countdown.toml");
        let buffer = fs::read_to_string(file)?;
        let toml: question::toml::Question =
            toml::from_str(&buffer).map_err(|e| Error::Question(e.into()))?;
        let question_res = Question::build_from_toml(&config, toml, &dir_path);
        assert!(question_res.is_err());
        assert!(matches!(
            question_res.unwrap_err(),
            QuestionError::MissingKey(_)
        ));
        Ok(())
    }
}

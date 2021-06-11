use crate::config::Config;
// use crate::question::*;
use crate::question::{self, QuestionError, Submission, Test};
use std::path::Path;

#[derive(Debug)]
pub struct Question {
    submit_directory: String,
    question_directory: String,
    subject_directory: String,
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
        let submit_directory =
            format!("{}/{}", config.directories.submit_directory, toml.info.name);
        let question_directory = dir_path.to_string();
        let subject_directory =
            Self::validate_subject_directory(&question_directory, &toml.test.subject)?;
        let test: Test = Test::build_from_toml(toml.test, dir_path)?;
        let submission: Submission = Submission::build_from_toml(toml.submission)?;
        Ok(Self {
            submit_directory,
            question_directory,
            subject_directory,
            submission,
            test,
        })
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
            "executable" => Ok(()),
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
            config.directories.question_directory, "no_sub_question/"
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
}

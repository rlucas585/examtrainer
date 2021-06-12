use crate::question;
use crate::question::error::MissingKeys;
use crate::question::QuestionError;
// use crate::utils::ProgramOutput; // TODO needed later

#[derive(Debug)]
pub struct Exec {
    binary: String,
}

impl Exec {
    fn build_from_toml(toml: question::toml::Submission) -> Result<Self, MissingKeys> {
        match toml.binary {
            Some(binary) => Ok(Self { binary }),
            _ => Err(MissingKeys::SubExec),
        }
    }

    pub fn name(&self) -> &str {
        &self.binary
    }
}

#[derive(Debug)]
pub struct Sources {
    sources: Vec<String>,
    compiler: Option<String>,
    flags: Option<Vec<String>>,
}

impl Sources {
    fn build_from_toml(toml: question::toml::Submission) -> Result<Self, MissingKeys> {
        match toml.sources {
            Some(sources) => Ok(Self {
                sources,
                compiler: toml.compiler,
                flags: toml.flags,
            }),
            _ => Err(MissingKeys::SubSources),
        }
    }

    // Will panic if there is no compiler, which should be validated during Submission
    // initialization
    pub fn compiler(&self) -> &str {
        self.compiler.as_ref().unwrap()
    }

    pub fn sources(&self) -> &Vec<String> {
        &self.sources
    }

    pub fn flags(&self) -> &Option<Vec<String>> {
        &self.flags
    }
}

#[derive(Debug)]
pub enum Submission {
    Exec(Exec),
    Sources(Sources),
}

impl Submission {
    pub fn build_from_toml(toml: question::toml::Submission) -> Result<Self, QuestionError> {
        match &toml.submission_type[..] {
            "executable" => Ok(Self::Exec(Exec::build_from_toml(toml)?)),
            "sources" => Ok(Self::Sources(Sources::build_from_toml(toml)?)),
            invalid => Err(QuestionError::InvalidSubmissionType(invalid.into())),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    #[test]
    fn read_submission_toml() -> Result<(), QuestionError> {
        let buffer = fs::read_to_string("tst/resources/question_1.toml")?;
        let question_toml: question::toml::Question = toml::from_str(&buffer)?;
        let submission_toml: question::toml::Submission = question_toml.submission;
        let submission: Submission = Submission::build_from_toml(submission_toml)?;
        assert!(matches!(submission, Submission::Sources(_)));
        match submission {
            Submission::Sources(sources) => {
                assert_eq!(sources.sources, vec!("hello_world.c".to_string()));
                assert_eq!(sources.compiler, None);
                assert_eq!(sources.flags, None);
                Ok(())
            }
            _ => Err(QuestionError::InvalidSubmissionType("not good".into())),
        }
    }
}

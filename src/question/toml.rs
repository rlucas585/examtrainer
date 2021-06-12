//! Deserialized Question structs available for easy use with the `toml` crate
//!
//! Provide a simple parsing target for the `toml` crate when parsing `Question` .toml files. The
//! resultant structs are a Passive Data Structure (PDS), used as a source to build more
//! sophisticated structs.

use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Info {
    pub name: String,
    pub authors: Option<Vec<String>>,
    pub difficulty: u32,
}

#[derive(Deserialize, Debug)]
pub struct Submission {
    pub submission_type: String,
    pub sources: Option<Vec<String>>,
    pub binary: Option<String>,
    pub compiler: Option<String>,
    pub flags: Option<Vec<String>>,
}

#[derive(Deserialize, Debug)]
pub struct Test {
    pub test_type: String,
    pub subject: String,
    pub sources: Option<Vec<String>>,
    pub compiler: Option<String>,
    pub flags: Option<Vec<String>>,
    pub binary: Option<String>,
    pub args: Option<Vec<Vec<String>>>,
    pub expected_stdout: Option<String>,
    pub expected_stderr: Option<String>,
}

/// A PDS used as a target for parsing of Question .toml files
///
/// The `toml::Question` struct is easily formed using the `toml` crate, and required as a
/// parameter to build `question::Question`, which has extended functionality.
#[derive(Deserialize, Debug)]
pub struct Question {
    pub info: Info,
    pub submission: Submission,
    pub test: Test,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::question::QuestionError;
    use std::fs;
    #[test]
    fn read_question_toml() -> Result<(), QuestionError> {
        let buffer = fs::read_to_string("tst/resources/question_1.toml")?;
        let toml: Question = toml_parse::from_str(&buffer)?;
        assert_eq!(toml.info.name, "hello_world");
        assert_eq!(toml.info.authors, Some(vec!("Ryan Lucas".into())));
        assert_eq!(toml.info.difficulty, 2);
        assert_eq!(toml.submission.submission_type, "sources");
        assert_eq!(toml.submission.sources, Some(vec!("hello_world.c".into())));
        assert_eq!(toml.test.test_type, "expected-output");
        assert_eq!(toml.test.sources, Some(vec!("main.c".into())));
        assert_eq!(
            toml.test.flags,
            Some(vec!("-Wall".into(), "-Wextra".into(), "-Werror".into()))
        );
        assert_eq!(toml.test.subject, "hello_world.subject");
        assert_eq!(toml.test.expected_stdout, Some("hello_world.out".into()));
        assert_eq!(toml.test.expected_stderr, Some("hello_world.err".into()));
        assert_eq!(
            toml.test.args,
            Some(vec!(
                Vec::new(),
                vec!("Ryan".into(), "Lucas".into()),
                vec!(
                    "did".into(),
                    "you".into(),
                    "know".into(),
                    "shinigami".into(),
                    "love".into(),
                    "apples".into()
                )
            ))
        );
        Ok(())
    }

    #[test]
    fn read_invalid_toml() -> Result<(), QuestionError> {
        let buffer = fs::read_to_string("tst/resources/invalid_1.toml")?;
        let toml_result: Result<Question, toml_parse::de::Error> = toml_parse::from_str(&buffer);
        assert!(toml_result.is_err());
        assert_eq!(
            "expected a right bracket, found a left bracket at line 23 column 5",
            toml_result.unwrap_err().to_string()
        );
        Ok(())
    }
}

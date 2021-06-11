//!  A type to compile, run and grade submitted answers
//!
//! The `Test` module is centered around the [`Test`] enum. This enum is built using the
//! [`examtrainer::question::toml::Test`] toml struct, one of the components of a `Question`,
//! using the [`Test::build_from_toml`] constructor. Different types of tests require different
//! parameters, the different test types are:
//! * 'executable' - Expects a user to submit their own sources/executable as an answer, then will
//! run both the user's executable and the test executable side by side: comparing output (both
//! stdout and stderr), and generating a trace file if necessary.
//! * 'unit-test' - Takes source files from a user, and compiles them with Unit Test files supplied
//! in the Question module. Runs the Unit Test, and places output directly in a trace code if the
//! Unit Test returns a non-zero value.
//! * 'sources' - Functions identically to the 'expected' test type, except requires that the
//! Question module contains sources to be compiled into an executable before testing.
//! * 'expected-output' - Compiles user code together with test sources to produce an executable.
//! The executable will then be run, with stdout compared against a
//! `.out` file, and stderr compared against a `.err` file.

use crate::question;
use crate::question::error::MissingKeys;
use crate::question::{QuestionError, Submission};
use std::path::Path;
// use crate::utils::ProgramOutput; // TODO needed later

#[derive(Debug)]
pub struct Exec {
    binary: String,
    args: Vec<Vec<String>>,
}

impl Exec {
    fn build_from_toml(toml: question::toml::Test, dir_path: &str) -> Result<Self, MissingKeys> {
        match (toml.binary, toml.args) {
            (Some(binary), Some(args)) => Ok(Self {
                binary: format!("{}/{}", dir_path, binary),
                args,
            }),
            _ => Err(MissingKeys::Exec),
        }
    }
}

#[derive(Debug)]
pub struct UnitTest {
    compiler: String,
    sources: Vec<String>,
}

impl UnitTest {
    fn build_from_toml(toml: question::toml::Test, dir_path: &str) -> Result<Self, MissingKeys> {
        match (toml.compiler, toml.sources) {
            (Some(compiler), Some(sources)) => Ok(Self {
                compiler,
                sources: sources
                    .into_iter()
                    .map(|elem| format!("{}/{}", dir_path, elem))
                    .collect(),
            }),
            _ => Err(MissingKeys::UnitTest),
        }
    }
}

#[derive(Debug)]
pub struct Sources {
    compiler: String,
    sources: Vec<String>,
}

impl Sources {
    fn build_from_toml(toml: question::toml::Test, dir_path: &str) -> Result<Self, MissingKeys> {
        match (toml.compiler, toml.sources) {
            (Some(compiler), Some(sources)) => Ok(Self {
                compiler,
                sources: sources
                    .into_iter()
                    .map(|elem| format!("{}/{}", dir_path, elem))
                    .collect(),
            }),
            _ => Err(MissingKeys::Sources),
        }
    }
}

#[derive(Debug)]
pub struct CompiledTogether {
    compiler: String,
    flags: Option<Vec<String>>,
    sources: Vec<String>,
    stdout_file: String,
    stderr_file: String,
    args: Vec<Vec<String>>,
}

impl CompiledTogether {
    fn build_from_toml(toml: question::toml::Test, dir_path: &str) -> Result<Self, QuestionError> {
        match (
            toml.compiler,
            toml.sources,
            toml.expected_stdout,
            toml.expected_stderr,
            toml.args,
        ) {
            (Some(compiler), Some(sources), Some(stdout_file), Some(stderr_file), Some(args)) => {
                let stdout_file = format!("{}/{}", dir_path, stdout_file);
                let stderr_file = format!("{}/{}", dir_path, stderr_file);
                Self::validate_output_files(&stdout_file, &stderr_file)?;
                Ok(Self {
                    compiler,
                    flags: toml.flags,
                    sources,
                    stdout_file,
                    stderr_file,
                    args,
                })
            }
            _ => Err(MissingKeys::CompiledTogether.into()),
        }
    }

    fn validate_output_files(out_path: &str, err_path: &str) -> Result<(), QuestionError> {
        let out_path = Path::new(&out_path);
        let err_path = Path::new(&err_path);
        match (
            out_path.exists() && out_path.is_file(),
            err_path.exists() && out_path.is_file(),
        ) {
            (true, true) => Ok(()),
            (true, false) => Err(QuestionError::NoStderr),
            (false, true) => Err(QuestionError::NoStdout),
            _ => Err(QuestionError::NoStdout),
        }
    }
}

#[derive(Debug)]
pub enum Test {
    Exec(Exec),
    UnitTest(UnitTest),
    Sources(Sources),
    CompiledTogether(CompiledTogether),
}

impl Test {
    pub fn build_from_toml(
        toml: question::toml::Test,
        dir_path: &str,
    ) -> Result<Self, QuestionError> {
        match &toml.test_type[..] {
            "executable" => Ok(Self::Exec(Exec::build_from_toml(toml, dir_path)?)),
            "unit-test" => Ok(Self::UnitTest(UnitTest::build_from_toml(toml, dir_path)?)),
            "sources" => Ok(Self::Sources(Sources::build_from_toml(toml, dir_path)?)),
            "expected-output" => Ok(Self::CompiledTogether(CompiledTogether::build_from_toml(
                toml, dir_path,
            )?)),
            invalid => Err(QuestionError::InvalidTestType(invalid.into())),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    #[test]
    fn read_test_toml() -> Result<(), QuestionError> {
        let buffer = fs::read_to_string("tst/resources/questions/hello_world/hello_world.toml")?;
        let dir_path = String::from("tst/resources/questions/hello_world");
        let question_toml: question::toml::Question = toml::from_str(&buffer)?;
        let test_toml: question::toml::Test = question_toml.test;
        let test: Test = Test::build_from_toml(test_toml, &dir_path)?;
        assert!(matches!(test, Test::CompiledTogether(_)));
        match test {
            Test::CompiledTogether(test) => {
                assert_eq!(test.compiler, "gcc");
                assert_eq!(
                    test.flags,
                    Some(vec!("-Wall".into(), "-Wextra".into(), "-Werror".into()))
                );
                assert_eq!(test.sources, vec!("main.c"));
                assert_eq!(
                    test.stdout_file,
                    "tst/resources/questions/hello_world/hello_world.out"
                );
                assert_eq!(
                    test.stderr_file,
                    "tst/resources/questions/hello_world/hello_world.err"
                );
            }
            _ => (),
        }

        Ok(())
    }
}

use crate::question;
use crate::question::error::MissingKeys;
use crate::question::QuestionError;
use crate::utils::ProgramOutput;

#[derive(Debug)]
struct Exec {
    binary: String,
    args: Vec<Vec<String>>,
}

impl Exec {
    fn build_from_toml(toml: question::toml::Test) -> Result<Self, MissingKeys> {
        match (toml.binary, toml.args) {
            (Some(binary), Some(args)) => Ok(Self { binary, args }),
            _ => Err(MissingKeys::Exec),
        }
    }
}

#[derive(Debug)]
struct UnitTest {
    compiler: String,
    sources: Vec<String>,
}

impl UnitTest {
    fn build_from_toml(toml: question::toml::Test) -> Result<Self, MissingKeys> {
        match (toml.compiler, toml.sources) {
            (Some(compiler), Some(sources)) => Ok(Self { compiler, sources }),
            _ => Err(MissingKeys::UnitTest),
        }
    }
}

#[derive(Debug)]
struct Sources {
    compiler: String,
    sources: Vec<String>,
}

impl Sources {
    fn build_from_toml(toml: question::toml::Test) -> Result<Self, MissingKeys> {
        match (toml.compiler, toml.sources) {
            (Some(compiler), Some(sources)) => Ok(Self { compiler, sources }),
            _ => Err(MissingKeys::Sources),
        }
    }
}

#[derive(Debug)]
struct CompiledTogether {
    compiler: String,
    flags: Option<Vec<String>>,
    sources: Vec<String>,
    stdout_file: String,
    stderr_file: String,
    args: Vec<Vec<String>>,
}

impl CompiledTogether {
    fn build_from_toml(toml: question::toml::Test) -> Result<Self, MissingKeys> {
        match (
            toml.compiler,
            toml.sources,
            toml.expected_stdout,
            toml.expected_stderr,
            toml.args,
        ) {
            (Some(compiler), Some(sources), Some(stdout_file), Some(stderr_file), Some(args)) => {
                Ok(Self {
                    compiler,
                    flags: toml.flags,
                    sources,
                    stdout_file,
                    stderr_file,
                    args,
                })
            }
            _ => Err(MissingKeys::CompiledTogether),
        }
    }
}

#[derive(Debug)]
enum Test {
    Exec(Exec),
    UnitTest(UnitTest),
    Sources(Sources),
    CompiledTogether(CompiledTogether),
}

impl Test {
    pub fn build_from_toml(toml: question::toml::Test) -> Result<Self, QuestionError> {
        match &toml.test_type[..] {
            "executable" => Ok(Self::Exec(Exec::build_from_toml(toml)?)),
            "unit-test" => Ok(Self::UnitTest(UnitTest::build_from_toml(toml)?)),
            "sources" => Ok(Self::Sources(Sources::build_from_toml(toml)?)),
            "expected-output" => Ok(Self::CompiledTogether(CompiledTogether::build_from_toml(
                toml,
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
    fn read_question_toml() -> Result<(), QuestionError> {
        let buffer = fs::read_to_string("tst/resources/question_1.toml")?;
        let question_toml: question::toml::Question = toml::from_str(&buffer)?;
        let test_toml: question::toml::Test = question_toml.test;
        let test: Test = Test::build_from_toml(test_toml)?;
        assert!(matches!(test, Test::CompiledTogether(_)));
        match test {
            Test::CompiledTogether(test) => {
                assert_eq!(test.compiler, "gcc");
                assert_eq!(
                    test.flags,
                    Some(vec!("-Wall".into(), "-Wextra".into(), "-Werror".into()))
                );
                assert_eq!(test.sources, vec!("main.c"));
                assert_eq!(test.stdout_file, "hello_world.out");
                assert_eq!(test.stderr_file, "hello_world.err");
            }
            _ => (),
        }

        Ok(())
    }
}

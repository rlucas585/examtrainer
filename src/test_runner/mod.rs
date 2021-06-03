use serde::Deserialize;
use std::fmt;
use std::io;
use std::path::Path;
use std::process::{Command, Output};
use std::str::FromStr;

#[derive(Debug)]
pub enum Error {
    Read(String),
    Parse(String),
}

impl std::error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Read(e) => write!(f, "Error reading Test config file: {}", e),
            Error::Parse(e) => write!(f, "Error parsing Test config file: {}", e),
        }
    }
}

impl From<&str> for Error {
    fn from(input: &str) -> Self {
        Error::Parse(input.to_string())
    }
}

impl From<io::Error> for Error {
    fn from(input: io::Error) -> Self {
        Error::Read(input.to_string())
    }
}

impl From<toml::de::Error> for Error {
    fn from(input: toml::de::Error) -> Self {
        Error::Parse(input.to_string())
    }
}

#[derive(Debug)]
struct Exec {
    binary: String,
    args: Vec<Vec<String>>,
}

impl Exec {
    pub fn build_from_toml(toml: TestToml) -> Result<Self, Error> {
        match (toml.compilation.binary, toml.compilation.args) {
            (Some(binary), Some(args)) => Ok(Exec { binary, args }),
            (Some(_), None) => Err("No binary specified for executable type module".into()),
            (None, Some(_)) => Err("No arguments specified for executable type module".into()),
            _ => Err("No binary or arguments supplied for executable type module".into()),
        }
    }
}

#[derive(Debug)]
struct UnitTest {}

#[derive(Debug)]
struct UnitTestBuilder {}
impl UnitTestBuilder {
    pub fn new() -> Self {
        Self {}
    }
}

#[derive(Debug)]
struct Compiled {}

#[derive(Debug)]
struct CompiledBuilder {}
impl CompiledBuilder {
    pub fn new() -> Self {
        Self {}
    }
}

#[derive(Debug)]
enum TestBuilder {
    Exec,
    UnitTest,
    Compiled,
}

impl TestBuilder {
    pub fn build(self, toml: TestToml) -> Result<TestRunner, Error> {
        match self {
            Self::Exec => {
                let exec = Exec::build_from_toml(toml)?;
                Ok(TestRunner::Exec(exec))
            }
            Self::UnitTest => Ok(TestRunner::UnitTest(UnitTest {})),
            Self::Compiled => Ok(TestRunner::Compiled(Compiled {})),
        }
    }
}

impl FromStr for TestBuilder {
    type Err = Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "executable" => Ok(Self::Exec),
            "unit-test" => Ok(Self::UnitTest),
            "sources" => Ok(Self::Compiled),
            invalid => Err(Error::Parse(format!("Invalid test type: {}", invalid))),
        }
    }
}

#[derive(Debug)]
enum TestRunner {
    Exec(Exec),
    UnitTest(UnitTest),
    Compiled(Compiled),
}

#[derive(Deserialize, Debug)]
struct ModuleInfo {
    name: String,
    authors: Option<Vec<String>>,
}

#[derive(Deserialize, Debug)]
struct CompilationInfo {
    test_type: String,
    binary: Option<String>,
    args: Option<Vec<Vec<String>>>,
}

#[derive(Deserialize, Debug)]
struct TestToml {
    exam_module: ModuleInfo,
    compilation: CompilationInfo,
}

impl TestRunner {
    pub fn new_from_file(filename: &str) -> Result<Self, Error> {
        let toml = std::fs::read_to_string(filename)?;
        let toml: TestToml = toml::from_str(&toml)?;
        println!("{:?}", toml);
        TestBuilder::from_str(&toml.compilation.test_type)?.build(toml)
    }
}

pub fn development_func() -> Result<(), Error> {
    let test_runner = TestRunner::new_from_file("tst/modules/module_1.toml")?;
    println!("{:?}", test_runner);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn build_executable_config() -> Result<(), Error> {
        let toml = std::fs::read_to_string("tst/modules/module_1.toml")?;
        let toml: TestToml = toml::from_str(&toml)?;
        let tester = TestBuilder::from_str(&toml.compilation.test_type)?.build(toml)?;
        assert!(matches!(tester, TestRunner::Exec(_)));
        if let TestRunner::Exec(exec) = tester {
            assert_eq!(exec.binary, "example_binary");
            assert_eq!(exec.args, vec!(vec!(), vec!("Ryan", "Lucas")));
        }
        Ok(())
    }
}

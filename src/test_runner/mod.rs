use crate::toml::TestToml;
use std::process::{Command, Output};
use std::str::FromStr;

mod program_output;
mod submission;

use crate::config::Config;
use crate::error::Error;
use crate::toml::ModuleToml;
use program_output::ProgramOutput;
use submission::Submission;

#[derive(Debug)]
struct Exec {
    binary: String,
    args: Vec<Vec<String>>,
}

impl Exec {
    pub fn build_from_toml(toml: TestToml) -> Result<Self, Error> {
        match (toml.binary, toml.args) {
            (Some(binary), Some(args)) => Ok(Exec { binary, args }),
            (Some(_), None) => Err("No binary specified for executable type module".into()),
            (None, Some(_)) => Err("No arguments specified for executable type module".into()),
            _ => Err("No binary or arguments supplied for executable type module".into()),
        }
    }
}

#[derive(Debug)] // TODO: Implement
struct UnitTest {}

#[derive(Debug)] // TODO: Implement
struct Sources {}

#[derive(Debug)]
struct CompiledWithAnswer {
    sources: Vec<String>,
    compiler: String,
    args: Vec<Vec<String>>,
    flags: Option<Vec<String>>,
} // TODO: Implement

impl CompiledWithAnswer {
    pub fn build_from_toml(toml: TestToml) -> Result<Self, Error> {
        match (toml.sources, toml.compiler, toml.args) {
            (Some(sources), Some(compiler), Some(args)) => Ok(Self {
                sources,
                compiler,
                args,
                flags: toml.flags,
            }),
            _ => Err(
                "expected-output type module must have sources, compiler, and args arguments"
                    .into(),
            ),
        }
    }
}

#[derive(Debug)]
enum TestBuilder {
    Exec,
    UnitTest,
    Sources,
    CompiledWithAnswer,
}

impl TestBuilder {
    pub fn build(self, toml: TestToml) -> Result<Test, Error> {
        match self {
            Self::Exec => {
                let exec = Exec::build_from_toml(toml)?;
                Ok(Test::Exec(exec))
            }
            Self::UnitTest => Ok(Test::UnitTest(UnitTest {})),
            Self::Sources => Ok(Test::Sources(Sources {})),
            Self::CompiledWithAnswer => {
                let compile = CompiledWithAnswer::build_from_toml(toml)?;
                Ok(Test::CompiledWithAnswer(compile))
            }
        }
    }
}

impl FromStr for TestBuilder {
    type Err = Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "executable" => Ok(Self::Exec),
            "unit-test" => Ok(Self::UnitTest),
            "sources" => Ok(Self::Sources),
            "expected-output" => Ok(Self::CompiledWithAnswer),
            invalid => Err(Error::Parse(format!("Invalid test type: {}", invalid))),
        }
    }
}

#[derive(Debug)]
enum Test {
    Exec(Exec),
    UnitTest(UnitTest),
    Sources(Sources),
    CompiledWithAnswer(CompiledWithAnswer),
}

impl Test {
    pub fn generate_from_toml(toml: TestToml) -> Result<Self, Error> {
        TestBuilder::from_str(&toml.test_type)?.build(toml)
    }

    #[cfg(test)]
    pub fn test_run(&self, module_path: &String) -> Vec<ProgramOutput> {
        match self {
            Self::Exec(e) => {
                let mut output = Vec::new();
                let exec_path = format!("{}{}", module_path, e.binary);
                for args in e.args.iter() {
                    let mut exec = Command::new(&exec_path);
                    for arg in args.iter() {
                        exec.arg(arg);
                    }
                    match exec.output() {
                        Ok(out) => output.push(ProgramOutput::new(out)),
                        Err(e) => output.push(ProgramOutput::from_strings(
                            1,
                            String::from(""),
                            e.to_string(),
                        )),
                    }
                }
                output
            }
            Self::UnitTest(_) => Vec::new(),
            Self::Sources(_) => Vec::new(),
            Self::CompiledWithAnswer(_) => Vec::new(),
        }
    }

    // TODO: Change to function that takes the Submission info
    pub fn run(&self, submission: &Submission, answer_path: &str, submit_path: &str) {}
}

#[derive(Debug)]
pub struct TestRunner {
    submit_directory: String,
    module_directory: String,
    submission: Submission,
    test: Test,
}

impl TestRunner {
    pub fn new(module_path: &str, submit_path: &str) -> Result<Self, Error> {
        let toml = std::fs::read_to_string(module_path)?;
        let toml: ModuleToml = toml::from_str(&toml)?;
        let test = Test::generate_from_toml(toml.test)?;
        let submission = Submission::generate_from_toml(toml.submission)?;
        Ok(Self {
            submit_directory: submit_path.to_owned(),
            module_directory: module_path.to_owned(),
            submission,
            test,
        })
    }

    pub fn build_from_toml(config: &Config, toml: ModuleToml) -> Result<Self, Error> {
        let test = Test::generate_from_toml(toml.test)?;
        let submission = Submission::generate_from_toml(toml.submission)?;
        Ok(Self {
            submit_directory: format!(
                "{}/{}/",
                config.directories.submit_directory, toml.info.name
            ),
            module_directory: format!(
                "{}/{}/",
                config.directories.module_directory, toml.info.name
            ),
            submission,
            test,
        })
    }

    pub fn submit_location(&self) -> &str {
        &self.submit_directory
    }
}

// use crate::toml::ModuleToml; // TODO remove this at some point
//
// pub fn development_func() -> Result<(), Error> {
//     let toml = std::fs::read_to_string("tst/modules/module_1.toml")?;
//     let toml: ModuleToml = toml::from_str(&toml)?;
//     let tester = Test::generate_from_toml(toml.test)?;
//     let module_path = "tst/modules/".to_owned();
//     println!("{:?}", tester);
//     // let results = tester.run(&module_path);
//     // println!("STDOUT: {:?}", results.0);
//     // println!("STDERR: {:?}", results.1);
//     Ok(())
// }

#[cfg(test)]
mod tests {
    use super::*;
    use crate::toml::ModuleToml;
    #[test]
    fn test_executable_creation() -> Result<(), Error> {
        let toml = std::fs::read_to_string("tst/modules/module_1.toml")?;
        let toml: ModuleToml = toml::from_str(&toml)?;
        let test = Test::generate_from_toml(toml.test)?;
        assert!(matches!(test, Test::Exec(_)));
        if let Test::Exec(exec) = test {
            assert_eq!(exec.binary, "example_binary");
            assert_eq!(
                exec.args,
                vec!(
                    vec!(),
                    vec!("Ryan", "Lucas"),
                    vec!("did", "you", "know", "shinigami", "love", "apples")
                )
            );
        }
        Ok(())
    }

    #[test]
    fn test_executable_output() -> Result<(), Error> {
        let toml = std::fs::read_to_string("tst/modules/module_1.toml")?;
        let toml: ModuleToml = toml::from_str(&toml)?;
        let test = Test::generate_from_toml(toml.test)?;
        let module_path = "tst/modules/".to_owned();
        let results = test.test_run(&module_path);
        assert_eq!(results.len(), 3);
        assert_eq!(
            results[0],
            ProgramOutput {
                status: 0,
                stdout: String::from("hello\n"),
                stderr: String::from(""),
            }
        );
        assert_eq!(
            results[1],
            ProgramOutput {
                status: 0,
                stdout: String::from("hello Ryan Lucas\n"),
                stderr: String::from(""),
            }
        );
        assert_eq!(
            results[2],
            ProgramOutput {
                status: 0,
                stdout: String::from("hello did you know shinigami love apples\n"),
                stderr: String::from(""),
            }
        );
        Ok(())
    }
}

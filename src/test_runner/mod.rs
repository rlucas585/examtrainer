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
    subject: String,
    binary: String,
    args: Vec<Vec<String>>,
}

impl Exec {
    pub fn build_from_toml(toml: TestToml) -> Result<Self, Error> {
        match (toml.binary, toml.args) {
            (Some(binary), Some(args)) => Ok(Exec {
                subject: toml.subject,
                binary,
                args,
            }),
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
    subject: String,
    sources: Vec<String>,
    compiler: String,
    args: Vec<Vec<String>>,
    flags: Option<Vec<String>>,
    expected_stdout_file: String,
    expected_stderr_file: String,
}

impl CompiledWithAnswer {
    pub fn build_from_toml(toml: TestToml) -> Result<Self, Error> {
        match (
            toml.sources,
            toml.compiler,
            toml.args,
            toml.expected_stdout,
            toml.expected_stderr,
        ) {
            (Some(sources), Some(compiler), Some(args), Some(stdout), Some(stderr)) => Ok(Self {
                subject: toml.subject,
                sources,
                compiler,
                args,
                flags: toml.flags,
                expected_stdout_file: stdout,
                expected_stderr_file: stderr,
            }),
            _ => Err(
                "expected-output type module must have sources, compiler, expected_stdout,\
 expected_stderr, and args arguments"
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

pub enum TestResult {
    Passed,
    Failed,
}

#[derive(Debug)]
enum Test {
    Exec(Exec),
    UnitTest(UnitTest),
    Sources(Sources),
    CompiledWithAnswer(CompiledWithAnswer), // Tested against an expected output
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
    // pub fn run(&self, submission: &Submission, answer_path: &str, submit_path: &str) -> TestResult {
    // }
}

#[derive(Debug)]
pub struct TestRunner {
    submit_directory: String,
    module_directory: String,
    subject_directory: String,
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
            subject_directory: "placeholder".to_owned(),
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
            subject_directory: format!(
                "{}/{}/",
                config.directories.subject_directory, toml.info.name
            ),
            submission,
            test,
        })
    }

    pub fn submit_location(&self) -> &str {
        &self.submit_directory
    }

    pub fn subject_location(&self) -> &str {
        &self.subject_directory
    }

    pub fn subject_source(&self) -> String {
        let subject_directory = match &self.test {
            Test::Exec(e) => &e.subject,
            Test::UnitTest(_) => "", // TODO implement
            Test::Sources(_) => "",
            Test::CompiledWithAnswer(c) => &c.subject,
        };
        format!("{}{}", self.module_directory, subject_directory)
    }

    pub fn run(&self) -> Result<TestResult, Error> {
        match &self.test {
            Test::Exec(exec) => self.exec_run(exec),
            Test::UnitTest(test) => self.unit_test_run(test),
            Test::Sources(sources) => self.sources_run(sources),
            Test::CompiledWithAnswer(compile) => self.compiled_run(compile),
        }
    }

    fn exec_run(&self, exec: &Exec) -> Result<TestResult, Error> {
        unimplemented!("Need to implement Test::Exec");
    }
    fn unit_test_run(&self, unit_test: &UnitTest) -> Result<TestResult, Error> {
        unimplemented!("Need to implement Test::UnitTest");
    }
    fn sources_run(&self, sources: &Sources) -> Result<TestResult, Error> {
        unimplemented!("Need to implement Test::Sources");
    }
    fn compiled_run(&self, compile: &CompiledWithAnswer) -> Result<TestResult, Error> {
        let compilation_result = self.compile_answer_with_submission(compile)?;
        if let Err(_compilation_error) = compilation_result {
            // TODO write error message to trace in future
            // eprintln!("error: {}", _compilation_error);
            return Ok(TestResult::Failed);
        }
        let test_binary = compilation_result.unwrap();
        let run_result = self.run_compiled_answer(compile, test_binary)?;
        Ok(run_result)
    }

    fn compile_answer_with_submission(
        &self,
        test: &CompiledWithAnswer,
    ) -> Result<Result<String, String>, Error> {
        match &self.submission {
            Submission::Exec(_) => {
                panic!("Compile with sources type test used with binary submission")
            }
            Submission::Sources(sub_source) => {
                let test_binary = String::from("test_binary");
                // TODO: These verifications should take place when creating TestRunner, rather
                // than now
                if let Some(source_compiler) = &sub_source.compiler {
                    if source_compiler != &test.compiler {
                        panic!("Compilers different for submission and answer");
                    }
                }
                let mut compile_builder = Command::new(&test.compiler);
                compile_builder.arg("-o").arg(&test_binary);
                if let Some(flags) = &sub_source.flags {
                    for flag in flags {
                        compile_builder.arg(flag);
                    }
                }
                if let Some(flags) = &test.flags {
                    for flag in flags {
                        compile_builder.arg(flag);
                    }
                }
                for source in sub_source.sources.iter() {
                    compile_builder.arg(format!("{}/{}", self.submit_directory, source));
                }
                for source in test.sources.iter() {
                    compile_builder.arg(format!("{}/{}", self.module_directory, source));
                }
                match compile_builder.output() {
                    Ok(out) => {
                        if out.status.code().unwrap() != 0 {
                            Ok(Err(format!(
                                "Compilation error: {}",
                                std::str::from_utf8(out.stderr.as_slice()).unwrap()
                            )))
                        } else {
                            Ok(Ok(test_binary))
                        }
                    }
                    Err(e) => Err(e.into()),
                }
            }
        }
    }

    fn run_compiled_answer(
        &self,
        test: &CompiledWithAnswer,
        test_binary: String,
    ) -> Result<TestResult, Error> {
        let expected_out = std::fs::read_to_string(format!(
            "{}/{}",
            self.module_directory, test.expected_stdout_file
        ))?;
        let expected_err = std::fs::read_to_string(format!(
            "{}/{}",
            self.module_directory, test.expected_stderr_file
        ))?;
        let mut actual_out = String::new();
        let mut actual_err = String::new();
        let exec_path = format!("./{}", test_binary);
        for args in test.args.iter() {
            let mut exec = Command::new(&exec_path);
            for arg in args.iter() {
                exec.arg(arg);
            }
            match exec.output() {
                Ok(out) => {
                    let program_output = ProgramOutput::new(out);
                    actual_out.push_str(&program_output.stdout);
                    actual_err.push_str(&program_output.stderr);
                }
                Err(e) => return Err(e.into()),
            }
        }
        Self::remove_binary(test_binary)?;
        if actual_out == expected_out && actual_err == expected_err {
            Ok(TestResult::Passed)
        } else {
            Ok(TestResult::Failed)
        }
    }

    fn remove_binary(binary: String) -> Result<Output, Error> {
        Command::new("rm")
            .arg(binary)
            .output()
            .map_err(|e| e.into())
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

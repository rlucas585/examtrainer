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

use crate::config::Config;
use crate::question;
use crate::question::compiler::{remove_binary, CompileResult, Compiler};
use crate::question::error::MissingKeys;
use crate::question::{
    run_binary_with_args, BinaryResult, QuestionDirs, QuestionError, Submission, Trace,
};
use std::fmt;
use std::fs;
use std::path::Path;

#[derive(Debug)]
pub enum TestError {
    DoesNotCompile(String),
    IncorrectOutput(Trace),
    FailedUnitTest(Trace),
    Timeout,
}

impl fmt::Display for TestError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::DoesNotCompile(s) => write!(f, "Compilation error: {}", s),
            Self::IncorrectOutput(trace) => write!(f, "IncorrectOutput, Trace: {}", trace),
            Self::FailedUnitTest(trace) => write!(f, "Unit test failed, Trace: {}", trace),
            Self::Timeout => write!(f, "Submission executable timedout"),
        }
    }
}

impl std::error::Error for TestError {}

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

    fn run(
        &self,
        submission: &Submission,
        dirs: &QuestionDirs,
    ) -> Result<TestResult, QuestionError> {
        match submission {
            // TODO: Add a check here to confirm the binary file exists
            Submission::Exec(exec) => self.run_with_binary(exec.name()),
            Submission::Sources(sources) => {
                let mut compiler = Compiler::new(sources.compiler());
                for source in sources.sources().iter() {
                    compiler.add_source(format!("{}/{}", dirs.submit_directory, source));
                }
                if let Some(flags) = sources.flags() {
                    for flag in flags.iter() {
                        compiler.add_flag(flag);
                    }
                }
                let compile_result = compiler.compile()?;
                let binary = match compile_result {
                    CompileResult::Ok(binary_name) => binary_name,
                    CompileResult::Err(error) => return Ok(TestResult::Failed(error)),
                };
                let binary = format!("./{}", binary);
                let result_val = self.run_with_binary(&binary);
                remove_binary(&binary)?;
                result_val
            }
        }
    }

    fn run_with_binary(&self, binary: &str) -> Result<TestResult, QuestionError> {
        let mut trace = Trace::new();
        for args in self.args.iter() {
            let test_output = match run_binary_with_args(&self.binary, args)? {
                BinaryResult::Output(output) => output,
                BinaryResult::Timeout => panic!("A questions test timed out, question is invalid"),
            };
            let submit_output = match run_binary_with_args(binary, args)? {
                BinaryResult::Output(output) => output,
                BinaryResult::Timeout => {
                    return Ok(TestResult::Failed(TestError::Timeout));
                }
            };
            if test_output != submit_output {
                trace.binary_output(args, test_output, submit_output);
            }
        }
        if trace.exists() {
            Ok(TestResult::Failed(TestError::IncorrectOutput(trace)))
        } else {
            Ok(TestResult::Passed)
        }
    }
}

#[derive(Debug)]
pub struct UnitTest {
    compiler: String,
    sources: Vec<String>,
    flags: Option<Vec<String>>,
    framework: Option<String>,
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
                flags: toml.flags,
                framework: toml.framework,
            }),
            _ => Err(MissingKeys::UnitTest),
        }
    }

    fn invalid_framework(&self, config: &crate::config::Config) -> Result<(), String> {
        if let Some(test_framework) = &self.framework {
            if config.get_framework(&test_framework).is_none() {
                return Err(test_framework.clone());
            } else {
                return Ok(());
            }
        }
        Ok(())
    }

    fn run(
        &self,
        submission: &Submission,
        dirs: &QuestionDirs,
        config: &Config,
    ) -> Result<TestResult, QuestionError> {
        match submission {
            Submission::Sources(sources) => {
                let compile_result = self.compile_binary(sources, dirs, config)?;
                let binary = match compile_result {
                    CompileResult::Ok(binary_name) => format!("./{}", binary_name),
                    CompileResult::Err(error) => return Ok(TestResult::Failed(error)),
                };
                let result_val = self.run_with_binary(&binary);
                remove_binary(&binary)?;
                result_val
            }
            _ => Err(QuestionError::InvalidTestType(String::from(
                "Unit test cannot be run with any submission type other than sources",
            ))),
        }
    }

    fn compile_binary(
        &self,
        sources: &crate::question::submission::Sources,
        dirs: &QuestionDirs,
        config: &Config,
    ) -> Result<CompileResult, QuestionError> {
        let mut compiler = Compiler::new(&self.compiler);
        for source in sources.sources().iter() {
            compiler.add_source(format!("{}/{}", dirs.submit_directory, source));
        }
        for source in self.sources.iter() {
            compiler.add_source(source.clone());
        }
        if let Some(flags) = &self.flags {
            for flag in flags.iter() {
                compiler.add_flag(flag);
            }
        }
        if let Some(framework_name) = &self.framework {
            let framework_flags = config.get_framework(framework_name).unwrap();
            for flag in framework_flags.iter() {
                compiler.add_flag(flag);
            }
        }
        compiler.compile()
    }

    fn run_with_binary(&self, binary: &str) -> Result<TestResult, QuestionError> {
        let mut trace = Trace::new();
        let dummy_args: [String; 0] = [];
        let output = match run_binary_with_args(binary, &dummy_args)? {
            BinaryResult::Output(output) => output,
            BinaryResult::Timeout => return Ok(TestResult::Failed(TestError::Timeout)),
        };
        if output.code() != 0 {
            trace.unit_test_output(output);
        }
        if trace.exists() {
            Ok(TestResult::Failed(TestError::FailedUnitTest(trace)))
        } else {
            Ok(TestResult::Passed)
        }
    }
}

#[derive(Debug)]
pub struct Sources {
    compiler: String,
    sources: Vec<String>,
    args: Vec<Vec<String>>,
    flags: Option<Vec<String>>,
}

impl Sources {
    fn build_from_toml(toml: question::toml::Test, dir_path: &str) -> Result<Self, MissingKeys> {
        match (toml.compiler, toml.sources, toml.args) {
            (Some(compiler), Some(sources), Some(args)) => Ok(Self {
                compiler,
                sources: sources
                    .into_iter()
                    .map(|elem| format!("{}/{}", dir_path, elem))
                    .collect(),
                args,
                flags: toml.flags,
            }),
            _ => Err(MissingKeys::Sources),
        }
    }

    fn run(
        &self,
        submission: &Submission,
        dirs: &QuestionDirs,
    ) -> Result<TestResult, QuestionError> {
        match submission {
            Submission::Exec(exec) => {
                let test_binary = self.compile_test_binary()?;
                let return_val = self.run_with_binaries(&test_binary, exec.name());
                remove_binary(&test_binary)?;
                return_val
            }
            Submission::Sources(sources) => {
                let compile_result = self.compile_submit_binary(sources, dirs)?;
                let submit_binary = match compile_result {
                    CompileResult::Ok(binary_name) => format!("./{}", binary_name),
                    CompileResult::Err(error) => return Ok(TestResult::Failed(error)),
                };
                let test_binary = self.compile_test_binary()?;
                let return_val = self.run_with_binaries(&test_binary, &submit_binary);
                remove_binary(&test_binary)?;
                remove_binary(&submit_binary)?;
                return_val
            }
        }
    }

    fn compile_submit_binary(
        &self,
        sources: &crate::question::submission::Sources,
        dirs: &QuestionDirs,
    ) -> Result<CompileResult, QuestionError> {
        let mut compiler = Compiler::new(&self.compiler);
        for source in sources.sources().iter() {
            compiler.add_source(format!("{}/{}", dirs.submit_directory, source));
        }
        if let Some(flags) = &sources.flags() {
            for flag in flags.iter() {
                compiler.add_flag(flag);
            }
        }
        compiler.compile()
    }

    fn compile_test_binary(&self) -> Result<String, QuestionError> {
        let mut compiler = Compiler::new(&self.compiler);
        for source in self.sources.iter() {
            compiler.add_source(source.clone());
        }
        if let Some(flags) = &self.flags {
            for flag in flags.iter() {
                compiler.add_flag(flag);
            }
        }
        let compile_result = compiler.compile()?;
        let binary = match compile_result {
            CompileResult::Ok(binary_name) => format!("./{}", binary_name),
            CompileResult::Err(error) => {
                panic!("Test compilation failed, invalid question: {}", error)
            }
        };
        Ok(binary)
    }

    fn run_with_binaries(
        &self,
        test_binary: &str,
        submit_binary: &str,
    ) -> Result<TestResult, QuestionError> {
        let mut trace = Trace::new();
        for args in self.args.iter() {
            let test_output = match run_binary_with_args(test_binary, args)? {
                BinaryResult::Output(output) => output,
                BinaryResult::Timeout => panic!("A question's test timed out, question is invalid"),
            };
            let submit_output = match run_binary_with_args(submit_binary, args)? {
                BinaryResult::Output(output) => output,
                BinaryResult::Timeout => return Ok(TestResult::Failed(TestError::Timeout)),
            };
            if test_output != submit_output {
                trace.binary_output(args, test_output, submit_output);
            }
        }
        if trace.exists() {
            Ok(TestResult::Failed(TestError::IncorrectOutput(trace)))
        } else {
            Ok(TestResult::Passed)
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

    fn run(
        &self,
        submission: &Submission,
        dirs: &QuestionDirs,
    ) -> Result<TestResult, QuestionError> {
        match submission {
            Submission::Sources(sources) => {
                let compile_result = self.compile_binary(sources, dirs)?;
                let binary = match compile_result {
                    CompileResult::Ok(binary_name) => format!("./{}", binary_name),
                    CompileResult::Err(error) => return Ok(TestResult::Failed(error)),
                };
                let result_val = self.run_with_binary(&binary);
                remove_binary(&binary)?;
                result_val
            }
            _ => Err(QuestionError::InvalidTestType(String::from(
                "Expected Output cannot be run with any submission type other than sources",
            ))),
        }
    }

    fn compile_binary(
        &self,
        sources: &crate::question::submission::Sources,
        dirs: &QuestionDirs,
    ) -> Result<CompileResult, QuestionError> {
        let mut compiler = Compiler::new(&self.compiler);
        for source in sources.sources().iter() {
            compiler.add_source(format!("{}/{}", dirs.submit_directory, source));
        }
        for source in self.sources.iter() {
            compiler.add_source(source.clone());
        }
        if let Some(flags) = &self.flags {
            for flag in flags.iter() {
                compiler.add_flag(flag);
            }
        }
        compiler.compile()
    }

    fn run_with_binary(&self, binary: &str) -> Result<TestResult, QuestionError> {
        let mut trace = Trace::new();
        let expected_out = fs::read_to_string(&self.stdout_file)?;
        let expected_err = fs::read_to_string(&self.stderr_file)?;
        let mut actual_out = String::new();
        let mut actual_err = String::new();
        for args in self.args.iter() {
            let output = match run_binary_with_args(binary, args)? {
                BinaryResult::Output(output) => output,
                BinaryResult::Timeout => return Ok(TestResult::Failed(TestError::Timeout)),
            };
            actual_out.push_str(output.stdout());
            actual_err.push_str(output.stderr());
        }
        if actual_out != expected_out || actual_err != expected_err {
            trace.file_outputs((expected_out, expected_err), (actual_out, actual_err));
            Ok(TestResult::Failed(TestError::IncorrectOutput(trace)))
        } else {
            Ok(TestResult::Passed)
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

    pub fn run(
        &self,
        submission: &Submission,
        dirs: &QuestionDirs,
        config: &Config,
    ) -> Result<TestResult, QuestionError> {
        match self {
            Self::Exec(exec) => exec.run(submission, dirs),
            Self::UnitTest(unit_test) => unit_test.run(submission, dirs, config),
            Self::Sources(sources) => sources.run(submission, dirs),
            Self::CompiledTogether(compiled_together) => compiled_together.run(submission, dirs),
        }
    }

    pub fn invalid_framework(&self, config: &crate::config::Config) -> Result<(), String> {
        match self {
            Self::UnitTest(unit_test) => unit_test.invalid_framework(config),
            _ => Ok(()),
        }
    }
}

#[derive(Debug)]
pub enum TestResult {
    Passed,
    Failed(TestError),
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::question::toml;
    use std::fs;
    #[test]
    fn read_test_toml() -> Result<(), QuestionError> {
        let buffer = fs::read_to_string("tst/resources/questions/hello_world/hello_world.toml")?;
        let dir_path = String::from("tst/resources/questions/hello_world");
        let question_toml: toml::Question = toml_parse::from_str(&buffer)?;
        let test_toml: toml::Test = question_toml.test;
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

    #[test]
    fn run_passing_test_exec() -> Result<(), QuestionError> {
        let config = Config::new_from("tst/resources/test_config2.toml").unwrap();
        let buffer = fs::read_to_string("tst/resources/questions/ft_countdown/ft_countdown.toml")?;
        let dir_path = String::from("tst/resources/questions/ft_countdown");
        let question_toml: toml::Question = toml_parse::from_str(&buffer)?;
        let dirs = QuestionDirs {
            submit_directory: "tst/resources/rendu_test/ft_countdown".into(),
            subject_directory: "tst/resources/questions/ft_countdown/ft_countdown.subject".into(),
            question_directory: "tst/resources/questions/ft_countdown".into(),
        };
        let test_toml: question::toml::Test = question_toml.test;
        let submission_toml: question::toml::Submission = question_toml.submission;
        let test: Test = Test::build_from_toml(test_toml, &dir_path)?;
        let submission: Submission = Submission::build_from_toml(submission_toml)?;
        let test_result = test.run(&submission, &dirs, &config)?;
        assert!(matches!(test_result, TestResult::Passed));
        Ok(())
    }

    #[test]
    fn run_failing_test() -> Result<(), QuestionError> {
        let config = Config::new_from("tst/resources/test_config2.toml").unwrap();
        let buffer = fs::read_to_string("tst/resources/questions/ft_countdown/ft_countdown.toml")?;
        let dir_path = String::from("tst/resources/questions/ft_countdown");
        let question_toml: toml::Question = toml_parse::from_str(&buffer)?;
        let dirs = QuestionDirs {
            submit_directory: "tst/resources/rendu_test/Z_failed_countdown".into(),
            subject_directory: "tst/resources/questions/ft_countdown/ft_countdown.subject".into(),
            question_directory: "tst/resources/questions/ft_countdown".into(),
        };
        let test_toml: toml::Test = question_toml.test;
        let submission_toml: toml::Submission = question_toml.submission;
        let test: Test = Test::build_from_toml(test_toml, &dir_path)?;
        let submission: Submission = Submission::build_from_toml(submission_toml)?;
        let test_result = test.run(&submission, &dirs, &config)?;
        let error = match test_result {
            TestResult::Passed => panic!("Test should have failed"),
            TestResult::Failed(error) => error,
        };
        let trace = match error {
            TestError::DoesNotCompile(e) => {
                panic!("This test case should compile correctly, but {}", e)
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
}

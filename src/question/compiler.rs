use crate::question::test::TestError;
use crate::question::QuestionError;
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use std::process::Command;

#[derive(Debug)]
pub enum CompileResult {
    Ok(String),
    Err(TestError),
}

impl CompileResult {
    pub fn unwrap(self) -> String {
        match self {
            Self::Ok(s) => s,
            Self::Err(_) => panic!("Unwrap called on CompileResult::Err"),
        }
    }
}

#[derive(Debug)]
pub struct Compiler<'a> {
    compiler: &'a str,
    sources: Vec<String>,
    flags: Vec<&'a str>,
}

impl<'a> Compiler<'a> {
    pub fn new(compiler: &'a str) -> Self {
        Self {
            compiler,
            sources: Vec::new(),
            flags: Vec::new(),
        }
    }

    pub fn add_source(&mut self, source: String) {
        self.sources.push(source);
    }

    pub fn add_flag(&mut self, flag: &'a str) {
        self.flags.push(flag);
    }

    pub fn compile(&self) -> Result<CompileResult, QuestionError> {
        let binary_name: String = thread_rng()
            .sample_iter(&Alphanumeric)
            .take(10)
            .map(char::from)
            .collect();
        let mut compile_exec = Command::new(self.compiler);
        compile_exec.arg("-o").arg(&binary_name);
        for flag in self.flags.iter() {
            compile_exec.arg(flag);
        }
        for source in self.sources.iter() {
            compile_exec.arg(source);
        }
        let output = compile_exec.output()?;
        if output.status.code().unwrap() != 0 {
            Ok(CompileResult::Err(TestError::DoesNotCompile(
                std::str::from_utf8(output.stderr.as_slice())
                    .unwrap()
                    .to_owned(),
            )))
        } else {
            Ok(CompileResult::Ok(binary_name))
        }
    }
}

pub fn remove_binary(binary: &str) -> Result<std::process::Output, QuestionError> {
    Command::new("rm")
        .arg(binary)
        .output()
        .map_err(|e| e.into())
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::utils::ProgramOutput;
    #[test]
    fn compile_test() -> Result<(), QuestionError> {
        let submit_sources = vec!["tst/resources/rendu_test/hello_world/hello_world.c"];
        let test_sources = vec!["tst/resources/questions/hello_world/main.c"];
        let submit_flags = vec![];
        let test_flags = vec!["-Wall", "-Wextra", "-Werror"];
        let test_compiler = String::from("gcc");
        let mut compiler = Compiler::new(&&test_compiler);
        for flag in test_flags.iter().chain(submit_flags.iter()) {
            compiler.add_flag(flag);
        }
        for source in test_sources.iter().chain(submit_sources.iter()) {
            compiler.add_source(source.to_string());
        }
        let compile_result = compiler.compile()?;
        assert!(matches!(compile_result, CompileResult::Ok(_)));
        let binary = format!("./{}", compile_result.unwrap());
        let output = Command::new(&binary).output()?;
        let output = ProgramOutput::new(output);
        assert_eq!(output.code(), 0);
        assert_eq!(output.stdout(), "hello world!\n");
        assert_eq!(output.stderr(), "");

        remove_binary(&binary)?;
        Ok(())
    }
}

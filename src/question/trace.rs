use crate::utils::ProgramOutput;
use std::fmt;

#[derive(Debug)]
pub struct Trace {
    data: String,
}

impl Trace {
    pub fn new() -> Self {
        Self {
            data: String::new(),
        }
    }

    pub fn exists(&self) -> bool {
        !self.data.is_empty()
    }

    pub fn binary_output(
        &mut self,
        args: &[String],
        expected: ProgramOutput,
        actual: ProgramOutput,
    ) {
        self.data += "Failure with args: ";
        for arg in args.iter() {
            self.data += arg;
            self.data += ", ";
        }
        self.data += "\n";
        self.data += "Expected Output:\n";
        self.data += &expected.to_string();
        self.data += "Actual Output:\n";
        self.data += &actual.to_string();
    }

    pub fn unit_test_output(&mut self, output: ProgramOutput) {
        self.data += "Unit Test failed. Output:\n";
        self.data += &output.to_string();
    }

    pub fn file_outputs(&mut self, expected: (String, String), actual: (String, String)) {
        let (expected_stdout, expected_stderr) = expected;
        let (actual_stdout, actual_stderr) = actual;
        self.data += "\nExpected Stdout:\n";
        self.data += &expected_stdout;
        self.data += "\n";
        self.data += "Actual Stdout:\n";
        self.data += &actual_stdout;
        self.data += "\n";
        self.data += "Expected Stderr:\n";
        self.data += &expected_stderr;
        self.data += "\n";
        self.data += "Actual Stderr:\n";
        self.data += &actual_stderr;
        self.data += "\n";
    }

    pub fn custom_message(&mut self, message: &str) {
        self.data += message;
        self.data += "\n";
    }
}

impl Default for Trace {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for Trace {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.data)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::Error;
    use std::fs;
    use std::process::Command;

    #[test]
    fn basic_trace() -> Result<(), Error> {
        let mut trace = Trace::new();
        let args = vec!["i_dont_exist.txt".to_owned(), "-e".to_owned()];
        let mut exec = Command::new("cat");
        for arg in args.iter() {
            exec.arg(arg);
        }
        let output: ProgramOutput = exec.output()?.into();
        assert!(output.code() != 0);
        trace.binary_output(&args, output.clone(), output);
        assert_eq!(
            trace.to_string(),
            format!(
                "{}{}{}{}{}{}{}{}{}",
                "Failure with args: i_dont_exist.txt, -e, \n",
                "Expected Output:\n",
                "Exit Code: 1\n",
                "Stdout: \n",
                "Stderr: cat: i_dont_exist.txt: No such file or directory\n\n",
                "Actual Output:\n",
                "Exit Code: 1\n",
                "Stdout: \n",
                "Stderr: cat: i_dont_exist.txt: No such file or directory\n\n",
            )
        );
        Ok(())
    }

    #[test]
    fn trace_against_files() -> Result<(), Error> {
        let mut trace = Trace::new();
        let expected_out = fs::read_to_string("tst/resources/questions/aff_a/aff_a.out")?;
        let expected_err = fs::read_to_string("tst/resources/questions/aff_a/aff_a.err")?;
        let actual_out = String::from("\na\n\n\na\na\n\na\n");
        let actual_err = String::new();
        trace.file_outputs((expected_out, expected_err), (actual_out, actual_err));
        assert_eq!(
            trace.to_string(),
            format!(
                "{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}",
                "\nExpected Stdout:\n",
                "\n",
                "a\n",
                "a\n",
                "\n",
                "a\n",
                "a\n",
                "\n",
                "a\n",
                "\nActual Stdout:\n",
                "\n",
                "a\n",
                "\n",
                "\n",
                "a\n",
                "a\n",
                "\n",
                "a\n",
                "\nExpected Stderr:\n",
                "\nActual Stderr:\n\n",
            )
        );
        Ok(())
    }
}

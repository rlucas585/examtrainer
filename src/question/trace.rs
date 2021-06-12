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

    pub fn extend(&mut self, args: &Vec<String>, expected: ProgramOutput, actual: ProgramOutput) {
        self.data += "Failure with args: ";
        for arg in args.iter() {
            self.data += arg;
            self.data += ", ";
        }
        self.data += "\n";
        self.data += "Expected Output: \n";
        self.data += &expected.to_string();
        self.data += "Actual Output: \n";
        self.data += &actual.to_string();
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
    use std::process::Command;

    // TODO: Change this to compare actual incorrect output against expected output. Not a
    // priority.
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
        trace.extend(&args, output.clone(), output);
        assert_eq!(
            trace.to_string(),
            format!(
                "{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n",
                "Failure with args: i_dont_exist.txt, -e, ",
                "Expected Output: ",
                "Exit Code: 1",
                "Stdout: ",
                "Stderr: cat: i_dont_exist.txt: No such file or directory",
                "Actual Output: ",
                "Exit Code: 1",
                "Stdout: ",
                "Stderr: cat: i_dont_exist.txt: No such file or directory",
            )
        );
        Ok(())
    }
}

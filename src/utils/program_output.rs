use std::process::Output;

#[derive(Debug, PartialEq)]
pub struct ProgramOutput {
    pub status: i32,
    pub stdout: String,
    pub stderr: String,
}

impl ProgramOutput {
    pub fn new(output: Output) -> Self {
        Self {
            status: output.status.code().unwrap(),
            stdout: String::from_utf8(output.stdout.as_slice().to_vec()).unwrap(),
            stderr: String::from_utf8(output.stderr.as_slice().to_vec()).unwrap(),
        }
    }

    pub fn from_strings(status: i32, stdout: String, stderr: String) -> Self {
        Self {
            status,
            stdout,
            stderr,
        }
    }

    pub fn code(&self) -> i32 {
        self.status
    }
    pub fn stdout(&self) -> &str {
        &self.stdout
    }
    pub fn stderr(&self) -> &str {
        &self.stderr
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::Error;
    use std::process::Command;
    #[test]
    fn test_output() -> Result<(), Error> {
        let output = Command::new("echo").arg("hello").arg("there").output()?;
        let output = ProgramOutput::new(output);
        assert_eq!(output.code(), 0);
        assert_eq!(output.stdout(), "hello there\n");
        assert_eq!(output.stderr(), "");
        Ok(())
    }

    #[test]
    fn test_error() -> Result<(), Error> {
        let output = Command::new("cat").arg("i_dont_exist.txt").output()?;
        let output = ProgramOutput::new(output);
        assert_eq!(output.code(), 1);
        assert_eq!(output.stdout(), "");
        assert_eq!(
            output.stderr(),
            "cat: i_dont_exist.txt: No such file or directory\n"
        );
        Ok(())
    }
}

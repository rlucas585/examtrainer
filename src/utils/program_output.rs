//! A simpler version of std::process::Output
//!
//! [`ProgramOutput`] is a simpler version of std::process::Output, reduced to just three components:
//! * The return code of the process
//! * STDOUT of the process
//! * STDERR of the process
//!
//! It implements [`PartialEq`], and is used to compare the results of an exam submission against the
//! expected answer, and to produce trace files.
//!
//! ## Initialization
//!
//! Initialization of a [`ProgramOutput`] is trivially done using [`ProgramOutput::new`], and an
//! [`Output`].
//!
//! ```rust
//! use examtrainer::utils::ProgramOutput;
//! use examtrainer::error::Error;
//! use std::process::Command;
//!
//! fn main() -> Result<(), Error> {
//!     let output = Command::new("echo").arg("hello").arg("there").output()?;
//!     let output = ProgramOutput::new(output);
//!     assert_eq!(output.code(), 0);
//!     assert_eq!(output.stdout(), "hello there\n");
//!     assert_eq!(output.stderr(), "");
//!     Ok(())
//! }
//! ```
//!
//! ## Comparison
//!
//! ```rust
//! use examtrainer::utils::ProgramOutput;
//! use examtrainer::error::Error;
//! use std::process::Command;
//!
//! fn main() -> Result<(), Error> {
//!     let output1 = Command::new("echo").arg("hello").arg("there").output()?;
//!     let output2 = Command::new("echo").arg("hello").arg("there").output()?;
//!     let output3 = Command::new("cat").arg("i_dont_exist.txt").arg("-n").output()?;
//!     assert_eq!(output1, output2);
//!     assert_ne!(output1, output3);
//!     assert_ne!(output2, output3);
//!     Ok(())
//! }
//! ```

use std::fmt;
use std::process::Output;

#[derive(Debug, PartialEq, Clone)]
pub struct ProgramOutput {
    status: i32,
    stdout: String,
    stderr: String,
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

    pub fn combine(self, other: ProgramOutput) -> Self {
        let status = self.status.max(other.status);
        let stdout = self.stdout + &other.stdout;
        let stderr = self.stderr + &other.stderr;
        Self {
            status,
            stdout,
            stderr,
        }
    }
}

pub fn join_outputs(outputs: Vec<ProgramOutput>) -> (String, String) {
    let mut stdout = String::new();
    let mut stderr = String::new();
    for output in outputs.into_iter() {
        stdout += output.stdout();
        stderr += output.stderr();
    }
    (stdout, stderr)
}

impl fmt::Display for ProgramOutput {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "Exit Code: {}", self.status)?;
        writeln!(f, "Stdout: {}", self.stdout)?;
        writeln!(f, "Stderr: {}", self.stderr)
    }
}

impl From<Output> for ProgramOutput {
    fn from(input: Output) -> ProgramOutput {
        ProgramOutput::new(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::Error;
    use std::process::Command;

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

    #[test]
    fn test_display() -> Result<(), Error> {
        let output1: ProgramOutput = Command::new("echo")
            .arg("hello")
            .arg("there")
            .output()?
            .into();
        let output2: ProgramOutput = Command::new("cat").arg("i_dont_exist.txt").output()?.into();
        let joined = output1.combine(output2);
        assert_eq!(
            joined.to_string(),
            format!(
                "{}{}{}",
                "Exit Code: 1\n",
                "Stdout: hello there\n\n",
                "Stderr: cat: i_dont_exist.txt: No such file or directory\n\n"
            )
        );
        Ok(())
    }
}

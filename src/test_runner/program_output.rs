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
}

use crate::question::QuestionError;
use crate::utils::ProgramOutput;
use std::process::{Command, Stdio};
use std::thread;
use std::time::{Duration, Instant};

// This time is for each instance of a binary running, this could perhaps be a part of the Question
// instead of a constant.
const TIMEOUT: u64 = 10;

#[derive(Debug)]
pub enum BinaryResult {
    Output(ProgramOutput),
    Timeout,
}

pub fn run_binary_with_args(binary: &str, args: &[String]) -> Result<BinaryResult, QuestionError> {
    let mut exec = Command::new(&binary);
    for arg in args.iter() {
        exec.arg(arg);
    }
    exec.stdout(Stdio::piped()).stderr(Stdio::piped());
    let mut child = exec.spawn()?;

    let start = Instant::now();
    let timeout = Duration::from_secs(TIMEOUT);
    let mut process_finished = false;

    while start.elapsed() < timeout {
        match child.try_wait() {
            Ok(Some(_)) => {
                process_finished = true;
                break;
            }
            Ok(None) => thread::sleep(Duration::from_millis(10)),
            Err(e) => return Err(e.into()),
        }
    }
    if process_finished {
        let output = child.wait_with_output()?;
        Ok(BinaryResult::Output(ProgramOutput::new(output)))
    } else {
        Ok(BinaryResult::Timeout)
    }
}

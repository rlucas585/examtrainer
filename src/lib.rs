pub mod config;
pub mod error;
mod output;
mod test_runner;
mod toml;

use crate::output::*;
use config::exams::{select_exam, Exam, Grade, Status};
use config::Config;
use error::Error;
use std::io::{self, Read, Write};

pub fn run(mut config: Config) -> Result<(), Error> {
    // TODO make some sort of loop here in future, to keep program open.
    create_submission_directory(&config.directories.submit_directory)?;
    let exam = select_exam(&config.directories.exam_directory)?;
    begin_exam(&config, &exam)?;
    Ok(())
}

fn create_submission_directory(submit_dir: &str) -> Result<(), Error> {
    if std::path::Path::new(submit_dir).exists() {
        Ok(())
    } else {
        println!("Creating submission directory...");
        match std::fs::create_dir(submit_dir).map_err(|e| e.into()) {
            Ok(_) => {
                println!("Submission directory created at {}", submit_dir);
                Ok(())
            }
            Err(e) => Err(e),
        }
    }
}

// TODO: Change to return some sort of Exam Result in future
fn begin_exam(config: &Config, exam: &Exam) -> Result<(), Error> {
    let mut status = Status::new(Grade::new(exam.pass_grade));
    print_exam_intro(&exam);
    std::io::stdin().read(&mut [0]).unwrap();

    status.start_exam(&exam);
    let _ = question_mode(config, exam, &mut status)?; // TODO change of course
    Ok(())
}

fn question_mode(config: &Config, exam: &Exam, status: &mut Status) -> Result<(), Error> {
    let assignment = exam.select_question(config, status)?;
    println!("{:?}", assignment);
    status.give_assignment(assignment)?;

    output::print_status(&status);

    let mut buffer = String::new();
    let stdin = io::stdin();

    loop {
        stdin.read_line(&mut buffer)?;
        match &buffer.trim().to_lowercase()[..] {
            "status" => output::print_status(&status),
            "grademe" => grade_assignment(status)?,
            "help" => output::print_help(),
            "quit" => return Ok(()),
            _ => output::print_unrecognised(&buffer),
        }
        buffer.clear();
    }
}

fn grade_assignment(status: &mut Status) -> Result<(), Error> {
    let mut buffer = String::new();
    let stdin = io::stdin();

    println!("You are about to grade:\n{}\n", status.current_assignment());
    print!("Are you sure you're reading to submit? [y/n]: ");
    io::stdout().flush()?;
    stdin.read_line(&mut buffer)?;
    match &buffer.trim().to_lowercase()[..] {
        "y" => {
            // TODO continue work in the Assignment and test_runner
        }
        _ => output::print_status(&status),
    }
    Ok(())
}

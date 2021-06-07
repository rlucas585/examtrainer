pub mod config;
pub mod error;
mod output;
mod test_runner;
mod toml;

use crate::output::*;
use config::exams::{select_exam, AttemptStatus, Exam, Grade, Status};
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

            "grademe" => {
                let _result = grade_assignment(exam, status)?;
                // TODO handle result of grading assignment (give new, or end exam)
            }

            "clear" => {
                print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
                io::stdout().flush()?;
                output::print_prompt();
            }

            "help" => output::print_help(),

            "quit" => {
                if quit()? == true {
                    return Ok(());
                }
            }
            _ => output::print_unrecognised(&buffer),
        }
        buffer.clear();
    }
}

fn quit() -> Result<bool, Error> {
    let mut buffer = String::new();
    let stdin = io::stdin();
    buffer.clear();
    print!("Quit exam session and return to Exam Select? [y/n]: ");
    io::stdout().flush()?;
    stdin.read_line(&mut buffer)?;
    match &buffer.trim().to_lowercase()[..] {
        "y" => Ok(true),
        _ => {
            output::print_prompt();
            Ok(false)
        }
    }
}

fn grade_assignment(exam: &Exam, status: &mut Status) -> Result<AttemptStatus, Error> {
    let mut buffer = String::new();
    let stdin = io::stdin();

    println!("You are about to grade:\n{}\n", status.current_assignment());
    print!("Are you sure you're reading to submit? [y/n]: ");
    io::stdout().flush()?;
    stdin.read_line(&mut buffer)?;
    match &buffer.trim().to_lowercase()[..] {
        "y" => status.grade_current_assignment(),
        _ => {
            output::print_status(&status);
            Ok(AttemptStatus::Current)
        }
    }
}

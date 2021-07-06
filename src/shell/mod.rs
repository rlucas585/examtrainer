mod exam;
pub mod main_menu;
mod single_question;

use crate::config::Config;
use crate::output;
use crate::question::test::TestError;
use crate::question::test::TestResult;
use crate::user::User;
use crate::utils::timestamp;
use crate::Error;
use colored::*;
use std::io::{self, Read, Write};

enum YesNoAnswer {
    Yes,
    No,
}

use YesNoAnswer::{No, Yes};

fn read_input() -> Result<String, Error> {
    let mut buffer = String::new();
    let stdin = io::stdin();

    stdin.read_line(&mut buffer)?;
    let new_line = buffer.find(|c: char| c == '\n');
    if let Some(trim_point) = new_line {
        buffer.truncate(trim_point);
    }
    Ok(buffer)
}

fn ask_yes_or_no() -> Result<YesNoAnswer, Error> {
    let input = read_input()?;
    match &input.to_lowercase()[..] {
        "y" => Ok(Yes),
        "yes" => Ok(Yes),
        _ => Ok(No),
    }
}

fn wait_for_enter() {
    println!("\n(Press Enter to continue...)\n");
    io::stdin().read_exact(&mut [0]).unwrap();
}

fn create_standard_directories(config: &Config) -> Result<(), Error> {
    std::process::Command::new("mkdir")
        .arg("-p")
        .arg(config.subject_dir())
        .arg(config.submit_dir())
        .arg(config.trace_dir())
        .output()?;
    Ok(())
}

fn ask_for_trace(trace_file: &str) -> Result<YesNoAnswer, Error> {
    println!("\nWould you like to save a trace file (y/n)? ");
    println!("File will be located at {}", trace_file.yellow());
    let answer = ask_yes_or_no()?;

    Ok(answer)
}

fn write_trace(trace_file: &str, test_error: TestError) -> Result<(), Error> {
    let mut trace_file = std::fs::File::create(trace_file)?;

    write!(&mut trace_file, "{}", test_error)?;
    Ok(())
}

fn grade(config: &Config, user: &mut User) -> Result<bool, Error> {
    println!("\nAre you sure you're ready to submit? (y/n)? ");
    let answer = ask_yes_or_no()?;
    match answer {
        Yes => {
            let test_result = user.grade(config)?;
            match test_result {
                TestResult::Passed => {
                    output::print_success();
                    wait_for_enter();
                    Ok(true)
                }
                TestResult::Failed(test_error) => {
                    output::print_failure();

                    let question_name = &user.get_last_assignment().unwrap().question_name;
                    let trace_file =
                        format!("{}/{}-{}", config.trace_dir(), timestamp(), question_name);

                    match ask_for_trace(&trace_file)? {
                        Yes => write_trace(&trace_file, test_error)?,
                        No => (),
                    }

                    Ok(false)
                }
            }
        }
        No => Ok(false),
    }
}

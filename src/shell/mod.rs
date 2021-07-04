use crate::config::Config;
use crate::output;
use crate::question::{Question, QuestionDB};
use crate::user::User;
use crate::Error;
use std::io::{self, Read, Write};

pub fn read_input() -> Result<String, Error> {
    let mut buffer = String::new();
    let stdin = io::stdin();

    stdin.read_line(&mut buffer)?;
    let new_line = buffer.find(|c: char| c == '\n');
    if let Some(trim_point) = new_line {
        buffer.truncate(trim_point);
    }
    Ok(buffer)
}

pub enum YesNoAnswer {
    Yes,
    No,
}

use YesNoAnswer::{No, Yes};

pub fn ask_yes_or_no() -> Result<YesNoAnswer, Error> {
    let input = read_input()?;
    match &input.to_lowercase()[..] {
        "y" => Ok(Yes),
        "yes" => Ok(Yes),
        _ => Ok(No),
    }
}

pub fn wait_for_enter() {
    println!("\n(Press Enter to continue...)\n");
    io::stdin().read(&mut [0]).unwrap();
}

pub fn create_standard_directories(config: &Config) -> Result<(), Error> {
    std::process::Command::new("mkdir")
        .arg("-p")
        .arg(config.subject_directory())
        .arg(config.submit_dir())
        .output()?;
    Ok(())
}

pub fn single_question_mode(
    config: &Config,
    question_name: &str,
    questions: &QuestionDB,
) -> Result<(), Error> {
    if let Some(question) = questions.get_question_by_name(question_name) {
        create_standard_directories(config)?;

        question.create_directories(config)?;

        let mut user = User::new();
        let mut input;
        user.assign_question(question, 1)?;

        output::single_question_intro(question);
        wait_for_enter();
        output::single_question_status(config, &user)?;

        loop {
            output::prompt();
            input = read_input()?;

            match &input[..] {
                "status" => output::single_question_status(config, &user)?,
                "help" => output::single_question_help(),
                "clear" => output::clear_screen()?,
                "exit" | "quit" => {
                    let answer = single_question_exit(config, question)?;
                    if matches!(answer, Yes) {
                        return Ok(());
                    }
                }
                _ => output::unrecognised_command(&input),
            }
        }
    } else {
        println!("The question '{}' was not found", question_name);
        Ok(())
    }
}

pub fn single_question_exit(config: &Config, question: &Question) -> Result<YesNoAnswer, Error> {
    println!("\nAre you sure you would like to exit without answering (y/n)? ");
    let answer = ask_yes_or_no()?;
    match answer {
        Yes => {
            println!("\nWould you like to delete the subject & answer directories? (y/n)? ");
            let answer = ask_yes_or_no()?;
            if matches!(answer, Yes) {
                question.delete_directories(config)?;
            }
            Ok(Yes)
        }
        No => Ok(No),
    }
}

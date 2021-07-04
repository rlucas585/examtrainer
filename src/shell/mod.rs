use crate::output;
use crate::question::QuestionDB;
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

pub fn wait_for_enter() {
    println!("(Press Enter to continue...)");
    io::stdin().read(&mut [0]).unwrap();
}

pub fn single_question_mode(question_name: &str, questions: &QuestionDB) -> Result<(), Error> {
    if let Some(question) = questions.get_question_by_name(question_name) {
        let mut user = User::new();
        let mut input;
        user.assign_question(question, 1)?;
        output::single_question_status(&user)?;

        loop {
            output::prompt();
            input = read_input()?;

            match &input[..] {
                "status" => output::single_question_status(&user)?,
                "help" => output::single_question_help(),
                "clear" => output::clear_screen()?,
                "exit" => return Ok(()),
                "quit" => return Ok(()),
                _ => output::unrecognised_command(&input),
            }
        }
    } else {
        println!("The question '{}' was not found", question_name);
        Ok(())
    }
}

use super::YesNoAnswer::{self, No, Yes};
use crate::config::Config;
use crate::output;
use crate::question::{Question, QuestionDB};
use crate::user::User;
use crate::Error;

pub fn run(config: &Config, question_name: &str, questions: &QuestionDB) -> Result<(), Error> {
    if let Some(question) = questions.get_question_by_name(question_name) {
        run_internal(config, question)
    } else {
        println!("The question '{}' was not found", question_name);
        Ok(())
    }
}

fn run_internal(config: &Config, question: &Question) -> Result<(), Error> {
    question.create_directories(config)?;

    let mut user = User::new();
    let mut input;
    user.assign_question(question, 1)?;

    output::single_question_intro(question);
    super::wait_for_enter();
    output::single_question_status(config, &user)?;

    loop {
        output::prompt();
        input = super::read_input()?;

        match &input[..] {
            "grademe" => {
                let answer_is_correct = super::grade(config, &mut user)?;
                match answer_is_correct {
                    true => return Ok(()),
                    false => user.assign_question(question, 1)?,
                }
            }
            "status" => output::single_question_status(config, &user)?,
            "clear" => output::clear_screen()?,
            "help" => output::single_question_help(),
            "exit" | "quit" => {
                let answer = exit(config, question)?;
                if matches!(answer, Yes) {
                    return Ok(());
                }
            }
            _ => output::unrecognised_command(&input),
        }
    }
}

fn exit(config: &Config, question: &Question) -> Result<YesNoAnswer, Error> {
    println!("\nAre you sure you would like to exit without answering (y/n)? ");
    let answer = super::ask_yes_or_no()?;
    match answer {
        Yes => {
            println!("\nWould you like to delete the subject & answer directories? (y/n)? ");
            println!("The following directories would be deleted:");
            println!("- {}", question.directories().submit_directory);
            println!(
                "- {}",
                format!("{}/{}", config.subject_dir(), question.name())
            );
            let answer = super::ask_yes_or_no()?;
            if matches!(answer, Yes) {
                question.delete_directories(config)?;
            }
            Ok(Yes)
        }
        No => Ok(No),
    }
}

use super::YesNoAnswer::{self, No, Yes};
use crate::config::Config;
use crate::output;
use crate::question::test::TestResult;
use crate::question::{Question, QuestionDB};
use crate::user::User;
use crate::utils::timestamp;
use crate::Error;

pub fn run(config: &Config, question_name: &str, questions: &QuestionDB) -> Result<(), Error> {
    if let Some(question) = questions.get_question_by_name(question_name) {
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
                    let correct_answer = grade(config, &mut user, question_name)?;
                    match correct_answer {
                        Yes => return Ok(()),
                        No => user.assign_question(question, 1)?,
                    }
                }
                "status" => output::single_question_status(config, &user)?,
                "help" => output::single_question_help(),
                "clear" => output::clear_screen()?,
                "exit" | "quit" => {
                    let answer = exit(config, question)?;
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

fn grade(config: &Config, user: &mut User, question_name: &str) -> Result<YesNoAnswer, Error> {
    println!("\nAre you sure you're ready to submit? (y/n)? ");
    let answer = super::ask_yes_or_no()?;
    match answer {
        Yes => {
            let test_result = user.grade(config)?;
            match test_result {
                TestResult::Passed => {
                    output::print_success();
                    super::wait_for_enter();
                    Ok(Yes)
                }
                TestResult::Failed(test_error) => {
                    output::print_failure();

                    let trace_file =
                        format!("{}/{}-{}", config.trace_dir(), timestamp(), question_name);

                    match super::ask_for_trace(&trace_file)? {
                        Yes => super::write_trace(&trace_file, test_error)?,
                        No => (),
                    }

                    Ok(No)
                }
            }
        }
        No => Ok(No),
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

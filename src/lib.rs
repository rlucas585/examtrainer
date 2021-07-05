pub mod config;
pub mod error;
pub mod exam;
pub mod output;
pub mod question;
pub mod shell;
pub mod user;
pub mod utils;

pub use error::Error;

use config::Config;
use exam::ExamDB;
use question::QuestionDB;
use shell::YesNoAnswer::Yes;

pub fn run(config: Config, questions: QuestionDB, exams: ExamDB) -> Result<(), Error> {
    let mut input;
    output::intro();

    // Main Menu loop
    loop {
        output::main_menu_prompt();
        input = shell::read_input()?;

        let args = input.as_str().split(' ').collect::<Vec<&str>>();
        let args = args.as_slice();

        match args {
            ["list", "questions"] => print!("{}", questions),
            ["list", "exams"] => print!("{}", exams),
            ["question", name] => shell::single_question_mode(&config, *name, &questions)?,
            ["config"] => output::print_config_info(&config),
            ["help"] => output::main_menu_help(),
            ["clear"] => output::clear_screen()?,
            ["exit"] | ["quit"] => return main_menu_exit(&config),
            _ => output::unrecognised_command(&input),
        }
    }
}

pub fn main_menu_exit(config: &Config) -> Result<(), Error> {
    println!("\nWould you like to delete the subject & answer directories? (y/n)? ");
    println!("The following directories would be deleted:");
    println!("- {}", config.subject_dir());
    println!("- {}", config.submit_dir());
    let answer = shell::ask_yes_or_no()?;
    if matches!(answer, Yes) {
        crate::utils::delete_directory(config.submit_dir())?;
        crate::utils::delete_directory(config.subject_dir())?;
    }
    Ok(())
}

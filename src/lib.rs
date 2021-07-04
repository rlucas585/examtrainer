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

pub fn run(config: Config, questions: QuestionDB, exams: ExamDB) -> Result<(), Error> {
    let mut input;
    output::intro();

    // Main Menu loop
    loop {
        output::main_menu_prompt();
        input = shell::read_input()?;

        match &input[..] {
            "list questions" => print!("{}", questions),
            "list exams" => print!("{}", exams),
            "help" => output::main_menu_help(),
            "clear" => output::clear_screen()?,
            "exit" => return Ok(()),
            "quit" => return Ok(()),
            _ => output::unrecognised_command(&input),
        }
    }
}

pub mod config;
pub mod error;
pub mod exam;
pub mod output;
pub mod question;
mod shell;
pub mod user;
pub mod utils;

pub use error::Error;

use config::Config;
use exam::ExamDB;
use question::QuestionDB;

pub fn run(config: Config, questions: QuestionDB, exams: ExamDB) -> Result<(), Error> {
    output::intro();

    shell::main_menu::run(config, questions, exams)
}

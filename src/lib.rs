pub mod config;
mod output;
mod test_runner;
mod toml;

use crate::output::*;
use config::exams::{Exam, QuestionAttempt};
use config::{Config, Error};

pub fn run(mut config: Config) -> Result<(), Error> {
    // TODO make some sort of loop here in future, to keep program open.
    create_submission_directory(&config.directories.submit_directory)?;
    let exam = Exam::select_exam(&config.directories.exam_directory)?;
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
    let mut status = Status::new(Grade::new(exam.config.pass_grade));
    print_exam_intro(exam);

    question_mode(config, exam, &status);
    Ok(())
}

fn question_mode(config: &Config, exam: &Exam, status: &Status) {
    exam.select_question(config, status);
    output::print_question_intro(exam, &status);
}

pub struct Grade {
    inner: u32,
    max: u32,
}

impl Grade {
    pub fn new(max: u32) -> Self {
        Self { inner: 0, max }
    }
}

pub struct Status {
    level: u32,
    points: u32,
    attempt: u32,
    grade: Grade,
    history: Vec<QuestionAttempt>,
}

impl Status {
    pub fn new(grade: Grade) -> Self {
        Self {
            level: 0,
            points: 0,
            attempt: 0,
            grade,
            history: Vec::new(),
        }
    }
}

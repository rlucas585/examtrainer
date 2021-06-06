use super::Error;
use crate::test_runner::TestRunner;
use crate::toml::ModuleToml;
use crate::{Config, Status};
use colored::*;
use serde::Deserialize;
use std::fmt;

#[derive(Debug, Deserialize)]
pub struct Info {
    pub name: String,
    pub authors: Option<Vec<String>>,
}

// TODO: Add validation for times at some point in future (allow max 59 for minutes and seconds)
#[derive(Debug, Deserialize)]
pub struct Time {
    hours: u32,
    minutes: u32,
    seconds: u32,
}

impl fmt::Display for Time {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}hrs, {}mins and {}sec",
            self.hours, self.minutes, self.seconds
        )
    }
}

#[derive(Debug, Deserialize)]
struct Range {
    min: u32,
    max: u32,
}

#[derive(Debug, Deserialize)]
pub struct ExamConfig {
    pub exam_type: String,
    exam_order: Option<String>,
    specific_order: Option<Vec<String>>,
    general_order: Option<Vec<Range>>,
    pub time: Time,
    points: Vec<usize>,
    points_config: Vec<Vec<u32>>,
    pub pass_grade: u32,
}

#[derive(Debug, Deserialize)]
pub struct Exam {
    pub info: Info,
    pub config: ExamConfig,
}

impl Exam {
    // TODO: Could greatly improve the selection process, display information about exams, allow
    // moving through options with arrow keys, etc.
    // Potentially store another file at $HOME/.config/examtrainer/userdata with information on
    // the user's history with an exam
    pub fn select_exam(exam_dir: &str) -> Result<Self, Error> {
        let toml = std::fs::read_to_string(format!("{}/{}", exam_dir, "Exam1.toml"))?;
        toml::from_str(&toml).map_err(|e| e.into())
    }

    // TODO: change to return TestRunner after development
    pub fn select_question(&self, config: &Config, status: &Status) -> Result<(), Error> {
        let mut module_iter = std::fs::read_dir(&config.directories.module_directory)?
            .filter(|elem| elem.as_ref().unwrap().path().is_file())
            .map(|elem| {
                toml::from_str::<ModuleToml>(
                    &std::fs::read_to_string(elem.as_ref().unwrap().path()).unwrap(),
                )
            })
            .filter(|elem| elem.is_ok())
            .map(|elem| elem.unwrap());
        for val in module_iter {
            println!("{:?}", val);
        }
        Ok(())
    }
}

pub enum AttemptStatus {
    Current,
    Passed,
    Failed,
}

impl fmt::Display for AttemptStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Current => write!(f, "{}", "Current".blue()),
            Self::Passed => write!(f, "{}", "Passed".green()),
            Self::Failed => write!(f, "{}", "Failed".red()),
        }
    }
}

pub struct QuestionAttempt {
    pub name: String,
    pub points: u32,
    pub attempt: u32,
    pub status: AttemptStatus,
}

impl fmt::Display for QuestionAttempt {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "    {}, {} for {} potential points ({})",
            format!("{}", self.attempt).yellow(),
            format!("{}", self.name).green(),
            format!("{}", self.points).green(),
            self.status
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::Error;
    #[test]
    fn read_exam() -> Result<(), Error> {
        let toml = std::fs::read_to_string("tst/modules/exam_1.toml")?;
        let exam: Exam = toml::from_str(&toml)?;
        assert_eq!(exam.config.exam_type, "specific");
        assert_eq!(exam.config.exam_order, Some(String::from("in_order")));
        println!("{:?}", exam);
        Ok(())
    }
}

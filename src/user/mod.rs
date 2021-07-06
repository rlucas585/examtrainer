pub mod attempt;

use crate::config::Config;
use crate::exam::Exam;
use crate::question::test::TestResult;
use crate::question::Question;
use crate::Error;
use colored::*;
use std::fmt;

use attempt::{Attempt, AttemptBuilder, Status};

struct History {
    pub attempts: Vec<Attempt>,
}

impl History {
    pub fn new() -> Self {
        Self {
            attempts: Vec::new(),
        }
    }

    pub fn push(&mut self, new_assignment: Attempt) {
        self.attempts.push(new_assignment)
    }

    pub fn has_passed_question(&self, question: &str) -> bool {
        self.attempts
            .iter()
            .any(|attempt| attempt.question_name == question && attempt.status == Status::Passed)
    }
}

impl fmt::Display for History {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.attempts.is_empty() {
            write!(f, "")
        } else {
            let mut current_level = self.attempts.last().unwrap().level;
            writeln!(f, "Assignments: ")?;
            writeln!(f, "  Level {}", format!("{}", current_level).green())?;
            for assignment in self.attempts.iter().rev() {
                current_level = if assignment.level < current_level {
                    writeln!(f, "  Level {}", format!("{}", assignment.level).green())?;
                    assignment.level
                } else {
                    current_level
                };
                writeln!(f, "    {}", assignment)?;
            }
            Ok(())
        }
    }
}

pub struct User<'a> {
    history: History,
    current_question: Option<&'a Question>,
    level: u32,
    attempt: u32,
    points: u32,
}

impl<'a> User<'a> {
    pub fn new() -> Self {
        Self {
            history: History::new(),
            current_question: None,
            level: 0,
            attempt: 0,
            points: 0,
        }
    }

    pub fn assign_question(&mut self, question: &'a Question, points: u32) -> Result<(), Error> {
        let new_assignment = AttemptBuilder::new()
            .name(question.name().to_string())
            .level(self.level)
            .attempt(self.attempt)
            .points(points)
            .build()?;
        self.history.push(new_assignment);
        self.current_question = Some(question);
        Ok(())
    }

    pub fn get_last_assignment(&self) -> Option<&Attempt> {
        self.history.attempts.last()
    }

    pub fn current_question(&self) -> Option<&Question> {
        self.current_question
    }

    pub fn current_question_name(&self) -> Option<&'a str> {
        self.current_question.map(|q| q.name())
    }

    pub fn grade(&mut self, config: &Config) -> Result<TestResult, Error> {
        let result = match self.current_question {
            Some(question) => question.grade(config).map_err(Error::Question)?,
            None => {
                return Err(Error::General(
                    "grade called on User with no assigned question!".to_string(),
                ))
            }
        };
        match result {
            TestResult::Passed => self.pass_question(result),
            TestResult::Failed(_) => self.fail_question(result),
        }
    }

    fn pass_question(&mut self, result: TestResult) -> Result<TestResult, Error> {
        let active_assignment = self.history.attempts.last_mut().ok_or_else(|| {
            Error::General("pass_question called for User without question assigned".to_string())
        })?;
        let points_gained = active_assignment.pass();
        self.points += points_gained;
        self.level += 1;
        self.attempt = 0;
        self.current_question = None;
        Ok(result)
    }

    fn fail_question(&mut self, result: TestResult) -> Result<TestResult, Error> {
        let active_assignment = self.history.attempts.last_mut().ok_or_else(|| {
            Error::General("pass_question called for User without question assigned".to_string())
        })?;
        active_assignment.fail();
        self.attempt += 1;
        self.current_question = None;
        Ok(result)
    }

    pub fn level(&self) -> u32 {
        self.level
    }

    pub fn attempt(&self) -> u32 {
        self.attempt
    }

    pub fn points(&self) -> u32 {
        self.points
    }

    pub fn completed_exam(&mut self, exam: &Exam) -> bool {
        if self.points >= exam.max_grade() {
            self.points = exam.max_grade();
            true
        } else {
            false
        }
    }

    pub fn print_history(&self) {
        println!("{}", self.history)
    }

    pub fn has_passed_question(&self, question: &str) -> bool {
        self.history.has_passed_question(question)
    }
}

impl<'a> Default for User<'a> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::exam::error::LevelError;
    use crate::exam::ExamError;
    use crate::question::QuestionDB;
    use attempt::Status;

    #[test]
    fn user_creation() -> Result<(), Error> {
        let user = User::new();

        assert_eq!(user.history.attempts.len(), 0);
        assert_eq!(user.points, 0);
        Ok(())
    }

    #[test]
    fn user_question_assign() -> Result<(), Error> {
        let config = Config::new_from("tst/resources/test_config1.toml")?;
        let question_database = QuestionDB::new(&config)?;
        let mut user = User::new();

        let question = question_database
            .get_question_by_name("hello_world")
            .ok_or(Error::Exam(ExamError::InvalidLevel(
                0,
                LevelError::MissingQuestion("hello_world".to_owned()),
            )))?;

        assert_eq!(user.current_question_name(), None);
        user.assign_question(question, 16)?;
        assert_eq!(user.current_question_name(), Some("hello_world"));

        let assignment = user.get_last_assignment();
        assert!(assignment.is_some());
        let assignment = assignment.unwrap();
        assert_eq!(assignment.question_name, "hello_world");
        assert_eq!(assignment.level, 0);
        assert_eq!(assignment.attempt, 0);
        assert_eq!(assignment.points, 16);
        assert!(matches!(assignment.status, Status::Current));
        Ok(())
    }

    #[test]
    fn user_grade_correct_answer() -> Result<(), Error> {
        let config = Config::new_from("tst/resources/test_config2.toml")?;
        let question_database = QuestionDB::new(&config)?;
        let question = question_database.get_question_by_name("aff_a");

        assert!(question.is_some());
        let question = question.unwrap();

        let mut user = User::new();
        user.assign_question(question, 16)?;

        assert_eq!(user.points, 0);
        assert_eq!(user.level, 0);

        let test_result = user.grade(&config)?;
        match test_result {
            TestResult::Passed => (),
            TestResult::Failed(_) => panic!("This test should have passed"),
        }

        assert_eq!(user.points, 16);
        assert_eq!(user.level, 1);

        let assignment = user.get_last_assignment();
        assert!(assignment.is_some());
        let assignment = assignment.unwrap();
        assert!(matches!(assignment.status, Status::Passed));
        Ok(())
    }
}

#[cfg(test)]
mod display {
    use super::*;
    use crate::question::QuestionDB;

    #[test]
    fn history_display() -> Result<(), Error> {
        let config = Config::new_from("tst/resources/test_config2.toml")?;
        let question_database = QuestionDB::new(&config)?;

        let question = question_database.get_question_by_name("aff_a");
        assert!(question.is_some());
        let question = question.unwrap();

        let mut user = User::new();

        println!("{}", user.history);
        user.assign_question(question, 16)?;
        println!("{}", user.history);
        user.grade(&config)?;
        println!("{}", user.history);

        let question = question_database.get_question_by_name("hello_world");
        assert!(question.is_some());
        let question = question.unwrap();

        user.assign_question(question, 10)?;
        println!("{}", user.history);
        user.grade(&config)?;
        // println!("{}", user.history);

        Ok(())
    }
}

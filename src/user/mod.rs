use crate::config::Config;
use crate::question::test::TestResult;
use crate::question::Question;
use crate::Error;

enum Status {
    Current,
    Passed,
    Failed,
}

pub struct Attempt {
    question_name: String,
    level: u32,
    attempt: u32,
    points: u32,
    status: Status,
}

impl Attempt {
    pub fn pass(&mut self) -> u32 {
        self.status = Status::Passed;
        self.points
    }

    pub fn fail(&mut self) {
        self.status = Status::Failed;
    }
}

struct AttemptBuilder {
    question_name: Option<String>,
    level: Option<u32>,
    attempt: Option<u32>,
    points: Option<u32>,
}

impl AttemptBuilder {
    pub fn new() -> Self {
        Self {
            question_name: None,
            level: None,
            attempt: None,
            points: None,
        }
    }
    pub fn name(mut self, name: String) -> Self {
        self.question_name = Some(name);
        self
    }
    pub fn level(mut self, level: u32) -> Self {
        self.level = Some(level);
        self
    }
    pub fn attempt(mut self, attempt: u32) -> Self {
        self.attempt = Some(attempt);
        self
    }
    pub fn points(mut self, points: u32) -> Self {
        self.points = Some(points);
        self
    }
    pub fn build(self) -> Result<Attempt, Error> {
        match self {
            Self {
                question_name: Some(question_name),
                level: Some(level),
                attempt: Some(attempt),
                points: Some(points),
            } => Ok(Attempt {
                question_name,
                level,
                attempt,
                points,
                status: Status::Current,
            }),
            _ => Err(Error::General(
                "build called on incomplete AttemptBuilder".to_string(),
            )),
        }
    }
}

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

    pub fn get_current_assignment(&self) -> Option<&Attempt> {
        self.history.attempts.last()
    }

    pub fn current_question_name(&self) -> Option<&'a str> {
        match self.current_question {
            Some(q) => Some(q.name()),
            None => None,
        }
    }

    pub fn grade(&mut self, config: &Config) -> Result<TestResult, Error> {
        let result = match self.current_question {
            Some(question) => question.grade(config).map_err(|e| Error::Question(e))?,
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
        let active_assignment = self.history.attempts.last_mut().ok_or(Error::General(
            "pass_question called for User without question assigned".to_string(),
        ))?;
        let points_gained = active_assignment.pass();
        self.points += points_gained;
        self.level += 1;
        self.attempt = 0;
        self.current_question = None;
        Ok(result)
    }

    fn fail_question(&mut self, result: TestResult) -> Result<TestResult, Error> {
        let active_assignment = self.history.attempts.last_mut().ok_or(Error::General(
            "pass_question called for User without question assigned".to_string(),
        ))?;
        active_assignment.fail();
        self.attempt += 1;
        self.current_question = None;
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::exam::error::LevelError;
    use crate::exam::ExamError;
    use crate::question::QuestionDB;
    use crate::Error;

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

        let assignment = user.get_current_assignment();
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

        let assignment = user.get_current_assignment();
        assert!(assignment.is_some());
        let assignment = assignment.unwrap();
        assert!(matches!(assignment.status, Status::Passed));
        Ok(())
    }

    #[test]
    fn attempt() -> Result<(), Error> {
        let attempt = AttemptBuilder::new()
            .name("hello_world".to_string())
            .level(0)
            .attempt(0)
            .points(16)
            .build()?;
        assert_eq!(attempt.question_name, "hello_world");
        assert_eq!(attempt.level, 0);
        assert_eq!(attempt.attempt, 0);
        assert_eq!(attempt.points, 16);
        assert!(matches!(attempt.status, Status::Current));
        Ok(())
    }
}

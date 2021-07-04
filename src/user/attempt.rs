use crate::Error;
use colored::*;
use std::fmt;

pub enum Status {
    Current,
    Passed,
    Failed,
}

impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Status::Current => write!(f, "{}", format!("Current").blue()),
            Status::Passed => write!(f, "{}", format!("Passed").green()),
            Status::Failed => write!(f, "{}", format!("Failed").red()),
        }
    }
}

pub struct Attempt {
    pub question_name: String,
    pub level: u32,
    pub attempt: u32,
    pub points: u32,
    pub status: Status,
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

impl fmt::Display for Attempt {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}: {} for {} potential points ({})",
            format!("{}", self.attempt).yellow(),
            format!("{}", self.question_name).green(),
            self.points,
            self.status
        )
    }
}

pub struct AttemptBuilder {
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

#[cfg(test)]
mod tests {
    use super::*;

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

#[cfg(test)]
mod display {
    use super::*;

    #[test]
    fn attempt_display() -> Result<(), Error> {
        let attempt = AttemptBuilder::new()
            .name("hello_world".to_string())
            .level(0)
            .attempt(0)
            .points(16)
            .build()?;
        println!("{}", attempt);
        Ok(())
    }
}

pub mod error;
mod grades;
mod level;
mod toml;

pub use error::ExamError;
use grades::Grades;
use std::time::Duration;

#[derive(Debug)]
pub struct Exam {
    name: String,
    grades: Grades,
    time: Duration,
}

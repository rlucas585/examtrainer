pub mod error;
mod grades;
mod level;
mod toml;

use crate::question::QuestionDB;
pub use error::ExamError;
use grades::Grades;
use level::Level;
use std::fs::DirEntry;
use std::time::Duration;

#[derive(Debug)]
pub struct Exam {
    name: String,
    grades: Grades,
    time: Duration,
    levels: Vec<Level>,
}

impl Exam {
    pub fn build_from_toml(toml: toml::Exam, database: &QuestionDB) -> Result<Self, ExamError> {
        let name = toml.info.name;
        let grades = Grades::new_from_toml(toml.grades)?;
        let time = convert_time_to_duration(toml.time)?;
        let levels = create_levels(toml.levels, database)?;
        Ok(Self {
            name,
            grades,
            time,
            levels,
        })
    }
}

fn create_levels(
    toml_vec: Vec<toml::Level>,
    database: &QuestionDB,
) -> Result<Vec<Level>, ExamError> {
    if toml_vec.len() == 0 {
        return Err(ExamError::NoLevels);
    }

    let mut out_vec = Vec::new();
    for (index, level) in toml_vec.into_iter().enumerate() {
        let new_level = match Level::build_from_toml(level, database) {
            Ok(new_level) => new_level,
            Err(error) => return Err(ExamError::InvalidLevel(index, error)),
        };
        out_vec.push(new_level);
    }
    Ok(out_vec)
}

fn convert_time_to_duration(time: toml::Time) -> Result<Duration, ExamError> {
    let toml::Time {
        hours,
        minutes,
        seconds,
    } = time;
    if minutes > 59 || seconds > 59 {
        Err(ExamError::InvalidTime)
    } else {
        let mut total_seconds = seconds;
        total_seconds += minutes * 60;
        total_seconds += hours * 3600;
        Ok(Duration::from_secs(total_seconds as u64))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::Config;
    use crate::error::Error;
    #[test]
    fn exam_creation() -> Result<(), Error> {
        let config = Config::new_from("tst/resources/test_config2.toml")?;
        let question_database = QuestionDB::new(&config)?;
        let exam_text = r#"
            [info]
            name = "Exam_prototype"
            authors = [
                "Ryan Lucas"
            ]

            [time]
            hours = 0
            minutes = 20
            seconds = 0

            [grades]
            pass = 50
            max = 100
                        
            [[levels]]
            type = "random"
            questions = ["only_a", "only_z", "hello", "ft_countdown", "ft_print_numbers"]
            points = [16, 11, 7, 2, 0]

            [[levels]]
            type = "random"
            questions = ["aff_a", "aff_first_param", "aff_last_param"]
            points = [16, 11, 7, 2, 0]
            "#;
        let decoded: toml::Exam =
            toml_parse::from_str(exam_text).map_err(|e| Error::Exam(e.into()))?;
        let exam = Exam::build_from_toml(decoded, &question_database)?;
        assert_eq!(exam.name, "Exam_prototype");
        assert_eq!((exam.grades.pass(), exam.grades.max()), (50, 100));
        assert_eq!(exam.time, Duration::from_secs(1200));
        assert_eq!(exam.levels.len(), 2);
        Ok(())
    }
}

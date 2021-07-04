use super::{Exam, ExamError};
use crate::config::Config;
use crate::question::QuestionDB;
use colored::*;
use std::collections::HashMap;
use std::fmt;
use std::fs::DirEntry;

#[derive(Debug)]
pub struct ExamDB {
    exams: HashMap<String, Exam>,
}

impl ExamDB {
    pub fn new(config: &Config, database: &QuestionDB) -> Result<Self, ExamError> {
        let exam_files = std::fs::read_dir(config.exam_dir())?.filter(|entry| {
            if let Ok(file) = entry {
                file.path().is_file()
            } else {
                false
            }
        });
        let mut exams = HashMap::new();
        for file in exam_files.into_iter().flatten() {
            match Exam::build_from_dir_entry(&file, database) {
                Ok(exam) => insert_new_exam(&mut exams, exam),
                Err(e) => print_exam_error(&file, e),
            }
        }
        Ok(Self { exams })
    }

    pub fn get_exam_by_name(&self, name: &str) -> Option<&Exam> {
        self.exams.get(name)
    }
}

impl fmt::Display for ExamDB {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (_, exam) in self.exams.iter() {
            writeln!(f, "{}", exam)?;
        }
        Ok(())
    }
}

fn insert_new_exam(exams: &mut HashMap<String, Exam>, exam: Exam) {
    if exams.get(exam.name()).is_some() {
        eprintln!(
            "{}",
            format!(
                "Warning: The exam {} appeared twice, second instance was ignored",
                exam.name()
            )
            .yellow()
        );
    } else {
        exams.insert(exam.name().to_string(), exam);
    }
}

fn print_exam_error(dir_entry: &DirEntry, e: ExamError) {
    eprintln!(
        "{}",
        format!(
            "Warning: Unable to generate exam from {}: \"{}\"",
            dir_entry.path().display(),
            e
        )
        .yellow()
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::Error;
    use std::time::Duration;

    #[test]
    fn get_exam_by_name() -> Result<(), Error> {
        let config = Config::new_from("tst/resources/test_config2.toml")?;
        let question_database = QuestionDB::new(&config)?;
        let exam_database = ExamDB::new(&config, &question_database)?;
        let exam = exam_database.get_exam_by_name("Exam_prototype");
        assert!(exam.is_some());
        let exam = exam.unwrap();
        assert_eq!(exam.name, "Exam_prototype");
        assert_eq!((exam.grades.pass(), exam.grades.max()), (50, 100));
        assert_eq!(exam.time, Duration::from_secs(1200));
        assert_eq!(exam.levels.len(), 2);
        Ok(())
    }
}

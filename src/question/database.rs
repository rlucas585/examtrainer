use super::{Question, QuestionError};
use crate::config::Config;
use crate::error::Error;
use crate::utils::Range;
use colored::*;
use std::collections::HashMap;
use std::fs::DirEntry;

#[derive(Debug)]
pub struct QuestionDB {
    questions: HashMap<String, Question>,
}

impl QuestionDB {
    /// Generate a database ([`HashMap`]) of [`Question`]'s.
    ///
    /// [`QuestionDB::new`] iterates over all directories within the `question_directory` specified
    /// in [`Config`], and attempts to generate a [`Question`] from each of them. If a [`Question`]
    /// cannot be created for any reason, then a Warning is displayed on the screen, along with the
    /// reason for failure (a [`QuestionError`]).
    pub fn new(config: &Config) -> Result<Self, Error> {
        let question_dirs =
            std::fs::read_dir(&config.directories.question_directory)?.filter(|entry| {
                if let Ok(file) = entry {
                    file.path().is_dir()
                } else {
                    false
                }
            });
        let mut questions = HashMap::new();
        for dir in question_dirs.into_iter().flatten() {
            match Question::build_from_dir_entry(config, &dir) {
                Ok(question) => insert_new_question(&mut questions, question),
                Err(e) => print_question_error(&dir, e),
            }
        }
        Ok(Self { questions })
    }

    pub fn get_question_by_name(&self, name: &str) -> Option<&Question> {
        self.questions.get(name)
    }

    pub fn get_questions_by_difficulty_range(&self, range: &Range) -> Vec<&Question> {
        let mut out = Vec::new();
        for (_, question) in self.questions.iter() {
            if question.has_difficulty_in_range(range) {
                out.push(question);
            }
        }
        out
    }
}

fn insert_new_question(questions: &mut HashMap<String, Question>, question: Question) {
    if questions.get(question.name()).is_some() {
        eprintln!(
            "{}",
            format!(
                "Warning: The question {} appeared twice, second instance was ignored",
                question.name()
            )
            .yellow()
        );
    } else {
        questions.insert(question.name().to_string(), question);
    }
}

fn print_question_error(dir_entry: &DirEntry, e: QuestionError) {
    eprintln!(
        "{}",
        format!(
            "Warning: Unable to generate question from {}: \"{}\"",
            dir_entry.path().display(),
            e
        )
        .yellow()
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    // This test may need to be updated as more test questions are added to the tst/resources
    // directory.
    #[test]
    fn generate_questions() -> Result<(), Error> {
        let config = Config::new_from("tst/resources/test_config1.toml")?;
        let _question_database = QuestionDB::new(&config)?;
        Ok(())
    }

    #[test]
    fn get_question_by_name() -> Result<(), Error> {
        let config = Config::new_from("tst/resources/test_config1.toml")?;
        let question_database = QuestionDB::new(&config)?;
        let question = question_database.get_question_by_name("hello_world");
        assert!(question.is_some());
        let question = question.unwrap();
        assert_eq!(question.difficulty(), 2);
        Ok(())
    }

    #[test]
    fn get_questions_by_difficulty() -> Result<(), Error> {
        let config = Config::new_from("tst/resources/test_config1.toml")?;
        let question_database = QuestionDB::new(&config)?;
        let range = Range::new(2, 4)?;
        let viable_questions = question_database.get_questions_by_difficulty_range(&range);
        assert!(viable_questions.iter().any(|&q| q.name() == "hello_world"));
        assert!(viable_questions.iter().any(|&q| q.name() == "aff_a"));
        assert!(!viable_questions.iter().any(|&q| q.name() == "first_word"));
        let range = Range::new(4, 8)?;
        let viable_questions = question_database.get_questions_by_difficulty_range(&range);
        assert!(!viable_questions.iter().any(|&q| q.name() == "hello_world"));
        assert!(viable_questions.iter().any(|&q| q.name() == "aff_a"));
        assert!(viable_questions.iter().any(|&q| q.name() == "first_word"));
        Ok(())
    }
}

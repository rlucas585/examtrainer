use crate::exam;
use crate::exam::error::LevelError;
use crate::question::QuestionDB;
use crate::user::User;
use rand::Rng;

#[derive(Debug)]
enum LevelType {
    Random,
    Repeat,
}

impl LevelType {
    fn new(origin: String) -> Result<Self, LevelError> {
        match &origin[..] {
            "random" => Ok(Self::Random),
            "random_repeat" => Ok(Self::Repeat),
            _ => Err(LevelError::InvalidType(origin)),
        }
    }
}

#[derive(Debug)]
pub struct Level {
    kind: LevelType,
    questions: Vec<String>,
    points: Vec<u32>,
}

impl Level {
    pub fn build_from_toml(
        toml: exam::toml::Level,
        database: &QuestionDB,
    ) -> Result<Self, LevelError> {
        let kind = LevelType::new(toml.kind)?;
        if toml.questions.is_empty() {
            Err(LevelError::NoQuestions)
        } else if toml.points.is_empty() {
            Err(LevelError::NoPoints)
        } else {
            for question in toml.questions.iter() {
                if database.get_question_by_name(question).is_none() {
                    return Err(LevelError::MissingQuestion(question.clone()));
                }
            }
            Ok(Self {
                kind,
                questions: toml.questions,
                points: toml.points,
            })
        }
    }

    pub fn select_question(&self, user: &User) -> Option<&str> {
        match self.kind {
            LevelType::Random => self.random_select(user),
            LevelType::Repeat => self.random_repeat_select(user),
        }
    }

    fn random_select(&self, user: &User) -> Option<&str> {
        let possible_questions = self.possible_questions(user);

        if possible_questions.is_empty() {
            None
        } else {
            let size = possible_questions.len();
            let index = rand::thread_rng().gen_range(0..size);
            self.questions.get(index).map(|x| &**x)
        }
    }

    fn random_repeat_select(&self, user: &User) -> Option<&str> {
        if let Some(last_assignment) = user.get_last_assignment() {
            if last_assignment.is_failed() {
                self.questions
                    .iter()
                    .find(|elem| **elem == last_assignment.question_name)
                    .map(|x| &**x)
            } else {
                self.random_select(user)
            }
        } else {
            self.random_select(user)
        }
    }

    fn possible_questions(&self, user: &User) -> Vec<&str> {
        self.questions
            .iter()
            .filter(|q| !user.has_passed_question(q))
            .map(|e| e.as_ref())
            .collect()
    }

    pub fn get_points(&self, user: &User) -> u32 {
        let index = (user.attempt() as usize).min(self.points.len() - 1);
        self.points[index]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::Config;
    #[test]
    fn level_creation() -> Result<(), LevelError> {
        let config = Config::new_from("tst/resources/test_config2.toml")
            .map_err(|_| LevelError::NoQuestions)?;
        let question_database = QuestionDB::new(&config).map_err(|_| LevelError::NoQuestions)?;
        let level_text = r#"
            type = "random"
            questions = ["only_a", "only_z", "hello", "ft_countdown", "ft_print_numbers"]
            points = [16, 11, 7, 2, 0]
            "#;
        let decoded: exam::toml::Level =
            toml_parse::from_str(level_text).map_err(|_| LevelError::NoQuestions)?;
        let level = Level::build_from_toml(decoded, &question_database)?;
        assert!(matches!(level.kind, LevelType::Random));
        assert_eq!(
            level.questions,
            vec![
                "only_a",
                "only_z",
                "hello",
                "ft_countdown",
                "ft_print_numbers"
            ]
        );
        assert_eq!(level.points, vec![16, 11, 7, 2, 0]);
        Ok(())
    }
}

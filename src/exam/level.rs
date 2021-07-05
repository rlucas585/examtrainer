use crate::exam;
use crate::exam::error::LevelError;
use crate::question::QuestionDB;

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

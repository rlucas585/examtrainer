use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Info {
    name: String,
    authors: Option<Vec<String>>,
}

// TODO: Add validation for times at some point in future (allow max 59 for minutes and seconds)
#[derive(Debug, Deserialize)]
struct Time {
    hours: u32,
    minutes: u32,
    seconds: u32,
}

#[derive(Debug, Deserialize)]
struct Range {
    min: u32,
    max: u32,
}

#[derive(Debug, Deserialize)]
pub struct ExamConfig {
    exam_type: String,
    exam_order: Option<String>,
    specific_order: Option<Vec<String>>,
    general_order: Option<Vec<Range>>,
    time: Time,
}

#[derive(Debug, Deserialize)]
pub struct Exam {
    info: Info,
    config: ExamConfig,
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

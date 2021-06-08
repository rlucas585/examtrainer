use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Info {
    pub name: String,
    pub authors: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
pub struct Time {
    pub hours: u32,
    pub minutes: u32,
    pub seconds: u32,
}

#[derive(Debug, Deserialize)]
struct Range {
    min: u32,
    max: u32,
}

#[derive(Debug, Deserialize)]
pub struct ExamConfig {
    pub exam_order: Option<String>,
    pub specific_order: Option<Vec<String>>,
    pub general_order: Option<Vec<Vec<u32>>>,
    pub point_indexes: Vec<usize>,
    pub points: Vec<Vec<u32>>,
}

#[derive(Debug, Deserialize)]
pub struct Exam {
    pub info: Info,
    pub exam_type: String,
    pub config: ExamConfig,
    pub time: Time,
    pub pass_grade: u32,
    pub max_grade: u32,
}

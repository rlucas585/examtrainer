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
pub struct Grades {
    pub pass: u32,
    pub max: u32,
}

#[derive(Debug, Deserialize)]
pub struct Level {
    #[serde(rename = "type")]
    pub kind: String,
    pub questions: Vec<String>,
    pub points: Vec<u32>,
}

#[derive(Debug, Deserialize)]
pub struct Exam {
    pub info: Info,
    pub time: Time,
    pub grades: Grades,
    pub levels: Vec<Level>,
}

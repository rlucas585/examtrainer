use super::toml;
use super::ExamError;

#[derive(Debug)]
pub struct Grades {
    pass: u32,
    max: u32,
}

impl Grades {
    pub fn new_from_toml(toml: toml::Grades) -> Result<Self, ExamError> {
        if toml.pass > toml.max {
            Err(ExamError::InvalidGrade)
        } else {
            Ok(Self {
                pass: toml.pass,
                max: toml.max,
            })
        }
    }

    pub fn pass(&self) -> u32 {
        self.pass
    }
    pub fn max(&self) -> u32 {
        self.max
    }
}

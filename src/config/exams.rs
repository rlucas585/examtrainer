use crate::error::Error;
use crate::test_runner::TestRunner;
use crate::toml::ModuleToml;
use crate::Config;
use chrono::prelude::*;
use colored::*;
use std::fmt;
use std::str::FromStr;
use std::time::{Duration, Instant};

struct PointsSelector {
    indexes: Vec<usize>,
    points: Vec<Vec<u32>>,
}

impl PointsSelector {
    pub fn new(indexes: Vec<usize>, points: Vec<Vec<u32>>) -> Result<Self, Error> {
        for index in indexes.iter() {
            if *index >= points.len() {
                return Err(Error::Parse(format!(
                    "Invalid points configuration. \
Index of {} supplied but length of points array is {}",
                    index,
                    points.len()
                )));
            }
        }
        Ok(Self { indexes, points })
    }

    fn get_points(&self, level: usize, attempt: usize) -> u32 {
        let level = if level > self.indexes.len() {
            self.indexes.len() - 1
        } else {
            level
        };
        let attempt = if self.points[level].len() > attempt {
            self.points[level].len() - 1
        } else {
            attempt
        };
        self.points[level][attempt]
    }
}

struct Specific {
    order: Vec<String>,
    point_selector: PointsSelector,
}

impl Specific {
    pub fn build_from_toml(toml: crate::toml::exam::ExamConfig) -> Result<Self, Error> {
        if let Some(specific_order) = toml.specific_order {
            let point_selector = PointsSelector::new(toml.point_indexes, toml.points)?;
            Ok(Self {
                order: specific_order,
                point_selector,
            })
        } else {
            Err("No specific_order supplied for exam type \"specific\"".into())
        }
    }
}

struct Range {
    min: u32,
    max: u32,
}

impl Range {
    pub fn from_u32_vec(general_order: Vec<Vec<u32>>) -> Result<Vec<Self>, Error> {
        let mut output = Vec::new();
        for val in general_order.into_iter() {
            if val.len() != 2 {
                return Err("Index in general_order without exactly 2 values".into());
            }
            output.push(Range {
                min: val[0],
                max: val[1],
            })
        }
        Ok(output)
    }
}

struct General {
    order: Vec<Range>,
    point_selector: PointsSelector,
}

impl General {
    pub fn build_from_toml(toml: crate::toml::exam::ExamConfig) -> Result<Self, Error> {
        if let Some(general_order) = toml.general_order {
            let point_selector = PointsSelector::new(toml.point_indexes, toml.points)?;
            let order = Range::from_u32_vec(general_order)?;
            Ok(Self {
                order,
                point_selector,
            })
        } else {
            Err("No general_order supplied for exam type \"general\"".into())
        }
    }
}

enum ExamBuilder {
    Specific,
    General,
}

impl ExamBuilder {
    pub fn build(self, toml: crate::toml::exam::Exam) -> Result<Exam, Error> {
        match self {
            Self::Specific => {
                let specific = Specific::build_from_toml(toml.config)?;
                let time = Time::build_from_toml(toml.time)?;
                Ok(Exam::new(
                    toml.info.name,
                    time,
                    ExamType::Specific(specific),
                    toml.pass_grade,
                ))
            }
            Self::General => {
                let general = General::build_from_toml(toml.config)?;
                let time = Time::build_from_toml(toml.time)?;
                Ok(Exam::new(
                    toml.info.name,
                    time,
                    ExamType::General(general),
                    toml.pass_grade,
                ))
            }
        }
    }
}

impl FromStr for ExamBuilder {
    type Err = Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "specific" => Ok(Self::Specific),
            "general" => Ok(Self::General),
            invalid => Err(Error::Parse(format!("Invalid exam type: {}", invalid))),
        }
    }
}

enum ExamType {
    Specific(Specific),
    General(General),
}

impl ExamType {
    pub fn select_question(
        &self,
        config: &Config,
        status: &Status,
    ) -> Result<(TestRunner, String), Error> {
        // TODO When the program's structure is better defined, use some sort of database to
        // perform selection, instead of expensive reads.
        let modules = std::fs::read_dir(&config.directories.module_directory)?
            .filter(|elem| elem.as_ref().unwrap().path().is_file())
            .map(|elem| {
                toml::from_str::<ModuleToml>(
                    &std::fs::read_to_string(elem.as_ref().unwrap().path()).unwrap(),
                )
            })
            .filter(|elem| elem.is_ok())
            .map(|elem| elem.unwrap())
            .filter(|elem| self.filter_with_examtype(elem, status))
            .collect::<Vec<_>>();

        if modules.len() == 0 {
            return Err("No modules available".into());
        }

        let module = if matches!(self, Self::General(_)) {
            println!("picking a random choice..."); // TODO: implement
            modules.into_iter().next().unwrap()
        } else {
            modules.into_iter().next().unwrap()
        };
        let name = module.info.name.clone(); // TODO: Shouldn't need this clone
        let test_runner = TestRunner::build_from_toml(config, module)?;
        Ok((test_runner, name))
    }

    fn filter_with_examtype(&self, module: &ModuleToml, status: &Status) -> bool {
        match self {
            Self::Specific(specific) => {
                if specific.order[status.level] == module.info.name {
                    true
                } else {
                    false
                }
            }
            Self::General(general) => true,
        }
    }

    fn get_points(&self, status: &Status) -> u32 {
        match self {
            Self::Specific(s) => s.point_selector.get_points(status.level, status.attempt),
            Self::General(g) => g.point_selector.get_points(status.level, status.attempt),
        }
    }
}

pub struct Time {
    hours: u32,
    minutes: u32,
    seconds: u32,
}

impl Time {
    pub fn new(hours: u32, minutes: u32, seconds: u32) -> Result<Self, Error> {
        if minutes > 59 || seconds > 59 {
            Err(Error::Parse(
                "Invalid exam.time supplied, minutes and seconds cannot be over 59".to_owned(),
            ))
        } else {
            Ok(Self {
                hours,
                minutes,
                seconds,
            })
        }
    }

    pub fn new_from_seconds(mut seconds: u64) -> Self {
        let hours = seconds / 3600;
        seconds %= 3600;
        let minutes = seconds / 60;
        seconds %= 60;
        Self {
            hours: hours as u32,
            minutes: minutes as u32,
            seconds: seconds as u32,
        }
    }

    pub fn build_from_toml(toml: crate::toml::exam::Time) -> Result<Self, Error> {
        Self::new(toml.hours, toml.minutes, toml.seconds)
    }

    pub fn seconds(&self) -> u32 {
        3600 * self.hours + 60 * self.minutes + self.seconds
    }
}

impl fmt::Display for Time {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}hrs, {}mins and {}sec",
            self.hours, self.minutes, self.seconds
        )
    }
}

pub struct Exam {
    pub name: String,
    pub time: Time,
    exam_type: ExamType,
    pub pass_grade: u32,
}

impl Exam {
    fn new(name: String, time: Time, exam_type: ExamType, pass_grade: u32) -> Self {
        Self {
            name,
            time,
            exam_type,
            pass_grade,
        }
    }

    pub fn select_question(&self, config: &Config, status: &Status) -> Result<Assignment, Error> {
        let (test_runner, name) = self.exam_type.select_question(config, status)?;
        let points = self.exam_type.get_points(status);
        Ok(Assignment::new(name, test_runner, status, points))
    }
}

#[derive(Debug)]
pub struct Assignment {
    name: String,
    test: Option<TestRunner>,
    points: u32,
    pub status: AttemptStatus,
    level: usize,
    attempt: usize,
}

impl Assignment {
    pub fn new(name: String, test: TestRunner, status: &Status, points: u32) -> Self {
        Self {
            name,
            test: Some(test),
            points,
            status: AttemptStatus::Current,
            level: status.level,
            attempt: status.attempt,
        }
    }

    pub fn set_as_complete(&mut self) {
        self.test = None;
        self.status = AttemptStatus::Passed;
    }

    pub fn set_as_failed(&mut self) {
        self.test = None;
        self.status = AttemptStatus::Failed;
    }

    pub fn print(&self) {
        println!(
            "Your current assignment is {} for {} potential points",
            format!("{}", self.name).green(),
            format!("{}", self.points).green()
        );
    }

    pub fn submit_location(&self) -> &str {
        if let Some(test) = self.test.as_ref() {
            test.submit_location()
        } else {
            ""
        }
    }

    pub fn grade(&mut self) -> AttemptStatus {
        if let Some(test_runner) = self.test {
            // TODO: Continue from here, and in test_runner module
        } else {
            panic!("grade() called on Assignment with no TestRunner");
        }
    }
}

impl fmt::Display for Assignment {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "    {}: {} for {} potential points ({})",
            format!("{}", self.attempt).yellow(),
            format!("{}", self.name).green(),
            self.points,
            self.status
        )
    }
}

pub fn select_exam(exam_dir: &str) -> Result<Exam, Error> {
    let toml = std::fs::read_to_string(format!("{}/{}", exam_dir, "Exam1.toml"))?;
    let toml: crate::toml::exam::Exam = toml::from_str(&toml)?;
    ExamBuilder::from_str(&toml.exam_type)?.build(toml)
}

#[derive(Debug, PartialEq)]
pub enum AttemptStatus {
    Current,
    Passed,
    Failed,
}

impl fmt::Display for AttemptStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Current => write!(f, "{}", "Current".blue()),
            Self::Passed => write!(f, "{}", "Passed".green()),
            Self::Failed => write!(f, "{}", "Failed".red()),
        }
    }
}

pub struct QuestionAttempt {
    pub name: String,
    pub points: u32,
    pub attempt: u32,
    pub status: AttemptStatus,
}

impl fmt::Display for QuestionAttempt {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "    {}, {} for {} potential points ({})",
            format!("{}", self.attempt).yellow(),
            format!("{}", self.name).green(),
            format!("{}", self.points).green(),
            self.status
        )
    }
}

pub struct Grade {
    inner: u32,
    max: u32,
}

impl Grade {
    pub fn new(max: u32) -> Self {
        Self { inner: 0, max }
    }
}

impl fmt::Display for Grade {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}/{}", format!("{}", self.inner).green(), self.max)
    }
}

pub struct Status {
    pub level: usize,
    pub attempt: usize,
    pub grade: Grade,
    pub assignments: Vec<Assignment>,
    start_time: Option<Instant>,
    end_time: Option<Instant>,
}

impl Status {
    pub fn new(grade: Grade) -> Self {
        Self {
            level: 0,
            attempt: 0,
            grade,
            assignments: Vec::new(),
            start_time: None,
            end_time: None,
        }
    }

    pub fn current_assignment(&self) -> &Assignment {
        if self.assignments.len() == 0 {
            panic!("Calling current_assignment on Status with no assignment");
        }
        self.assignments.get(self.assignments.len() - 1).unwrap()
    }

    pub fn give_assignment(&mut self, assignment: Assignment) -> Result<(), Error> {
        self.assignments.push(assignment);
        Ok(())
    }

    pub fn start_exam(&mut self, exam: &Exam) {
        self.start_time = Some(Instant::now());
        self.end_time =
            Some(self.start_time.unwrap() + Duration::from_secs(exam.time.seconds() as u64));
    }

    pub fn time_passed(&self) -> Duration {
        let now = Instant::now();
        if let Some(start_time) = self.start_time {
            now.duration_since(start_time)
        } else {
            Duration::from_secs(0)
        }
    }

    pub fn time_remaining(&self) -> Duration {
        if let Some(_) = self.start_time {
            let end = self.end_time.as_ref().unwrap();
            let now = Instant::now();
            if let Some(time_left) = end.checked_duration_since(now) {
                time_left
            } else {
                Duration::from_secs(0)
            }
        } else {
            Duration::from_secs(0)
        }
    }

    pub fn end_time(&self) -> DateTime<Utc> {
        if let Some(_) = self.end_time {
            let now = Utc::now();
            let end = Utc::now() + chrono::Duration::from_std(self.time_remaining()).unwrap();
            end
        } else {
            Utc::now()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn basic_exam_test() -> Result<(), Error> {
        Ok(())
    }
}

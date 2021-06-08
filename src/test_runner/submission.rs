use crate::test_runner::{Error, ProgramOutput};
use crate::toml::SubmissionToml;
use std::process::Command;
use std::str::FromStr;

enum SubmissionBuilder {
    Exec,
    Sources,
}

impl SubmissionBuilder {
    pub fn build(self, toml: SubmissionToml) -> Result<Submission, Error> {
        match self {
            Self::Exec => {
                let exec = Exec::build_from_toml(toml)?;
                Ok(Submission::Exec(exec))
            }
            Self::Sources => {
                let sources = Sources::build_from_toml(toml)?;
                Ok(Submission::Sources(sources))
            }
        }
    }
}

impl FromStr for SubmissionBuilder {
    type Err = Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "executable" => Ok(Self::Exec),
            "sources" => Ok(Self::Sources),
            invalid => Err((format!("Invalid submission type: {}", invalid)).into()),
        }
    }
}

#[derive(Debug)]
pub struct Exec {
    binary: String,
}

impl Exec {
    pub fn build_from_toml(toml: SubmissionToml) -> Result<Self, Error> {
        match toml.binary {
            Some(binary) => Ok(Exec { binary }),
            _ => Err("No binary supplied for executable type submission".into()),
        }
    }
}

#[derive(Debug)]
pub struct Sources {
    pub sources: Vec<String>,
    pub compiler: Option<String>,
    pub flags: Option<Vec<String>>,
}

impl Sources {
    pub fn build_from_toml(toml: SubmissionToml) -> Result<Self, Error> {
        match toml.sources {
            Some(sources) => Ok(Self {
                sources,
                compiler: toml.compiler,
                flags: toml.flags,
            }),
            _ => Err("No sources supplied for sources type submission".into()),
        }
    }
}

#[derive(Debug)]
pub enum Submission {
    Exec(Exec),
    Sources(Sources),
}

impl Submission {
    pub fn generate_from_toml(toml: SubmissionToml) -> Result<Self, Error> {
        SubmissionBuilder::from_str(&toml.submission_type)?.build(toml)
    }

    #[cfg(test)]
    pub fn test_run(&self, submit_path: &String, args: &Vec<Vec<String>>) -> Vec<ProgramOutput> {
        match self {
            Self::Exec(e) => {
                let mut output = Vec::new();
                let exec_path = format!("{}{}", submit_path, e.binary);
                for args in args.iter() {
                    let mut exec = Command::new(&exec_path);
                    for arg in args.iter() {
                        exec.arg(arg);
                    }
                    match exec.output() {
                        Ok(out) => output.push(ProgramOutput::new(out)),
                        Err(e) => output.push(ProgramOutput::from_strings(
                            1,
                            String::from(""),
                            e.to_string(),
                        )),
                    }
                }
                output
            }
            Self::Sources(_) => Vec::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::toml::ModuleToml;
    #[test]
    fn test_submission_output() -> Result<(), Error> {
        let toml = std::fs::read_to_string("tst/modules/module_1.toml")?;
        let toml: ModuleToml = toml::from_str(&toml)?;
        let args = toml.test.args.unwrap();
        let submission = Submission::generate_from_toml(toml.submission)?;
        assert!(matches!(submission, Submission::Exec(_)));
        let submit_path = "tst/mock_submit/".to_owned();
        let results = submission.test_run(&submit_path, &args);
        assert_eq!(results.len(), 3);
        assert_eq!(
            results[0],
            ProgramOutput {
                status: 0,
                stdout: String::from("goodbye\n"),
                stderr: String::from(""),
            }
        );
        assert_eq!(
            results[1],
            ProgramOutput {
                status: 0,
                stdout: String::from("goodbye Ryan Lucas\n"),
                stderr: String::from(""),
            }
        );
        assert_eq!(
            results[2],
            ProgramOutput {
                status: 0,
                stdout: String::from("goodbye did you know shinigami love apples\n"),
                stderr: String::from(""),
            }
        );
        Ok(())
    }
}

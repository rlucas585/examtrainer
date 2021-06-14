use super::ConfigError;
use home::home_dir;
use serde::Deserialize;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::path::Path;

macro_rules! check_if_dir_exists {
    ($path:ident, $error:expr) => {
        match Path::new(&$path).exists() {
            true => (),
            false => ask_to_create_directory(&$path, $error)?,
        }
    };
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct Framework {
    pub name: String,
    pub flags: Vec<String>,
}

#[derive(Deserialize, Debug)]
pub struct Frameworks(pub Vec<Framework>);

#[derive(Deserialize, Debug)]
pub struct Directories {
    pub submit_directory: String,
    pub question_directory: String,
    pub exam_directory: String,
    pub subject_directory: String,
}

#[derive(Deserialize, Debug)]
pub struct Config {
    pub directories: Directories,
    pub frameworks: Option<Frameworks>,
}

impl Config {
    pub fn new() -> Result<Self, ConfigError> {
        Self::new_internal(None)
    }

    /// Parse config file given as argument to create a [`Config`]
    pub fn new_from(config_path: &str) -> Result<Self, ConfigError> {
        Self::new_internal(Some(config_path))
    }

    fn new_internal(config_path: Option<&str>) -> Result<Self, ConfigError> {
        let mut buffer = String::new();
        let mut config = open_config_file(config_path)?;
        config.read_to_string(&mut buffer)?;
        let config: Config = toml_parse::from_str(&buffer)?;
        let question_dir = &config.directories.question_directory;
        let exam_dir = &config.directories.exam_directory;
        check_if_dir_exists!(
            question_dir,
            ConfigError::NoQuestionDirectory(question_dir.clone())
        );
        check_if_dir_exists!(exam_dir, ConfigError::NoExamDirectory(exam_dir.clone()));
        Ok(config)
    }
}

fn open_config_file(config_path: Option<&str>) -> Result<File, ConfigError> {
    if let Some(config_path) = config_path {
        File::open(&config_path).map_err(|e| e.into())
    } else {
        let home = home_dir().ok_or(ConfigError::NoHomeDirectory)?;
        let config_dir = format!("{}/{}", home.display(), ".config");
        let examtrainer_dir = format!("{}/{}", config_dir, "examtrainer");
        let config_file = format!("{}/{}", examtrainer_dir, "config.toml");

        check_if_dir_exists!(config_dir, ConfigError::NoConfigDirectory);
        check_if_dir_exists!(examtrainer_dir, ConfigError::NoExamTrainerDirectory);
        File::open(&config_file).or_else(|error| {
            if error.kind() == io::ErrorKind::NotFound {
                ask_to_create_default_config(home.to_str().unwrap(), &config_file)
            } else {
                Err(error.into())
            }
        })
    }
}

fn create_directory(path: &str) -> Result<(), ConfigError> {
    println!("Creating directory {}...", path);
    std::fs::create_dir(path)?;
    println!("Success!");
    Ok(())
}

fn ask_to_create_directory(path: &str, error: ConfigError) -> Result<(), ConfigError> {
    let mut buffer = String::new();
    let stdin = io::stdin();
    println!("    Warning: Directory {} does not exist", path);
    println!("Create this directory? [y/n]: ");
    stdin.read_line(&mut buffer)?;
    match &buffer.trim().to_lowercase()[..] {
        "y" => create_directory(path),
        _ => Err(error),
    }
}

fn create_default_config(home: &str, path: &str) -> Result<File, ConfigError> {
    println!("Creating default configuration...");
    let mut file = File::create(path)?;
    let default_config = format!(
        "[directories]
submit_directory = \"{0}/rendu\"
question_directory = \"{0}/.config/examtrainer/questions\"
exam_directory = \"{0}/.config/examtrainer/exams\"
subject_directory = \"{0}/subjects\"
",
        home
    );
    file.write_all(default_config.as_bytes())?;
    println!("Success!");
    File::open(path).map_err(|e| e.into())
}

fn ask_to_create_default_config(home: &str, path: &str) -> Result<File, ConfigError> {
    let mut buffer = String::new();
    let stdin = io::stdin();
    println!("    Warning: Config file {} does not exist", path);
    println!("Create default configuration file? [y/n]: ");
    stdin.read_line(&mut buffer)?;
    match &buffer.trim().to_lowercase()[..] {
        "y" => create_default_config(home, path),
        _ => Err(ConfigError::ConfigFileNotFound),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::ConfigError;
    #[test]
    fn initialize_toml() -> Result<(), ConfigError> {
        let buffer = std::fs::read_to_string("tst/resources/test_config2.toml")?;
        let config_toml: Config = toml_parse::from_str(&buffer)?;
        assert_eq!(
            config_toml.directories.submit_directory,
            "tst/resources/rendu_test"
        );
        assert_eq!(
            config_toml.directories.question_directory,
            "tst/resources/questions"
        );
        assert_eq!(
            config_toml.directories.exam_directory,
            "tst/resources/exams"
        );
        assert_eq!(
            config_toml.directories.subject_directory,
            "tst/resources/subjects"
        );
        assert!(config_toml.frameworks.is_some());
        let frameworks = config_toml.frameworks.unwrap().0;
        assert_eq!(frameworks.len(), 1);
        assert_eq!(
            frameworks[0],
            Framework {
                name: "gtest".into(),
                flags: vec![
                    "-lgtest".into(),
                    "-lpthread".into(),
                    "-L/mnt/hard_drive/usr/lib/googletest/build/lib".into()
                ]
            }
        );
        Ok(())
    }
}

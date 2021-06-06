use home::home_dir;
use serde::Deserialize;
use std::fmt;
use std::fs::File;
use std::io::{self, Read, Write};
use std::path::Path;

static DEFAULT_CONF: &str = "[directories]
submit_directory = \"/home/rlucas/rendu\"
module_directory = \"/home/rlucas/.config/examtrainer/modules\"
";

#[derive(Debug)]
enum ErrorKind {
    AbsentConfig,
    IOError(String),
    InvalidConfig(String),
}

#[derive(Debug)]
pub struct Error(ErrorKind);

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.0 {
            ErrorKind::AbsentConfig => {
                if let Some(home) = home_dir() {
                    write!(f, "No config file found in {}/.config/", home.display())
                } else {
                    write!(f, "Unable to locate home directory")
                }
            }
            ErrorKind::IOError(e) => {
                write!(f, "IO Error while loading config: {}", e)
            }
            ErrorKind::InvalidConfig(e) => write!(f, "Invalid config file: {}", e),
        }
    }
}

impl From<io::Error> for Error {
    fn from(input: io::Error) -> Self {
        Error(ErrorKind::IOError(input.to_string()))
    }
}

impl From<toml::de::Error> for Error {
    fn from(input: toml::de::Error) -> Self {
        Error(ErrorKind::InvalidConfig(input.to_string()))
    }
}

#[derive(Deserialize, Debug)]
struct Directories {
    submit_directory: String,
    module_directory: String,
}

#[derive(Deserialize, Debug)]
pub struct Config {
    directories: Directories,
}

impl Config {
    pub fn submit_directory(&self) -> &str {
        &self.directories.submit_directory
    }

    pub fn module_directory(&self) -> &str {
        &self.directories.module_directory
    }
}

fn create_directory(path: &str) -> Result<(), Error> {
    let mut buffer = String::new();
    let stdin = io::stdin();
    println!("    Warning: Directory {} does not exist", path);
    println!("Create this directory? [y/n]: ");
    stdin.read_line(&mut buffer)?;
    match &buffer.trim().to_uppercase()[..] {
        "Y" | "YES" => {
            println!("Creating directory...");
            match std::fs::create_dir(path).map_err(|e| e.into()) {
                Ok(_) => {
                    println!("Success!");
                    Ok(())
                }
                Err(e) => Err(e),
            }
        }
        _ => Err(Error(ErrorKind::AbsentConfig)),
    }
}

fn create_default_config(path: &str) -> Result<File, Error> {
    let mut buffer = String::new();
    let stdin = io::stdin();
    println!("    Warning: Config file {} does not exist", path);
    println!("Create default configuration file? [y/n]: ");
    stdin.read_line(&mut buffer)?;
    match &buffer.trim().to_uppercase()[..] {
        "Y" | "YES" => {
            println!("Creating default configuration...");
            match File::create(path).map_err(|e| e.into()) {
                Ok(mut file) => {
                    file.write(DEFAULT_CONF.as_bytes())?;
                    println!("Success!");
                    File::open(path).map_err(|e| e.into())
                }
                Err(e) => Err(e),
            }
        }
        _ => Err(Error(ErrorKind::AbsentConfig)),
    }
}

impl Config {
    pub fn new() -> Result<Self, Error> {
        let mut buffer = String::new();
        let mut config = Config::open_config_file()?;
        config.read_to_string(&mut buffer)?;
        let config: Config = toml::from_str(&buffer)?;
        Ok(config)
        // let config_text = std::fs::read_to_string(format!("{}/{}", home.display()))
    }

    fn open_config_file() -> Result<File, Error> {
        let home = home_dir().ok_or_else(|| Error(ErrorKind::AbsentConfig))?;
        let config_dir = format!("{}/.config", home.display());
        let examtrainer_dir = config_dir.clone() + "/examtrainer";
        let config_file = examtrainer_dir.clone() + "/config.toml";

        match Path::new(&config_dir).exists() {
            true => (),
            false => create_directory(&config_dir)?,
        }
        match Path::new(&examtrainer_dir).exists() {
            true => (),
            false => create_directory(&examtrainer_dir)?,
        }
        File::open(&config_file)
            .or_else(|error| {
                if error.kind() == io::ErrorKind::NotFound {
                    create_default_config(&config_file)
                } else {
                    Err(error.into())
                }
            })
            .map_err(|e| e.into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn config_generation() {
        Config::new().unwrap();
    }
}

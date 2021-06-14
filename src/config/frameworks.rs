use super::{toml, ConfigError};
use std::collections::HashMap;
use std::path::Path;

#[derive(Debug)]
pub struct FrameworkManager {
    frameworks: HashMap<String, Vec<String>>,
}

impl FrameworkManager {
    pub fn new(toml: Option<toml::Frameworks>) -> Result<Self, ConfigError> {
        let mut frameworks = HashMap::new();
        if let Some(frameworks_struct) = toml {
            let vec = frameworks_struct.0;
            for framework in vec.into_iter() {
                add_to_hashmap(framework, &mut frameworks)?;
            }
            Ok(Self { frameworks })
        } else {
            Ok(Self {
                frameworks: HashMap::new(),
            })
        }
    }

    pub fn get(&self, name: &str) -> Option<&Vec<String>> {
        self.frameworks.get(name)
    }
}

fn add_to_hashmap(
    toml: toml::Framework,
    map: &mut HashMap<String, Vec<String>>,
) -> Result<(), ConfigError> {
    for val in toml.flags.iter() {
        if !val.starts_with("-l") && !val.starts_with("-L") {
            return Err(ConfigError::InvalidFramework);
        }
        if let Some(path_str) = val.strip_prefix("-L") {
            let path = Path::new(path_str);
            if !path.exists() || !path.is_dir() {
                return Err(ConfigError::InvalidFrameworkDir(path_str.into()));
            }
        }
    }
    map.insert(toml.name, toml.flags);
    Ok(())
}

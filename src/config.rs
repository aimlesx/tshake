use std::collections::{HashMap, HashSet};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct GlobalConfig {
    pub skip: Option<Vec<String>>,
}

#[derive(Deserialize)]
pub struct ProjectConfig {
    pub skip: Option<Vec<String>>,
    pub detect: Option<Vec<String>>,
    pub remove: Option<Vec<String>>,
}

impl GlobalConfig {
    pub fn get_skip(&self) -> HashSet<String> {
        if let Some(skip) = &self.skip {
            skip.clone().into_iter().collect()
        } else {
            HashSet::new()
        }
    }
}
impl ProjectConfig {
    // Write method "getSkip" that returns a HashSet of strings from skip and remove
    pub fn get_skip(&self) -> HashSet<String> {
        let mut combined = Vec::new();
        if let Some(skip) = &self.skip {
            combined.extend(skip.clone());
        }

        if let Some(remove) = &self.remove {
            combined.extend(remove.clone());
        }

        combined.into_iter().collect()
    }
    pub fn get_remove(&self) -> HashSet<String> {
        if let Some(remove) = &self.remove {
            remove.clone().into_iter().collect()
        } else {
            HashSet::new()
        }
    }
    pub fn get_detect(&self) -> HashSet<String> {
        if let Some(detect) = &self.detect {
            detect.clone().into_iter().collect()
        } else {
            HashSet::new()
        }
    }
}


#[derive(Deserialize)]
pub struct Config {
    pub all: Option<GlobalConfig>,
    #[serde(flatten)]
    pub projects: HashMap<String, ProjectConfig>,
}

impl Config {
    pub fn get_skip(&self) -> HashSet<String> {
        let mut skip: HashSet<String> = self.projects
            .values()
            .map(|project|
                project.get_skip()
            )
            .flatten()
            .collect();

        if let Some(all) = &self.all {
            skip.extend(all.get_skip());
        }

        skip
    }
}

pub fn get_config() -> Option<Config> {
    let config = include_str!("../default.toml");
    match toml::from_str(config) {
        Ok(cfg) => Some(cfg),
        Err(e) => {
            eprintln!("Error parsing config: {}", e);
            None
        }
    }
}
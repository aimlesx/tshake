use std::collections::HashMap;
use serde::Deserialize;

#[derive(Deserialize)]
struct GlobalConfig {
    pub skip: Option<Vec<String>>,
}

#[derive(Deserialize)]
struct ProjectConfig {
    pub skip: Option<Vec<String>>,
    pub detect: Option<Vec<String>>,
    pub remove: Option<Vec<String>>,
}

#[derive(Deserialize)]
pub struct Config {
    pub all: Option<GlobalConfig>,
    #[serde(flatten)]
    pub projects: HashMap<String, ProjectConfig>,
}

pub fn get_config() -> Config {
    let config = include_str!("../default.toml");
    toml::from_str(config).unwrap()
}
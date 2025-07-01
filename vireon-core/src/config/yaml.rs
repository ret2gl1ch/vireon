use serde::Deserialize;
use std::{fs, path::Path};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("Failed to read config file: {0}")]
    Io(#[from] std::io::Error),
    #[error("Failed to parse config: {0}")]
    Yaml(#[from] serde_yaml::Error),
}

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub device: String,
    pub threshold: f32,
    pub max_attempts: u8,
    pub model_path: ModelPath,
    pub log_level: Option<String>,
    pub log_console: Option<bool>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ModelPath {
    pub detector: String,
    pub recognizer: String,
}

impl Config {
    pub fn load_from_file<P: AsRef<Path>>(path: P) -> Result<Self, ConfigError> {
        let contents = fs::read_to_string(path)?;
        let config: Config = serde_yaml::from_str(&contents)?;
        Ok(config)
    }
}

pub mod config;

use config::yaml::Config;

pub fn load_config() -> Result<Config, config::yaml::ConfigError> {
    Config::load_from_file("/etc/vireon/config.yaml")
}

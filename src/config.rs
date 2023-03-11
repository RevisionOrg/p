use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Config {
    pub projects_dir: String,
    pub project_management_tool: String,
    pub version_repositories: Option<Vec<String>>,
    pub editor: Option<String>
}

pub fn read_config() -> Config {
    let config_path = get_config_path();
    let config_content = std::fs::read_to_string(config_path).expect("Unable to read config file");
    let config_parsed: Config = toml::from_str(&config_content).expect("Invalid config file");

    config_parsed
}

pub fn write_config(config: Config) {
    let config_path = get_config_path();
    let config_content = toml::to_string(&config).expect("Unable to convert config to TOML");

    std::fs::write(config_path, config_content).expect("Unable to write config file");
}

pub fn get_config_directory() -> PathBuf {
    let mut config_directory = dirs::home_dir().expect("Could not get home directory");
    config_directory.push(".p");

    if !config_directory.exists() {
        create_default_config();
    }

    config_directory
}

pub fn get_config_path() -> PathBuf {
    let mut config_path = get_config_directory();
    config_path.push("config.toml");

    config_path
}

pub fn create_default_config() {
    let config_directory = dirs::home_dir().expect("Could not get home directory");

    std::fs::create_dir_all(&config_directory).expect("Unable to create config directory");

    let mut config_path = config_directory.clone();
    let config_content = r#"
projects_dir = "~/Projects"
project_management_tool = "./project"
"#;

    config_path.push("config.toml");
    std::fs::write(&config_directory, config_content).expect("Unable to write default config file");
}

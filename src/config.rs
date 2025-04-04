use log::error;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Deserialize, Serialize)]
pub struct UserConfigSchema {
    pub projects_dir: String,
    pub project_management_tool: String,
    pub version_repositories: Option<Vec<String>>,
    pub editor: Option<String>,
}

pub fn read_config() -> UserConfigSchema {
    let config_path = get_config_path();
    let config_content = std::fs::read_to_string(config_path).unwrap_or_else(|_| {
        error!("Unable to read config file");
        std::process::exit(1)
    });
    let config_parsed: UserConfigSchema = toml::from_str(&config_content).unwrap_or_else(|_| {
        error!("Invalid config file");
        std::process::exit(1)
    });

    config_parsed
}

pub fn write_config(config: UserConfigSchema) {
    let config_path = get_config_path();
    let config_content = toml::to_string(&config).unwrap_or_else(|_| {
        error!("Unable to convert config to TOML");
        std::process::exit(1)
    });

    std::fs::write(config_path, config_content).unwrap_or_else(|_| {
        error!("Unable to write config file");
        std::process::exit(1)
    });
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

    if !config_path.exists() {
        create_default_config();
    }

    config_path
}

pub fn create_default_config() {
    let mut config_directory_path = dirs::home_dir().expect("Could not get home directory");
    config_directory_path.push(".p");

    let mut config_file_path = config_directory_path.clone();
    config_file_path.push("config.toml");

    let config_content = r#"
projects_dir = "~/Projects"
project_management_tool = "./project"
"#;

    std::fs::create_dir_all(&config_directory_path).unwrap_or_else(|_| {
        error!("Unable to create config directory");
        std::process::exit(1)
    });
    std::fs::write(&config_file_path, config_content).unwrap_or_else(|_| {
        error!("Unable to write default config file");
        std::process::exit(1)
    });
}

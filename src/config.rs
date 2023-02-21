use serde::Deserialize;

use crate::shell::log_shell_aliases;

#[derive(Deserialize)]
pub struct Config {
    pub projects_dir: String,
    pub project_management_tool: String,
}

pub fn read_config() -> Config {
    let mut config_path = dirs::home_dir().unwrap();
    config_path.push(".p/config.toml");

    if !config_path.exists() {
        create_default_config();
        println!("This is the first time you're running p.");
        println!("The default configuration is located at {}", config_path.to_str().unwrap());
        log_shell_aliases();
        std::process::exit(0);
    }

    let config_content = std::fs::read_to_string(config_path).expect("Unable to read config file");
    let config: Config = toml::from_str(&config_content).expect("Invalid config file");

    config
}

pub fn create_default_config() {
    let mut config_path = dirs::home_dir().expect("Unable to get home directory");
    config_path.push(".p");

    std::fs::create_dir_all(config_path.clone()).expect("Unable to create config directory");

    config_path.push("config.toml");

    let config_content = r#"
projects_dir = "~/Projects"
project_management_tool = "./project"
    "#;

    std::fs::write(config_path, config_content).expect("Unable to write config file");
}

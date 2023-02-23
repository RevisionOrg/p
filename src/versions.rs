use std::path::PathBuf;
use serde::{Deserialize, Serialize};

use crate::config::{get_config_directory};

#[derive(Deserialize, Serialize)]
pub struct VersionConfig {
    pub version: String,
    pub description: String,
    pub files_needed: Vec<String>,
    pub directories_needed: Vec<String>,
    pub specificity: u8,
    pub project_management_tool: Option<String>,
}

pub fn get_versions_directory() -> PathBuf {
    let mut versions_directory = get_config_directory();
    versions_directory.push("versions");

    if !versions_directory.exists() {
        std::fs::create_dir_all(&versions_directory).expect("Unable to create versions directory");
        create_sample_version_in_versions_directory();
    }

    versions_directory
}

pub fn get_current_directory_versions() -> Vec<VersionConfig> {
    let current_directory = std::env::current_dir().expect("Unable to get current directory");
    let directory_versions = get_directory_versions(&current_directory);

    directory_versions
}

pub fn get_directory_versions(directory: &PathBuf) -> Vec<VersionConfig> {
    let versions_directory = get_versions_directory();
    let versions_configs = std::fs::read_dir(versions_directory).expect("Unable to read versions directory").filter(|entry| entry.as_ref().unwrap().path().extension().unwrap() == "toml");
    let mut versions: Vec<VersionConfig> = vec![];

    for version_config in versions_configs {
        let version_config_content = std::fs::read_to_string(version_config.expect("Unable to read version config").path()).expect("Unable to read version config content");
        let version_config_parsed: VersionConfig = toml::from_str(&version_config_content).expect("Unable to convert version config to TOML");

        let mut files_needed = version_config_parsed.files_needed.clone();
        let mut directories_needed = version_config_parsed.directories_needed.clone();
        let mut should_include_version = true;

        for file_needed in files_needed.iter_mut() {
            let mut file_needed_path = directory.clone();
            file_needed_path.push(file_needed);

            if !file_needed_path.exists() {
                should_include_version = false;
                break;
            }
        }

        for directory_needed in directories_needed.iter_mut() {
            let mut directory_needed_path = directory.clone();
            directory_needed_path.push(directory_needed);

            if !directory_needed_path.exists() {
                should_include_version = false;
                break;
            }
            else if !directory_needed_path.is_dir() {
                should_include_version = false;
                break;
            }
        }

        if should_include_version {
            versions.push(version_config_parsed);
        }
    }

    if versions.len() == 0 {
        versions.push(VersionConfig {
            version: "Unknown".to_string(),
            description: "Unknown version".to_string(),
            files_needed: vec![],
            directories_needed: vec![],
            specificity: 0,
            project_management_tool: None,
        });

        versions
    }
    else {
        let sorted_versions = sort_versions_by_specificity(versions);

        sorted_versions
    }
}

pub fn sort_versions_by_specificity(versions: Vec<VersionConfig>) -> Vec<VersionConfig> {
    let mut sorted_versions = versions;

    sorted_versions.sort_by(|a, b| b.specificity.cmp(&a.specificity));
    sorted_versions
}

pub fn create_sample_version_in_versions_directory() {
    let mut versions_path = get_versions_directory();
    versions_path.push("rust.toml");

    let version_config = VersionConfig {
        version: "Rust".to_string(),
        description: "A Rust project".to_string(),
        files_needed: vec!["Cargo.toml".to_string()],
        directories_needed: vec!["src".to_string()],
        specificity: 1,
        project_management_tool: Some("project".to_string()),
    };
    let version_config = toml::to_string(&version_config).expect("Unable to convert version config to TOML");

    std::fs::write(versions_path, version_config).expect("Unable to write version config to file");
}
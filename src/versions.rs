use log::error;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

use crate::{config::get_config_directory, repositories};

#[derive(Deserialize, Serialize)]
pub struct VersionConfigSchema {
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
        std::fs::create_dir_all(&versions_directory).unwrap_or_else(|_| {
            error!("Unable to create versions directory");
            std::process::exit(1)
        });
        create_sample_version_in_versions_directory();
    }

    versions_directory
}

pub fn get_current_directory_versions() -> Vec<VersionConfigSchema> {
    let current_directory = std::env::current_dir().unwrap_or_else(|_| {
        error!("Unable to get current directory");
        std::process::exit(1)
    });
    let directory_versions = get_directory_versions(&current_directory);

    directory_versions
}

pub fn get_directory_versions(directory: &PathBuf) -> Vec<VersionConfigSchema> {
    let versions_directory = get_versions_directory();
    let versions_configs = std::fs::read_dir(versions_directory)
        .unwrap_or_else(|_| {
            error!("Unable to read versions directory");
            std::process::exit(1)
        })
        .filter(|entry| entry.as_ref().unwrap().path().extension().unwrap() == "toml");
    let external_versions_configs = repositories::get_repositories_configs();
    let all_versions_configs = versions_configs.chain(external_versions_configs);
    let mut directory_versions: Vec<VersionConfigSchema> = vec![];

    // Loop through all known versions configs to find which ones match the current directory
    for version_config in all_versions_configs {
        let version_config_content = std::fs::read_to_string(
            version_config
                .unwrap_or_else(|_| {
                    error!("Unable to read version config");
                    std::process::exit(1)
                })
                .path(),
        )
        .unwrap_or_else(|_| {
            error!("Unable to read version config content");
            std::process::exit(1)
        });
        let version_config_parsed: VersionConfigSchema = toml::from_str(&version_config_content)
            .unwrap_or_else(|_| {
                error!("Unable to convert version config to TOML");
                std::process::exit(1)
            });

        let mut files_needed = version_config_parsed.files_needed.clone();
        let mut directories_needed = version_config_parsed.directories_needed.clone();
        let mut should_include_version_in_versions = true;

        for file_needed in files_needed.iter_mut() {
            let mut file_needed_path = directory.clone();
            file_needed_path.push(file_needed);

            if !file_needed_path.exists() {
                should_include_version_in_versions = false;
                break;
            }
        }

        for directory_needed in directories_needed.iter_mut() {
            let mut directory_needed_path = directory.clone();
            directory_needed_path.push(directory_needed);

            if !directory_needed_path.exists() {
                should_include_version_in_versions = false;
                break;
            } else if !directory_needed_path.is_dir() {
                should_include_version_in_versions = false;
                break;
            }
        }

        if should_include_version_in_versions {
            directory_versions.push(version_config_parsed);
        }
    }

    // Show arbitrary "Unknown" version if no version is found
    if directory_versions.len() == 0 {
        directory_versions.push(VersionConfigSchema {
            version: "Unknown".to_string(),
            description: "Unknown version".to_string(),
            files_needed: vec![],
            directories_needed: vec![],
            specificity: 0,
            project_management_tool: None,
        });

        directory_versions
    } else {
        let sorted_versions = sort_versions_by_specificity(directory_versions);

        sorted_versions
    }
}

pub fn sort_versions_by_specificity(
    versions: Vec<VersionConfigSchema>,
) -> Vec<VersionConfigSchema> {
    let mut sorted_versions = versions;

    sorted_versions.sort_by(|a, b| b.specificity.cmp(&a.specificity));
    sorted_versions
}

pub fn create_sample_version_in_versions_directory() {
    let mut versions_path = get_versions_directory();
    versions_path.push("rust.toml");

    let version_config = VersionConfigSchema {
        version: "Rust".to_string(),
        description: "A Rust project".to_string(),
        files_needed: vec!["Cargo.toml".to_string()],
        directories_needed: vec!["src".to_string()],
        specificity: 1,
        project_management_tool: Some("./project".to_string()),
    };
    let version_config = toml::to_string(&version_config).unwrap_or_else(|_| {
        error!("Unable to convert version config to TOML");
        std::process::exit(1)
    });

    std::fs::write(versions_path, version_config).unwrap_or_else(|_| {
        error!("Unable to write version config to file");
        std::process::exit(1)
    });
}

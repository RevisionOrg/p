use std::path::PathBuf;
use serde::{Deserialize, Serialize};

use crate::config::{get_config_directory, self};

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
    let external_versions_directories = std::fs::read_dir(get_config_directory().join("external_versions")).expect("Unable to read external versions directory").filter(|entry| entry.as_ref().unwrap().path().is_dir());
    let versions_configs = std::fs::read_dir(versions_directory).expect("Unable to read versions directory").filter(|entry| entry.as_ref().unwrap().path().extension().unwrap() == "toml");
    let mut external_versions_configs = vec![];

    for external_versions_directory in external_versions_directories {
        let external_versions_directory = external_versions_directory.expect("Unable to read external versions directory");
        let external_versions_directory_configs;

        if external_versions_directory.path().join("versions").exists() {
            external_versions_directory_configs = std::fs::read_dir(external_versions_directory.path().join("versions")).expect("Unable to read external versions directory").filter(|entry| entry.as_ref().unwrap().path().extension().unwrap() == "toml");
        }
        else {
            break;
        }

        for external_versions_directory_config in external_versions_directory_configs {
            external_versions_configs.push(external_versions_directory_config);
        }
    }

    let all_versions_configs = versions_configs.chain(external_versions_configs);
    let mut versions: Vec<VersionConfig> = vec![];

    for version_config in all_versions_configs {
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
        project_management_tool: Some("./project".to_string()),
    };
    let version_config = toml::to_string(&version_config).expect("Unable to convert version config to TOML");

    std::fs::write(versions_path, version_config).expect("Unable to write version config to file");
}

pub fn sync_version_sources() {
    let config_directory = get_config_directory();
    let config = config::read_config();
    let mut version_source_names: Vec<String> = vec![];
    let mut external_versions_directory = config_directory.clone();

    external_versions_directory.push("external_versions");

    if !external_versions_directory.exists() {
        std::fs::create_dir_all(&external_versions_directory).expect("Unable to create external versions directory");
    }

    if let Some(version_sources) = config.version_sources {
        println!("Syncing version sources...");

        // Clone or pull version sources
        for version_source in &version_sources {
            let mut version_source_path = external_versions_directory.clone();
            let version_source_name = version_source.split("/").last().unwrap().replace(".git", "");

            version_source_names.push(version_source_name.clone());
            version_source_path.push(&version_source_name);

            if !version_source_path.exists() {
                let mut command = std::process::Command::new("git");

                println!("Cloning {}...", version_source_name);
                command.arg("clone").arg(version_source).arg(&version_source_name);
                command.current_dir(&external_versions_directory);
                command.output().expect("Unable to clone version source");
            } else {
                let mut clean_command = std::process::Command::new("git");
                let mut pull_command = std::process::Command::new("git");
                let mut reset_command = std::process::Command::new("git");

                println!("Pulling {}...", version_source_name);
                clean_command.arg("clean").arg("-df");
                clean_command.current_dir(&version_source_path);
                clean_command.output().expect("Unable to clean version source");

                pull_command.arg("pull");
                pull_command.current_dir(&version_source_path);
                pull_command.output().expect("Unable to pull version source");

                reset_command.arg("reset").arg("--hard");
                reset_command.current_dir(&version_source_path);
                reset_command.output().expect("Unable to clean version source");
            }
        }

        // Remove any external versions that are no longer in the config but are in the directory
        let external_versions_directories = std::fs::read_dir(&external_versions_directory).expect("Unable to read external versions directory");

        for external_versions_directory in external_versions_directories {
            let external_versions_directory = external_versions_directory.expect("Unable to read external versions directory");
            let external_versions_directory_name = external_versions_directory.file_name().into_string().expect("Unable to convert external versions directory name to string");

            if !&version_source_names.contains(&external_versions_directory_name) {
                println!("Removing {}...", external_versions_directory_name);
                std::fs::remove_dir_all(external_versions_directory.path()).expect("Unable to remove external versions directory");
            }
        }

        println!("Done syncing version sources")
    }
    else {
        // Clean up any external versions that are in the directory if there are no version sources in the config
        let external_versions_directories = std::fs::read_dir(&external_versions_directory).expect("Unable to read external versions directory");

        for external_versions_directory in external_versions_directories {
            let external_versions_directory = external_versions_directory.expect("Unable to read external versions directory");
            let external_versions_directory_name = external_versions_directory.file_name().into_string().expect("Unable to convert external versions directory name to string");

            println!("Removing {}...", external_versions_directory_name);
            std::fs::remove_dir_all(external_versions_directory.path()).expect("Unable to remove external versions directory");
        }

        println!("No external version sources found in config");
    }
}

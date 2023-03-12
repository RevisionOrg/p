use std::path::PathBuf;

use clap::{Args, Subcommand};
use colored::Colorize;

use crate::config;

#[derive(Args)]
pub struct Repo {
    #[command(subcommand)]
    pub command: RepositoryCommands,
}

#[derive(Subcommand)]
pub enum RepositoryCommands {
    /// Sync external version repositories
    Sync(RepositorySyncArgs),
    /// Add a new repository
    Add(RepositoryAddArgs),
    /// Remove a repository
    Remove(RepositoryRemoveArgs),
    /// List repositories
    List(RepositoryListArgs),
    /// Get the path of the repositories directory
    Go(RepositoryGoArgs),
    /// Initialize a new repository
    New(RepositoryNewArgs),
}

#[derive(Args)]
pub struct RepositorySyncArgs {}

#[derive(Args)]
pub struct RepositoryAddArgs {
    pub repository: String,
}

#[derive(Args)]
pub struct RepositoryRemoveArgs {
    pub repository: String,
}

#[derive(Args)]
pub struct RepositoryNewArgs {
    pub name: String,
}

#[derive(Args)]
pub struct RepositoryGoArgs {}

#[derive(Args)]
pub struct RepositoryListArgs {}

pub fn get_repositories_directory() -> PathBuf {
    let mut repositories_directory = config::get_config_directory();
    repositories_directory.push("external_versions");

    if !repositories_directory.exists() {
        std::fs::create_dir_all(&repositories_directory)
            .expect("Unable to create repositories directory");
    }

    repositories_directory
}

pub fn get_repositories_configs() -> Vec<Result<std::fs::DirEntry, std::io::Error>> {
    let external_versions_directories = std::fs::read_dir(get_repositories_directory())
        .expect("Could not read external repositories directory")
        .filter(|entry| entry.as_ref().unwrap().path().is_dir());
    let mut external_versions_configs = vec![];

    for external_versions_directory in external_versions_directories {
        let external_versions_directory =
            external_versions_directory.expect("Unable to read external versions directory");
        let external_versions_directory_configs;

        if external_versions_directory.path().join("versions").exists() {
            external_versions_directory_configs =
                std::fs::read_dir(external_versions_directory.path().join("versions"))
                    .expect("Unable to read external versions directory")
                    .filter(|entry| entry.as_ref().unwrap().path().extension().unwrap() == "toml");
        } else {
            break;
        }

        for external_versions_directory_config in external_versions_directory_configs {
            external_versions_configs.push(external_versions_directory_config);
        }
    }

    external_versions_configs
}

pub fn sync_version_repositories() {
    let config = config::read_config();
    let mut version_repository_names: Vec<String> = vec![];
    let external_versions_directory = get_repositories_directory();

    if let Some(version_repositories) = config.version_repositories {
        println!("Syncing version repositories...");

        // Clone or pull version repositories
        for version_repository in &version_repositories {
            let mut version_repository_path = external_versions_directory.clone();
            let version_repository_name = version_repository
                .split("/")
                .last()
                .unwrap()
                .replace(".git", "");

            version_repository_names.push(version_repository_name.clone());
            version_repository_path.push(&version_repository_name);

            if !version_repository_path.exists() {
                let mut command = std::process::Command::new("git");

                println!("Cloning {}...", version_repository_name);
                command
                    .arg("clone")
                    .arg(version_repository)
                    .arg(&version_repository_name);
                command.current_dir(&external_versions_directory);
                command
                    .output()
                    .expect("Unable to clone version repository");
            } else {
                let mut clean_command = std::process::Command::new("git");
                let mut pull_command = std::process::Command::new("git");
                let mut reset_command = std::process::Command::new("git");

                println!("Pulling {}...", version_repository_name);
                clean_command.arg("clean").arg("-df");
                clean_command.current_dir(&version_repository_path);
                clean_command
                    .output()
                    .expect("Unable to clean version repository");

                pull_command.arg("pull");
                pull_command.current_dir(&version_repository_path);
                pull_command
                    .output()
                    .expect("Unable to pull version repository");

                reset_command.arg("reset").arg("--hard");
                reset_command.current_dir(&version_repository_path);
                reset_command
                    .output()
                    .expect("Unable to clean version repository");
            }
        }

        // Remove any external versions that are no longer in the config but are in the directory
        let external_versions_directories = std::fs::read_dir(&external_versions_directory)
            .expect("Unable to read external versions directory");

        for external_versions_directory in external_versions_directories {
            let external_versions_directory =
                external_versions_directory.expect("Unable to read external versions directory");
            let external_versions_directory_name = external_versions_directory
                .file_name()
                .into_string()
                .expect("Unable to convert external versions directory name to string");

            if !&version_repository_names.contains(&external_versions_directory_name) {
                println!("Removing {}...", external_versions_directory_name);
                std::fs::remove_dir_all(external_versions_directory.path())
                    .expect("Unable to remove external versions directory");
            }
        }

        println!("Done syncing version repositories")
    } else {
        // Clean up any external versions that are in the directory if there are no version repositories in the config
        let external_versions_directories = std::fs::read_dir(&external_versions_directory)
            .expect("Unable to read external versions directory");

        for external_versions_directory in external_versions_directories {
            let external_versions_directory =
                external_versions_directory.expect("Unable to read external versions directory");
            let external_versions_directory_name = external_versions_directory
                .file_name()
                .into_string()
                .expect("Unable to convert external versions directory name to string");

            println!("Removing {}...", external_versions_directory_name);
            std::fs::remove_dir_all(external_versions_directory.path())
                .expect("Unable to remove external versions directory");
        }

        println!("No external version repositories found in config");
    }
}

pub fn add_repository_url_to_config(repository_url: &str) {
    let mut config = config::read_config();

    if let Some(version_repositories) = &mut config.version_repositories {
        version_repositories.push(repository_url.to_string());
    } else {
        config.version_repositories = Some(vec![repository_url.to_string()]);
    }

    config::write_config(config);
}

pub fn remove_repository_url_from_config(repository_url: &str) {
    let mut config = config::read_config();

    if let Some(version_repositories) = &mut config.version_repositories {
        version_repositories.retain(|version_repository| version_repository != repository_url);
    }

    config::write_config(config);
}

pub fn list_version_repositories() {
    let config = config::read_config();

    if let Some(version_repositories) = &config.version_repositories {
        let number_of_version_repositories = config
            .version_repositories
            .as_ref()
            .map(|version_repositories| version_repositories.len())
            .unwrap_or(0);

        println!(
            "{}",
            format!(
                "{} External version {}:",
                number_of_version_repositories,
                if number_of_version_repositories == 1 {
                    "repository"
                } else {
                    "repositories"
                }
            )
            .bold()
            .underline()
        );
        println!();
        for version_repository in version_repositories {
            println!("{}", version_repository);
        }
    } else {
        println!("No external version repositories found in config");
    }
}

pub fn create_new_repository(repository_name: &str) {
    let current_dir = std::env::current_dir().expect("Unable to get current directory");
    let mut version_repository_path = current_dir.clone();
    version_repository_path.push(&repository_name);

    if !version_repository_path.exists() {
        let mut command = std::process::Command::new("git");

        println!("Creating {}...", repository_name);
        command.arg("init").arg(&repository_name);
        command.current_dir(&current_dir);
        command
            .output()
            .expect("Unable to create version repository");

        let mut versions_path = version_repository_path.clone();
        versions_path.push("versions");

        std::fs::create_dir(&versions_path).expect("Unable to create versions directory");

        println!("New external version repository \"{}\" created in {}", repository_name, &version_repository_path.display());
        println!("Add version configs to {}/versions, commit and push the results.", repository_name);
        println!("You will then be able to add the repository to your config by running \"p repo add REPOSITORY_URL\"");
    } else {
        println!("{} already exists", repository_name);
    }
}
use clap::CommandFactory;
use clap::{Args, Parser, Subcommand, ValueEnum};
use clap_complete::generate;
use clap_complete::Shell::{Bash, Zsh};
use colored::Colorize;
use repositories::{Repo, RepositoryCommands};

mod config;
mod versions;
mod shell;
mod repositories;

#[derive(Parser)]
#[command(name = "p")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Get information about the current project
    Info(InfoArgs),
    /// List all projects in the projects directory
    List(ListArgs),
    /// Execute a command in the current directory using the project management tool
    Execute(ExecuteArgs),
    /// Get the path of a project
    Go(GoArgs),
    /// Generate a shell completion script and print it to stdout
    Completions(CompletionsArgs),
    /// Get aliases for your shell (p execute -> px)
    Aliases(CompletionsArgs),
    /// Repository management
    Repo(Repo),
}

#[derive(Args)]
struct InfoArgs {}

#[derive(Args)]
struct ListArgs {}

#[derive(Args)]
struct RepoSyncArgs {}

#[derive(Args)]
struct ExecuteArgs {
    arguments: Vec<String>,
}

#[derive(Args)]
struct GoArgs {
    project: String,
}


#[derive(Args)]
struct CompletionsArgs {
    completions: Option<Shell>,
}

#[derive(ValueEnum, Copy, Clone)]
pub enum Shell {
    Bash,
    Zsh,
}

fn main() {
    let config = config::read_config();
    let cli = Cli::parse();

    match &cli.command {
        Commands::Info(_) => {
            let current_directory_versions = versions::get_current_directory_versions();

            println!("{}", format!("Project: {}", std::fs::canonicalize(".").unwrap().file_name().unwrap().to_str().unwrap()).bold().underline());
            if current_directory_versions.len() > 1 {
                println!("{}", format!("{} Versions:", current_directory_versions.len()).bold());
                for version in current_directory_versions {
                    println!("{} - {}", version.version, version.description);
                }
            }
            else {
                println!("{}", format!("Version: {}", versions::get_current_directory_versions()[0].version).bold());
                println!("{}", versions::get_current_directory_versions()[0].description);
            }
        }
        Commands::List(_) => {
            let projects_dir = shellexpand::tilde(&config.projects_dir).into_owned();
            let projects = std::fs::read_dir(&projects_dir)
                .unwrap_or_else(|_| panic!("Unable to read projects directory: {}", projects_dir));
            let projects_count_dir = projects_dir.clone();
            let projects_count = std::fs::read_dir(&projects_count_dir)
                .unwrap_or_else(|_| panic!("Unable to read projects directory: {}", projects_count_dir))
                .count();
            let projects_name_dir = projects_dir;
            let projects_name = std::fs::read_dir(&projects_name_dir)
                .unwrap_or_else(|_| panic!("Unable to read projects directory: {}", projects_name_dir));
            let mut longest_project_name = 0;
            let projects_string = format!("{} Projects:", projects_count);

            println!("{}", projects_string.bold().underline());
            println!();

            for project in projects_name {
                let project = project.expect("Unable to read project");
                let project_path = project.path();

                if project_path.is_dir() {
                    let project_name = project_path.file_name().unwrap().to_str().unwrap();
                    let project_name_length = project_name.chars().count();

                    if project_name_length > longest_project_name {
                        longest_project_name = project_name_length;
                    }
                }
            }

            for project in projects {
                let project = project.expect("Unable to read project");
                let project_path = project.path();

                if project_path.is_dir() {
                    let project_name = project_path.file_name().unwrap().to_str().unwrap();
                    let project_name_length = project_name.chars().count();
                    let dots_between_name_and_version = longest_project_name - project_name_length + 5;
                    let project_versions_string;
                    let mut project_versions = versions::get_directory_versions(&project_path);
                    let mut dots = String::new();

                    for _ in 0..dots_between_name_and_version {
                        dots.push('.');
                    }

                    if project_versions.len() > 3 {
                        project_versions.truncate(3);
                        project_versions_string = project_versions.iter().map(|version| version.version.clone()).collect::<Vec<String>>().join(", ") + ", ...";
                    }
                    else {
                        project_versions_string = project_versions.iter().map(|version| version.version.clone()).collect::<Vec<String>>().join(", ");
                    }

                    println!("{}{}{}", project_name.bold(), dots.truecolor(30, 30, 30), project_versions_string);
                }
            }
        }
        Commands::Execute(execute_args) => {
            let project_version = &versions::get_current_directory_versions()[0];
            let project_management_tool = match &project_version.project_management_tool {
                Some(project_management_tool) => project_management_tool,
                None => &config.project_management_tool,
            };
            let mut command = std::process::Command::new(&project_management_tool);

            command.args(&execute_args.arguments);
            command.spawn().expect("Error executing command");
        }
        Commands::Go(go_args) => {
            let mut project_path_string = shellexpand::tilde(&config.projects_dir).into_owned();
            project_path_string.push_str("/");
            project_path_string.push_str(&go_args.project);

            let project_path = std::path::Path::new(&project_path_string);
            if !project_path.exists() {
                println!("Project {} does not exist", go_args.project);
                return;
            }

            println!("{}", project_path.to_str().unwrap());
        }
        Commands::Completions(completions_args) => {
            let mut cmd = Cli::command();
            let available_shells = vec!["bash", "zsh"];
            let shell = match completions_args.completions {
                Some(shell) => shell,
                None => {
                    println!("Please specify a shell. Available shells: {}", available_shells.join(", "));
                    return;
                }
            };

            match shell {
                Shell::Bash => generate(Bash, &mut cmd, "p", &mut std::io::stdout()),
                Shell::Zsh => generate(Zsh, &mut cmd, "p", &mut std::io::stdout()),
            }
        }
        Commands::Aliases(_) => {
            shell::log_shell_aliases();
        }
        Commands::Repo(repo) => {
            match &repo.command {
                RepositoryCommands::Sync(_) => {
                    repositories::sync_version_repositories();
                }
                RepositoryCommands::Add(add_repo) => {
                    repositories::add_repository_url_to_config(&add_repo.repository);
                }
                RepositoryCommands::Remove(remove_repo) => {
                    repositories::remove_repository_url_from_config(&remove_repo.repository);
                }
                RepositoryCommands::List(_) => {
                    repositories::list_version_repositories();
                }
            }
        }
    }
}

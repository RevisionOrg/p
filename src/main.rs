use clap::{CommandFactory};
use clap::{Args, Parser, Subcommand, ValueEnum};
use clap_complete::generate;
use clap_complete::Shell::{Bash, Zsh};
use colored::Colorize;

use crate::versions::{get_current_directory_version, get_directory_version};

mod config;
mod versions;
mod shell;

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
    Ls(LsArgs),
    /// Execute a command in the current directory using the project management tool
    Execute(ExecuteArgs),
    /// Get the path of a project
    Go(GoArgs),
    /// Generate a shell completion script and print it to stdout
    Completions(CompletionsArgs),
}

#[derive(Args)]
struct InfoArgs {}

#[derive(Args)]
struct LsArgs {}

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
            println!("{}", format!("Project: {}", std::fs::canonicalize(".").unwrap().file_name().unwrap().to_str().unwrap()).bold().underline());
            println!("{}", format!("Version: {}", get_current_directory_version()[0].version).bold());
            println!("{}", get_current_directory_version()[0].description);
        }
        Commands::Ls(_) => {
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
                    let mut dots = String::new();

                    for _ in 0..dots_between_name_and_version {
                        dots.push('.');
                    }

                    println!("{}{}{}", project_name.bold(), dots.truecolor(30, 30, 30), get_directory_version(&project_path)[0].version);
                }
            }
        }
        Commands::Execute(execute_args) => {
            let project_version = &get_current_directory_version()[0];
            let project_management_tool = match &project_version.project_management_tool {
                Some(project_management_tool) => project_management_tool,
                None => &config.project_management_tool,
            };
            let mut command = std::process::Command::new(&project_management_tool);

            command.args(&execute_args.arguments);
            command.spawn().expect("Error executing command");
        }
        Commands::Go(go_args) => {
            let mut project_path = shellexpand::tilde(&config.projects_dir).into_owned();
            project_path.push_str("/");
            project_path.push_str(&go_args.project);

            let project_path = std::path::Path::new(&project_path);

            if !project_path.exists() {
                println!("Project {} does not exist", go_args.project);
                return;
            }

            println!("{}", project_path.to_str().unwrap());
        }
        Commands::Completions(completions_args) => {
            let shell = match completions_args.completions {
                Some(shell) => shell,
                None => {
                    println!("Please specify a shell");
                    return;
                }
            };
            let mut cmd = Cli::command();

            match shell {
                Shell::Bash => generate(Bash, &mut cmd, "p", &mut std::io::stdout()),
                Shell::Zsh => generate(Zsh, &mut cmd, "p", &mut std::io::stdout()),
            }
        }
    }
}

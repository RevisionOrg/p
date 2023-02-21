use clap::{Args, Parser, Subcommand};

use crate::versions::*;

mod config;
mod versions;
mod shell;

#[derive(Parser)]
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
    X(XArgs),
    /// Get the path of a project
    G(GArgs),
}

#[derive(Args)]
struct InfoArgs {}

#[derive(Args)]
struct LsArgs {}

#[derive(Args)]
struct XArgs {
    arguments: Vec<String>,
}

#[derive(Args)]
struct GArgs {
    project: String,
}

fn main() {
    let config = config::read_config();
    let cli = Cli::parse();

    match &cli.command {
        Commands::Info(_) => {
            println!("Version: {}", get_current_directory_version()[0].version);
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

            println!("Projects: ({})", projects_count);

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
                    let spaces_between_name_and_version = longest_project_name - project_name_length + 5;
                    let mut spaces = String::new();

                    for _ in 0..spaces_between_name_and_version {
                        spaces.push(' ');
                    }

                    println!("• {}{}{}", project_name, spaces, get_directory_version(&project_path)[0].version);
                }
            }
        }
        Commands::X(x_args) => {
            let project_version = &get_current_directory_version()[0];
            let project_management_tool = match &project_version.project_management_tool {
                Some(project_management_tool) => project_management_tool,
                None => &config.project_management_tool,
            };
            let mut command = std::process::Command::new(&project_management_tool);

            command.args(&x_args.arguments);
            command.spawn().expect("Error executing command");
        }
        Commands::G(g_args) => {
            let mut project_path = shellexpand::tilde(&config.projects_dir).into_owned();
            project_path.push_str("/");
            project_path.push_str(&g_args.project);

            let project_path = std::path::Path::new(&project_path);

            if !project_path.exists() {
                println!("Project {} does not exist", g_args.project);
                return;
            }

            println!("{}", project_path.to_str().unwrap());
        }
    }
}

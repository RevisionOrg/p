use std::{env, process::exit};

use clap::{Args, Parser, Subcommand, ValueEnum};
use repositories::{Repo, RepositoryCommands};

pub mod config;
pub mod projects;
pub mod repositories;
pub mod shell;
pub mod update;
pub mod versions;

#[derive(Parser)]
#[command(name = "p")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,

    /// Print the version of p
    #[clap(short, long)]
    version: bool,
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
    /// Find a project
    Find(FindArgs),
    /// Open your preferred editor in the current project
    Edit(EditArgs),
    /// Update p
    Update(UpdateArgs),
}

#[derive(Args)]
pub struct InfoArgs {
    /// Which directory to get information of
    #[clap(short, long)]
    directory: Option<String>,
}

#[derive(Args)]
pub struct ListArgs {}

#[derive(Args)]
pub struct RepoSyncArgs {}

#[derive(Args)]
pub struct EditArgs {
    /// Path of the editor to use
    #[clap(short, long)]
    editor: Option<String>,

    /// Run editor in a detached process
    #[clap(short, long)]
    detach: bool,
}

#[derive(Args)]
pub struct ExecuteArgs {
    /// Execution arguments
    arguments: Vec<String>,
}

#[derive(Args)]
pub struct GoArgs {
    /// Name of the project
    project: String,
}

#[derive(Args)]
pub struct FindArgs {
    /// Name of the project
    project: String,

    /// Only show minimal output
    #[clap(short, long)]
    compact: bool,

    /// Amount of results to display
    #[clap(short, long)]
    amount: Option<usize>,
}

#[derive(Args)]
pub struct CompletionsArgs {
    /// Name of the shell
    shell: Option<Shell>,
}

#[derive(Args)]
pub struct UpdateArgs {}

#[derive(ValueEnum, Copy, Clone)]
pub enum Shell {
    Bash,
    Zsh,
}

fn main() {
    let config = config::read_config();
    let cli = Cli::parse();

    if cfg!(debug_assertions) {
        env::set_var("RUST_BACKTRACE", "full");
    }

    if cli.version {
        println!("v{}", env!("CARGO_PKG_VERSION"));
        return;
    }

    if cli.command.is_some() {
        match &cli.command.unwrap() {
            Commands::Info(info_args) => {
                let directory = info_args.directory.as_ref().map(|s| s.as_str());

                projects::get_info_for_project_in_directory(directory);
            }
            Commands::List(_) => {
                projects::list_projects_in_projects_directory(&config);
            }
            Commands::Execute(execute_args) => {
                projects::execute_in_current_project(&config, &execute_args);
            }
            Commands::Go(go_args) => {
                let project_path = projects::get_project_path(&config, &go_args);

                if project_path.is_some() {
                    println!("{}", project_path.unwrap().to_string());
                } else {
                    println!("Project {} does not exist", go_args.project);
                    exit(1)
                }
            }
            Commands::Completions(completions_args) => {
                projects::get_shell_completions(&completions_args)
            }
            Commands::Aliases(_) => {
                shell::log_shell_aliases();
            }
            Commands::Find(find_args) => {
                projects::find_project_in_projects_directory(&config, &find_args)
            }
            Commands::Edit(edit_args) => {
                projects::open_editor_in_current_project(
                    &config,
                    &edit_args.editor,
                    edit_args.detach,
                );
            }
            Commands::Update(_) => {
                let update_res = update::update();

                if update_res.is_err() {
                    println!("Failed to update p: {}", update_res.err().unwrap());
                    exit(1);
                }
            }
            Commands::Repo(repo) => match &repo.command {
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
                RepositoryCommands::Go(_) => {
                    println!(
                        "{}",
                        repositories::get_repositories_directory().to_str().unwrap()
                    );
                }
                RepositoryCommands::New(new_repo) => {
                    repositories::create_new_repository(&new_repo.name);
                }
            },
        }
    } else {
        projects::get_info_for_project_in_directory(None);
    }
}

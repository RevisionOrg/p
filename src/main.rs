use clap::{Args, Parser, Subcommand, ValueEnum};
use repositories::{Repo, RepositoryCommands};

pub mod config;
pub mod projects;
pub mod repositories;
pub mod shell;
pub mod versions;

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
    /// Find a project
    Find(FindArgs),
    /// Open your preferred editor in the current project
    Edit(EditArgs),
}

#[derive(Args)]
pub struct InfoArgs {}

#[derive(Args)]
pub struct ListArgs {}

#[derive(Args)]
pub struct RepoSyncArgs {}

#[derive(Args)]
pub struct EditArgs {}

#[derive(Args)]
pub struct ExecuteArgs {
    arguments: Vec<String>,
}

#[derive(Args)]
pub struct GoArgs {
    project: String,
}

#[derive(Args)]
pub struct FindArgs {
    project: String,
}

#[derive(Args)]
pub struct CompletionsArgs {
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
            projects::get_info_for_project_in_current_directory();
        }
        Commands::List(_) => {
            projects::list_projects_in_projects_directory(&config);
        }
        Commands::Execute(execute_args) => {
            projects::execute_in_current_project(&config, &execute_args);
        }
        Commands::Go(go_args) => {
            projects::get_project_path(&config, &go_args);
        }
        Commands::Completions(completions_args) => {
            projects::get_shell_completions(&completions_args)
        }
        Commands::Aliases(_) => {
            shell::log_shell_aliases();
        }
        Commands::Find(find_args) => {
            projects::find_project_in_projects_directory(&config, &find_args.project)
        }
        Commands::Edit(_) => {
            projects::open_editor(&config);
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
}

use clap::CommandFactory;
use clap_complete::generate;
use clap_complete::Shell::{Bash, Zsh};
use colored::Colorize;
use simsearch::SimSearch;

use crate::Shell;
use crate::{config::Config, versions, Cli, CompletionsArgs, ExecuteArgs, GoArgs};

pub fn get_info_for_project_in_current_directory() {
    let current_directory_versions = versions::get_current_directory_versions();

    println!(
        "{}",
        format!(
            "Project: {}",
            std::fs::canonicalize(".")
                .unwrap()
                .file_name()
                .unwrap()
                .to_str()
                .unwrap()
        )
        .bold()
        .underline()
    );
    if current_directory_versions.len() > 1 {
        println!(
            "{}",
            format!("{} Versions:", current_directory_versions.len()).bold()
        );
        for version in current_directory_versions {
            println!("{} - {}", version.version, version.description);
        }
    } else {
        println!(
            "{}",
            format!(
                "Version: {}",
                versions::get_current_directory_versions()[0].version
            )
            .bold()
        );
        println!(
            "{}",
            versions::get_current_directory_versions()[0].description
        );
    }
}

pub fn list_projects_in_projects_directory(config: &Config) {
    let projects_dir = shellexpand::tilde(&config.projects_dir).into_owned();
    let projects = std::fs::read_dir(&projects_dir)
        .unwrap_or_else(|_| panic!("Unable to read projects directory: {}", projects_dir));
    let projects_count = std::fs::read_dir(&projects_dir.clone())
        .unwrap_or_else(|_| panic!("Unable to read projects directory"))
        .count();
    let projects_string = format!(
        "{} {}:",
        projects_count,
        if projects_count == 1 {
            "Project"
        } else {
            "Projects"
        }
    );

    println!("{}", projects_string.bold().underline());
    println!();

    for project in projects {
        let project = project.expect("Unable to read project");
        let project_path = project.path();

        if project_path.is_dir() {
            let project_name = project_path.file_name().unwrap().to_str().unwrap();
            let project_versions_string;
            let project_versions = versions::get_directory_versions(&project_path);

            project_versions_string = project_versions
                .iter()
                .map(|version| version.version.clone())
                .collect::<Vec<String>>()
                .join(", ");

            println!("{} ({})", project_name.bold(), project_versions_string);
        }
    }
}

pub fn execute_in_current_project(config: &Config, execute_args: &ExecuteArgs) {
    let project_version = &versions::get_current_directory_versions()[0];
    let project_management_tool = match &project_version.project_management_tool {
        Some(project_management_tool) => project_management_tool,
        None => &config.project_management_tool,
    };
    let mut command = std::process::Command::new(&project_management_tool);

    command.args(&execute_args.arguments);
    command
        .spawn()
        .expect("Error executing command in current project")
        .wait()
        .expect("Error executing command in current project");
}

pub fn get_project_path(config: &Config, go_args: &GoArgs) {
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

pub fn get_shell_completions(completions_args: &CompletionsArgs) {
    let mut cmd = Cli::command();
    let available_shells = vec!["bash", "zsh"];
    let shell = match completions_args.completions {
        Some(shell) => shell,
        None => {
            println!(
                "Please specify a shell. Available shells: {}",
                available_shells.join(", ")
            );
            return;
        }
    };

    match shell {
        Shell::Bash => generate(Bash, &mut cmd, "p", &mut std::io::stdout()),
        Shell::Zsh => generate(Zsh, &mut cmd, "p", &mut std::io::stdout()),
    }
}

pub fn find_project_in_projects_directory(config: &Config, project_name: &str) {
    let projects_dir = shellexpand::tilde(&config.projects_dir).into_owned();
    let projects = std::fs::read_dir(&projects_dir)
        .unwrap_or_else(|_| panic!("Unable to read projects directory: {}", projects_dir));
    let mut engine: SimSearch<u32> = SimSearch::new();
    let mut engine_insert_index = 0;
    let mut project_names: Vec<String> = Vec::new();

    for project in projects {
        let project = project.expect("Unable to read project");
        let project_path = project.path();

        project_names.push(
            project_path
                .file_name()
                .unwrap()
                .to_str()
                .unwrap()
                .to_string(),
        );
    }

    for project_name in &project_names {
        engine.insert(engine_insert_index, &project_name);
        engine_insert_index += 1;
    }

    let project_search_result = engine.search(project_name);

    if project_search_result.len() == 0 {
        println!("No project \"{}\" found", project_name);
        return;
    }

    let top_five_project_search_results =
        project_search_result.iter().take(5).collect::<Vec<&u32>>();

    println!(
        "{}",
        format!("Search results for \"{}\":", project_name)
            .bold()
            .underline()
    );
    for project_search_result_index in top_five_project_search_results {
        let project_at_index = &project_names[*project_search_result_index as usize];

        println!("{}", project_at_index);
    }
}

pub fn open_editor(config: &Config, editor: &Option<String>, detach: bool) {
    let editor = match editor.to_owned() {
        Some(editor) => Some(editor),
        None => config.editor.clone(),
    };

    if editor.clone().is_none() {
        println!(
            "No editor set. Please set your preferred code editor or IDE in your config file. Or specify an editor with the --editor flag."
        );
        return;
    }

    // If the detach flag is set, run the editor in the background
    if detach {
        std::process::Command::new("sh")
            .arg("-c")
            .arg(editor.unwrap())
            .spawn()
            .expect("Error: Failed to run editor");
        return;
    }

    // Otherwise, run the editor in the foreground
    std::process::Command::new("sh")
        .arg("-c")
        .arg(editor.unwrap())
        .spawn()
        .expect("Error: Failed to run editor")
        .wait()
        .expect("Error: Editor returned a non-zero status");

}

use std::path::PathBuf;

use clap::CommandFactory;
use clap_complete::generate;
use clap_complete::Shell::{Bash, Zsh};
use colored::Colorize;
use simsearch::SimSearch;

use crate::versions::VersionConfigSchema;
use crate::{config, FindArgs, Shell};
use crate::{config::UserConfigSchema, versions, Cli, CompletionsArgs, ExecuteArgs, GoArgs};

pub struct Project {
    versions: Vec<VersionConfigSchema>,
    root: PathBuf,
    name: String,
}

pub fn get_project_for_directory(custom_directory: Option<&str>) -> Option<Project> {
    let user_config = config::read_config();
    let projects_directory =
        std::path::PathBuf::from(shellexpand::tilde(&user_config.projects_dir).into_owned());
    let current_directory = match custom_directory {
        Some(custom_directory) => std::path::PathBuf::from(custom_directory),
        None => std::env::current_dir().unwrap(),
    };

    if !current_directory.starts_with(&projects_directory) {
        return None;
    }

    let project_directory = current_directory
        .strip_prefix(&projects_directory)
        .unwrap()
        .components()
        .next()
        .expect("Unable to get project directory. Is this a project?")
        .as_os_str()
        .to_str()
        .unwrap();

    Some(Project {
        versions: versions::get_directory_versions(&projects_directory.join(project_directory)),
        root: projects_directory.join(project_directory),
        name: project_directory.to_string(),
    })
}

pub fn get_info_for_project_in_directory(directory: Option<&str>) {
    let current_project =
        get_project_for_directory(directory).expect("Unable to get current project");

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
    if current_project.versions.len() > 1 {
        println!(
            "{}",
            format!("{} Versions:", current_project.versions.len()).bold()
        );
        for version in current_project.versions {
            println!("{} - {}", version.version, version.description);
        }
    } else {
        println!(
            "{}",
            format!("Version: {}", current_project.versions[0].version).bold()
        );
        println!("{}", current_project.versions[0].description);
    }
}

pub fn list_projects_in_projects_directory(config: &UserConfigSchema) {
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

    for projects_subdirectory in projects {
        let projects_subdirectory = &projects_subdirectory.unwrap();

        if projects_subdirectory.path().is_dir() {
            let project =
                get_project_for_directory(Some(projects_subdirectory.path().to_str().unwrap()))
                    .unwrap();
            let project_versions_string = project
                .versions
                .iter()
                .map(|version| version.version.clone())
                .collect::<Vec<String>>()
                .join(", ");

            println!("{} ({})", project.name.bold(), project_versions_string);
        }
    }
}

pub fn execute_in_current_project(config: &UserConfigSchema, execute_args: &ExecuteArgs) {
    let project = get_project_for_directory(None).expect("Unable to get current project");
    let project_version = &project.versions[0];
    let project_management_tool = match &project_version.project_management_tool {
        Some(project_management_tool) => project_management_tool,
        None => &config.project_management_tool,
    };
    let mut command = std::process::Command::new(&project_management_tool);

    command.current_dir(&project.root);
    command.args(&execute_args.arguments);
    command
        .spawn()
        .expect("Error executing command in current project")
        .wait()
        .expect("Error executing command in current project");
}

pub fn get_project_path(config: &UserConfigSchema, go_args: &GoArgs) {
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
    let shell = match completions_args.shell {
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

pub fn find_project_in_projects_directory(config: &UserConfigSchema, command_config: &FindArgs) {
    let projects_dir = shellexpand::tilde(&config.projects_dir).into_owned();
    let projects = std::fs::read_dir(&projects_dir)
        .unwrap_or_else(|_| panic!("Unable to read projects directory: {}", projects_dir));
    let mut engine: SimSearch<u32> = SimSearch::new();
    let mut engine_insert_index = 0;
    let mut project_names: Vec<String> = Vec::new();
    let project_name = &command_config.project.to_owned();
    let compact = &command_config.compact.to_owned();
    let amount = match &command_config.amount {
        Some(amount) => amount.to_owned(),
        None => 5,
    };

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

    if !compact {
        println!(
            "{}",
            format!("Search results for \"{}\":", project_name)
                .bold()
                .underline()
        );
    }

    let top_project_search_results = project_search_result
        .iter()
        .take(amount)
        .collect::<Vec<&u32>>();

    for project_search_result_index in top_project_search_results {
        let project_at_index = &project_names[*project_search_result_index as usize];

        println!("{}", project_at_index);
    }
}

pub fn open_editor_in_current_project(
    config: &UserConfigSchema,
    editor: &Option<String>,
    detach: bool,
) {
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

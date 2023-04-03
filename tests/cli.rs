use assert_cmd::prelude::*; // Add methods on commands
use predicates::prelude::*; // Used for writing assertions
use std::process::Command; // Run programs
use regex::Regex;

fn parse_help_string(help_str: &str) -> Vec<String> {
    let blacklisted_commands = vec!["help", "edit", "execute"];
    let re = Regex::new(r"^\s+(\w+)\s+.*$").unwrap();
    let mut commands = vec![];
    for line in help_str.lines() {
        if let Some(captures) = re.captures(line) {
            commands.push(captures[1].to_string());
        }
    }

    commands.retain(|command| !blacklisted_commands.contains(&command.as_str()));

    commands
}

#[test]
fn help_test() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("p")?;

    cmd.arg("help");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Usage: p"));

    Ok(())
}

#[test]
fn subcommands_exit_codes_test() -> Result<(), Box<dyn std::error::Error>> {
    let mut main_cmd = Command::cargo_bin("p")?;
    main_cmd.arg("help");
    let main_output = main_cmd.output().expect("Failed to execute command");
    let main_output_string = String::from_utf8(main_output.stdout).unwrap();
    let main_subcommands = parse_help_string(&main_output_string);

    println!("Main Subcommands: {:?}", main_subcommands);

    let mut repo_cmd = Command::cargo_bin("p")?;
    repo_cmd.arg("repo").arg("help");
    let repo_output = repo_cmd.output().expect("Failed to execute command");
    let repo_output_string = String::from_utf8(repo_output.stdout).unwrap();
    let repo_subcommands = parse_help_string(&repo_output_string);

    println!("Repo Subcommands: {:?}", repo_subcommands);

    for subcommand in main_subcommands {
        let mut cmd = Command::cargo_bin("p")?;

        cmd.arg(subcommand);

        let assert_success = cmd.status().expect("Failed to execute command").success();

        if !assert_success {
            cmd.assert().failure().stderr(predicate::str::contains("Usage").or(predicate::str::contains("Could not get a project in the given directory")));
        } else {
            cmd.assert().success();
        }
    }

    for subcommand in repo_subcommands {
        let mut cmd = Command::cargo_bin("p")?;

        cmd.arg("repo").arg(subcommand);

        let assert_success = cmd.status().expect("Failed to execute command").success();

        if !assert_success {
            cmd.assert().failure().stderr(predicate::str::contains("Usage").or(predicate::str::contains("Could not get a project in the given directory")));
        } else {
            cmd.assert().success();
        }
    }

    Ok(())
}
use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;

#[test]
fn help_test() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("p")?;

    cmd.arg("help");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Usage: p"));

    Ok(())
}

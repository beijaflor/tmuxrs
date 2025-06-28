use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn test_cli_help_displays() {
    let mut cmd = Command::cargo_bin("tmuxrs").unwrap();
    cmd.arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("A modern tmux session manager"));
}

#[test]
fn test_start_command_exists() {
    let mut cmd = Command::cargo_bin("tmuxrs").unwrap();
    cmd.arg("start")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("Start a tmux session"));
}

#[test]
fn test_list_command_exists() {
    let mut cmd = Command::cargo_bin("tmuxrs").unwrap();
    cmd.arg("list")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains(
            "List available session configurations",
        ));
}

#[test]
fn test_stop_command_exists() {
    let mut cmd = Command::cargo_bin("tmuxrs").unwrap();
    cmd.arg("stop")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("Stop a tmux session"));
}

#[test]
fn test_start_command_shows_attach_flags() {
    let mut cmd = Command::cargo_bin("tmuxrs").unwrap();
    cmd.arg("start")
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("--attach"))
        .stdout(predicate::str::contains("--no-attach"))
        .stdout(predicate::str::contains("--append"));
}

#[test]
fn test_start_with_no_attach_flag_parsing() {
    let mut cmd = Command::cargo_bin("tmuxrs").unwrap();
    cmd.arg("start")
        .arg("nonexistent-session")
        .arg("--no-attach")
        .assert()
        .failure() // Should fail because no config exists
        .stderr(predicate::str::contains("Configuration file not found"));
}

#[test]
fn test_start_with_append_flag_parsing() {
    let mut cmd = Command::cargo_bin("tmuxrs").unwrap();
    cmd.arg("start")
        .arg("nonexistent-session")
        .arg("--append")
        .assert()
        .failure() // Should fail because no config exists
        .stderr(predicate::str::contains("Configuration file not found"));
}

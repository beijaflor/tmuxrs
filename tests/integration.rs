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
        .stdout(predicate::str::contains("List available session configurations"));
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
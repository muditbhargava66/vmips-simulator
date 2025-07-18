use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use tempfile::tempdir;

#[test]
fn test_functional_simulator_help() {
    let mut cmd = Command::cargo_bin("vmips_rust").unwrap();
    cmd.arg("functional").arg("--help");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Run the functional simulator"));
}

#[test]
fn test_timing_simulator_help() {
    let mut cmd = Command::cargo_bin("vmips_rust").unwrap();
    cmd.arg("timing").arg("--help");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Run the timing simulator"));
}

#[test]
fn test_functional_simulator_runs() {
    let mut cmd = Command::cargo_bin("vmips_rust").unwrap();
    cmd.arg("functional").arg("--memory-size").arg("4096");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Running functional simulator"));
}

#[test]
fn test_timing_simulator_runs() {
    let mut cmd = Command::cargo_bin("vmips_rust").unwrap();
    cmd.arg("timing")
        .arg("--memory-size")
        .arg("4096")
        .arg("--max-cycles")
        .arg("50");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Running timing simulator"));
}

#[test]
fn test_log_file_creation() {
    let temp_dir = tempdir().unwrap();
    let log_path = temp_dir.path().join("test.log");

    let mut cmd = Command::cargo_bin("vmips_rust").unwrap();
    cmd.arg("functional")
        .arg("--output")
        .arg(&log_path)
        .arg("--memory-size")
        .arg("1024");

    cmd.assert().success();

    assert!(log_path.exists());
    let log_content = fs::read_to_string(&log_path).unwrap();
    assert!(log_content.contains("Starting VMIPS Rust"));
}

#[test]
fn test_invalid_log_level_defaults_to_info() {
    let mut cmd = Command::cargo_bin("vmips_rust").unwrap();
    cmd.arg("functional")
        .arg("--log-level")
        .arg("invalid")
        .arg("--memory-size")
        .arg("1024");

    // Should still run successfully with default log level
    cmd.assert().success();
}

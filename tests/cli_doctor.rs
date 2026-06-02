use assert_cmd::Command;
use predicates::prelude::*;
use serde_json::{Value, json};
use std::fs;
use std::time::{SystemTime, UNIX_EPOCH};

fn doctor_output(command: &mut Command) -> Value {
    let output = command.assert().success().get_output().stdout.clone();

    serde_json::from_slice(&output).expect("doctor output should be valid JSON")
}

#[test]
fn doctor_reports_missing_key_without_failing() {
    let mut command = Command::cargo_bin("fmp-agent").unwrap();
    command
        .current_dir(isolated_cwd("doctor-missing-key"))
        .env_remove("FMP_API_KEY")
        .env_remove("FMP_BASE_URL")
        .arg("doctor");

    let body = doctor_output(&mut command);

    assert_eq!(body["ok"], false);
    assert_eq!(body["version"], env!("CARGO_PKG_VERSION"));
    assert_eq!(body["api_key"]["configured"], false);
    assert_eq!(body["api_key"]["status"], "missing");
    assert_eq!(body["base_url"]["valid"], true);
    assert_eq!(body["live_connectivity"]["checked"], false);
}

fn isolated_cwd(name: &str) -> std::path::PathBuf {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let dir = std::env::temp_dir().join(format!("fmp-agent-{name}-{nanos}"));
    fs::create_dir_all(&dir).unwrap();
    dir
}

#[test]
fn doctor_reports_present_key_without_revealing_it() {
    let mut command = Command::cargo_bin("fmp-agent").unwrap();
    command
        .current_dir(isolated_cwd("doctor-present-key"))
        .env("FMP_API_KEY", "secret-test-key")
        .env_remove("FMP_BASE_URL")
        .arg("doctor");

    let output = command.assert().success().get_output().stdout.clone();
    let stdout = String::from_utf8(output).unwrap();
    assert!(!stdout.contains("secret-test-key"));

    let body: Value = serde_json::from_str(&stdout).unwrap();
    assert_eq!(body["ok"], true);
    assert_eq!(
        body["api_key"],
        json!({ "configured": true, "status": "ok" })
    );
}

#[test]
fn doctor_reports_invalid_base_url_as_structured_status() {
    let mut command = Command::cargo_bin("fmp-agent").unwrap();
    command
        .current_dir(isolated_cwd("doctor-invalid-base-url"))
        .env("FMP_API_KEY", "test-key")
        .env("FMP_BASE_URL", "not a url")
        .arg("doctor");

    let body = doctor_output(&mut command);

    assert_eq!(body["ok"], false);
    assert_eq!(body["api_key"]["configured"], true);
    assert_eq!(body["base_url"]["valid"], false);
    assert_eq!(body["base_url"]["value"], "<invalid URL>");
    assert_eq!(body["base_url"]["error"]["kind"], "invalid_base_url");
}

#[test]
fn doctor_redacts_credentials_from_invalid_base_url() {
    let mut command = Command::cargo_bin("fmp-agent").unwrap();
    command
        .current_dir(isolated_cwd("doctor-redacts-base-url"))
        .env("FMP_API_KEY", "test-key")
        .env(
            "FMP_BASE_URL",
            "https://user:secret@example.test/stable?apikey=secret",
        )
        .arg("doctor");

    let output = command.assert().success().get_output().stdout.clone();
    let stdout = String::from_utf8(output).unwrap();

    assert!(!stdout.contains("secret"));
    assert!(!stdout.contains("apikey"));
    let body: Value = serde_json::from_str(&stdout).unwrap();
    assert_eq!(body["base_url"]["valid"], false);
    assert_eq!(body["base_url"]["value"], "https://example.test/stable");
}

#[test]
fn doctor_help_documents_no_network_access() {
    Command::cargo_bin("fmp-agent")
        .unwrap()
        .current_dir(isolated_cwd("doctor-help"))
        .env_remove("FMP_API_KEY")
        .env_remove("FMP_BASE_URL")
        .args(["doctor", "--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains(
            "does not make any network requests",
        ))
        .stdout(predicate::str::contains("Examples:"));
}

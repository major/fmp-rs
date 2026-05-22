use std::process::Command;

use httpmock::Method::GET;
use httpmock::MockServer;
use serde_json::json;

#[test]
fn bare_command_prints_help() {
    let output = Command::new(env!("CARGO_BIN_EXE_fmp-agent"))
        .env("CLAP_COLOR", "never")
        .env("FMP_BASE_URL", "")
        .env("NO_COLOR", "1")
        .output()
        .unwrap();

    let help = String::from_utf8(output.stdout).unwrap();
    let stderr = String::from_utf8(output.stderr).unwrap();

    assert!(
        output.status.success(),
        "expected bare command to succeed, got status {:?}\nstdout:\n{}\nstderr:\n{}",
        output.status.code(),
        help,
        stderr
    );
    assert!(stderr.is_empty(), "expected empty stderr, got:\n{stderr}");
    assert!(
        help.contains("Usage: fmp-agent [OPTIONS] <COMMAND>"),
        "expected help usage in stdout, got:\n{help}"
    );
    assert!(
        help.contains("Commands:"),
        "expected commands section in stdout, got:\n{help}"
    );
    assert!(!help.contains("requires a subcommand"));
}

#[test]
fn invalid_argument_uses_clap_error_path() {
    let output = Command::new(env!("CARGO_BIN_EXE_fmp-agent"))
        .arg("--definitely-invalid")
        .env("CLAP_COLOR", "never")
        .env("NO_COLOR", "1")
        .output()
        .unwrap();

    let stdout = String::from_utf8(output.stdout).unwrap();
    let stderr = String::from_utf8(output.stderr).unwrap();

    assert!(!output.status.success());
    assert!(stdout.is_empty(), "expected empty stdout, got:\n{stdout}");
    assert!(
        stderr.contains("unexpected argument '--definitely-invalid'"),
        "expected Clap error in stderr, got:\n{stderr}"
    );
}

#[test]
fn command_invocation_prints_compact_json() {
    let server = MockServer::start();
    let mock = server.mock(|when, then| {
        when.method(GET)
            .path("/search-symbol")
            .query_param("query", "Apple")
            .query_param("apikey", "test-key");
        then.status(200).json_body(json!([{ "symbol": "AAPL" }]));
    });

    let output = Command::new(env!("CARGO_BIN_EXE_fmp-agent"))
        .args([
            "--api-key",
            "test-key",
            "--base-url",
            &format!("{}/", server.base_url()),
            "search",
            "Apple",
        ])
        .output()
        .unwrap();

    mock.assert();

    let stdout = String::from_utf8(output.stdout).unwrap();
    let stderr = String::from_utf8(output.stderr).unwrap();

    assert!(
        output.status.success(),
        "expected command to succeed, got status {:?}\nstdout:\n{}\nstderr:\n{}",
        output.status.code(),
        stdout,
        stderr
    );
    assert!(stderr.is_empty(), "expected empty stderr, got:\n{stderr}");
    assert_eq!(
        stdout.lines().count(),
        1,
        "expected one JSON line, got:\n{stdout}"
    );

    let output: serde_json::Value = serde_json::from_str(stdout.trim_end()).unwrap();
    assert_eq!(output, json!([{ "symbol": "AAPL" }]));
}

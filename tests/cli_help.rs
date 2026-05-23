use assert_cmd::Command;

use httpmock::Method::GET;
use httpmock::MockServer;
use predicates::prelude::*;
use serde_json::json;

#[test]
fn bare_command_prints_help() {
    let mut cmd = Command::cargo_bin("fmp-agent").unwrap();
    cmd.env("CLAP_COLOR", "never")
        .env("FMP_BASE_URL", "")
        .env("NO_COLOR", "1")
        .assert()
        .success()
        .stderr(predicate::str::is_empty())
        .stdout(predicate::str::contains(
            "Usage: fmp-agent [OPTIONS] <COMMAND>",
        ))
        .stdout(predicate::str::contains("Commands:"))
        .stdout(predicate::str::contains("requires a subcommand").not());
}

#[test]
fn invalid_argument_uses_clap_error_path() {
    Command::cargo_bin("fmp-agent")
        .unwrap()
        .arg("--definitely-invalid")
        .env("CLAP_COLOR", "never")
        .env("NO_COLOR", "1")
        .assert()
        .failure()
        .stdout(predicate::str::is_empty())
        .stderr(predicate::str::contains(
            "unexpected argument '--definitely-invalid'",
        ));
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

    let assert = Command::cargo_bin("fmp-agent")
        .unwrap()
        .args([
            "--api-key",
            "test-key",
            "--base-url",
            &format!("{}/", server.base_url()),
            "search",
            "Apple",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains(r#"[{"symbol":"AAPL"}]"#));

    mock.assert();

    let output = assert.get_output();
    let stdout = String::from_utf8(output.stdout.clone()).unwrap();
    let stderr = String::from_utf8(output.stderr.clone()).unwrap();

    assert!(stderr.is_empty(), "expected empty stderr, got:\n{stderr}");
    assert_eq!(
        stdout.lines().count(),
        1,
        "expected one JSON line, got:\n{stdout}"
    );

    let output: serde_json::Value = serde_json::from_str(stdout.trim_end()).unwrap();
    assert_eq!(output, json!([{ "symbol": "AAPL" }]));
}

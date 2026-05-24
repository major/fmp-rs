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

#[test]
fn flat_command_help_present() {
    Command::cargo_bin("fmp-agent")
        .unwrap()
        .arg("--help")
        .env("CLAP_COLOR", "never")
        .env("NO_COLOR", "1")
        .assert()
        .success()
        .stdout(predicate::str::contains("company-profile"))
        .stdout(predicate::str::contains("market-quote"))
        .stdout(predicate::str::contains("news-stock"))
        .stdout(predicate::str::contains("sec-filings"))
        .stdout(predicate::str::contains("earnings-calendar"))
        .stdout(predicate::str::contains("treasury-rates"))
        .stdout(predicate::str::contains("\n  company ").not());
}

#[test]
fn no_old_domain_groups_in_help() {
    let output = Command::cargo_bin("fmp-agent")
        .unwrap()
        .arg("--help")
        .env("CLAP_COLOR", "never")
        .env("NO_COLOR", "1")
        .output()
        .unwrap();

    assert!(output.status.success(), "help should exit successfully");

    let stdout = String::from_utf8(output.stdout).unwrap();
    for command in [
        "company",
        "market",
        "fundamentals",
        "analyst",
        "calendar",
        "rates",
        "technical",
        "filings",
        "crypto",
        "forex",
        "news",
    ] {
        assert!(
            !stdout.contains(&format!("\n  {command} ")),
            "old domain group should not appear as command entry: {command}"
        );
    }
}

#[test]
fn flat_command_specific_help_present() {
    for command in ["company-profile", "market-quote", "news-stock"] {
        Command::cargo_bin("fmp-agent")
            .unwrap()
            .args([command, "--help"])
            .env("CLAP_COLOR", "never")
            .env("NO_COLOR", "1")
            .assert()
            .success()
            .stdout(predicate::str::contains(format!(
                "Usage: fmp-agent {command}"
            )));
    }
}

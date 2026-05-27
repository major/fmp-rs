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
fn help_explains_clap_and_json_error_paths() {
    Command::cargo_bin("fmp-agent")
        .unwrap()
        .arg("--help")
        .env("CLAP_COLOR", "never")
        .env("NO_COLOR", "1")
        .assert()
        .success()
        .stdout(predicate::str::contains("JSON envelope"))
        .stdout(predicate::str::contains(
            "Clap's native human-readable usage text",
        ))
        .stdout(predicate::str::contains(
            "check the exit code first, then parse stderr only for exit codes 3-6",
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
fn grouped_command_help_present() {
    Command::cargo_bin("fmp-agent")
        .unwrap()
        .arg("--help")
        .env("CLAP_COLOR", "never")
        .env("NO_COLOR", "1")
        .assert()
        .success()
        .stdout(predicate::str::contains("company"))
        .stdout(predicate::str::contains("market"))
        .stdout(predicate::str::contains("fundamentals"))
        .stdout(predicate::str::contains("macro"))
        .stdout(predicate::str::contains("technical"))
        .stdout(predicate::str::contains("news"))
        .stdout(predicate::str::contains("quote"))
        .stdout(predicate::str::contains("company-profile").not())
        .stdout(predicate::str::contains("market-quote").not());
}

#[test]
fn no_flat_commands_in_help() {
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
        "company-profile",
        "market-quote",
        "fundamentals-income-statement",
        "analyst-grades",
        "economic-indicators",
        "technical-sma",
        "sec-filings",
        "news-stock",
    ] {
        assert!(
            !stdout.contains(&format!("\n  {command} ")),
            "flat command should not appear as command entry: {command}"
        );
    }
}

#[test]
fn grouped_command_specific_help_present() {
    for (args, usage) in [
        (
            &["company", "profile", "--help"][..],
            "Usage: fmp-agent company profile",
        ),
        (
            &["market", "quote", "--help"][..],
            "Usage: fmp-agent market quote",
        ),
        (
            &["news", "stock", "--help"][..],
            "Usage: fmp-agent news stock",
        ),
    ] {
        Command::cargo_bin("fmp-agent")
            .unwrap()
            .args(args)
            .env("CLAP_COLOR", "never")
            .env("NO_COLOR", "1")
            .assert()
            .success()
            .stdout(predicate::str::contains(usage));
    }
}

#[test]
fn historical_rating_help_explains_limit_default() {
    Command::cargo_bin("fmp-agent")
        .unwrap()
        .args(["company", "historical-rating", "--help"])
        .env("CLAP_COLOR", "never")
        .env("NO_COLOR", "1")
        .assert()
        .success()
        .stdout(predicate::str::contains("[default: 10]"));
}

#[test]
fn technical_sma_help_explains_defaults() {
    Command::cargo_bin("fmp-agent")
        .unwrap()
        .args(["technical", "sma", "--help"])
        .env("CLAP_COLOR", "never")
        .env("NO_COLOR", "1")
        .assert()
        .success()
        .stdout(predicate::str::contains(
            "Defaults to a 10-period SMA on the 1day timeframe.",
        ))
        .stdout(predicate::str::contains(
            "Number of periods in the moving average window.",
        ))
        .stdout(predicate::str::contains(
            "FMP candle timeframe, for example 1day.",
        ));
}

#[test]
fn sec_filings_help_explains_required_default_date() {
    Command::cargo_bin("fmp-agent")
        .unwrap()
        .args(["sec", "filings", "--help"])
        .env("CLAP_COLOR", "never")
        .env("NO_COLOR", "1")
        .assert()
        .success()
        .stdout(predicate::str::contains(
            "Defaults --from to 90 days ago when omitted because FMP requires a start date.",
        ))
        .stdout(predicate::str::contains(
            "Omit for an open-ended start when supported.",
        ));
}

#[test]
fn annual_help_explains_limit_default_via_clap_marker() {
    Command::cargo_bin("fmp-agent")
        .unwrap()
        .args(["fundamentals", "income-statement", "--help"])
        .env("CLAP_COLOR", "never")
        .env("NO_COLOR", "1")
        .assert()
        .success()
        .stdout(predicate::str::contains("[default: 5]"));
}

#[test]
fn news_help_explains_limit_and_paging_defaults() {
    Command::cargo_bin("fmp-agent")
        .unwrap()
        .args(["news", "general", "--help"])
        .env("CLAP_COLOR", "never")
        .env("NO_COLOR", "1")
        .assert()
        .success()
        .stdout(predicate::str::contains("[default: 0]"))
        .stdout(predicate::str::contains("[default: 10]"))
        .stdout(predicate::str::contains("Zero-based result page."))
        .stdout(predicate::str::contains(
            "Maximum number of items to return.",
        ));
}

#[test]
fn stock_news_help_explains_limit_default() {
    Command::cargo_bin("fmp-agent")
        .unwrap()
        .args(["news", "stock", "--help"])
        .env("CLAP_COLOR", "never")
        .env("NO_COLOR", "1")
        .assert()
        .success()
        .stdout(predicate::str::contains("[default: 10]"));
}

#[test]
fn leaf_help_includes_global_api_key_and_base_url() {
    Command::cargo_bin("fmp-agent")
        .unwrap()
        .args(["market", "quote", "--help"])
        .env("CLAP_COLOR", "never")
        .env("NO_COLOR", "1")
        .assert()
        .success()
        .stdout(predicate::str::contains("--api-key"))
        .stdout(predicate::str::contains("FMP_API_KEY"))
        .stdout(predicate::str::contains("--base-url"))
        .stdout(predicate::str::contains("FMP_BASE_URL"));
}

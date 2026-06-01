use assert_cmd::Command;
use httpmock::Method::GET;
use httpmock::MockServer;
use predicates::prelude::*;
use serde_json::{Value, json};

#[test]
fn missing_api_key_returns_exit_code_3() {
    Command::cargo_bin("fmp-agent")
        .unwrap()
        .env("FMP_API_KEY", "")
        .args(["market", "quote", "AAPL"])
        .assert()
        .code(3);
}

#[test]
fn invalid_flag_returns_exit_code_2() {
    Command::cargo_bin("fmp-agent")
        .unwrap()
        .arg("--definitely-invalid")
        .assert()
        .code(2);
}

#[test]
fn missing_argument_returns_exit_code_2() {
    let server = MockServer::start();

    Command::cargo_bin("fmp-agent")
        .unwrap()
        .env("FMP_API_KEY", "test-key")
        .env("FMP_BASE_URL", format!("{}/", server.base_url()))
        .args(["news", "stock"])
        .assert()
        .code(2);
}

#[test]
fn http_error_returns_exit_code_4() {
    Command::cargo_bin("fmp-agent")
        .unwrap()
        .env("FMP_API_KEY", "test-key")
        .env("FMP_BASE_URL", "http://127.0.0.1:1/")
        .args(["market", "quote", "AAPL"])
        .assert()
        .code(4);
}

#[test]
fn api_error_returns_exit_code_5() {
    let server = MockServer::start();
    server.mock(|when, then| {
        when.method(GET).path("/quote");
        then.status(402)
            .body("Restricted Endpoint: upgrade required");
    });

    Command::cargo_bin("fmp-agent")
        .unwrap()
        .env("FMP_API_KEY", "test-key")
        .env("FMP_BASE_URL", format!("{}/", server.base_url()))
        .args(["market", "quote", "AAPL"])
        .assert()
        .code(5);
}

#[test]
fn etf_holdings_subscription_error_is_structured() {
    let server = MockServer::start();
    let mock = server.mock(|when, then| {
        when.method(GET)
            .path("/etf/holdings")
            .query_param("symbol", "SPY")
            .query_param("apikey", "test-key");
        then.status(402)
            .body("Restricted Endpoint: upgrade required");
    });

    let output = Command::cargo_bin("fmp-agent")
        .unwrap()
        .env("FMP_API_KEY", "test-key")
        .env("FMP_BASE_URL", format!("{}/", server.base_url()))
        .args(["etf", "holdings", "SPY"])
        .output()
        .unwrap();

    mock.assert();
    assert_eq!(output.status.code(), Some(5));
    assert!(output.stdout.is_empty());

    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(!stderr.contains("test-key"));

    let body: Value = serde_json::from_str(stderr.trim_end()).unwrap();
    assert_eq!(body["ok"], false);
    assert_eq!(body["error"]["kind"], "api_error");

    let message = body["error"]["message"].as_str().unwrap();
    assert!(message.contains("HTTP 402"));
    assert!(message.contains("Restricted Endpoint"));
}

#[test]
fn unavailable_legacy_endpoint_returns_structured_error_without_http_request() {
    let cases: &[&[&str]] = &[
        &["company", "outlook", "AAPL"],
        &["analyst", "price-target", "AAPL"],
        &["analyst", "upgrades-downgrades", "AAPL"],
        &["analyst", "earnings-surprises", "AAPL"],
    ];

    for args in cases {
        let server = MockServer::start();
        let mock = server.mock(|when, then| {
            when.method(GET);
            then.status(200).json_body(json!([{ "unexpected": true }]));
        });

        let output = Command::cargo_bin("fmp-agent")
            .unwrap()
            .env("FMP_API_KEY", "test-key")
            .env("FMP_BASE_URL", format!("{}/", server.base_url()))
            .args(*args)
            .output()
            .unwrap();

        mock.assert_calls(0);
        assert_eq!(output.status.code(), Some(5), "command: {args:?}");
        assert!(output.stdout.is_empty(), "command: {args:?}");

        let stderr = String::from_utf8(output.stderr).unwrap();
        assert!(!stderr.contains("test-key"));

        let body: Value = serde_json::from_str(stderr.trim_end()).unwrap();
        assert_eq!(body["ok"], false, "command: {args:?}");
        assert_eq!(
            body["error"]["kind"], "endpoint_unavailable",
            "command: {args:?}"
        );

        let message = body["error"]["message"].as_str().unwrap();
        assert!(message.contains("unavailable"), "command: {args:?}");
        assert!(message.contains("legacy endpoint"), "command: {args:?}");
    }
}

#[test]
fn rate_limited_error_is_structured_and_retryable() {
    let server = MockServer::start();
    let mock = server.mock(|when, then| {
        when.method(GET)
            .path("/quote")
            .query_param("symbol", "AAPL")
            .query_param("apikey", "test-key");
        then.status(429)
            .body(r#"{"Error Message":"Limit Reach for test-key. Please wait and try again."}"#);
    });

    let output = Command::cargo_bin("fmp-agent")
        .unwrap()
        .env("FMP_API_KEY", "test-key")
        .env("FMP_BASE_URL", format!("{}/", server.base_url()))
        .args(["market", "quote", "AAPL"])
        .output()
        .unwrap();

    mock.assert();
    assert_eq!(output.status.code(), Some(5));
    assert!(output.stdout.is_empty());

    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(!stderr.contains("test-key"));

    let body: Value = serde_json::from_str(stderr.trim_end()).unwrap();
    assert_eq!(body["ok"], false);
    assert_eq!(body["error"]["kind"], "rate_limited");

    let message = body["error"]["message"].as_str().unwrap();
    assert!(message.contains("HTTP 429"));
    assert!(message.contains("Limit Reach"));
    assert!(message.contains("***"));
}

#[test]
fn json_parse_error_returns_exit_code_6() {
    let server = MockServer::start();
    server.mock(|when, then| {
        when.method(GET).path("/quote");
        then.status(200).body("this is not valid json");
    });

    Command::cargo_bin("fmp-agent")
        .unwrap()
        .env("FMP_API_KEY", "test-key")
        .env("FMP_BASE_URL", format!("{}/", server.base_url()))
        .args(["market", "quote", "AAPL"])
        .assert()
        .code(6);
}

#[test]
fn success_returns_exit_code_0() {
    let server = MockServer::start();
    let _mock = server.mock(|when, then| {
        when.method(GET)
            .path("/quote")
            .query_param("symbol", "AAPL");
        then.status(200)
            .json_body(json!([{"symbol": "AAPL", "price": 150.0}]));
    });

    Command::cargo_bin("fmp-agent")
        .unwrap()
        .env("FMP_API_KEY", "test-key")
        .env("FMP_BASE_URL", format!("{}/", server.base_url()))
        .args(["market", "quote", "AAPL"])
        .assert()
        .code(0);
}

#[test]
fn empty_symbol_result_defaults_to_raw_success() {
    let server = MockServer::start();
    let mock = server.mock(|when, then| {
        when.method(GET)
            .path("/quote")
            .query_param("symbol", "NOTAREALSYMBOL12345");
        then.status(200).json_body(json!([]));
    });

    let output = Command::cargo_bin("fmp-agent")
        .unwrap()
        .env("FMP_API_KEY", "test-key")
        .env("FMP_BASE_URL", format!("{}/", server.base_url()))
        .args(["market", "quote", "NOTAREALSYMBOL12345"])
        .output()
        .unwrap();

    mock.assert();
    assert_eq!(output.status.code(), Some(0));
    assert!(output.stderr.is_empty());

    let stdout = String::from_utf8(output.stdout).unwrap();
    assert_eq!(stdout.trim_end(), "[]");
}

#[test]
fn strict_empty_symbol_result_returns_exit_code_7() {
    let server = MockServer::start();
    let mock = server.mock(|when, then| {
        when.method(GET)
            .path("/quote")
            .query_param("symbol", "NOTAREALSYMBOL12345")
            .query_param("apikey", "test-key");
        then.status(200).json_body(json!([]));
    });

    let output = Command::cargo_bin("fmp-agent")
        .unwrap()
        .env("FMP_API_KEY", "test-key")
        .env("FMP_BASE_URL", format!("{}/", server.base_url()))
        .args(["--strict-empty", "market", "quote", "NOTAREALSYMBOL12345"])
        .output()
        .unwrap();

    mock.assert();
    assert_eq!(output.status.code(), Some(7));
    assert!(output.stdout.is_empty());

    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(!stderr.contains("test-key"));

    let body: Value = serde_json::from_str(stderr.trim_end()).unwrap();
    assert_eq!(body["ok"], false);
    assert_eq!(body["error"]["kind"], "empty_result");

    let message = body["error"]["message"].as_str().unwrap();
    assert!(message.contains("NOTAREALSYMBOL12345"));
    assert!(message.contains("search NOTAREALSYMBOL12345"));
    assert!(message.contains("--strict-empty"));
}

#[test]
fn invalid_from_date_returns_exit_code_2() {
    let server = MockServer::start();

    Command::cargo_bin("fmp-agent")
        .unwrap()
        .env("FMP_API_KEY", "test-key")
        .env("FMP_BASE_URL", format!("{}/", server.base_url()))
        .args(["market", "historical", "AAPL", "--from", "not-a-date"])
        .assert()
        .code(2)
        .stderr(predicate::str::contains("YYYY-MM-DD"));
}

#[test]
fn invalid_date_range_command_returns_exit_code_2() {
    let server = MockServer::start();

    Command::cargo_bin("fmp-agent")
        .unwrap()
        .env("FMP_API_KEY", "test-key")
        .env("FMP_BASE_URL", format!("{}/", server.base_url()))
        .args(["calendar", "earnings", "--from", "not-a-date"])
        .assert()
        .code(2)
        .stderr(predicate::str::contains("YYYY-MM-DD"));
}

#[test]
fn invalid_date_no_http_request() {
    // When date validation fails, no HTTP request should be made.
    let server = MockServer::start();
    let mock = server.mock(|when, then| {
        when.method(GET).path("/historical-price-eod/full");
        then.status(200).json_body(json!([]));
    });

    Command::cargo_bin("fmp-agent")
        .unwrap()
        .env("FMP_API_KEY", "test-key")
        .env("FMP_BASE_URL", format!("{}/", server.base_url()))
        .args([
            "market",
            "historical",
            "AAPL",
            "--from",
            "2026-99-99",
            "--to",
            "2026-05",
        ])
        .assert()
        .code(2)
        .stderr(predicate::str::contains("YYYY-MM-DD"));

    mock.assert_calls(0);
}

#[test]
fn valid_date_range_command_succeeds() {
    let server = MockServer::start();
    let mock = server.mock(|when, then| {
        when.method(GET)
            .path("/historical-price-eod/full")
            .query_param("from", "2025-01-02")
            .query_param("to", "2025-01-10");
        then.status(200).json_body(json!({ "historical": [] }));
    });

    let output = Command::cargo_bin("fmp-agent")
        .unwrap()
        .env("FMP_API_KEY", "test-key")
        .env("FMP_BASE_URL", format!("{}/", server.base_url()))
        .args([
            "market",
            "historical",
            "AAPL",
            "--from",
            "2025-01-02",
            "--to",
            "2025-01-10",
        ])
        .output()
        .unwrap();

    mock.assert();
    assert_eq!(output.status.code(), Some(0));
}

#[test]
fn invalid_date_on_sec_filings_returns_exit_code_2() {
    let server = MockServer::start();

    Command::cargo_bin("fmp-agent")
        .unwrap()
        .env("FMP_API_KEY", "test-key")
        .env("FMP_BASE_URL", format!("{}/", server.base_url()))
        .args(["sec", "filings", "AAPL", "--from", "not-a-date"])
        .assert()
        .code(2)
        .stderr(predicate::str::contains("YYYY-MM-DD"));
}

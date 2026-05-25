use assert_cmd::Command;
use httpmock::Method::GET;
use httpmock::MockServer;
use serde_json::{Value, json};

#[test]
fn missing_api_key_returns_exit_code_3() {
    Command::cargo_bin("fmp-agent")
        .unwrap()
        .env("FMP_API_KEY", "")
        .args(["market-quote", "AAPL"])
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
        .args(["news-stock"])
        .assert()
        .code(2);
}

#[test]
fn http_error_returns_exit_code_4() {
    Command::cargo_bin("fmp-agent")
        .unwrap()
        .env("FMP_API_KEY", "test-key")
        .env("FMP_BASE_URL", "http://127.0.0.1:1/")
        .args(["market-quote", "AAPL"])
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
        .args(["market-quote", "AAPL"])
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
        .args(["etf-holdings", "SPY"])
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
        .args(["market-quote", "AAPL"])
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
        .args(["market-quote", "AAPL"])
        .assert()
        .code(0);
}

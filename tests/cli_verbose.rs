use assert_cmd::Command;
use httpmock::Method::GET;
use httpmock::MockServer;
use serde_json::json;

#[test]
fn verbose_flag_produces_debug_output() {
    let server = MockServer::start();
    let _mock = server.mock(|when, then| {
        when.method(GET)
            .path("/quote")
            .query_param("symbol", "AAPL");
        then.status(200)
            .json_body(json!([{"symbol": "AAPL", "price": 150.0}]));
    });

    let output = Command::cargo_bin("fmp-agent")
        .unwrap()
        .env("FMP_API_KEY", "secret-test-key")
        .env("FMP_BASE_URL", format!("{}/", server.base_url()))
        .args(["-vvv", "market", "quote", "AAPL"])
        .output()
        .unwrap();

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("GET "),
        "expected debug log with GET URL on stderr, got: {stderr}"
    );
}

#[test]
fn verbose_flag_redacts_api_key() {
    let server = MockServer::start();
    let _mock = server.mock(|when, then| {
        when.method(GET)
            .path("/quote")
            .query_param("symbol", "AAPL");
        then.status(200)
            .json_body(json!([{"symbol": "AAPL", "price": 150.0}]));
    });

    let output = Command::cargo_bin("fmp-agent")
        .unwrap()
        .env("FMP_API_KEY", "secret-test-key")
        .env("FMP_BASE_URL", format!("{}/", server.base_url()))
        .args(["-vvv", "market", "quote", "AAPL"])
        .output()
        .unwrap();

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        !stderr.contains("secret-test-key"),
        "API key must not appear in log output"
    );
}

#[test]
fn no_verbose_output_without_flag() {
    let server = MockServer::start();
    let _mock = server.mock(|when, then| {
        when.method(GET)
            .path("/quote")
            .query_param("symbol", "AAPL");
        then.status(200)
            .json_body(json!([{"symbol": "AAPL", "price": 150.0}]));
    });

    let output = Command::cargo_bin("fmp-agent")
        .unwrap()
        .env("FMP_API_KEY", "secret-test-key")
        .env("FMP_BASE_URL", format!("{}/", server.base_url()))
        .args(["market", "quote", "AAPL"])
        .output()
        .unwrap();

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        !stderr.contains("DEBUG"),
        "no debug output expected without verbose flag, got: {stderr}"
    );
}

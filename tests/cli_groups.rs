//! TDD integration tests for the grouped command tree (issue #38).
//!
//! These tests are intentionally RED until the grouped command enum lands.
//! Each test invokes a grouped command path (`fmp-agent <group> <subcommand>`)
//! that does not exist in the current flat CLI, so the assertions expect the
//! command to succeed and should fail for now.

use assert_cmd::Command;
use httpmock::Method::GET;
use httpmock::MockServer;
use serde_json::json;

fn run_group_test(args: &[&str], path: &str, query: &[(&str, &str)], body: serde_json::Value) {
    let server = MockServer::start();
    let _mock = server.mock(|when, then| {
        let _when = query
            .iter()
            .fold(when.method(GET).path(path), |when, (key, value)| {
                when.query_param(*key, *value)
            });
        then.status(200).json_body(body);
    });

    Command::cargo_bin("fmp-agent")
        .unwrap()
        .env("FMP_API_KEY", "test-key")
        .env("FMP_BASE_URL", format!("{}/", server.base_url()))
        .args(args)
        .assert()
        .success();
}

#[test]
fn group_company_profile() {
    run_group_test(
        &["company", "profile", "AAPL"],
        "/profile",
        &[("symbol", "AAPL"), ("apikey", "test-key")],
        json!([{ "symbol": "AAPL" }]),
    );
}

#[test]
fn group_market_quote() {
    run_group_test(
        &["market", "quote", "AAPL"],
        "/quote",
        &[("symbol", "AAPL"), ("apikey", "test-key")],
        json!([{ "symbol": "AAPL" }]),
    );
}

#[test]
fn group_fundamentals_income_statement() {
    run_group_test(
        &["fundamentals", "income-statement", "AAPL"],
        "/income-statement",
        &[("symbol", "AAPL"), ("apikey", "test-key")],
        json!([{}]),
    );
}

#[test]
fn group_analyst_grades() {
    run_group_test(
        &["analyst", "grades", "AAPL"],
        "/analyst-stock-ratings",
        &[("symbol", "AAPL"), ("apikey", "test-key")],
        json!([{}]),
    );
}

#[test]
fn group_insider_latest() {
    run_group_test(
        &["insider", "latest"],
        "/insider-trading",
        &[("apikey", "test-key")],
        json!([{}]),
    );
}

#[test]
fn group_calendar_earnings() {
    run_group_test(
        &["calendar", "earnings"],
        "/earning_calendar",
        &[("apikey", "test-key")],
        json!([{}]),
    );
}

#[test]
fn group_macro_treasury_rates() {
    run_group_test(
        &["macro", "treasury-rates"],
        "/treasury",
        &[("apikey", "test-key")],
        json!([{}]),
    );
}

#[test]
fn group_technical_sma() {
    run_group_test(
        &["technical", "sma", "AAPL"],
        "/technical_indicator/1day/AAPL",
        &[("apikey", "test-key")],
        json!([{}]),
    );
}

#[test]
fn group_sec_filings() {
    run_group_test(
        &["sec", "filings", "AAPL"],
        "/sec_filings",
        &[("apikey", "test-key")],
        json!([{}]),
    );
}

#[test]
fn group_etf_holdings() {
    run_group_test(
        &["etf", "holdings", "SPY"],
        "/etf/holdings",
        &[("apikey", "test-key")],
        json!([{}]),
    );
}

#[test]
fn group_crypto_quote() {
    run_group_test(
        &["crypto", "quote", "BTCUSD"],
        "/quote",
        &[("apikey", "test-key")],
        json!([{}]),
    );
}

#[test]
fn group_forex_quote() {
    run_group_test(
        &["forex", "quote", "EURUSD"],
        "/quote",
        &[("apikey", "test-key")],
        json!([{}]),
    );
}

#[test]
fn group_news_stock() {
    run_group_test(
        &["news", "stock", "AAPL"],
        "/stock_news",
        &[("apikey", "test-key")],
        json!([{}]),
    );
}

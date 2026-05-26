use assert_cmd::Command;
use predicates::prelude::*;

fn assert_bare_group_help(group: &str) {
    let usage = format!("fmp-agent {group}");

    Command::cargo_bin("fmp-agent")
        .unwrap()
        .env_remove("FMP_API_KEY")
        .arg(group)
        .assert()
        .success()
        .stdout(predicate::str::contains("Usage:"))
        // The usage line includes the full binary-qualified path with
        // [OPTIONS] and [COMMAND], matching `fmp-agent <group> --help`.
        .stdout(predicate::str::contains(&usage))
        .stdout(predicate::str::contains("[OPTIONS]"))
        .stdout(predicate::str::contains("[COMMAND]"));
}

#[test]
fn bare_company_prints_help() {
    assert_bare_group_help("company");
}

#[test]
fn bare_market_prints_help() {
    assert_bare_group_help("market");
}

#[test]
fn bare_fundamentals_prints_help() {
    assert_bare_group_help("fundamentals");
}

#[test]
fn bare_analyst_prints_help() {
    assert_bare_group_help("analyst");
}

#[test]
fn bare_insider_prints_help() {
    assert_bare_group_help("insider");
}

#[test]
fn bare_calendar_prints_help() {
    assert_bare_group_help("calendar");
}

#[test]
fn bare_macro_prints_help() {
    assert_bare_group_help("macro");
}

#[test]
fn bare_technical_prints_help() {
    assert_bare_group_help("technical");
}

#[test]
fn bare_sec_prints_help() {
    assert_bare_group_help("sec");
}

#[test]
fn bare_etf_prints_help() {
    assert_bare_group_help("etf");
}

#[test]
fn bare_crypto_prints_help() {
    assert_bare_group_help("crypto");
}

#[test]
fn bare_forex_prints_help() {
    assert_bare_group_help("forex");
}

#[test]
fn bare_news_prints_help() {
    assert_bare_group_help("news");
}

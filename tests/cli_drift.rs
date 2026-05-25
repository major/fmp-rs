use std::collections::BTreeSet;

use assert_cmd::Command;
use clap::CommandFactory;
use predicates::prelude::*;

const COMMANDS: &[&str] = &[
    "company-profile",
    "company-executives",
    "company-peers",
    "company-financial-scores",
    "company-share-float",
    "company-rating",
    "company-historical-rating",
    "market-stock-list",
    "market-quote",
    "market-historical",
    "market-dividends",
    "market-splits",
    "market-price-change",
    "fundamentals-income-statement",
    "fundamentals-income-statement-as-reported",
    "fundamentals-balance-sheet",
    "fundamentals-cash-flow",
    "fundamentals-ratios",
    "fundamentals-metrics",
    "fundamentals-income-statement-growth",
    "fundamentals-balance-sheet-growth",
    "fundamentals-cash-flow-growth",
    "fundamentals-enterprise-values",
    "fundamentals-analyst-estimates",
    "fundamentals-report-dates",
    "fundamentals-annual-report-form",
    "analyst-price-target-consensus",
    "analyst-price-target-summary",
    "analyst-grades",
    "treasury-rates",
    "economic-indicators",
    "insider-trading-latest",
    "technical-sma",
    "sec-filings",
    "news-stock",
    "news-general",
    "news-articles",
    "news-forex",
    "news-crypto",
    "search",
    "crypto-list",
    "crypto-quote",
    "crypto-historical",
    "forex-quote",
    "forex-historical",
    "etf-holdings",
    "earnings-calendar",
    "schema",
];

#[test]
fn root_help_includes_every_command() {
    let output = Command::cargo_bin("fmp-agent")
        .unwrap()
        .arg("--help")
        .env("CLAP_COLOR", "never")
        .env("NO_COLOR", "1")
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();

    let stdout = String::from_utf8(output).unwrap();

    for command in COMMANDS {
        assert!(
            stdout.contains(command),
            "expected root help to contain command {command}, got:\n{stdout}"
        );
    }
}

#[test]
fn introspected_subcommands_match_array() {
    let command = <rusty_fmp::Cli as CommandFactory>::command();
    let introspected: BTreeSet<&str> = command
        .get_subcommands()
        .map(|subcommand| subcommand.get_name())
        .collect();
    let listed: BTreeSet<&str> = COMMANDS.iter().copied().collect();

    assert_eq!(introspected, listed);
}

#[test]
fn every_command_help_contains_examples() {
    for command in COMMANDS {
        Command::cargo_bin("fmp-agent")
            .unwrap()
            .arg(command)
            .arg("--help")
            .env("CLAP_COLOR", "never")
            .env("NO_COLOR", "1")
            .assert()
            .success()
            .stdout(predicate::str::contains("Examples:"));
    }
}

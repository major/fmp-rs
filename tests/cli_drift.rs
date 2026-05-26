use std::collections::BTreeSet;

use assert_cmd::Command;
use clap::CommandFactory;
use predicates::prelude::*;

const COMMANDS: &[&str] = &[
    "search",
    "schema",
    "quote",
    "historical",
    "profile",
    "earnings",
    "company",
    "market",
    "fundamentals",
    "analyst",
    "insider",
    "calendar",
    "macro",
    "technical",
    "sec",
    "etf",
    "crypto",
    "forex",
    "news",
];

#[test]
fn root_help_lists_every_command() {
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

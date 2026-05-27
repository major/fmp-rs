use std::collections::BTreeSet;

use clap::CommandFactory;

/// Recursively walk a clap command tree and collect all leaf command paths.
///
/// A leaf is a subcommand that either has no children or whose only child is the
/// clap-injected `help` subcommand.  Group nodes (commands that have real child
/// subcommands) are recursed into rather than collected.
fn collect_leaf_commands(cmd: &clap::Command, prefix: Vec<String>) -> Vec<Vec<String>> {
    let mut leaves = Vec::new();
    for sub in cmd.get_subcommands() {
        if sub.get_name() == "help" {
            continue;
        }
        let mut child_path = prefix.clone();
        child_path.push(sub.get_name().to_owned());

        let real_children: Vec<_> = sub
            .get_subcommands()
            .filter(|c| c.get_name() != "help")
            .collect();

        if real_children.is_empty() {
            leaves.push(child_path);
        } else {
            leaves.extend(collect_leaf_commands(sub, child_path));
        }
    }
    leaves
}

/// Convert a leaf path to a comparable string for readable assertion diffs.
fn path_str(path: &[String]) -> String {
    path.join(" ")
}

/// All expected leaf command paths in the CLI tree.
///
/// Grouped by domain: top-level aliases, top-level standalone commands, then
/// each group alphabetically.
const EXPECTED_LEAVES: &[&[&str]] = &[
    // Top-level aliases (4)
    &["quote"],
    &["historical"],
    &["profile"],
    &["earnings"],
    // Top-level standalone (5)
    &["search"],
    &["schema"],
    &["commands"],
    &["completions"],
    &["doctor"],
    // company (7)
    &["company", "profile"],
    &["company", "executives"],
    &["company", "peers"],
    &["company", "scores"],
    &["company", "float"],
    &["company", "rating"],
    &["company", "historical-rating"],
    // market (7)
    &["market", "quote"],
    &["market", "historical"],
    &["market", "dividends"],
    &["market", "splits"],
    &["market", "batch-quote"],
    &["market", "price-change"],
    &["market", "stock-list"],
    // fundamentals (13)
    &["fundamentals", "income-statement"],
    &["fundamentals", "income-statement-as-reported"],
    &["fundamentals", "balance-sheet"],
    &["fundamentals", "cash-flow"],
    &["fundamentals", "ratios"],
    &["fundamentals", "metrics"],
    &["fundamentals", "income-statement-growth"],
    &["fundamentals", "balance-sheet-growth"],
    &["fundamentals", "cash-flow-growth"],
    &["fundamentals", "enterprise-values"],
    &["fundamentals", "analyst-estimates"],
    &["fundamentals", "report-dates"],
    &["fundamentals", "annual-report"],
    // analyst (3)
    &["analyst", "price-target-consensus"],
    &["analyst", "price-target-summary"],
    &["analyst", "grades"],
    // insider (1)
    &["insider", "latest"],
    // calendar (1)
    &["calendar", "earnings"],
    // macro (2)
    &["macro", "treasury-rates"],
    &["macro", "economic-indicators"],
    // technical (1)
    &["technical", "sma"],
    // sec (1)
    &["sec", "filings"],
    // etf (1)
    &["etf", "holdings"],
    // crypto (3)
    &["crypto", "list"],
    &["crypto", "quote"],
    &["crypto", "historical"],
    // forex (2)
    &["forex", "quote"],
    &["forex", "historical"],
    // news (5)
    &["news", "stock"],
    &["news", "general"],
    &["news", "articles"],
    &["news", "forex"],
    &["news", "crypto"],
];

/// Verify that the clap command tree contains exactly the expected set of leaf
/// commands, with no missing or unexpected entries.
#[test]
fn leaf_commands_match_expected() {
    let cmd = <rusty_fmp::Cli as CommandFactory>::command();
    let actual: BTreeSet<String> = collect_leaf_commands(&cmd, Vec::new())
        .iter()
        .map(|p| path_str(p))
        .collect();

    let expected: BTreeSet<String> = EXPECTED_LEAVES
        .iter()
        .map(|parts| parts.join(" "))
        .collect();

    let missing: BTreeSet<_> = expected.difference(&actual).collect();
    let unexpected: BTreeSet<_> = actual.difference(&expected).collect();

    assert!(
        missing.is_empty() && unexpected.is_empty(),
        "leaf command mismatch\n  missing:    {missing:?}\n  unexpected: {unexpected:?}"
    );

    assert_eq!(
        actual.len(),
        EXPECTED_LEAVES.len(),
        "expected {} leaves, found {}",
        EXPECTED_LEAVES.len(),
        actual.len()
    );
}

/// Every leaf command must have `about` set.
#[test]
fn every_leaf_has_about() {
    let cmd = <rusty_fmp::Cli as CommandFactory>::command();
    check_leaf_help(&cmd, Vec::new(), &HelpField::About);
}

/// Every leaf command must have `long_about` set.
#[test]
fn every_leaf_has_long_about() {
    let cmd = <rusty_fmp::Cli as CommandFactory>::command();
    check_leaf_help(&cmd, Vec::new(), &HelpField::LongAbout);
}

enum HelpField {
    About,
    LongAbout,
}

fn check_leaf_help(cmd: &clap::Command, prefix: Vec<String>, field: &HelpField) {
    for sub in cmd.get_subcommands() {
        if sub.get_name() == "help" {
            continue;
        }
        let mut child_path = prefix.clone();
        child_path.push(sub.get_name().to_owned());

        let real_children: Vec<_> = sub
            .get_subcommands()
            .filter(|c| c.get_name() != "help")
            .collect();

        if real_children.is_empty() {
            let display = path_str(&child_path);
            match field {
                HelpField::About => {
                    assert!(
                        sub.get_about().is_some(),
                        "leaf `{display}` is missing `about`"
                    );
                }
                HelpField::LongAbout => {
                    assert!(
                        sub.get_long_about().is_some(),
                        "leaf `{display}` is missing `long_about`"
                    );
                }
            }
        } else {
            check_leaf_help(sub, child_path, field);
        }
    }
}

/// No top-level command should use a flat kebab-case name like `market-quote`.
/// Kebab-case is fine inside groups (e.g. `historical-rating` under `company`),
/// but top-level commands must be single words or top-level aliases.
#[test]
fn no_flat_kebab_commands_at_top_level() {
    let cmd = <rusty_fmp::Cli as CommandFactory>::command();

    for sub in cmd.get_subcommands() {
        let name = sub.get_name();
        if name == "help" {
            continue;
        }
        assert!(
            !name.contains('-'),
            "top-level command `{name}` looks like a flat kebab-case name; \
             commands should be grouped (e.g. `market quote`) or single-word aliases"
        );
    }
}

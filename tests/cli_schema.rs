use assert_cmd::Command;
use serde_json::Value;

#[test]
fn schema_emits_valid_json_with_required_fields() {
    let output = Command::cargo_bin("fmp-agent")
        .unwrap()
        .env("FMP_API_KEY", "test_only_not_used_by_schema")
        .arg("schema")
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();

    let body: Value = serde_json::from_slice(&output).expect("schema output should be valid JSON");

    assert_eq!(body["schema_version"], 1, "schema_version must be 1");
    assert_eq!(body["binary"], "fmp-agent", "binary must be fmp-agent");

    let version = env!("CARGO_PKG_VERSION");
    assert_eq!(
        body["version"], version,
        "version must match CARGO_PKG_VERSION"
    );

    let commands = body["commands"]
        .as_array()
        .expect("commands must be an array");
    // Schema v1 enumerates top-level commands (13 groups + 4 aliases + search + schema = 19).
    // Task 15 rewrites schema to v2 with recursive walk that surfaces all leaf commands.
    assert!(
        commands.len() >= 19,
        "must have at least 19 top-level commands (13 groups + 4 aliases + search + schema), got {}",
        commands.len()
    );

    for cmd in commands {
        assert!(
            cmd["name"].is_string(),
            "each command must have a string name"
        );
        assert!(
            cmd["about"].is_string() || cmd["about"].is_null(),
            "each command must have about (string or null)"
        );
    }

    // At least one command's long_about contains Examples:
    let has_examples = commands.iter().any(|cmd| {
        cmd["long_about"]
            .as_str()
            .is_some_and(|s| s.contains("Examples:"))
    });
    assert!(
        has_examples,
        "at least one command long_about must contain Examples:"
    );

    // Schema classifies args into positional, option, and flag kinds
    let all_kinds: Vec<&str> = commands
        .iter()
        .flat_map(|cmd| {
            cmd["args"]
                .as_array()
                .into_iter()
                .flatten()
                .filter_map(|a| a["kind"].as_str())
        })
        .collect();
    assert!(
        all_kinds.contains(&"option"),
        "at least one arg should have kind 'option'"
    );
    assert!(
        all_kinds.contains(&"positional"),
        "at least one arg should have kind 'positional'"
    );
}

#[test]
fn schema_works_without_api_key() {
    let output = Command::cargo_bin("fmp-agent")
        .unwrap()
        .env_remove("FMP_API_KEY")
        .arg("schema")
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();

    let body: Value =
        serde_json::from_slice(&output).expect("schema output without key should be valid JSON");
    assert_eq!(body["schema_version"], 1);
}

#[test]
fn other_commands_still_require_api_key() {
    Command::cargo_bin("fmp-agent")
        .unwrap()
        .env_remove("FMP_API_KEY")
        .args(["market", "quote", "AAPL"])
        .assert()
        .failure()
        .code(3);
}

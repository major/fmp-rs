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
    assert!(
        commands.len() >= 48,
        "must have at least 48 commands (47 + schema), got {}",
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
}

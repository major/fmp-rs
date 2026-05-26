use assert_cmd::Command;
use serde_json::Value;

/// Parse schema output from the binary.
fn schema_body() -> Value {
    let output = Command::cargo_bin("fmp-agent")
        .unwrap()
        .env("FMP_API_KEY", "test_only_not_used_by_schema")
        .arg("schema")
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();

    serde_json::from_slice(&output).expect("schema output should be valid JSON")
}

#[test]
fn schema_version_is_2() {
    let body = schema_body();
    assert_eq!(body["schema_version"], 2, "schema_version must be 2");
    assert_eq!(body["binary"], "fmp-agent");

    let version = env!("CARGO_PKG_VERSION");
    assert_eq!(body["version"], version);
}

#[test]
fn schema_has_13_groups_in_order() {
    let body = schema_body();
    let groups = body["groups"].as_array().expect("groups must be an array");
    let expected = [
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
    let names: Vec<&str> = groups.iter().filter_map(|g| g.as_str()).collect();
    assert_eq!(
        names, expected,
        "groups must list 13 names in declared order"
    );
}

#[test]
fn schema_market_quote_leaf() {
    let body = schema_body();
    let commands = body["commands"].as_array().unwrap();
    let mq = commands
        .iter()
        .find(|c| c["path"] == serde_json::json!(["market", "quote"]))
        .expect("must have market quote leaf");

    assert_eq!(mq["name"], "quote");
    assert_eq!(mq["api_key_required"], true);
    assert_eq!(mq["preferred_path"], "market quote");
    assert_eq!(mq["aliases"], serde_json::json!(["quote"]));
    assert!(mq["about"].is_string(), "about must be a string");
}

#[test]
fn schema_leaf_is_not_api_key_required() {
    let body = schema_body();
    let commands = body["commands"].as_array().unwrap();
    let schema_cmd = commands
        .iter()
        .find(|c| c["name"] == "schema" && c["path"] == serde_json::json!(["schema"]))
        .expect("must have schema leaf");

    assert_eq!(schema_cmd["api_key_required"], false);
    assert_eq!(schema_cmd["preferred_path"], "schema");
}

#[test]
fn schema_alias_quote_has_preferred_path() {
    let body = schema_body();
    let commands = body["commands"].as_array().unwrap();
    let alias = commands
        .iter()
        .find(|c| c["path"] == serde_json::json!(["quote"]))
        .expect("must have top-level quote alias");

    assert_eq!(alias["preferred_path"], "market quote");
    assert_eq!(alias["aliases"], serde_json::json!([]));
    assert_eq!(alias["api_key_required"], true);
}

#[test]
fn schema_canonical_has_alias_backref() {
    let body = schema_body();
    let commands = body["commands"].as_array().unwrap();

    // company profile has alias "profile"
    let cp = commands
        .iter()
        .find(|c| c["path"] == serde_json::json!(["company", "profile"]))
        .expect("must have company profile leaf");
    assert_eq!(cp["aliases"], serde_json::json!(["profile"]));

    // calendar earnings has alias "earnings"
    let ce = commands
        .iter()
        .find(|c| c["path"] == serde_json::json!(["calendar", "earnings"]))
        .expect("must have calendar earnings leaf");
    assert_eq!(ce["aliases"], serde_json::json!(["earnings"]));

    // market historical has alias "historical"
    let mh = commands
        .iter()
        .find(|c| c["path"] == serde_json::json!(["market", "historical"]))
        .expect("must have market historical leaf");
    assert_eq!(mh["aliases"], serde_json::json!(["historical"]));
}

#[test]
fn schema_commands_have_args() {
    let body = schema_body();
    let commands = body["commands"].as_array().unwrap();

    // At least one command has a positional arg
    let has_positional = commands.iter().any(|c| {
        c["args"]
            .as_array()
            .into_iter()
            .flatten()
            .any(|a| a["kind"] == "positional")
    });
    assert!(
        has_positional,
        "at least one command should have a positional arg"
    );

    // At least one command has an option arg
    let has_option = commands.iter().any(|c| {
        c["args"]
            .as_array()
            .into_iter()
            .flatten()
            .any(|a| a["kind"] == "option")
    });
    assert!(has_option, "at least one command should have an option arg");

    // At least one long_about contains Examples:
    let has_examples = commands.iter().any(|c| {
        c["long_about"]
            .as_str()
            .is_some_and(|s| s.contains("Examples:"))
    });
    assert!(
        has_examples,
        "at least one long_about must contain Examples:"
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
    assert_eq!(body["schema_version"], 2);
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

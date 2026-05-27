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
fn schema_version_is_3() {
    let body = schema_body();
    assert_eq!(body["schema_version"], 3, "schema_version must be 3");
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

    // Metadata-only commands that do not need FMP_API_KEY
    for &name in &["schema", "commands", "completions"] {
        let cmd = commands
            .iter()
            .find(|c| c["name"] == name && c["path"] == serde_json::json!([name]))
            .unwrap_or_else(|| panic!("must have {name} leaf"));

        assert_eq!(
            cmd["api_key_required"], false,
            "{name} should not require an API key"
        );
        assert_eq!(cmd["preferred_path"], name);
    }
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
fn schema_positional_arg_has_metadata() {
    let body = schema_body();
    let commands = body["commands"].as_array().unwrap();

    // market quote has a positional "symbol" arg
    let mq = commands
        .iter()
        .find(|c| c["path"] == serde_json::json!(["market", "quote"]))
        .expect("must have market quote leaf");

    let symbol_arg = mq["args"]
        .as_array()
        .unwrap()
        .iter()
        .find(|a| a["name"] == "symbol")
        .expect("market quote must have symbol arg");

    // Existing fields
    assert_eq!(symbol_arg["kind"], "positional");
    assert_eq!(symbol_arg["value_name"], "SYMBOL");
    assert_eq!(symbol_arg["required"], true);

    // New v3 fields
    assert!(
        symbol_arg["long"].is_null(),
        "positional arg should not have long flag"
    );
    assert!(
        symbol_arg["short"].is_null(),
        "positional arg should not have short flag"
    );
    assert_eq!(symbol_arg["parser"]["hint"], "string");
    assert!(
        symbol_arg["possible_values"].is_null(),
        "positional arg should not have possible values"
    );
    assert_eq!(symbol_arg["multi_value"], false);
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
    assert_eq!(body["schema_version"], 3);
}

#[test]
fn schema_option_with_default_has_long_and_integer_hint() {
    let body = schema_body();
    let commands = body["commands"].as_array().unwrap();

    // technical sma has --period-length with default 10
    let sma = commands
        .iter()
        .find(|c| c["path"] == serde_json::json!(["technical", "sma"]))
        .expect("must have technical sma leaf");

    let period_arg = sma["args"]
        .as_array()
        .unwrap()
        .iter()
        .find(|a| a["name"] == "period_length")
        .expect("technical sma must have period_length arg");

    assert_eq!(period_arg["kind"], "option");
    assert_eq!(period_arg["long"], "period-length");
    assert!(period_arg["short"].is_null());
    assert_eq!(period_arg["default"], 10);
    assert_eq!(period_arg["parser"]["hint"], "integer");
    assert!(period_arg["possible_values"].is_null());
    assert_eq!(period_arg["multi_value"], false);
}

#[test]
fn schema_enum_arg_has_possible_values() {
    let body = schema_body();
    let commands = body["commands"].as_array().unwrap();

    // completions has a shell arg with value_enum
    let completions = commands
        .iter()
        .find(|c| c["name"] == "completions")
        .expect("must have completions leaf");

    let shell_arg = completions["args"]
        .as_array()
        .unwrap()
        .iter()
        .find(|a| a["name"] == "shell")
        .expect("completions must have shell arg");

    assert_eq!(shell_arg["kind"], "positional");
    assert_eq!(shell_arg["required"], true);
    assert_eq!(shell_arg["parser"]["hint"], "enum");

    let possible = shell_arg["possible_values"]
        .as_array()
        .expect("shell arg must have possible_values array");
    assert!(!possible.is_empty(), "shell should have possible values");

    // Each possible value has name and help
    for pv in possible {
        assert!(
            pv["name"].is_string(),
            "each possible value must have a name"
        );
    }
}

#[test]
fn schema_option_has_short_flag_when_present() {
    let body = schema_body();
    let commands = body["commands"].as_array().unwrap();

    // commands --grouped uses a long flag (bool)
    let cmd = commands
        .iter()
        .find(|c| c["name"] == "commands")
        .expect("must have commands leaf");

    let grouped_arg = cmd["args"]
        .as_array()
        .unwrap()
        .iter()
        .find(|a| a["name"] == "grouped")
        .expect("commands must have grouped arg");

    assert_eq!(grouped_arg["kind"], "option");
    assert_eq!(grouped_arg["long"], "grouped");
    assert!(grouped_arg["short"].is_null());
    assert_eq!(grouped_arg["parser"]["hint"], "bool");
}

#[test]
fn schema_args_long_flag_matches_clap_spelling() {
    let body = schema_body();
    let commands = body["commands"].as_array().unwrap();

    // fundamentals income-statement has --limit with default 5
    let income = commands
        .iter()
        .find(|c| c["path"] == serde_json::json!(["fundamentals", "income-statement"]))
        .expect("must have income statement leaf");

    let limit_arg = income["args"]
        .as_array()
        .unwrap()
        .iter()
        .find(|a| a["name"] == "limit")
        .expect("income statement must have limit arg");

    assert_eq!(limit_arg["long"], "limit");
    assert_eq!(limit_arg["default"], 5);

    // sec filings has --from and --to
    let filings = commands
        .iter()
        .find(|c| c["path"] == serde_json::json!(["sec", "filings"]))
        .expect("must have sec filings leaf");

    let from_arg = filings["args"]
        .as_array()
        .unwrap()
        .iter()
        .find(|a| a["name"] == "from")
        .expect("sec filings must have from arg");

    assert_eq!(from_arg["long"], "from");
    assert!(from_arg["short"].is_null());
    assert_eq!(from_arg["required"], false);
}

#[test]
fn other_commands_still_require_api_key() {
    // dotenvy may load a valid key from .env, so set it to empty to
    // confirm non-discovery commands still require a populated key.
    Command::cargo_bin("fmp-agent")
        .unwrap()
        .env("FMP_API_KEY", "")
        .args(["market", "quote", "AAPL"])
        .assert()
        .failure()
        .code(3);
}

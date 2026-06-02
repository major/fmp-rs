use assert_cmd::Command;
use predicates::prelude::*;
use std::process::{Command as StdCommand, Stdio};

/// A broken-pipe write must not cause a panic or non-zero exit.
/// We run the binary with stdout piped, then immediately drop the read end so
/// the child's next write hits a broken pipe. The child should exit 0, not
/// panic inside human-panic.
fn assert_broken_pipe(args: &[&str]) {
    let mut child = StdCommand::new(Command::cargo_bin("fmp-agent").unwrap().get_program())
        .env_remove("FMP_API_KEY")
        .args(args)
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    // Close the read end of the pipe.
    drop(child.stdout.take());
    let status = child.wait().unwrap();
    assert!(
        status.success(),
        "process should exit cleanly on broken pipe"
    );
}

#[test]
fn broken_pipe_schema_does_not_panic() {
    assert_broken_pipe(&["schema"]);
}

#[test]
fn broken_pipe_commands_does_not_panic() {
    assert_broken_pipe(&["commands"]);
}

#[test]
fn broken_pipe_help_topic_does_not_panic() {
    assert_broken_pipe(&["help", "examples"]);
}

#[test]
fn broken_pipe_completions_does_not_panic() {
    assert_broken_pipe(&["completions", "bash"]);
}

#[test]
fn commands_lists_leaf_paths_without_api_key() {
    Command::cargo_bin("fmp-agent")
        .unwrap()
        .env_remove("FMP_API_KEY")
        .args(["commands"])
        .assert()
        .success()
        .stdout(predicate::str::contains("company profile"))
        .stdout(predicate::str::contains("market quote"))
        .stdout(predicate::str::contains("fundamentals income-statement"))
        .stdout(predicate::str::contains("help environment"))
        .stdout(predicate::str::contains("help exit-codes"))
        .stdout(predicate::str::contains("help schema"))
        .stdout(predicate::str::contains("help troubleshooting"))
        .stdout(predicate::str::contains("help examples"))
        .stdout(predicate::str::contains("news stock"));
}

#[test]
fn help_topics_work_without_api_key() {
    for (topic, expected) in [
        ("environment", "FMP_API_KEY"),
        ("exit-codes", "structured JSON line"),
        ("schema", "tool-calling"),
        ("troubleshooting", "analyst grades SYMBOL"),
        ("examples", "market batch-quote"),
    ] {
        Command::cargo_bin("fmp-agent")
            .unwrap()
            .env_remove("FMP_API_KEY")
            .args(["help", topic])
            .assert()
            .success()
            .stdout(predicate::str::contains(expected))
            .stderr(predicate::str::is_empty());
    }
}

#[test]
fn commands_output_is_sorted() {
    let output = Command::cargo_bin("fmp-agent")
        .unwrap()
        .env_remove("FMP_API_KEY")
        .args(["commands"])
        .output()
        .unwrap();

    assert!(output.status.success());

    let stdout = String::from_utf8(output.stdout).unwrap();
    let lines: Vec<&str> = stdout.lines().collect();
    assert!(!lines.is_empty(), "commands should produce output");

    let mut sorted = lines.clone();
    sorted.sort();
    assert_eq!(
        lines, sorted,
        "commands output must be sorted alphabetically"
    );
}

#[test]
fn completions_bash_produces_output() {
    Command::cargo_bin("fmp-agent")
        .unwrap()
        .env_remove("FMP_API_KEY")
        .args(["completions", "bash"])
        .assert()
        .success()
        .stdout(predicate::str::contains("_fmp-agent"))
        .stdout(predicate::str::contains("complete"));
}

#[test]
fn completions_zsh_produces_output() {
    Command::cargo_bin("fmp-agent")
        .unwrap()
        .env_remove("FMP_API_KEY")
        .args(["completions", "zsh"])
        .assert()
        .success()
        .stdout(predicate::str::contains("_fmp-agent"))
        .stdout(predicate::str::contains("compdef"));
}

#[test]
fn completions_fish_produces_output() {
    Command::cargo_bin("fmp-agent")
        .unwrap()
        .env_remove("FMP_API_KEY")
        .args(["completions", "fish"])
        .assert()
        .success()
        .stdout(predicate::str::contains("complete"))
        .stdout(predicate::str::contains("fmp-agent"));
}

#[test]
fn completions_powershell_produces_output() {
    Command::cargo_bin("fmp-agent")
        .unwrap()
        .env_remove("FMP_API_KEY")
        .args(["completions", "powershell"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Register-ArgumentCompleter"))
        .stdout(predicate::str::contains("fmp-agent"));
}

#[test]
fn invalid_shell_rejected_by_completions() {
    Command::cargo_bin("fmp-agent")
        .unwrap()
        .env_remove("FMP_API_KEY")
        .args(["completions", "nologin"])
        .assert()
        .code(2)
        .stderr(predicate::str::contains("invalid value"));
}

#[test]
fn version_flag_works_without_api_key() {
    Command::cargo_bin("fmp-agent")
        .unwrap()
        .env_remove("FMP_API_KEY")
        .arg("--version")
        .assert()
        .success()
        .stdout(predicate::str::contains("fmp-agent"))
        .stdout(predicate::str::is_empty().not());
}

#[test]
fn invalid_option_value_gives_parse_error() {
    Command::cargo_bin("fmp-agent")
        .unwrap()
        .env("FMP_API_KEY", "test-key")
        .env("FMP_BASE_URL", "http://127.0.0.1:1/")
        .args(["fundamentals", "income-statement", "AAPL", "--limit", "abc"])
        .assert()
        .code(2)
        .stderr(predicate::str::contains("abc"))
        .stderr(predicate::str::contains("limit"));
}

#[test]
fn unknown_subcommand_suggests_similar() {
    Command::cargo_bin("fmp-agent")
        .unwrap()
        .env_remove("FMP_API_KEY")
        .args(["compan"]) // close to "company"
        .env("CLAP_COLOR", "never")
        .env("NO_COLOR", "1")
        .assert()
        .code(2)
        .stderr(predicate::str::contains("similar"));
}

#[test]
fn unknown_group_subcommand_suggests_similar() {
    Command::cargo_bin("fmp-agent")
        .unwrap()
        .env_remove("FMP_API_KEY")
        .args(["company", "profil"]) // close to "profile"
        .env("CLAP_COLOR", "never")
        .env("NO_COLOR", "1")
        .assert()
        .code(2)
        .stderr(predicate::str::contains("similar"));
}

// Config precedence: --api-key flag overrides FMP_API_KEY env var.
#[test]
fn config_precedence_flag_overrides_env() {
    use httpmock::Method::GET;
    use httpmock::MockServer;
    use serde_json::json;

    let server = MockServer::start();
    let mock = server.mock(|when, then| {
        when.method(GET)
            .path("/quote")
            .query_param("symbol", "AAPL")
            .query_param("apikey", "correct-key");
        then.status(200)
            .json_body(json!([{"symbol": "AAPL", "price": 150.0}]));
    });

    // FMP_API_KEY=wrong-key should be overridden by --api-key correct-key.
    Command::cargo_bin("fmp-agent")
        .unwrap()
        .env("FMP_API_KEY", "wrong-key")
        .args([
            "--api-key",
            "correct-key",
            "--base-url",
            &format!("{}/", server.base_url()),
            "market",
            "quote",
            "AAPL",
        ])
        .assert()
        .success();

    mock.assert();
}

/// The `commands` command lists the `completions` and `schema` leaves.
#[test]
fn commands_includes_metadata_commands() {
    Command::cargo_bin("fmp-agent")
        .unwrap()
        .env_remove("FMP_API_KEY")
        .args(["commands"])
        .assert()
        .success()
        .stdout(predicate::str::contains("commands"))
        .stdout(predicate::str::contains("completions"))
        .stdout(predicate::str::contains("doctor"))
        .stdout(predicate::str::contains("schema"));
}

/// The `commands --grouped` output organizes commands by group with about text
/// and separates discovery commands from convenience aliases.
#[test]
fn commands_grouped_output_has_group_names() {
    Command::cargo_bin("fmp-agent")
        .unwrap()
        .env_remove("FMP_API_KEY")
        .args(["commands", "--grouped"])
        .assert()
        .success()
        .stdout(predicate::str::contains("market"))
        .stdout(predicate::str::contains("company"))
        .stdout(predicate::str::contains("fundamentals"))
        .stdout(predicate::str::contains("discovery"))
        .stdout(predicate::str::contains("aliases"));
}

/// The `commands --grouped` output includes about text for each subcommand.
#[test]
fn commands_grouped_output_has_about_text() {
    Command::cargo_bin("fmp-agent")
        .unwrap()
        .env_remove("FMP_API_KEY")
        .args(["commands", "--grouped"])
        .assert()
        .success()
        .stdout(predicate::str::contains(
            "Get the latest market quote for a stock ticker",
        ))
        .stdout(predicate::str::contains(
            "Get company profile and reference data for a stock ticker",
        ));
}

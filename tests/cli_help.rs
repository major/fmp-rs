use std::process::Command;

#[test]
fn bare_command_prints_help() {
    let output = Command::new(env!("CARGO_BIN_EXE_fmp-agent"))
        .env("CLAP_COLOR", "never")
        .env("FMP_BASE_URL", "")
        .env("NO_COLOR", "1")
        .output()
        .unwrap();

    let help = String::from_utf8(output.stdout).unwrap();
    let stderr = String::from_utf8(output.stderr).unwrap();

    assert!(
        output.status.success(),
        "expected bare command to succeed, got status {:?}\nstdout:\n{}\nstderr:\n{}",
        output.status.code(),
        help,
        stderr
    );
    assert!(stderr.is_empty(), "expected empty stderr, got:\n{stderr}");
    assert!(
        help.contains("Usage: fmp-agent [OPTIONS] <COMMAND>"),
        "expected help usage in stdout, got:\n{help}"
    );
    assert!(
        help.contains("Commands:"),
        "expected commands section in stdout, got:\n{help}"
    );
    assert!(!help.contains("requires a subcommand"));
}

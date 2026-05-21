use std::process::Command;

#[test]
fn bare_command_prints_help() {
    let output = Command::new(env!("CARGO_BIN_EXE_fmp-agent"))
        .output()
        .unwrap();

    assert!(output.status.success());
    assert!(output.stderr.is_empty());

    let help = String::from_utf8(output.stdout).unwrap();
    assert!(help.contains("Usage: fmp-agent [OPTIONS] <COMMAND>"));
    assert!(help.contains("Commands:"));
    assert!(!help.contains("requires a subcommand"));
}

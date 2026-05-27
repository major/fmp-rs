//! Command line parsing and output rendering.

use std::io::{self, Write};

mod args;
mod commands;
mod dispatch;
mod groups;
mod help;
mod output;
mod schema;

#[cfg(test)]
mod tests;

pub use args::Cli;

use clap::CommandFactory;

use commands::execute;
use output::render_output;

use crate::client::FmpClient;
use crate::error::{Error, Result};

/// Writes `output` to stdout followed by a newline, suppressing broken pipe
/// errors so the CLI exits cleanly when a downstream consumer closes the pipe.
fn print_stdout_line(output: &str) {
    if let Err(e) = writeln!(io::stdout(), "{output}")
        && e.kind() != io::ErrorKind::BrokenPipe
    {
        panic!("failed printing to stdout: {e}");
    }
}

/// Runs a parsed CLI invocation.
///
/// # Errors
///
/// Returns an error when configuration is missing, an API call fails, or JSON output cannot be rendered.
pub async fn run(cli: Cli) -> Result<()> {
    // Schema is metadata-only and does not require an API key.
    if let args::Command::Schema = &cli.command {
        let data = schema::schema_payload();
        let output = serde_json::to_string(&data).map_err(crate::error::Error::Json)?;
        print_stdout_line(&output);
        return Ok(());
    }

    // Commands listing is metadata-only and does not require an API key.
    if let args::Command::Commands { grouped } = &cli.command {
        let cmd = <Cli as CommandFactory>::command();
        let output = if *grouped {
            grouped_commands(&cmd)
        } else {
            leaf_commands(&cmd)
        };
        for line in output {
            print_stdout_line(&line);
        }
        return Ok(());
    }

    // Shell completions are metadata-only and do not require an API key.
    if let args::Command::Completions { shell } = &cli.command {
        let mut cmd = <Cli as CommandFactory>::command();
        let mut buf = Vec::new();
        clap_complete::generate(shell.to_owned(), &mut cmd, "fmp-agent", &mut buf);
        let output = String::from_utf8(buf).expect("completions output is valid UTF-8");
        print_stdout_line(&output);
        return Ok(());
    }

    if let Some(group_name) = bare_group_name(&cli.command) {
        print_group_help(group_name)?;
        return Ok(());
    }

    let Some(api_key) = cli.api_key.filter(|key| !key.trim().is_empty()) else {
        return Err(Error::MissingApiKey);
    };

    let client = FmpClient::with_base_url(api_key, &cli.base_url)?;
    let payload = execute(&client, &cli.command).await?;
    if let Some(output) = render_output(payload)? {
        print_stdout_line(&output);
    }

    Ok(())
}

fn bare_group_name(command: &args::Command) -> Option<&'static str> {
    match command {
        args::Command::Company { command: None } => Some("company"),
        args::Command::Market { command: None } => Some("market"),
        args::Command::Fundamentals { command: None } => Some("fundamentals"),
        args::Command::Analyst { command: None } => Some("analyst"),
        args::Command::Insider { command: None } => Some("insider"),
        args::Command::Calendar { command: None } => Some("calendar"),
        args::Command::MacroEcon { command: None } => Some("macro"),
        args::Command::Technical { command: None } => Some("technical"),
        args::Command::Sec { command: None } => Some("sec"),
        args::Command::Etf { command: None } => Some("etf"),
        args::Command::Crypto { command: None } => Some("crypto"),
        args::Command::Forex { command: None } => Some("forex"),
        args::Command::News { command: None } => Some("news"),
        _ => None,
    }
}

/// Collect leaf command paths from a clap command tree, one `group subcommand`
/// string per leaf, sorted alphabetically.
fn leaf_commands(cmd: &clap::Command) -> Vec<String> {
    let mut leaves = Vec::new();
    for sub in cmd.get_subcommands() {
        if sub.get_name() == "help" {
            continue;
        }

        let real_children: Vec<_> = sub
            .get_subcommands()
            .filter(|c| c.get_name() != "help")
            .collect();

        if real_children.is_empty() {
            leaves.push(sub.get_name().to_owned());
        } else {
            for child in leaf_commands(sub) {
                leaves.push(format!("{} {child}", sub.get_name()));
            }
        }
    }
    leaves.sort();
    leaves
}

/// Collect grouped command paths from a clap command tree, organized by group
/// with about text, suitable for human reading.
fn grouped_commands(cmd: &clap::Command) -> Vec<String> {
    let mut lines = Vec::new();

    // First pass: collect groups and their children.  The help subcommand
    // (injected by Clap) is a leaf and falls through the children.is_empty()
    // check below without a dedicated name guard.
    for sub in cmd.get_subcommands() {
        let children: Vec<_> = sub
            .get_subcommands()
            .filter(|c| c.get_name() != "help")
            .collect();

        if children.is_empty() {
            // Top-level leaf (alias or metadata) -- handled in second pass.
            continue;
        }

        // Group with children.
        lines.push(sub.get_name().to_owned());

        let max_width = children
            .iter()
            .map(|c| format!("{} {}", sub.get_name(), c.get_name()).len())
            .max()
            .unwrap_or(0);

        for child in &children {
            let full_path = format!("{} {}", sub.get_name(), child.get_name());
            let about = child.get_about().map(|a| a.to_string()).unwrap_or_default();
            lines.push(format!("  {full_path:<max_width$}  {about}"));
        }
        lines.push(String::new());
    }

    // Second pass: collect top-level standalone commands, split into discovery
    // commands (schema, commands, completions) and convenience aliases (quote,
    // historical, profile, earnings).
    let leaves: Vec<&clap::Command> = cmd
        .get_subcommands()
        .filter(|s| {
            s.get_name() != "help"
                && s.get_subcommands()
                    .filter(|c| c.get_name() != "help")
                    .count()
                    == 0
        })
        .collect();

    const DISCOVERY: &[&str] = &["schema", "commands", "completions"];
    let discovery_leaves: Vec<_> = leaves
        .iter()
        .filter(|l| DISCOVERY.contains(&l.get_name()))
        .copied()
        .collect();
    let alias_leaves: Vec<_> = leaves
        .into_iter()
        .filter(|l| !DISCOVERY.contains(&l.get_name()))
        .collect();

    // Discovery commands section.
    if !discovery_leaves.is_empty() {
        lines.push("discovery".to_owned());
        let max_width = discovery_leaves
            .iter()
            .map(|l| l.get_name().len())
            .max()
            .unwrap_or(0);
        for leaf in &discovery_leaves {
            let about = leaf.get_about().map(|a| a.to_string()).unwrap_or_default();
            lines.push(format!("  {:<max_width$}  {about}", leaf.get_name()));
        }
        lines.push(String::new());
    }

    // Alias commands section.
    if !alias_leaves.is_empty() {
        lines.push("aliases".to_owned());
        let max_width = alias_leaves
            .iter()
            .map(|l| l.get_name().len())
            .max()
            .unwrap_or(0);
        for leaf in &alias_leaves {
            let about = leaf.get_about().map(|a| a.to_string()).unwrap_or_default();
            lines.push(format!("  {:<max_width$}  {about}", leaf.get_name()));
        }
        lines.push(String::new());
    }

    lines
}

/// Prints the help text for a named group subcommand.
///
/// Called when the user invokes `fmp-agent <group>` without a subcommand.
pub(crate) fn print_group_help(group_name: &str) -> Result<()> {
    let mut command = <Cli as CommandFactory>::command();
    let group = command
        .find_subcommand_mut(group_name)
        .expect("group name must match a registered Clap subcommand");

    // Override usage to include the full binary-qualified path and [OPTIONS],
    // matching the format shown by `fmp-agent <group> --help`. Without this,
    // a bare group invocation shows only the subcommand name (e.g. `market`).
    let help = group
        .clone()
        .override_usage(format!("fmp-agent {group_name} [OPTIONS] [COMMAND]"))
        .render_help()
        .to_string();

    print_stdout_line(&help);

    Ok(())
}

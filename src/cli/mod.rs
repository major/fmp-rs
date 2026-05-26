//! Command line parsing and output rendering.

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
        println!("{output}");
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
        println!("{output}");
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

/// Prints the help text for a named group subcommand.
///
/// Called when the user invokes `fmp-agent <group>` without a subcommand.
pub(crate) fn print_group_help(group_name: &str) -> Result<()> {
    let mut command = <Cli as CommandFactory>::command();
    let Some(group) = command.find_subcommand_mut(group_name) else {
        return Err(Error::MissingArgument("group"));
    };

    group
        .print_help()
        .map_err(|_| Error::MissingArgument("group"))?;
    println!();

    Ok(())
}

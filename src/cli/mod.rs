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

    let Some(api_key) = cli.api_key.filter(|key| !key.trim().is_empty()) else {
        return Err(Error::MissingApiKey);
    };

    let client = FmpClient::with_base_url(api_key, &cli.base_url)?;
    let payload = execute(&client, &cli.command).await?;
    let output = render_output(payload)?;

    println!("{output}");

    Ok(())
}

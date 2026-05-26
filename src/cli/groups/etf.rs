//! ETF command group.

use clap::Subcommand;

use crate::cli::args::SymbolArgs;
use crate::cli::dispatch::run_by_symbol;
use crate::cli::help;
use crate::cli::output::CommandPayload;
use crate::client::FmpClient;
use crate::endpoint;
use crate::error::Result;

/// ETF subcommands.
#[derive(Debug, Subcommand)]
pub(crate) enum Cmd {
    /// Get ETF holdings for a fund symbol.
    #[command(
        about = help::ETF_HOLDINGS_ABOUT,
        long_about = help::ETF_HOLDINGS_LONG
    )]
    Holdings(SymbolArgs),
}

/// Dispatch an ETF group command.
///
/// # Errors
///
/// Returns an error when the API request fails or the response cannot be parsed.
pub(crate) async fn dispatch(client: &FmpClient, cmd: &Cmd) -> Result<CommandPayload> {
    match cmd {
        Cmd::Holdings(args) => run_by_symbol(client, endpoint::ETF_HOLDINGS, &args.symbol).await,
    }
}

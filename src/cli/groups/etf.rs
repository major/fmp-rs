//! ETF command group.

use clap::Subcommand;

use crate::cli::args::SymbolArgs;
use crate::cli::dispatch::{run_by_symbol, run_endpoint};
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

    /// List supported ETF symbols.
    #[command(about = help::ETF_LIST_ABOUT, long_about = help::ETF_LIST_LONG)]
    List,

    /// Get ETF information for a fund symbol.
    #[command(about = help::ETF_INFO_ABOUT, long_about = help::ETF_INFO_LONG)]
    Info(SymbolArgs),
}

/// Dispatch an ETF group command.
///
/// # Errors
///
/// Returns an error when the API request fails or the response cannot be parsed.
pub(crate) async fn dispatch(client: &FmpClient, cmd: &Cmd) -> Result<CommandPayload> {
    match cmd {
        Cmd::Holdings(args) => run_by_symbol(client, endpoint::ETF_HOLDINGS, &args.symbol).await,
        Cmd::List => run_endpoint(client, endpoint::ETF_LIST).await,
        Cmd::Info(args) => run_by_symbol(client, endpoint::ETF_INFO, &args.symbol).await,
    }
}

//! Insider trading command group.

use clap::Subcommand;

use crate::cli::args::{PagedArgs, SymbolArgs};
use crate::cli::dispatch::{run_by_symbol, run_paged};
use crate::cli::help;
use crate::cli::output::CommandPayload;
use crate::client::FmpClient;
use crate::endpoint;
use crate::error::Result;

/// Insider trading commands.
#[derive(Debug, Subcommand)]
pub(crate) enum Cmd {
    /// Get latest insider trading rows.
    #[command(
        about = help::INSIDER_TRADING_LATEST_ABOUT,
        long_about = help::INSIDER_TRADING_LATEST_LONG
    )]
    Latest(PagedArgs),
    /// Search insider trading filings for a stock ticker.
    #[command(
        about = help::INSIDER_TRADING_SEARCH_ABOUT,
        long_about = help::INSIDER_TRADING_SEARCH_LONG
    )]
    Search(SymbolArgs),
}

/// Dispatches an insider trading command.
///
/// # Errors
///
/// Returns an error when the API request fails or the response cannot be parsed.
pub(crate) async fn dispatch(client: &FmpClient, cmd: &Cmd) -> Result<CommandPayload> {
    match cmd {
        Cmd::Latest(args) => {
            run_paged(
                client,
                endpoint::INSIDER_TRADING_LATEST,
                args.page,
                args.limit,
            )
            .await
        }
        Cmd::Search(args) => {
            run_by_symbol(client, endpoint::INSIDER_TRADING_SEARCH, &args.symbol).await
        }
    }
}

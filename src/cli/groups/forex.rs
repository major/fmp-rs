//! Forex command group.

use clap::Subcommand;

use crate::cli::args::{SymbolArgs, SymbolDateRangeArgs};
use crate::cli::commands::{dispatch_historical_eod, dispatch_quote};
use crate::cli::help;
use crate::cli::output::CommandPayload;
use crate::client::FmpClient;
use crate::error::Result;

/// Forex subcommands.
#[derive(Debug, Subcommand)]
pub(crate) enum Cmd {
    /// Get the latest quote for a forex pair.
    #[command(about = help::FOREX_QUOTE_ABOUT, long_about = help::FOREX_QUOTE_LONG)]
    Quote(SymbolArgs),

    /// Get historical end-of-day price bars for a forex pair.
    #[command(about = help::FOREX_HISTORICAL_ABOUT, long_about = help::FOREX_HISTORICAL_LONG)]
    Historical(SymbolDateRangeArgs),
}

/// Dispatch a forex subcommand.
pub(crate) async fn dispatch(client: &FmpClient, cmd: &Cmd) -> Result<CommandPayload> {
    match cmd {
        Cmd::Quote(args) => dispatch_quote(client, args).await,
        Cmd::Historical(args) => dispatch_historical_eod(client, args).await,
    }
}

#![allow(unused)]
//! Cryptocurrency command group.

use clap::Subcommand;

use crate::cli::args::{SymbolArgs, SymbolDateRangeArgs};
use crate::cli::commands::{dispatch_historical_eod, dispatch_quote};
use crate::cli::dispatch::run_endpoint;
use crate::cli::help;
use crate::cli::output::CommandPayload;
use crate::client::FmpClient;
use crate::error::Result;

/// Cryptocurrency subcommands.
#[derive(Debug, Subcommand)]
pub(crate) enum Cmd {
    /// List supported cryptocurrency symbols.
    #[command(about = help::CRYPTO_LIST_ABOUT, long_about = help::CRYPTO_LIST_LONG)]
    List,

    /// Get the latest quote for a cryptocurrency pair.
    #[command(about = help::CRYPTO_QUOTE_ABOUT, long_about = help::CRYPTO_QUOTE_LONG)]
    Quote(SymbolArgs),

    /// Get historical end-of-day price bars for a cryptocurrency pair.
    #[command(about = help::CRYPTO_HISTORICAL_ABOUT, long_about = help::CRYPTO_HISTORICAL_LONG)]
    Historical(SymbolDateRangeArgs),
}

/// Dispatch a cryptocurrency subcommand.
pub(crate) async fn dispatch(client: &FmpClient, cmd: Cmd) -> Result<CommandPayload> {
    match cmd {
        Cmd::List => run_endpoint(client, crate::endpoint::CRYPTOCURRENCY_LIST).await,
        Cmd::Quote(args) => dispatch_quote(client, &args).await,
        Cmd::Historical(args) => dispatch_historical_eod(client, &args).await,
    }
}

//! Market data command group.

use clap::Subcommand;

use crate::cli::args::{SymbolArgs, SymbolDateRangeArgs};
use crate::cli::commands::{dispatch_historical_eod, dispatch_quote};
use crate::cli::dispatch::{run_by_symbol, run_endpoint};
use crate::cli::help;
use crate::cli::output::CommandPayload;
use crate::client::FmpClient;
use crate::error::Result;

/// Market data subcommands.
#[derive(Debug, Subcommand)]
pub(crate) enum Cmd {
    /// Get the latest market quote for a stock ticker.
    #[command(about = help::MARKET_QUOTE_ABOUT, long_about = help::MARKET_QUOTE_LONG)]
    Quote(SymbolArgs),

    /// Get historical end-of-day price bars for a stock ticker.
    #[command(about = help::MARKET_HISTORICAL_ABOUT, long_about = help::MARKET_HISTORICAL_LONG)]
    Historical(SymbolDateRangeArgs),

    /// Get historical dividend events for a stock ticker.
    #[command(about = help::MARKET_DIVIDENDS_ABOUT, long_about = help::MARKET_DIVIDENDS_LONG)]
    Dividends(SymbolArgs),

    /// Get historical stock split events for a stock ticker.
    #[command(about = help::MARKET_SPLITS_ABOUT, long_about = help::MARKET_SPLITS_LONG)]
    Splits(SymbolArgs),

    /// Get price change percentages for a stock ticker.
    #[command(about = help::MARKET_PRICE_CHANGE_ABOUT, long_about = help::MARKET_PRICE_CHANGE_LONG)]
    PriceChange(SymbolArgs),

    /// List supported stock symbols.
    #[command(about = help::MARKET_STOCK_LIST_ABOUT, long_about = help::MARKET_STOCK_LIST_LONG)]
    StockList,
}

/// Dispatch a market subcommand.
pub(crate) async fn dispatch(client: &FmpClient, cmd: &Cmd) -> Result<CommandPayload> {
    match cmd {
        Cmd::Quote(args) => dispatch_quote(client, args).await,
        Cmd::Historical(args) => dispatch_historical_eod(client, args).await,
        Cmd::Dividends(args) => {
            run_by_symbol(client, crate::endpoint::DIVIDENDS, &args.symbol).await
        }
        Cmd::Splits(args) => run_by_symbol(client, crate::endpoint::SPLITS, &args.symbol).await,
        Cmd::PriceChange(args) => {
            run_by_symbol(client, crate::endpoint::STOCK_PRICE_CHANGE, &args.symbol).await
        }
        Cmd::StockList => run_endpoint(client, crate::endpoint::STOCK_LIST).await,
    }
}

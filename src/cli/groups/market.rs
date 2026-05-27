//! Market data command group.

use clap::Subcommand;

use crate::cli::args::{SymbolArgs, SymbolDateRangeArgs, SymbolsArgs};
use crate::cli::commands::{dispatch_historical_eod, dispatch_quote};
use crate::cli::dispatch::{run_by_symbol, run_by_symbols, run_endpoint};
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

    /// Get the latest market quotes for multiple stock tickers.
    #[command(
        about = help::MARKET_BATCH_QUOTE_ABOUT,
        long_about = help::MARKET_BATCH_QUOTE_LONG
    )]
    BatchQuote(SymbolsArgs),

    /// Get price change percentages for a stock ticker.
    #[command(about = help::MARKET_PRICE_CHANGE_ABOUT, long_about = help::MARKET_PRICE_CHANGE_LONG)]
    PriceChange(SymbolArgs),

    /// List supported stock symbols.
    #[command(about = help::MARKET_STOCK_LIST_ABOUT, long_about = help::MARKET_STOCK_LIST_LONG)]
    StockList,

    /// Get a real-time market quote for a stock ticker.
    #[command(
        about = help::MARKET_REALTIME_QUOTE_ABOUT,
        long_about = help::MARKET_REALTIME_QUOTE_LONG
    )]
    RealtimeQuote(SymbolArgs),

    /// Get aftermarket quote data for a stock ticker.
    #[command(
        about = help::MARKET_AFTERMARKET_QUOTE_ABOUT,
        long_about = help::MARKET_AFTERMARKET_QUOTE_LONG
    )]
    AftermarketQuote(SymbolArgs),

    /// Get aftermarket trade data for a stock ticker.
    #[command(
        about = help::MARKET_AFTERMARKET_TRADE_ABOUT,
        long_about = help::MARKET_AFTERMARKET_TRADE_LONG
    )]
    AftermarketTrade(SymbolArgs),
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
        Cmd::BatchQuote(args) => {
            run_by_symbols(client, crate::endpoint::BATCH_QUOTE, &args.symbols).await
        }
        Cmd::PriceChange(args) => {
            run_by_symbol(client, crate::endpoint::STOCK_PRICE_CHANGE, &args.symbol).await
        }
        Cmd::StockList => run_endpoint(client, crate::endpoint::STOCK_LIST).await,
        Cmd::RealtimeQuote(args) => {
            run_by_symbol(client, crate::endpoint::REALTIME_QUOTE, &args.symbol).await
        }
        Cmd::AftermarketQuote(args) => {
            run_by_symbol(client, crate::endpoint::AFTERMARKET_QUOTE, &args.symbol).await
        }
        Cmd::AftermarketTrade(args) => {
            run_by_symbol(client, crate::endpoint::AFTERMARKET_TRADE, &args.symbol).await
        }
    }
}

#![allow(unused)]
//! News command group.

use clap::Subcommand;

use crate::cli::args::{PagedArgs, StockNewsArgs};
use crate::cli::dispatch::{run_news, run_paged};
use crate::cli::help;
use crate::cli::output::CommandPayload;
use crate::client::FmpClient;
use crate::endpoint;
use crate::error::Result;

/// News subcommands.
#[derive(Debug, Subcommand)]
pub(crate) enum Cmd {
    /// Get recent stock news for a stock ticker.
    #[command(
        about = help::NEWS_STOCK_ABOUT,
        long_about = help::NEWS_STOCK_LONG
    )]
    Stock(StockNewsArgs),

    /// Get latest general market news.
    #[command(
        about = help::NEWS_GENERAL_ABOUT,
        long_about = help::NEWS_GENERAL_LONG
    )]
    General(PagedArgs),

    /// Get latest FMP articles.
    #[command(
        about = help::NEWS_ARTICLES_ABOUT,
        long_about = help::NEWS_ARTICLES_LONG
    )]
    Articles(PagedArgs),

    /// Get latest forex news.
    #[command(
        about = help::NEWS_FOREX_ABOUT,
        long_about = help::NEWS_FOREX_LONG
    )]
    Forex(PagedArgs),

    /// Get latest crypto news.
    #[command(
        about = help::NEWS_CRYPTO_ABOUT,
        long_about = help::NEWS_CRYPTO_LONG
    )]
    Crypto(PagedArgs),
}

/// Dispatch a news subcommand.
pub(crate) async fn dispatch(client: &FmpClient, cmd: Cmd) -> Result<CommandPayload> {
    match cmd {
        Cmd::Stock(args) => run_news(client, endpoint::STOCK_NEWS, &args.symbol, args.limit).await,
        Cmd::General(args) => {
            run_paged(client, endpoint::GENERAL_NEWS, args.page, args.limit).await
        }
        Cmd::Articles(args) => {
            run_paged(client, endpoint::FMP_ARTICLES, args.page, args.limit).await
        }
        Cmd::Forex(args) => run_paged(client, endpoint::FOREX_NEWS, args.page, args.limit).await,
        Cmd::Crypto(args) => {
            run_paged(client, endpoint::CRYPTO_NEWS, args.page, args.limit).await
        }
    }
}

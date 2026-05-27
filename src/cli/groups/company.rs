//! Company data command group.

use clap::Subcommand;

use crate::cli::args::{PagedArgs, SymbolArgs, SymbolLimitArgs};
use crate::cli::dispatch::{run_by_symbol, run_by_symbol_limit, run_paged};
use crate::cli::help;
use crate::cli::output::CommandPayload;
use crate::client::FmpClient;
use crate::error::Result;

/// Company data subcommands.
#[derive(Debug, Subcommand)]
pub(crate) enum Cmd {
    /// Get company profile for a stock ticker.
    #[command(
        about = help::COMPANY_PROFILE_ABOUT,
        long_about = help::COMPANY_PROFILE_LONG
    )]
    Profile(SymbolArgs),

    /// Get key executives for a stock ticker.
    #[command(
        about = help::COMPANY_EXECUTIVES_ABOUT,
        long_about = help::COMPANY_EXECUTIVES_LONG
    )]
    Executives(SymbolArgs),

    /// Get peer company tickers for a stock ticker.
    #[command(
        about = help::COMPANY_PEERS_ABOUT,
        long_about = help::COMPANY_PEERS_LONG
    )]
    Peers(SymbolArgs),

    /// Get financial quality scores for a company.
    #[command(
        about = help::COMPANY_FINANCIAL_SCORES_ABOUT,
        long_about = help::COMPANY_FINANCIAL_SCORES_LONG
    )]
    Scores(SymbolArgs),

    /// Get share float data for a stock ticker.
    #[command(
        about = help::COMPANY_SHARE_FLOAT_ABOUT,
        long_about = help::COMPANY_SHARE_FLOAT_LONG
    )]
    Float(SymbolArgs),

    /// Get analyst rating consensus for a stock ticker.
    #[command(
        about = help::COMPANY_RATING_ABOUT,
        long_about = help::COMPANY_RATING_LONG
    )]
    Rating(SymbolArgs),

    /// Get historical company ratings for a stock ticker.
    #[command(
        about = help::COMPANY_HISTORICAL_RATING_ABOUT,
        long_about = help::COMPANY_HISTORICAL_RATING_LONG
    )]
    HistoricalRating(SymbolLimitArgs),

    /// Get comprehensive company outlook for a stock ticker.
    #[command(
        about = help::COMPANY_OUTLOOK_ABOUT,
        long_about = help::COMPANY_OUTLOOK_LONG
    )]
    Outlook(SymbolArgs),

    /// List delisted companies.
    #[command(
        about = help::COMPANY_DELISTED_ABOUT,
        long_about = help::COMPANY_DELISTED_LONG
    )]
    Delisted(PagedArgs),
}

/// Dispatch a company subcommand.
pub(crate) async fn dispatch(client: &FmpClient, cmd: &Cmd) -> Result<CommandPayload> {
    match cmd {
        Cmd::Profile(args) => run_by_symbol(client, crate::endpoint::PROFILE, &args.symbol).await,
        Cmd::Executives(args) => {
            run_by_symbol(client, crate::endpoint::KEY_EXECUTIVES, &args.symbol).await
        }
        Cmd::Peers(args) => run_by_symbol(client, crate::endpoint::STOCK_PEERS, &args.symbol).await,
        Cmd::Scores(args) => {
            run_by_symbol(client, crate::endpoint::FINANCIAL_SCORES, &args.symbol).await
        }
        Cmd::Float(args) => {
            run_by_symbol(client, crate::endpoint::SHARES_FLOAT, &args.symbol).await
        }
        Cmd::Rating(args) => {
            run_by_symbol(client, crate::endpoint::GRADES_CONSENSUS, &args.symbol).await
        }
        Cmd::HistoricalRating(args) => {
            run_by_symbol_limit(
                client,
                crate::endpoint::RATINGS_HISTORICAL,
                &args.symbol,
                args.limit,
            )
            .await
        }
        Cmd::Outlook(args) => {
            run_by_symbol(client, crate::endpoint::COMPANY_OUTLOOK, &args.symbol).await
        }
        Cmd::Delisted(args) => {
            run_paged(
                client,
                crate::endpoint::DELISTED_COMPANIES,
                args.page,
                args.limit,
            )
            .await
        }
    }
}

//! Analyst command group.

use clap::Subcommand;

use crate::cli::args::SymbolArgs;
use crate::cli::dispatch::run_by_symbol;
use crate::cli::help;
use crate::cli::output::CommandPayload;
use crate::client::FmpClient;
use crate::endpoint;
use crate::error::Result;

/// Analyst command variants.
#[derive(Debug, Subcommand)]
pub(crate) enum Cmd {
    /// Get analyst price target consensus for a stock ticker.
    #[command(
        about = help::ANALYST_PRICE_TARGET_CONSENSUS_ABOUT,
        long_about = help::ANALYST_PRICE_TARGET_CONSENSUS_LONG
    )]
    PriceTargetConsensus(SymbolArgs),

    /// Get analyst price target summary for a stock ticker.
    #[command(
        about = help::ANALYST_PRICE_TARGET_SUMMARY_ABOUT,
        long_about = help::ANALYST_PRICE_TARGET_SUMMARY_LONG
    )]
    PriceTargetSummary(SymbolArgs),

    /// Get analyst grade action history for a stock ticker.
    #[command(about = help::ANALYST_GRADES_ABOUT, long_about = help::ANALYST_GRADES_LONG)]
    Grades(SymbolArgs),
}

/// Dispatch an analyst command.
pub(crate) async fn dispatch(client: &FmpClient, cmd: &Cmd) -> Result<CommandPayload> {
    match cmd {
        Cmd::PriceTargetConsensus(args) => {
            run_by_symbol(client, endpoint::PRICE_TARGET_CONSENSUS, &args.symbol).await
        }
        Cmd::PriceTargetSummary(args) => {
            run_by_symbol(client, endpoint::PRICE_TARGET_SUMMARY, &args.symbol).await
        }
        Cmd::Grades(args) => run_by_symbol(client, endpoint::GRADES, &args.symbol).await,
    }
}

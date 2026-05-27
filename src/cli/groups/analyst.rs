//! Analyst command group.

use clap::Subcommand;

use crate::cli::args::SymbolArgs;
use crate::cli::dispatch::{run_by_symbol, unavailable_endpoint};
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

    /// Get analyst upgrades and downgrades for a stock ticker.
    #[command(
        about = help::ANALYST_UPGRADES_DOWNGRADES_ABOUT,
        long_about = help::ANALYST_UPGRADES_DOWNGRADES_LONG
    )]
    UpgradesDowngrades(SymbolArgs),

    /// Get a snapshot of current analyst ratings for a stock ticker.
    #[command(
        about = help::ANALYST_RATINGS_SNAPSHOT_ABOUT,
        long_about = help::ANALYST_RATINGS_SNAPSHOT_LONG
    )]
    RatingsSnapshot(SymbolArgs),

    /// Get earnings surprises for a stock ticker.
    #[command(
        about = help::ANALYST_EARNINGS_SURPRISES_ABOUT,
        long_about = help::ANALYST_EARNINGS_SURPRISES_LONG
    )]
    EarningsSurprises(SymbolArgs),

    /// Get analyst price targets for a stock ticker.
    #[command(
        about = help::ANALYST_PRICE_TARGET_ABOUT,
        long_about = help::ANALYST_PRICE_TARGET_LONG
    )]
    PriceTarget(SymbolArgs),
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
        Cmd::UpgradesDowngrades(_) => unavailable_endpoint(
            endpoint::UPGRADES_DOWNGRADES.path(),
            "FMP only documents upgrades and downgrades as a legacy endpoint; use `analyst grades` for the stable analyst grade actions endpoint",
        ),
        Cmd::RatingsSnapshot(args) => {
            run_by_symbol(client, endpoint::RATINGS_SNAPSHOT, &args.symbol).await
        }
        Cmd::EarningsSurprises(_) => unavailable_endpoint(
            endpoint::EARNINGS_SURPRISES.path(),
            "FMP only documents symbol earnings surprises as a legacy endpoint; no stable per-symbol API path is available for this command",
        ),
        Cmd::PriceTarget(_) => unavailable_endpoint(
            endpoint::PRICE_TARGET.path(),
            "FMP only documents historical price targets as a legacy endpoint; use `analyst price-target-consensus` or `analyst price-target-summary` for stable price target data",
        ),
    }
}

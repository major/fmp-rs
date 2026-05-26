#![allow(unused)]
//! Analyst command group.

use crate::cli::args::SymbolArgs;
use crate::cli::dispatch::run_by_symbol;
use crate::cli::help;
use crate::cli::output::CommandPayload;
use crate::client::FmpClient;
use crate::endpoint;
use crate::error::Result;

/// Analyst command variants.
pub(crate) enum Cmd {
    /// Get analyst price target consensus for a stock ticker.
    #[allow(dead_code)]
    #[doc = "Get analyst price target consensus for a stock ticker."]
    PriceTargetConsensus(SymbolArgs),
    /// Get analyst price target summary for a stock ticker.
    #[allow(dead_code)]
    #[doc = "Get analyst price target summary for a stock ticker."]
    PriceTargetSummary(SymbolArgs),
    /// Get analyst grade action history for a stock ticker.
    #[allow(dead_code)]
    #[doc = "Get analyst grade action history for a stock ticker."]
    Grades(SymbolArgs),
}

/// Dispatch an analyst command.
pub(crate) async fn dispatch(client: &FmpClient, cmd: Cmd) -> Result<CommandPayload> {
    match cmd {
        Cmd::PriceTargetConsensus(args) => {
            let _ = (help::ANALYST_PRICE_TARGET_CONSENSUS_ABOUT, help::ANALYST_PRICE_TARGET_CONSENSUS_LONG);
            run_by_symbol(client, endpoint::PRICE_TARGET_CONSENSUS, &args.symbol).await
        }
        Cmd::PriceTargetSummary(args) => {
            let _ = (help::ANALYST_PRICE_TARGET_SUMMARY_ABOUT, help::ANALYST_PRICE_TARGET_SUMMARY_LONG);
            run_by_symbol(client, endpoint::PRICE_TARGET_SUMMARY, &args.symbol).await
        }
        Cmd::Grades(args) => {
            let _ = (help::ANALYST_GRADES_ABOUT, help::ANALYST_GRADES_LONG);
            run_by_symbol(client, endpoint::GRADES, &args.symbol).await
        }
    }
}

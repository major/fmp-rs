//! Macro command group implementation.

use clap::Subcommand;

use crate::cli::args::{DateRangeArgs, NameDateRangeArgs};
use crate::cli::dispatch::{run_by_date_range, run_by_name_date_range, run_endpoint};
use crate::cli::help;
use crate::cli::output::CommandPayload;
use crate::client::FmpClient;
use crate::endpoint;
use crate::error::Result;

/// Macro command group.
#[derive(Debug, Subcommand)]
pub(crate) enum Cmd {
    /// Get treasury rate rows for an optional date range.
    #[command(about = help::TREASURY_RATES_ABOUT, long_about = help::TREASURY_RATES_LONG)]
    TreasuryRates(DateRangeArgs),

    /// Get economic indicator rows by indicator name and optional date range.
    #[command(
        about = help::ECONOMIC_INDICATORS_ABOUT,
        long_about = help::ECONOMIC_INDICATORS_LONG
    )]
    EconomicIndicators(NameDateRangeArgs),

    /// Get the current market risk premium.
    #[command(
        about = help::MACRO_RISK_PREMIUM_ABOUT,
        long_about = help::MACRO_RISK_PREMIUM_LONG
    )]
    RiskPremium,
}

/// Dispatch a macro group command.
///
/// # Errors
/// Returns an error if the underlying FMP request fails or the response cannot be parsed.
pub(crate) async fn dispatch(client: &FmpClient, cmd: &Cmd) -> Result<CommandPayload> {
    match cmd {
        Cmd::TreasuryRates(args) => {
            run_by_date_range(client, endpoint::TREASURY_RATES, &args.from, &args.to).await
        }
        Cmd::EconomicIndicators(args) => {
            run_by_name_date_range(
                client,
                endpoint::ECONOMIC_INDICATORS,
                &args.name,
                &args.from,
                &args.to,
            )
            .await
        }
        Cmd::RiskPremium => run_endpoint(client, endpoint::MARKET_RISK_PREMIUM).await,
    }
}

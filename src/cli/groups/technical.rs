#![allow(unused)]
//! Technical analysis command group.

use crate::cli::args::TechnicalSmaArgs;
use crate::cli::dispatch::run_technical_sma;
use crate::cli::help;
use crate::cli::output::CommandPayload;
use crate::client::FmpClient;
use crate::endpoint;
use crate::error::Result;

/// Technical analysis commands.
#[derive(Debug)]
pub(crate) enum Cmd {
    /// Get simple moving average rows for a stock ticker.
    #[allow(dead_code)]
    Sma(TechnicalSmaArgs),
}

/// Dispatches a technical analysis command.
///
/// # Errors
///
/// Returns an error when the API request fails or the response cannot be parsed.
pub(crate) async fn dispatch(client: &FmpClient, cmd: Cmd) -> Result<CommandPayload> {
    match cmd {
        Cmd::Sma(args) => {
            run_technical_sma(
                client,
                endpoint::TECHNICAL_SMA,
                &args.symbol,
                args.period_length,
                &args.timeframe,
            )
            .await
        }
    }
}

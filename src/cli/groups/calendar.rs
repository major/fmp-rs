#![allow(unused)]
//! Calendar command group.

use crate::cli::args::DateRangeArgs;
use crate::cli::dispatch::run_by_date_range;
use crate::cli::help;
use crate::cli::output::CommandPayload;
use crate::client::FmpClient;
use crate::endpoint;
use crate::error::Result;

/// Calendar commands.
#[derive(Debug)]
pub(crate) enum Cmd {
    /// Get earnings calendar rows for a date range.
    #[allow(dead_code)]
    Earnings(DateRangeArgs),
}

/// Dispatches a calendar command.
///
/// # Errors
///
/// Returns an error when the API request fails or the response cannot be parsed.
pub(crate) async fn dispatch(client: &FmpClient, cmd: Cmd) -> Result<CommandPayload> {
    match cmd {
        Cmd::Earnings(args) => {
            run_by_date_range(client, endpoint::EARNINGS_CALENDAR, &args.from, &args.to).await
        }
    }
}

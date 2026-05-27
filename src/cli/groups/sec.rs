//! SEC command group for SEC filing searches.

use clap::Subcommand;

use crate::cli::args::SymbolDateRangeArgs;
use crate::cli::dispatch::{run_by_symbol_date_range, validate_date};
use crate::cli::help;
use crate::cli::output::CommandPayload;
use crate::client::FmpClient;
use crate::endpoint;
use crate::error::Result;

/// Default lookback period for SEC filings when `--from` is omitted.
const SEC_FILINGS_LOOKBACK_DAYS: i64 = 90;

/// SEC filing search subcommands.
#[derive(Debug, Subcommand)]
pub(crate) enum Cmd {
    /// Get SEC filing metadata for a stock ticker.
    #[command(
        about = help::SEC_FILINGS_ABOUT,
        long_about = help::SEC_FILINGS_LONG
    )]
    Filings(SymbolDateRangeArgs),
}

/// Dispatch an SEC group command.
///
/// # Errors
///
/// Returns an error when the API request fails or the response cannot be parsed.
pub(crate) async fn dispatch(client: &FmpClient, cmd: &Cmd) -> Result<CommandPayload> {
    match cmd {
        Cmd::Filings(args) => {
            let from = args.from.clone().or_else(|| {
                let ago = jiff::Zoned::now()
                    .checked_sub(jiff::Span::new().days(SEC_FILINGS_LOOKBACK_DAYS))
                    .expect("90-day subtraction from current date cannot fail");
                Some(ago.date().to_string())
            });
            validate_date("--from", &from)?;
            validate_date("--to", &args.to)?;
            run_by_symbol_date_range(
                client,
                endpoint::SEC_FILINGS_SEARCH_SYMBOL,
                &args.symbol,
                &from,
                &args.to,
            )
            .await
        }
    }
}

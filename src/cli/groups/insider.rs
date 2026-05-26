#![allow(unused)]
//! Insider trading command group.

use crate::cli::args::PagedArgs;
use crate::cli::dispatch::run_paged;
use crate::cli::help;
use crate::cli::output::CommandPayload;
use crate::client::FmpClient;
use crate::endpoint;
use crate::error::Result;

/// Insider trading commands.
#[derive(Debug)]
pub(crate) enum Cmd {
    /// Get latest insider trading rows.
    #[allow(dead_code)]
    Latest(PagedArgs),
}

/// Dispatches an insider trading command.
///
/// # Errors
///
/// Returns an error when the API request fails or the response cannot be parsed.
pub(crate) async fn dispatch(client: &FmpClient, cmd: Cmd) -> Result<CommandPayload> {
    match cmd {
        Cmd::Latest(args) => {
            run_paged(client, endpoint::INSIDER_TRADING_LATEST, args.page, args.limit).await
        }
    }
}

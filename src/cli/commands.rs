//! Command dispatch and domain execution logic.

use crate::client::FmpClient;
use crate::endpoint::{EARNINGS_CALENDAR, PROFILE, QUOTE, SEARCH_SYMBOL};
use crate::error::Result;

use super::args::{Command, SymbolArgs, SymbolDateRangeArgs};
use super::dispatch::{run_by_date_range, run_by_symbol, run_by_symbol_date_range, run_query};
use super::groups;
use super::output::CommandPayload;

/// Dispatch a shared quote command through the symbol-only helper.
pub(crate) async fn dispatch_quote(
    client: &FmpClient,
    args: &SymbolArgs,
) -> Result<CommandPayload> {
    run_by_symbol(client, QUOTE, &args.symbol).await
}

/// Dispatch a shared historical end-of-day command through the date-range helper.
pub(crate) async fn dispatch_historical_eod(
    client: &FmpClient,
    args: &SymbolDateRangeArgs,
) -> Result<CommandPayload> {
    run_by_symbol_date_range(
        client,
        crate::endpoint::HISTORICAL_PRICE_EOD_FULL,
        &args.symbol,
        &args.from,
        &args.to,
    )
    .await
}

pub(super) async fn execute(client: &FmpClient, command: &Command) -> Result<CommandPayload> {
    match command {
        Command::Search { query } => run_query(client, SEARCH_SYMBOL, query).await,
        Command::Schema => unreachable!("Schema command is handled before execute() in run()"),
        Command::Commands { .. } => {
            unreachable!("Commands command is handled before execute() in run()")
        }
        Command::Completions { .. } => {
            unreachable!("Completions command is handled before execute() in run()")
        }
        Command::Doctor => unreachable!("Doctor command is handled before execute() in run()"),
        Command::Quote(args) => dispatch_quote(client, args).await,
        Command::Historical(args) => dispatch_historical_eod(client, args).await,
        Command::Profile(args) => run_by_symbol(client, PROFILE, &args.symbol).await,
        Command::Earnings(args) => {
            run_by_date_range(client, EARNINGS_CALENDAR, &args.from, &args.to).await
        }
        Command::Company { command } => match command {
            Some(cmd) => groups::company::dispatch(client, cmd).await,
            None => unreachable!("bare group handled by run()"),
        },
        Command::Market { command } => match command {
            Some(cmd) => groups::market::dispatch(client, cmd).await,
            None => unreachable!("bare group handled by run()"),
        },
        Command::Fundamentals { command } => match command {
            Some(cmd) => groups::fundamentals::dispatch(client, cmd).await,
            None => unreachable!("bare group handled by run()"),
        },
        Command::Analyst { command } => match command {
            Some(cmd) => groups::analyst::dispatch(client, cmd).await,
            None => unreachable!("bare group handled by run()"),
        },
        Command::Insider { command } => match command {
            Some(cmd) => groups::insider::dispatch(client, cmd).await,
            None => unreachable!("bare group handled by run()"),
        },
        Command::Calendar { command } => match command {
            Some(cmd) => groups::calendar::dispatch(client, cmd).await,
            None => unreachable!("bare group handled by run()"),
        },
        Command::MacroEcon { command } => match command {
            Some(cmd) => groups::macro_econ::dispatch(client, cmd).await,
            None => unreachable!("bare group handled by run()"),
        },
        Command::Technical { command } => match command {
            Some(cmd) => groups::technical::dispatch(client, cmd).await,
            None => unreachable!("bare group handled by run()"),
        },
        Command::Sec { command } => match command {
            Some(cmd) => groups::sec::dispatch(client, cmd).await,
            None => unreachable!("bare group handled by run()"),
        },
        Command::Etf { command } => match command {
            Some(cmd) => groups::etf::dispatch(client, cmd).await,
            None => unreachable!("bare group handled by run()"),
        },
        Command::Crypto { command } => match command {
            Some(cmd) => groups::crypto::dispatch(client, cmd).await,
            None => unreachable!("bare group handled by run()"),
        },
        Command::Forex { command } => match command {
            Some(cmd) => groups::forex::dispatch(client, cmd).await,
            None => unreachable!("bare group handled by run()"),
        },
        Command::News { command } => match command {
            Some(cmd) => groups::news::dispatch(client, cmd).await,
            None => unreachable!("bare group handled by run()"),
        },
    }
}

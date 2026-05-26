//! Command dispatch and domain execution logic.

use crate::client::FmpClient;
use crate::endpoint::{EARNINGS_CALENDAR, PROFILE, QUOTE, SEARCH_SYMBOL};
use crate::error::Result;

use super::args::{Command, SymbolArgs, SymbolDateRangeArgs};
use super::dispatch::{run_by_date_range, run_by_symbol, run_by_symbol_date_range, run_query};
use super::output::CommandPayload;
use super::{groups, print_group_help};

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
        Command::Quote(args) => dispatch_quote(client, args).await,
        Command::Historical(args) => dispatch_historical_eod(client, args).await,
        Command::Profile(args) => run_by_symbol(client, PROFILE, &args.symbol).await,
        Command::Earnings(args) => {
            run_by_date_range(client, EARNINGS_CALENDAR, &args.from, &args.to).await
        }
        Command::Company { command } => match command {
            Some(cmd) => groups::company::dispatch(client, cmd).await,
            None => print_help_silently("company"),
        },
        Command::Market { command } => match command {
            Some(cmd) => groups::market::dispatch(client, cmd).await,
            None => print_help_silently("market"),
        },
        Command::Fundamentals { command } => match command {
            Some(cmd) => groups::fundamentals::dispatch(client, cmd).await,
            None => print_help_silently("fundamentals"),
        },
        Command::Analyst { command } => match command {
            Some(cmd) => groups::analyst::dispatch(client, cmd).await,
            None => print_help_silently("analyst"),
        },
        Command::Insider { command } => match command {
            Some(cmd) => groups::insider::dispatch(client, cmd).await,
            None => print_help_silently("insider"),
        },
        Command::Calendar { command } => match command {
            Some(cmd) => groups::calendar::dispatch(client, cmd).await,
            None => print_help_silently("calendar"),
        },
        Command::MacroEcon { command } => match command {
            Some(cmd) => groups::macro_econ::dispatch(client, cmd).await,
            None => print_help_silently("macro"),
        },
        Command::Technical { command } => match command {
            Some(cmd) => groups::technical::dispatch(client, cmd).await,
            None => print_help_silently("technical"),
        },
        Command::Sec { command } => match command {
            Some(cmd) => groups::sec::dispatch(client, cmd).await,
            None => print_help_silently("sec"),
        },
        Command::Etf { command } => match command {
            Some(cmd) => groups::etf::dispatch(client, cmd).await,
            None => print_help_silently("etf"),
        },
        Command::Crypto { command } => match command {
            Some(cmd) => groups::crypto::dispatch(client, cmd).await,
            None => print_help_silently("crypto"),
        },
        Command::Forex { command } => match command {
            Some(cmd) => groups::forex::dispatch(client, cmd).await,
            None => print_help_silently("forex"),
        },
        Command::News { command } => match command {
            Some(cmd) => groups::news::dispatch(client, cmd).await,
            None => print_help_silently("news"),
        },
    }
}

fn print_help_silently(group_name: &str) -> Result<CommandPayload> {
    print_group_help(group_name)?;
    Ok(CommandPayload::silent())
}

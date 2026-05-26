#![allow(unused)]
//! Fundamentals command group.
//!
//! Financial statements, ratios, metrics, growth, estimates, and annual
//! report form commands.

use clap::Subcommand;

use crate::cli::args::{AnnualArgs, AnnualReportFormArgs, SymbolArgs};
use crate::cli::dispatch::{run_annual, run_annual_report_form, run_by_symbol};
use crate::cli::help;
use crate::cli::output::CommandPayload;
use crate::client::FmpClient;
use crate::endpoint;
use crate::error::Result;

/// Fundamentals subcommands.
#[derive(Debug, Subcommand)]
pub(crate) enum Cmd {
    /// Income statement command.
    #[command(
        about = help::FUNDAMENTALS_INCOME_STATEMENT_ABOUT,
        long_about = help::FUNDAMENTALS_INCOME_STATEMENT_LONG
    )]
    Income(AnnualArgs),

    /// As-reported income statement command.
    #[command(
        about = help::FUNDAMENTALS_INCOME_STATEMENT_AS_REPORTED_ABOUT,
        long_about = help::FUNDAMENTALS_INCOME_STATEMENT_AS_REPORTED_LONG
    )]
    IncomeAsReported(AnnualArgs),

    /// Balance sheet command.
    #[command(
        about = help::FUNDAMENTALS_BALANCE_SHEET_ABOUT,
        long_about = help::FUNDAMENTALS_BALANCE_SHEET_LONG
    )]
    BalanceSheet(AnnualArgs),

    /// Cash flow command.
    #[command(
        about = help::FUNDAMENTALS_CASH_FLOW_ABOUT,
        long_about = help::FUNDAMENTALS_CASH_FLOW_LONG
    )]
    CashFlow(AnnualArgs),

    /// Financial ratios command.
    #[command(
        about = help::FUNDAMENTALS_RATIOS_ABOUT,
        long_about = help::FUNDAMENTALS_RATIOS_LONG
    )]
    Ratios(AnnualArgs),

    /// Key metrics command.
    #[command(
        about = help::FUNDAMENTALS_METRICS_ABOUT,
        long_about = help::FUNDAMENTALS_METRICS_LONG
    )]
    Metrics(AnnualArgs),

    /// Income statement growth command.
    #[command(
        about = help::FUNDAMENTALS_INCOME_STATEMENT_GROWTH_ABOUT,
        long_about = help::FUNDAMENTALS_INCOME_STATEMENT_GROWTH_LONG
    )]
    IncomeGrowth(AnnualArgs),

    /// Balance sheet growth command.
    #[command(
        about = help::FUNDAMENTALS_BALANCE_SHEET_GROWTH_ABOUT,
        long_about = help::FUNDAMENTALS_BALANCE_SHEET_GROWTH_LONG
    )]
    BalanceSheetGrowth(AnnualArgs),

    /// Cash flow growth command.
    #[command(
        about = help::FUNDAMENTALS_CASH_FLOW_GROWTH_ABOUT,
        long_about = help::FUNDAMENTALS_CASH_FLOW_GROWTH_LONG
    )]
    CashFlowGrowth(AnnualArgs),

    /// Enterprise values command.
    #[command(
        about = help::FUNDAMENTALS_ENTERPRISE_VALUES_ABOUT,
        long_about = help::FUNDAMENTALS_ENTERPRISE_VALUES_LONG
    )]
    EnterpriseValues(AnnualArgs),

    /// Analyst estimates command.
    #[command(
        about = help::FUNDAMENTALS_ANALYST_ESTIMATES_ABOUT,
        long_about = help::FUNDAMENTALS_ANALYST_ESTIMATES_LONG
    )]
    Estimates(AnnualArgs),

    /// Financial report dates command.
    #[command(
        about = help::FUNDAMENTALS_REPORT_DATES_ABOUT,
        long_about = help::FUNDAMENTALS_REPORT_DATES_LONG
    )]
    ReportDates(SymbolArgs),

    /// Annual report form command.
    #[command(
        about = help::FUNDAMENTALS_ANNUAL_REPORT_FORM_ABOUT,
        long_about = help::FUNDAMENTALS_ANNUAL_REPORT_FORM_LONG
    )]
    AnnualReport(AnnualReportFormArgs),
}

/// Dispatches a fundamentals subcommand to the FMP API.
pub(crate) async fn dispatch(client: &FmpClient, cmd: Cmd) -> Result<CommandPayload> {
    match cmd {
        Cmd::Income(args) => {
            run_annual(client, endpoint::INCOME_STATEMENT, &args.symbol, args.limit).await
        }
        Cmd::IncomeAsReported(args) => {
            run_annual(
                client,
                endpoint::INCOME_STATEMENT_AS_REPORTED,
                &args.symbol,
                args.limit,
            )
            .await
        }
        Cmd::BalanceSheet(args) => {
            run_annual(
                client,
                endpoint::BALANCE_SHEET_STATEMENT,
                &args.symbol,
                args.limit,
            )
            .await
        }
        Cmd::CashFlow(args) => {
            run_annual(
                client,
                endpoint::CASH_FLOW_STATEMENT,
                &args.symbol,
                args.limit,
            )
            .await
        }
        Cmd::Ratios(args) => run_annual(client, endpoint::RATIOS, &args.symbol, args.limit).await,
        Cmd::Metrics(args) => {
            run_annual(client, endpoint::KEY_METRICS, &args.symbol, args.limit).await
        }
        Cmd::IncomeGrowth(args) => {
            run_annual(
                client,
                endpoint::INCOME_STATEMENT_GROWTH,
                &args.symbol,
                args.limit,
            )
            .await
        }
        Cmd::BalanceSheetGrowth(args) => {
            run_annual(
                client,
                endpoint::BALANCE_SHEET_STATEMENT_GROWTH,
                &args.symbol,
                args.limit,
            )
            .await
        }
        Cmd::CashFlowGrowth(args) => {
            run_annual(
                client,
                endpoint::CASH_FLOW_STATEMENT_GROWTH,
                &args.symbol,
                args.limit,
            )
            .await
        }
        Cmd::EnterpriseValues(args) => {
            run_annual(
                client,
                endpoint::ENTERPRISE_VALUES,
                &args.symbol,
                args.limit,
            )
            .await
        }
        Cmd::Estimates(args) => {
            run_annual(
                client,
                endpoint::ANALYST_ESTIMATES,
                &args.symbol,
                args.limit,
            )
            .await
        }
        Cmd::ReportDates(args) => {
            run_by_symbol(client, endpoint::FINANCIAL_REPORTS_DATES, &args.symbol).await
        }
        Cmd::AnnualReport(args) => {
            run_annual_report_form(
                client,
                endpoint::FINANCIAL_REPORTS_JSON,
                &args.symbol,
                args.year,
                &args.period,
            )
            .await
        }
    }
}

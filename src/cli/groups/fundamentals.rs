//! Fundamentals command group.
//!
//! Financial statements, ratios, metrics, growth, estimates, and annual
//! report form commands.

use clap::Subcommand;

use crate::cli::args::{
    AnnualArgs, AnnualReportFormArgs, StatementArgs, SymbolArgs, SymbolLimitArgs,
};
use crate::cli::dispatch::{
    run_annual, run_annual_report_form, run_by_symbol, run_by_symbol_limit, run_statement,
};
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
    #[command(name = "income-statement")]
    Income(StatementArgs),

    /// As-reported income statement command.
    #[command(
        about = help::FUNDAMENTALS_INCOME_STATEMENT_AS_REPORTED_ABOUT,
        long_about = help::FUNDAMENTALS_INCOME_STATEMENT_AS_REPORTED_LONG
    )]
    #[command(name = "income-statement-as-reported")]
    IncomeAsReported(AnnualArgs),

    /// Balance sheet command.
    #[command(
        about = help::FUNDAMENTALS_BALANCE_SHEET_ABOUT,
        long_about = help::FUNDAMENTALS_BALANCE_SHEET_LONG
    )]
    BalanceSheet(StatementArgs),

    /// Cash flow command.
    #[command(
        about = help::FUNDAMENTALS_CASH_FLOW_ABOUT,
        long_about = help::FUNDAMENTALS_CASH_FLOW_LONG
    )]
    CashFlow(StatementArgs),

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
    #[command(name = "income-statement-growth")]
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
    #[command(name = "analyst-estimates")]
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
    #[command(name = "annual-report")]
    AnnualReport(AnnualReportFormArgs),

    /// Historical earnings per share data for a stock ticker.
    #[command(
        about = help::FUNDAMENTALS_EARNINGS_ABOUT,
        long_about = help::FUNDAMENTALS_EARNINGS_LONG
    )]
    Earnings(SymbolLimitArgs),

    /// Annual statement growth command.
    #[command(
        about = help::FUNDAMENTALS_STATEMENT_GROWTH_ABOUT,
        long_about = help::FUNDAMENTALS_STATEMENT_GROWTH_LONG
    )]
    #[command(name = "statement-growth")]
    StatementGrowth(AnnualArgs),
}

/// Dispatches a fundamentals subcommand to the FMP API.
pub(crate) async fn dispatch(client: &FmpClient, cmd: &Cmd) -> Result<CommandPayload> {
    match cmd {
        Cmd::Income(args) => {
            run_statement(
                client,
                endpoint::INCOME_STATEMENT,
                &args.symbol,
                args.period.as_str(),
                args.limit,
            )
            .await
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
            run_statement(
                client,
                endpoint::BALANCE_SHEET_STATEMENT,
                &args.symbol,
                args.period.as_str(),
                args.limit,
            )
            .await
        }
        Cmd::CashFlow(args) => {
            run_statement(
                client,
                endpoint::CASH_FLOW_STATEMENT,
                &args.symbol,
                args.period.as_str(),
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
        Cmd::Earnings(args) => {
            run_by_symbol_limit(client, endpoint::EARNINGS, &args.symbol, args.limit).await
        }
        Cmd::StatementGrowth(args) => {
            run_annual(
                client,
                endpoint::FINANCIAL_STATEMENT_GROWTH,
                &args.symbol,
                args.limit,
            )
            .await
        }
    }
}

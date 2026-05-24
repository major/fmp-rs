//! Command dispatch and domain execution logic.

use crate::client::FmpClient;
use crate::endpoint::{
    ANALYST_ESTIMATES, BALANCE_SHEET_STATEMENT, BALANCE_SHEET_STATEMENT_GROWTH,
    CASH_FLOW_STATEMENT, CASH_FLOW_STATEMENT_GROWTH, CRYPTO_NEWS, CRYPTOCURRENCY_LIST, DIVIDENDS,
    EARNINGS_CALENDAR, ENTERPRISE_VALUES, FINANCIAL_REPORTS_DATES, FINANCIAL_SCORES, FMP_ARTICLES,
    FOREX_NEWS, GENERAL_NEWS, GRADES, GRADES_CONSENSUS, HISTORICAL_PRICE_EOD_FULL,
    INCOME_STATEMENT, INCOME_STATEMENT_AS_REPORTED, INCOME_STATEMENT_GROWTH, KEY_EXECUTIVES,
    KEY_METRICS, PRICE_TARGET_CONSENSUS, PRICE_TARGET_SUMMARY, PROFILE, QUOTE, RATIOS,
    SEARCH_SYMBOL, SEC_FILINGS_SEARCH_SYMBOL, SHARES_FLOAT, SPLITS, STOCK_NEWS, STOCK_PEERS,
    STOCK_PRICE_CHANGE, TECHNICAL_SMA, TREASURY_RATES,
};
use crate::error::Result;

use super::args::{Command, SymbolDateRangeArgs};
use super::dispatch::{
    run_annual, run_by_date_range, run_by_symbol, run_by_symbol_date_range, run_endpoint, run_news,
    run_paged, run_query, run_technical_sma,
};
use super::output::CommandPayload;

pub(super) async fn execute(client: &FmpClient, command: &Command) -> Result<CommandPayload> {
    match command {
        Command::Search { query } => run_query(client, SEARCH_SYMBOL, query).await,
        Command::CompanyProfile(args) => run_by_symbol(client, PROFILE, &args.symbol).await,
        Command::CompanyExecutives(args) => {
            run_by_symbol(client, KEY_EXECUTIVES, &args.symbol).await
        }
        Command::CompanyPeers(args) => run_by_symbol(client, STOCK_PEERS, &args.symbol).await,
        Command::CompanyFinancialScores(args) => {
            run_by_symbol(client, FINANCIAL_SCORES, &args.symbol).await
        }
        Command::CompanyShareFloat(args) => run_by_symbol(client, SHARES_FLOAT, &args.symbol).await,
        Command::CompanyRating(args) => run_by_symbol(client, GRADES_CONSENSUS, &args.symbol).await,
        Command::MarketQuote(args) => run_by_symbol(client, QUOTE, &args.symbol).await,
        Command::MarketHistorical(args) => {
            run_by_symbol_date_range(
                client,
                HISTORICAL_PRICE_EOD_FULL,
                &args.symbol,
                &args.from,
                &args.to,
            )
            .await
        }
        Command::MarketDividends(args) => run_by_symbol(client, DIVIDENDS, &args.symbol).await,
        Command::MarketSplits(args) => run_by_symbol(client, SPLITS, &args.symbol).await,
        Command::MarketPriceChange(args) => {
            run_by_symbol(client, STOCK_PRICE_CHANGE, &args.symbol).await
        }
        Command::FundamentalsIncomeStatement(args) => {
            run_annual(client, INCOME_STATEMENT, &args.symbol, args.limit).await
        }
        Command::FundamentalsIncomeStatementAsReported(args) => {
            run_annual(
                client,
                INCOME_STATEMENT_AS_REPORTED,
                &args.symbol,
                args.limit,
            )
            .await
        }
        Command::FundamentalsBalanceSheet(args) => {
            run_annual(client, BALANCE_SHEET_STATEMENT, &args.symbol, args.limit).await
        }
        Command::FundamentalsCashFlow(args) => {
            run_annual(client, CASH_FLOW_STATEMENT, &args.symbol, args.limit).await
        }
        Command::FundamentalsRatios(args) => {
            run_annual(client, RATIOS, &args.symbol, args.limit).await
        }
        Command::FundamentalsMetrics(args) => {
            run_annual(client, KEY_METRICS, &args.symbol, args.limit).await
        }
        Command::FundamentalsIncomeStatementGrowth(args) => {
            run_annual(client, INCOME_STATEMENT_GROWTH, &args.symbol, args.limit).await
        }
        Command::FundamentalsBalanceSheetGrowth(args) => {
            run_annual(
                client,
                BALANCE_SHEET_STATEMENT_GROWTH,
                &args.symbol,
                args.limit,
            )
            .await
        }
        Command::FundamentalsCashFlowGrowth(args) => {
            run_annual(client, CASH_FLOW_STATEMENT_GROWTH, &args.symbol, args.limit).await
        }
        Command::FundamentalsEnterpriseValues(args) => {
            run_annual(client, ENTERPRISE_VALUES, &args.symbol, args.limit).await
        }
        Command::FundamentalsAnalystEstimates(args) => {
            run_annual(client, ANALYST_ESTIMATES, &args.symbol, args.limit).await
        }
        Command::FundamentalsReportDates(args) => {
            run_by_symbol(client, FINANCIAL_REPORTS_DATES, &args.symbol).await
        }
        Command::AnalystPriceTargetConsensus(args) => {
            run_by_symbol(client, PRICE_TARGET_CONSENSUS, &args.symbol).await
        }
        Command::AnalystPriceTargetSummary(args) => {
            run_by_symbol(client, PRICE_TARGET_SUMMARY, &args.symbol).await
        }
        Command::AnalystGrades(args) => run_by_symbol(client, GRADES, &args.symbol).await,
        Command::EarningsCalendar(args) => {
            run_by_date_range(client, EARNINGS_CALENDAR, &args.from, &args.to).await
        }
        Command::TreasuryRates(args) => {
            run_by_date_range(client, TREASURY_RATES, &args.from, &args.to).await
        }
        Command::TechnicalSma(args) => {
            run_technical_sma(
                client,
                TECHNICAL_SMA,
                &args.symbol,
                args.period_length,
                &args.timeframe,
            )
            .await
        }
        Command::SecFilings(args) => run_sec_filings(client, args).await,
        Command::CryptoList => run_endpoint(client, CRYPTOCURRENCY_LIST).await,
        Command::CryptoQuote(args) => run_by_symbol(client, QUOTE, &args.symbol).await,
        Command::CryptoHistorical(args) | Command::ForexHistorical(args) => {
            run_by_symbol_date_range(
                client,
                HISTORICAL_PRICE_EOD_FULL,
                &args.symbol,
                &args.from,
                &args.to,
            )
            .await
        }
        Command::ForexQuote(args) => run_by_symbol(client, QUOTE, &args.symbol).await,
        Command::NewsStock(args) => run_news(client, STOCK_NEWS, &args.symbol, args.limit).await,
        Command::NewsGeneral(args) => run_paged(client, GENERAL_NEWS, args.page, args.limit).await,
        Command::NewsArticles(args) => run_paged(client, FMP_ARTICLES, args.page, args.limit).await,
        Command::NewsForex(args) => run_paged(client, FOREX_NEWS, args.page, args.limit).await,
        Command::NewsCrypto(args) => run_paged(client, CRYPTO_NEWS, args.page, args.limit).await,
    }
}

/// Default lookback period for SEC filings when `--from` is omitted.
const SEC_FILINGS_LOOKBACK_DAYS: i64 = 90;

/// Dispatch SEC filings with a 90-day default for the `from` date.
async fn run_sec_filings(client: &FmpClient, args: &SymbolDateRangeArgs) -> Result<CommandPayload> {
    let from = args.from.clone().or_else(|| {
        let ago = jiff::Zoned::now()
            .checked_sub(jiff::Span::new().days(SEC_FILINGS_LOOKBACK_DAYS))
            .expect("90-day subtraction from current date cannot fail");
        Some(ago.date().to_string())
    });
    run_by_symbol_date_range(
        client,
        SEC_FILINGS_SEARCH_SYMBOL,
        &args.symbol,
        &from,
        &args.to,
    )
    .await
}

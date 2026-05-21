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
use crate::error::{Error, Result};

use super::args::{
    AnalystCommand, CalendarCommand, Command, CompanyCommand, CryptoCommand, FilingsCommand,
    ForexCommand, FundamentalsCommand, MarketCommand, NewsArgs, NewsCommand, RatesCommand,
    TechnicalCommand,
};
use super::dispatch::{
    run_annual, run_by_date_range, run_by_symbol, run_by_symbol_date_range, run_endpoint, run_news,
    run_paged, run_query, run_technical_sma,
};
use super::output::CommandPayload;

pub(super) async fn execute(client: &FmpClient, command: &Command) -> Result<CommandPayload> {
    match command {
        Command::Search { query } => run_query(client, SEARCH_SYMBOL, query).await,
        Command::Company(command) => execute_company(client, command).await,
        Command::Market(command) => execute_market(client, command).await,
        Command::Fundamentals(command) => execute_fundamentals(client, command).await,
        Command::Analyst(command) => execute_analyst(client, command).await,
        Command::Calendar(command) => execute_calendar(client, command).await,
        Command::Rates(command) => execute_rates(client, command).await,
        Command::Technical(command) => execute_technical(client, command).await,
        Command::Filings(command) => execute_filings(client, command).await,
        Command::Crypto(command) => execute_crypto(client, command).await,
        Command::Forex(command) => execute_forex(client, command).await,
        Command::News(args) => execute_news(client, args).await,
        Command::Profile(args) | Command::CompanyStats(args) => {
            run_by_symbol(client, PROFILE, &args.symbol).await
        }
        Command::KeyExecutives(args) => run_by_symbol(client, KEY_EXECUTIVES, &args.symbol).await,
        Command::Quote(args) => run_by_symbol(client, QUOTE, &args.symbol).await,
        Command::StockPeers(args) => run_by_symbol(client, STOCK_PEERS, &args.symbol).await,
        Command::Dividends(args) => run_by_symbol(client, DIVIDENDS, &args.symbol).await,
        Command::Splits(args) => run_by_symbol(client, SPLITS, &args.symbol).await,
        Command::StockPriceChange(args) => {
            run_by_symbol(client, STOCK_PRICE_CHANGE, &args.symbol).await
        }
        Command::Historical(args) | Command::DailyChart(args) => {
            run_by_symbol_date_range(
                client,
                HISTORICAL_PRICE_EOD_FULL,
                &args.symbol,
                &args.from,
                &args.to,
            )
            .await
        }
        Command::SecFilings(args) => {
            run_by_symbol_date_range(
                client,
                SEC_FILINGS_SEARCH_SYMBOL,
                &args.symbol,
                &args.from,
                &args.to,
            )
            .await
        }
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
        Command::IncomeStatement(args) => {
            run_annual(client, INCOME_STATEMENT, &args.symbol, args.limit).await
        }
        Command::IncomeStatementAsReported(args) => {
            run_annual(
                client,
                INCOME_STATEMENT_AS_REPORTED,
                &args.symbol,
                args.limit,
            )
            .await
        }
        Command::BalanceSheet(args) => {
            run_annual(client, BALANCE_SHEET_STATEMENT, &args.symbol, args.limit).await
        }
        Command::CashFlow(args) => {
            run_annual(client, CASH_FLOW_STATEMENT, &args.symbol, args.limit).await
        }
        Command::Ratios(args) => run_annual(client, RATIOS, &args.symbol, args.limit).await,
        Command::Metrics(args) => run_annual(client, KEY_METRICS, &args.symbol, args.limit).await,
        Command::IncomeStatementGrowth(args) => {
            run_annual(client, INCOME_STATEMENT_GROWTH, &args.symbol, args.limit).await
        }
        Command::BalanceSheetGrowth(args) => {
            run_annual(
                client,
                BALANCE_SHEET_STATEMENT_GROWTH,
                &args.symbol,
                args.limit,
            )
            .await
        }
        Command::CashFlowGrowth(args) => {
            run_annual(client, CASH_FLOW_STATEMENT_GROWTH, &args.symbol, args.limit).await
        }
        Command::EnterpriseValues(args) => {
            run_annual(client, ENTERPRISE_VALUES, &args.symbol, args.limit).await
        }
        Command::FinancialScores(args) => {
            run_by_symbol(client, FINANCIAL_SCORES, &args.symbol).await
        }
        Command::AnalystEstimates(args) => {
            run_annual(client, ANALYST_ESTIMATES, &args.symbol, args.limit).await
        }
        Command::StockNews(args) => run_news(client, STOCK_NEWS, &args.symbol, args.limit).await,
    }
}

async fn execute_company(client: &FmpClient, command: &CompanyCommand) -> Result<CommandPayload> {
    match command {
        CompanyCommand::Profile(args) | CompanyCommand::Stats(args) => {
            run_by_symbol(client, PROFILE, &args.symbol).await
        }
        CompanyCommand::Executives(args) => {
            run_by_symbol(client, KEY_EXECUTIVES, &args.symbol).await
        }
        CompanyCommand::Peers(args) => run_by_symbol(client, STOCK_PEERS, &args.symbol).await,
        CompanyCommand::FinancialScores(args) => {
            run_by_symbol(client, FINANCIAL_SCORES, &args.symbol).await
        }
        CompanyCommand::ShareFloat(args) => run_by_symbol(client, SHARES_FLOAT, &args.symbol).await,
        CompanyCommand::Rating(args) => run_by_symbol(client, GRADES_CONSENSUS, &args.symbol).await,
    }
}

async fn execute_market(client: &FmpClient, command: &MarketCommand) -> Result<CommandPayload> {
    match command {
        MarketCommand::Quote(args) => run_by_symbol(client, QUOTE, &args.symbol).await,
        MarketCommand::Historical(args) | MarketCommand::DailyChart(args) => {
            run_by_symbol_date_range(
                client,
                HISTORICAL_PRICE_EOD_FULL,
                &args.symbol,
                &args.from,
                &args.to,
            )
            .await
        }
        MarketCommand::Dividends(args) => run_by_symbol(client, DIVIDENDS, &args.symbol).await,
        MarketCommand::Splits(args) => run_by_symbol(client, SPLITS, &args.symbol).await,
        MarketCommand::PriceChange(args) => {
            run_by_symbol(client, STOCK_PRICE_CHANGE, &args.symbol).await
        }
    }
}

async fn execute_fundamentals(
    client: &FmpClient,
    command: &FundamentalsCommand,
) -> Result<CommandPayload> {
    match command {
        FundamentalsCommand::IncomeStatement(args) => {
            run_annual(client, INCOME_STATEMENT, &args.symbol, args.limit).await
        }
        FundamentalsCommand::IncomeStatementAsReported(args) => {
            run_annual(
                client,
                INCOME_STATEMENT_AS_REPORTED,
                &args.symbol,
                args.limit,
            )
            .await
        }
        FundamentalsCommand::BalanceSheet(args) => {
            run_annual(client, BALANCE_SHEET_STATEMENT, &args.symbol, args.limit).await
        }
        FundamentalsCommand::CashFlow(args) => {
            run_annual(client, CASH_FLOW_STATEMENT, &args.symbol, args.limit).await
        }
        FundamentalsCommand::Ratios(args) => {
            run_annual(client, RATIOS, &args.symbol, args.limit).await
        }
        FundamentalsCommand::Metrics(args) => {
            run_annual(client, KEY_METRICS, &args.symbol, args.limit).await
        }
        FundamentalsCommand::IncomeStatementGrowth(args) => {
            run_annual(client, INCOME_STATEMENT_GROWTH, &args.symbol, args.limit).await
        }
        FundamentalsCommand::BalanceSheetGrowth(args) => {
            run_annual(
                client,
                BALANCE_SHEET_STATEMENT_GROWTH,
                &args.symbol,
                args.limit,
            )
            .await
        }
        FundamentalsCommand::CashFlowGrowth(args) => {
            run_annual(client, CASH_FLOW_STATEMENT_GROWTH, &args.symbol, args.limit).await
        }
        FundamentalsCommand::EnterpriseValues(args) => {
            run_annual(client, ENTERPRISE_VALUES, &args.symbol, args.limit).await
        }
        FundamentalsCommand::AnalystEstimates(args) => {
            run_annual(client, ANALYST_ESTIMATES, &args.symbol, args.limit).await
        }
        FundamentalsCommand::ReportDates(args) => {
            run_by_symbol(client, FINANCIAL_REPORTS_DATES, &args.symbol).await
        }
    }
}

async fn execute_analyst(client: &FmpClient, command: &AnalystCommand) -> Result<CommandPayload> {
    match command {
        AnalystCommand::PriceTargetConsensus(args) => {
            run_by_symbol(client, PRICE_TARGET_CONSENSUS, &args.symbol).await
        }
        AnalystCommand::PriceTargetSummary(args) => {
            run_by_symbol(client, PRICE_TARGET_SUMMARY, &args.symbol).await
        }
        AnalystCommand::Grades(args) => run_by_symbol(client, GRADES, &args.symbol).await,
    }
}

async fn execute_calendar(client: &FmpClient, command: &CalendarCommand) -> Result<CommandPayload> {
    match command {
        CalendarCommand::Earnings(args) => {
            run_by_date_range(client, EARNINGS_CALENDAR, &args.from, &args.to).await
        }
    }
}

async fn execute_rates(client: &FmpClient, command: &RatesCommand) -> Result<CommandPayload> {
    match command {
        RatesCommand::Treasury(args) => {
            run_by_date_range(client, TREASURY_RATES, &args.from, &args.to).await
        }
    }
}

async fn execute_technical(
    client: &FmpClient,
    command: &TechnicalCommand,
) -> Result<CommandPayload> {
    match command {
        TechnicalCommand::Sma(args) => {
            run_technical_sma(
                client,
                TECHNICAL_SMA,
                &args.symbol,
                args.period_length,
                &args.timeframe,
            )
            .await
        }
    }
}

async fn execute_filings(client: &FmpClient, command: &FilingsCommand) -> Result<CommandPayload> {
    match command {
        FilingsCommand::Sec(args) => {
            run_by_symbol_date_range(
                client,
                SEC_FILINGS_SEARCH_SYMBOL,
                &args.symbol,
                &args.from,
                &args.to,
            )
            .await
        }
    }
}

async fn execute_crypto(client: &FmpClient, command: &CryptoCommand) -> Result<CommandPayload> {
    match command {
        CryptoCommand::List => run_endpoint(client, CRYPTOCURRENCY_LIST).await,
        CryptoCommand::Quote(args) => run_by_symbol(client, QUOTE, &args.symbol).await,
        CryptoCommand::Historical(args) => {
            run_by_symbol_date_range(
                client,
                HISTORICAL_PRICE_EOD_FULL,
                &args.symbol,
                &args.from,
                &args.to,
            )
            .await
        }
    }
}

async fn execute_forex(client: &FmpClient, command: &ForexCommand) -> Result<CommandPayload> {
    match command {
        ForexCommand::Quote(args) => run_by_symbol(client, QUOTE, &args.symbol).await,
        ForexCommand::Historical(args) => {
            run_by_symbol_date_range(
                client,
                HISTORICAL_PRICE_EOD_FULL,
                &args.symbol,
                &args.from,
                &args.to,
            )
            .await
        }
    }
}

async fn execute_news(client: &FmpClient, args: &NewsArgs) -> Result<CommandPayload> {
    match &args.command {
        Some(NewsCommand::Stock(args)) => {
            run_news(client, STOCK_NEWS, &args.symbol, args.limit).await
        }
        Some(NewsCommand::General(args)) => {
            run_paged(client, GENERAL_NEWS, args.page, args.limit).await
        }
        Some(NewsCommand::Articles(args)) => {
            run_paged(client, FMP_ARTICLES, args.page, args.limit).await
        }
        Some(NewsCommand::Forex(args)) => {
            run_paged(client, FOREX_NEWS, args.page, args.limit).await
        }
        Some(NewsCommand::Crypto(args)) => {
            run_paged(client, CRYPTO_NEWS, args.page, args.limit).await
        }
        None => {
            let symbol = args.symbol.as_deref().ok_or(Error::MissingArgument(
                "symbol; use `news stock <SYMBOL>` or legacy `news <SYMBOL>`",
            ))?;

            run_news(client, STOCK_NEWS, symbol, args.limit).await
        }
    }
}

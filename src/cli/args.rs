//! Clap command and argument definitions.

use crate::cli::help;

use clap::{Args, Parser, Subcommand};

const DEFAULT_BASE_URL: &str = "https://financialmodelingprep.com/stable/";

/// Financial Modeling Prep CLI configuration.
#[derive(Debug, Parser)]
#[command(name = "fmp-agent", version, about = help::CLI_ABOUT, after_help = help::EXIT_CODES)]
pub struct Cli {
    /// FMP API key.
    #[arg(long, env = "FMP_API_KEY", hide_env_values = true, help = help::API_KEY)]
    pub api_key: Option<String>,

    /// FMP stable API base URL.
    #[arg(long, env = "FMP_BASE_URL", default_value = DEFAULT_BASE_URL, help = help::BASE_URL)]
    pub base_url: String,

    /// Log verbosity flags.
    #[command(flatten)]
    pub verbose: clap_verbosity_flag::Verbosity,

    /// Command to execute.
    #[command(subcommand)]
    pub command: Command,
}

/// Supported FMP endpoint commands.
#[derive(Debug, Subcommand)]
pub enum Command {
    /// Search command.
    #[command(about = help::SEARCH_ABOUT)]
    Search {
        /// Search query.
        #[arg(help = help::SEARCH_QUERY)]
        query: String,
    },

    /// Company profile command.
    #[command(about = help::COMPANY_PROFILE_ABOUT)]
    CompanyProfile(SymbolArgs),

    /// Company executives command.
    #[command(about = help::COMPANY_EXECUTIVES_ABOUT)]
    CompanyExecutives(SymbolArgs),

    /// Company peers command.
    #[command(about = help::COMPANY_PEERS_ABOUT)]
    CompanyPeers(SymbolArgs),

    /// Stock list command.
    #[command(about = help::MARKET_STOCK_LIST_ABOUT)]
    MarketStockList,

    /// ETF holdings command.
    #[command(about = help::ETF_HOLDINGS_ABOUT)]
    EtfHoldings(SymbolArgs),

    /// Company financial scores command.
    #[command(about = help::COMPANY_FINANCIAL_SCORES_ABOUT)]
    CompanyFinancialScores(SymbolArgs),

    /// Company share float command.
    #[command(about = help::COMPANY_SHARE_FLOAT_ABOUT)]
    CompanyShareFloat(SymbolArgs),

    /// Company rating command.
    #[command(about = help::COMPANY_RATING_ABOUT)]
    CompanyRating(SymbolArgs),

    /// Company historical rating command.
    #[command(about = help::COMPANY_HISTORICAL_RATING_ABOUT)]
    CompanyHistoricalRating(SymbolLimitArgs),

    /// Market quote command.
    #[command(about = help::MARKET_QUOTE_ABOUT)]
    MarketQuote(SymbolArgs),

    /// Market historical prices command.
    #[command(about = help::MARKET_HISTORICAL_ABOUT)]
    MarketHistorical(SymbolDateRangeArgs),

    /// Market dividends command.
    #[command(about = help::MARKET_DIVIDENDS_ABOUT)]
    MarketDividends(SymbolArgs),

    /// Market splits command.
    #[command(about = help::MARKET_SPLITS_ABOUT)]
    MarketSplits(SymbolArgs),

    /// Market price change command.
    #[command(about = help::MARKET_PRICE_CHANGE_ABOUT)]
    MarketPriceChange(SymbolArgs),

    /// Income statement command.
    #[command(about = help::FUNDAMENTALS_INCOME_STATEMENT_ABOUT)]
    FundamentalsIncomeStatement(AnnualArgs),

    /// As-reported income statement command.
    #[command(about = help::FUNDAMENTALS_INCOME_STATEMENT_AS_REPORTED_ABOUT)]
    FundamentalsIncomeStatementAsReported(AnnualArgs),

    /// Balance sheet command.
    #[command(about = help::FUNDAMENTALS_BALANCE_SHEET_ABOUT)]
    FundamentalsBalanceSheet(AnnualArgs),

    /// Cash flow command.
    #[command(about = help::FUNDAMENTALS_CASH_FLOW_ABOUT)]
    FundamentalsCashFlow(AnnualArgs),

    /// Ratios command.
    #[command(about = help::FUNDAMENTALS_RATIOS_ABOUT)]
    FundamentalsRatios(AnnualArgs),

    /// Metrics command.
    #[command(about = help::FUNDAMENTALS_METRICS_ABOUT)]
    FundamentalsMetrics(AnnualArgs),

    /// Income statement growth command.
    #[command(about = help::FUNDAMENTALS_INCOME_STATEMENT_GROWTH_ABOUT)]
    FundamentalsIncomeStatementGrowth(AnnualArgs),

    /// Balance sheet growth command.
    #[command(about = help::FUNDAMENTALS_BALANCE_SHEET_GROWTH_ABOUT)]
    FundamentalsBalanceSheetGrowth(AnnualArgs),

    /// Cash flow growth command.
    #[command(about = help::FUNDAMENTALS_CASH_FLOW_GROWTH_ABOUT)]
    FundamentalsCashFlowGrowth(AnnualArgs),

    /// Enterprise values command.
    #[command(about = help::FUNDAMENTALS_ENTERPRISE_VALUES_ABOUT)]
    FundamentalsEnterpriseValues(AnnualArgs),

    /// Analyst estimates command.
    #[command(about = help::FUNDAMENTALS_ANALYST_ESTIMATES_ABOUT)]
    FundamentalsAnalystEstimates(AnnualArgs),

    /// Financial report dates command.
    #[command(about = help::FUNDAMENTALS_REPORT_DATES_ABOUT)]
    FundamentalsReportDates(SymbolArgs),

    /// Annual report form command.
    #[command(about = help::FUNDAMENTALS_ANNUAL_REPORT_FORM_ABOUT)]
    FundamentalsAnnualReportForm(AnnualReportFormArgs),

    /// Price target consensus command.
    #[command(about = help::ANALYST_PRICE_TARGET_CONSENSUS_ABOUT)]
    AnalystPriceTargetConsensus(SymbolArgs),

    /// Price target summary command.
    #[command(about = help::ANALYST_PRICE_TARGET_SUMMARY_ABOUT)]
    AnalystPriceTargetSummary(SymbolArgs),

    /// Analyst grades command.
    #[command(about = help::ANALYST_GRADES_ABOUT)]
    AnalystGrades(SymbolArgs),

    /// Insider trading command.
    #[command(about = help::INSIDER_TRADING_LATEST_ABOUT, long_about = help::INSIDER_TRADING_LATEST_LONG)]
    InsiderTradingLatest(PagedArgs),

    /// Earnings calendar command.
    #[command(about = help::EARNINGS_CALENDAR_ABOUT)]
    EarningsCalendar(DateRangeArgs),

    /// Treasury rates command.
    #[command(about = help::TREASURY_RATES_ABOUT)]
    TreasuryRates(DateRangeArgs),

    /// Economic indicators command.
    #[command(about = help::ECONOMIC_INDICATORS_ABOUT)]
    EconomicIndicators(NameDateRangeArgs),

    /// Technical SMA command.
    #[command(about = help::TECHNICAL_SMA_ABOUT, long_about = help::TECHNICAL_SMA_LONG)]
    TechnicalSma(TechnicalSmaArgs),

    /// SEC filings command.
    #[command(about = help::SEC_FILINGS_ABOUT, long_about = help::SEC_FILINGS_LONG)]
    SecFilings(SymbolDateRangeArgs),

    /// Cryptocurrency list command.
    #[command(about = help::CRYPTO_LIST_ABOUT)]
    CryptoList,

    /// Cryptocurrency quote command.
    #[command(about = help::CRYPTO_QUOTE_ABOUT)]
    CryptoQuote(SymbolArgs),

    /// Cryptocurrency historical prices command.
    #[command(about = help::CRYPTO_HISTORICAL_ABOUT)]
    CryptoHistorical(SymbolDateRangeArgs),

    /// Forex quote command.
    #[command(about = help::FOREX_QUOTE_ABOUT)]
    ForexQuote(SymbolArgs),

    /// Forex historical prices command.
    #[command(about = help::FOREX_HISTORICAL_ABOUT)]
    ForexHistorical(SymbolDateRangeArgs),

    /// Stock news command.
    #[command(about = help::NEWS_STOCK_ABOUT, long_about = help::NEWS_STOCK_LONG)]
    NewsStock(StockNewsArgs),

    /// General news command.
    #[command(about = help::NEWS_GENERAL_ABOUT, long_about = help::NEWS_GENERAL_LONG)]
    NewsGeneral(PagedArgs),

    /// FMP articles command.
    #[command(about = help::NEWS_ARTICLES_ABOUT, long_about = help::NEWS_ARTICLES_LONG)]
    NewsArticles(PagedArgs),

    /// Forex news command.
    #[command(about = help::NEWS_FOREX_ABOUT, long_about = help::NEWS_FOREX_LONG)]
    NewsForex(PagedArgs),

    /// Crypto news command.
    #[command(about = help::NEWS_CRYPTO_ABOUT, long_about = help::NEWS_CRYPTO_LONG)]
    NewsCrypto(PagedArgs),
}

/// Shared command symbol argument.
#[derive(Debug, Args)]
pub struct SymbolArgs {
    /// Command symbol.
    #[arg(help = help::SYMBOL)]
    pub symbol: String,
}

/// Shared symbol plus row limit arguments.
#[derive(Debug, Args)]
pub struct SymbolLimitArgs {
    /// Stock ticker symbol.
    #[arg(help = help::STOCK_SYMBOL)]
    pub symbol: String,

    /// Maximum number of rows to return.
    #[arg(long, help = help::LIMIT_ROWS)]
    pub limit: Option<u16>,
}

/// Annual report form arguments.
#[derive(Debug, Args)]
pub struct AnnualReportFormArgs {
    /// Stock ticker symbol.
    #[arg(help = help::STOCK_SYMBOL)]
    pub symbol: String,

    /// Fiscal year.
    #[arg(long, help = help::YEAR)]
    pub year: u16,

    /// Fiscal period.
    #[arg(long, default_value = "FY", help = help::PERIOD)]
    pub period: String,
}

/// Shared symbol and date range arguments.
#[derive(Debug, Args)]
pub struct SymbolDateRangeArgs {
    /// Symbol accepted by this command.
    #[arg(help = help::SYMBOL)]
    pub symbol: String,

    /// Inclusive start date.
    #[arg(long, help = help::FROM)]
    pub from: Option<String>,

    /// Inclusive end date.
    #[arg(long, help = help::TO)]
    pub to: Option<String>,
}

/// Shared date range arguments.
#[derive(Debug, Args)]
pub struct DateRangeArgs {
    /// Inclusive start date.
    #[arg(long, help = help::FROM)]
    pub from: Option<String>,

    /// Inclusive end date.
    #[arg(long, help = help::TO)]
    pub to: Option<String>,
}

/// Shared indicator name and date range arguments.
#[derive(Debug, Args)]
pub struct NameDateRangeArgs {
    /// Economic indicator name.
    #[arg(help = help::ECONOMIC_INDICATOR_NAME)]
    pub name: String,

    /// Inclusive start date.
    #[arg(long, help = help::FROM)]
    pub from: Option<String>,

    /// Inclusive end date.
    #[arg(long, help = help::TO)]
    pub to: Option<String>,
}

/// Shared annual endpoint arguments.
#[derive(Debug, Args)]
pub struct AnnualArgs {
    /// Stock ticker symbol.
    #[arg(help = help::STOCK_SYMBOL)]
    pub symbol: String,

    /// Maximum number of annual rows to return.
    #[arg(long, default_value_t = 5, help = help::ANNUAL_LIMIT)]
    pub limit: u16,
}

/// Simple moving average arguments.
#[derive(Debug, Args)]
pub struct TechnicalSmaArgs {
    /// Stock ticker symbol.
    #[arg(help = help::STOCK_SYMBOL)]
    pub symbol: String,

    /// Number of periods in the moving average window.
    #[arg(long, default_value_t = 10, help = help::PERIOD_LENGTH)]
    pub period_length: u16,

    /// FMP candle timeframe.
    #[arg(long, default_value = "1day", help = help::TIMEFRAME)]
    pub timeframe: String,
}

/// Stock news arguments.
#[derive(Debug, Args)]
pub struct StockNewsArgs {
    /// Stock ticker symbol.
    #[arg(help = help::STOCK_SYMBOL)]
    pub symbol: String,

    /// Maximum number of news items to return.
    #[arg(long, help = help::NEWS_LIMIT)]
    pub limit: Option<u16>,
}

/// Shared paginated endpoint arguments.
#[derive(Debug, Args)]
pub struct PagedArgs {
    /// Zero-based result page.
    #[arg(long, default_value_t = 0, help = help::PAGE)]
    pub page: u16,

    /// Maximum number of items to return.
    #[arg(long, default_value_t = 10, help = help::PAGE_LIMIT)]
    pub limit: u16,
}

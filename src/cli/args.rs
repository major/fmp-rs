//! Clap command and argument definitions.

use clap::{Args, Parser, Subcommand};

const DEFAULT_BASE_URL: &str = "https://financialmodelingprep.com/stable/";

/// Financial Modeling Prep CLI optimized for predictable JSON output.
#[derive(Debug, Parser)]
#[command(
    version,
    about,
    after_help = "EXIT CODES:\n  0  Success\n  2  Usage error (bad flags or arguments)\n  3  Configuration error (missing API key or invalid base URL)\n  4  Network error (HTTP request failed)\n  5  API error (server returned an error response)\n  6  Parse error (JSON deserialization failed)"
)]
pub struct Cli {
    /// FMP API key. Prefer `FMP_API_KEY` in `.env` or the environment so shells do not record it.
    #[arg(long, env = "FMP_API_KEY", hide_env_values = true)]
    pub api_key: Option<String>,

    /// FMP stable API base URL. Override for tests or proxies.
    #[arg(long, env = "FMP_BASE_URL", default_value = DEFAULT_BASE_URL)]
    pub base_url: String,

    /// Increase log verbosity. Use -v for INFO, -vv for DEBUG, -vvv for TRACE.
    #[command(flatten)]
    pub verbose: clap_verbosity_flag::Verbosity,

    /// Command to execute.
    #[command(subcommand)]
    pub command: Command,
}

/// Supported FMP endpoint commands.
#[derive(Debug, Subcommand)]
pub enum Command {
    /// Search for a tradable symbol by ticker or company name.
    Search {
        /// Ticker or company name query.
        query: String,
    },

    /// Get company profile/reference data for a symbol.
    CompanyProfile(SymbolArgs),

    /// Get key executives for a symbol.
    CompanyExecutives(SymbolArgs),

    /// Get peer companies for a symbol.
    CompanyPeers(SymbolArgs),

    /// List supported stock symbols.
    MarketStockList,

    /// Get financial scores for a symbol.
    CompanyFinancialScores(SymbolArgs),

    /// Get share float data for a symbol.
    CompanyShareFloat(SymbolArgs),

    /// Get analyst rating consensus for a symbol.
    CompanyRating(SymbolArgs),

    /// Get historical company rating rows for a symbol.
    CompanyHistoricalRating(SymbolLimitArgs),

    /// Get the latest quote for a symbol.
    MarketQuote(SymbolArgs),

    /// Get historical end-of-day price bars for a symbol.
    MarketHistorical(SymbolDateRangeArgs),

    /// Get historical dividends for a symbol.
    MarketDividends(SymbolArgs),

    /// Get historical stock splits for a symbol.
    MarketSplits(SymbolArgs),

    /// Get price change percentages for a symbol.
    MarketPriceChange(SymbolArgs),

    /// Get annual income statements for a symbol.
    FundamentalsIncomeStatement(AnnualArgs),

    /// Get annual income statements as reported for a symbol.
    FundamentalsIncomeStatementAsReported(AnnualArgs),

    /// Get annual balance sheets for a symbol.
    FundamentalsBalanceSheet(AnnualArgs),

    /// Get annual cash flow statements for a symbol.
    FundamentalsCashFlow(AnnualArgs),

    /// Get annual financial ratios for a symbol.
    FundamentalsRatios(AnnualArgs),

    /// Get annual key metrics for a symbol.
    FundamentalsMetrics(AnnualArgs),

    /// Get annual income statement growth for a symbol.
    FundamentalsIncomeStatementGrowth(AnnualArgs),

    /// Get annual balance sheet growth for a symbol.
    FundamentalsBalanceSheetGrowth(AnnualArgs),

    /// Get annual cash flow growth for a symbol.
    FundamentalsCashFlowGrowth(AnnualArgs),

    /// Get annual enterprise values for a symbol.
    FundamentalsEnterpriseValues(AnnualArgs),

    /// Get annual analyst estimates for a symbol.
    FundamentalsAnalystEstimates(AnnualArgs),

    /// Get available financial report dates for a symbol.
    FundamentalsReportDates(SymbolArgs),

    /// Get annual report form JSON for a symbol and fiscal year.
    FundamentalsAnnualReportForm(AnnualReportFormArgs),

    /// Get price target consensus for a symbol.
    AnalystPriceTargetConsensus(SymbolArgs),

    /// Get price target summary for a symbol.
    AnalystPriceTargetSummary(SymbolArgs),

    /// Get analyst grade actions for a symbol.
    AnalystGrades(SymbolArgs),

    /// Get latest insider trading rows.
    InsiderTradingLatest(PagedArgs),

    /// Get earnings calendar rows for a date range.
    EarningsCalendar(DateRangeArgs),

    /// Get treasury rates for a date range.
    TreasuryRates(DateRangeArgs),

    /// Get economic indicator rows by indicator name and optional date range.
    EconomicIndicators(NameDateRangeArgs),

    /// Get simple moving average technical indicator rows for a symbol.
    TechnicalSma(TechnicalSmaArgs),

    /// Get SEC filings for a symbol.
    SecFilings(SymbolDateRangeArgs),

    /// List supported cryptocurrency symbols.
    CryptoList,

    /// Get the latest quote for a cryptocurrency pair.
    CryptoQuote(SymbolArgs),

    /// Get historical end-of-day price bars for a cryptocurrency pair.
    CryptoHistorical(SymbolDateRangeArgs),

    /// Get the latest quote for a forex pair.
    ForexQuote(SymbolArgs),

    /// Get historical end-of-day price bars for a forex pair.
    ForexHistorical(SymbolDateRangeArgs),

    /// Get recent stock news for a symbol.
    NewsStock(StockNewsArgs),

    /// Get latest general market news.
    NewsGeneral(PagedArgs),

    /// Get latest FMP articles.
    NewsArticles(PagedArgs),

    /// Get latest forex news.
    NewsForex(PagedArgs),

    /// Get latest crypto news.
    NewsCrypto(PagedArgs),
}

/// Shared symbol argument.
#[derive(Debug, Args)]
pub struct SymbolArgs {
    /// Ticker symbol, forex pair, or crypto pair.
    pub symbol: String,
}

/// Shared symbol plus row limit arguments.
#[derive(Debug, Args)]
pub struct SymbolLimitArgs {
    /// Ticker symbol.
    pub symbol: String,

    /// Maximum number of rows to return.
    #[arg(long)]
    pub limit: Option<u16>,
}

/// Annual report form arguments.
#[derive(Debug, Args)]
pub struct AnnualReportFormArgs {
    /// Ticker symbol.
    pub symbol: String,

    /// Fiscal year.
    #[arg(long)]
    pub year: u16,

    /// Fiscal period, usually `FY`.
    #[arg(long, default_value = "FY")]
    pub period: String,
}

/// Shared symbol and date range arguments.
#[derive(Debug, Args)]
pub struct SymbolDateRangeArgs {
    /// Ticker symbol, forex pair, or crypto pair.
    pub symbol: String,

    /// Inclusive start date in `YYYY-MM-DD` format.
    #[arg(long)]
    pub from: Option<String>,

    /// Inclusive end date in `YYYY-MM-DD` format.
    #[arg(long)]
    pub to: Option<String>,
}

/// Shared date range arguments.
#[derive(Debug, Args)]
pub struct DateRangeArgs {
    /// Inclusive start date in `YYYY-MM-DD` format.
    #[arg(long)]
    pub from: Option<String>,

    /// Inclusive end date in `YYYY-MM-DD` format.
    #[arg(long)]
    pub to: Option<String>,
}

/// Shared indicator name and date range arguments.
#[derive(Debug, Args)]
pub struct NameDateRangeArgs {
    /// Indicator name, for example `GDP`.
    pub name: String,

    /// Inclusive start date in `YYYY-MM-DD` format.
    #[arg(long)]
    pub from: Option<String>,

    /// Inclusive end date in `YYYY-MM-DD` format.
    #[arg(long)]
    pub to: Option<String>,
}

/// Shared annual endpoint arguments.
#[derive(Debug, Args)]
pub struct AnnualArgs {
    /// Ticker symbol.
    pub symbol: String,

    /// Maximum number of annual rows to return.
    #[arg(long)]
    pub limit: Option<u16>,
}

/// Simple moving average arguments.
#[derive(Debug, Args)]
pub struct TechnicalSmaArgs {
    /// Ticker symbol.
    pub symbol: String,

    /// Moving average period length.
    #[arg(long, default_value_t = 10)]
    pub period_length: u16,

    /// FMP timeframe, for example `1day`.
    #[arg(long, default_value = "1day")]
    pub timeframe: String,
}

/// Stock news arguments.
#[derive(Debug, Args)]
pub struct StockNewsArgs {
    /// Ticker symbol.
    pub symbol: String,

    /// Maximum number of news items to return.
    #[arg(long)]
    pub limit: Option<u16>,
}

/// Shared paginated endpoint arguments.
#[derive(Debug, Args)]
pub struct PagedArgs {
    /// Zero-based result page.
    #[arg(long)]
    pub page: Option<u16>,

    /// Maximum number of items to return.
    #[arg(long)]
    pub limit: Option<u16>,
}

//! Clap command and argument definitions.

use clap::{Args, Parser, Subcommand};

const DEFAULT_BASE_URL: &str = "https://financialmodelingprep.com/stable/";

/// Financial Modeling Prep CLI optimized for predictable JSON output.
#[derive(Debug, Parser)]
#[command(version, about)]
pub struct Cli {
    /// FMP API key. Prefer `FMP_API_KEY` in `.env` or the environment so shells do not record it.
    #[arg(long, env = "FMP_API_KEY", hide_env_values = true)]
    pub api_key: Option<String>,

    /// FMP stable API base URL. Override for tests or proxies.
    #[arg(long, env = "FMP_BASE_URL", default_value = DEFAULT_BASE_URL)]
    pub base_url: String,

    /// Command to execute.
    #[command(subcommand)]
    pub command: Command,
}

/// Supported FMP endpoint command groups.
#[derive(Debug, Subcommand)]
pub enum Command {
    /// Search for a tradable symbol by ticker or company name.
    Search {
        /// Ticker or company name query.
        query: String,
    },

    /// Company profile, peers, executives, and scores.
    #[command(subcommand)]
    Company(CompanyCommand),

    /// Quotes, price history, distributions, and price changes.
    #[command(subcommand)]
    Market(MarketCommand),

    /// Annual statements, ratios, metrics, growth, and estimates.
    #[command(subcommand)]
    Fundamentals(FundamentalsCommand),

    /// Analyst ratings and price target endpoints.
    #[command(subcommand)]
    Analyst(AnalystCommand),

    /// Date-based market calendars.
    #[command(subcommand)]
    Calendar(CalendarCommand),

    /// Macro rates and yield data.
    #[command(subcommand)]
    Rates(RatesCommand),

    /// Technical indicators.
    #[command(subcommand)]
    Technical(TechnicalCommand),

    /// SEC filing lookups.
    #[command(subcommand)]
    Filings(FilingsCommand),

    /// News endpoints.
    News(NewsArgs),

    /// Hidden compatibility alias for `company profile`.
    #[command(hide = true)]
    Profile(SymbolArgs),

    /// Hidden compatibility alias for `company executives`.
    #[command(hide = true)]
    KeyExecutives(SymbolArgs),

    /// Hidden compatibility alias for `market quote`.
    #[command(hide = true)]
    Quote(SymbolArgs),

    /// Hidden compatibility alias for `market historical`.
    #[command(hide = true)]
    Historical(SymbolDateRangeArgs),

    /// Hidden compatibility alias for `market daily-chart`.
    #[command(hide = true)]
    DailyChart(SymbolDateRangeArgs),

    /// Hidden compatibility alias for `company peers`.
    #[command(hide = true)]
    StockPeers(SymbolArgs),

    /// Hidden compatibility alias for `market dividends`.
    #[command(hide = true)]
    Dividends(SymbolArgs),

    /// Hidden compatibility alias for `market splits`.
    #[command(hide = true)]
    Splits(SymbolArgs),

    /// Hidden compatibility alias for `calendar earnings`.
    #[command(hide = true)]
    EarningsCalendar(DateRangeArgs),

    /// Hidden compatibility alias for `rates treasury`.
    #[command(hide = true)]
    TreasuryRates(DateRangeArgs),

    /// Hidden compatibility alias for `technical sma`.
    #[command(hide = true)]
    TechnicalSma(TechnicalSmaArgs),

    /// Hidden compatibility alias for `market price-change`.
    #[command(hide = true)]
    StockPriceChange(SymbolArgs),

    /// Hidden compatibility alias for `filings sec`.
    #[command(hide = true)]
    SecFilings(SymbolDateRangeArgs),

    /// Hidden compatibility alias for `fundamentals income-statement`.
    #[command(hide = true)]
    IncomeStatement(AnnualArgs),

    /// Hidden compatibility alias for `fundamentals income-statement-as-reported`.
    #[command(hide = true)]
    IncomeStatementAsReported(AnnualArgs),

    /// Hidden compatibility alias for `fundamentals balance-sheet`.
    #[command(hide = true)]
    BalanceSheet(AnnualArgs),

    /// Hidden compatibility alias for `fundamentals cash-flow`.
    #[command(hide = true)]
    CashFlow(AnnualArgs),

    /// Hidden compatibility alias for `fundamentals ratios`.
    #[command(hide = true)]
    Ratios(AnnualArgs),

    /// Hidden compatibility alias for `fundamentals metrics`.
    #[command(hide = true)]
    Metrics(AnnualArgs),

    /// Hidden compatibility alias for `company profile`.
    #[command(hide = true)]
    CompanyStats(SymbolArgs),

    /// Hidden compatibility alias for `fundamentals income-statement-growth`.
    #[command(hide = true)]
    IncomeStatementGrowth(AnnualArgs),

    /// Hidden compatibility alias for `fundamentals balance-sheet-growth`.
    #[command(hide = true)]
    BalanceSheetGrowth(AnnualArgs),

    /// Hidden compatibility alias for `fundamentals cash-flow-growth`.
    #[command(hide = true)]
    CashFlowGrowth(AnnualArgs),

    /// Hidden compatibility alias for `fundamentals enterprise-values`.
    #[command(hide = true)]
    EnterpriseValues(AnnualArgs),

    /// Hidden compatibility alias for `company financial-scores`.
    #[command(hide = true)]
    FinancialScores(SymbolArgs),

    /// Hidden compatibility alias for `fundamentals analyst-estimates`.
    #[command(hide = true)]
    AnalystEstimates(AnnualArgs),

    /// Hidden compatibility alias for `news stock`.
    #[command(hide = true)]
    StockNews(StockNewsArgs),
}

/// Company-focused commands.
#[derive(Debug, Subcommand)]
pub enum CompanyCommand {
    /// Get company profile/reference data for a symbol.
    Profile(SymbolArgs),

    /// Get key executives for a symbol.
    Executives(SymbolArgs),

    /// Get peer companies for a symbol.
    Peers(SymbolArgs),

    /// Get company key statistics for a symbol.
    Stats(SymbolArgs),

    /// Get financial scores for a symbol.
    FinancialScores(SymbolArgs),

    /// Get share float data for a symbol.
    ShareFloat(SymbolArgs),

    /// Get analyst rating consensus for a symbol.
    Rating(SymbolArgs),
}

/// Market data commands.
#[derive(Debug, Subcommand)]
pub enum MarketCommand {
    /// Get the latest quote for a symbol.
    Quote(SymbolArgs),

    /// Get historical end-of-day price bars for a symbol.
    Historical(SymbolDateRangeArgs),

    /// Get daily chart bars for a symbol.
    DailyChart(SymbolDateRangeArgs),

    /// Get historical dividends for a symbol.
    Dividends(SymbolArgs),

    /// Get historical stock splits for a symbol.
    Splits(SymbolArgs),

    /// Get price change percentages for a symbol.
    PriceChange(SymbolArgs),
}

/// Fundamental analysis commands.
#[derive(Debug, Subcommand)]
pub enum FundamentalsCommand {
    /// Get annual income statements for a symbol.
    IncomeStatement(AnnualArgs),

    /// Get annual income statements as reported for a symbol.
    IncomeStatementAsReported(AnnualArgs),

    /// Get annual balance sheets for a symbol.
    BalanceSheet(AnnualArgs),

    /// Get annual cash flow statements for a symbol.
    CashFlow(AnnualArgs),

    /// Get annual financial ratios for a symbol.
    Ratios(AnnualArgs),

    /// Get annual key metrics for a symbol.
    Metrics(AnnualArgs),

    /// Get annual income statement growth for a symbol.
    IncomeStatementGrowth(AnnualArgs),

    /// Get annual balance sheet growth for a symbol.
    BalanceSheetGrowth(AnnualArgs),

    /// Get annual cash flow growth for a symbol.
    CashFlowGrowth(AnnualArgs),

    /// Get annual enterprise values for a symbol.
    EnterpriseValues(AnnualArgs),

    /// Get annual analyst estimates for a symbol.
    AnalystEstimates(AnnualArgs),

    /// Get available financial report dates for a symbol.
    ReportDates(SymbolArgs),
}

/// Analyst-focused commands.
#[derive(Debug, Subcommand)]
pub enum AnalystCommand {
    /// Get price target consensus for a symbol.
    PriceTargetConsensus(SymbolArgs),

    /// Get price target summary for a symbol.
    PriceTargetSummary(SymbolArgs),

    /// Get analyst grade actions for a symbol.
    Grades(SymbolArgs),
}

/// Calendar commands.
#[derive(Debug, Subcommand)]
pub enum CalendarCommand {
    /// Get earnings calendar rows for a date range.
    Earnings(DateRangeArgs),
}

/// Rates commands.
#[derive(Debug, Subcommand)]
pub enum RatesCommand {
    /// Get treasury rates for a date range.
    Treasury(DateRangeArgs),
}

/// Technical indicator commands.
#[derive(Debug, Subcommand)]
pub enum TechnicalCommand {
    /// Get simple moving average technical indicator rows for a symbol.
    Sma(TechnicalSmaArgs),
}

/// Filing commands.
#[derive(Debug, Subcommand)]
pub enum FilingsCommand {
    /// Get SEC filings for a symbol.
    Sec(SymbolDateRangeArgs),
}

/// News commands.
#[derive(Debug, Subcommand)]
pub enum NewsCommand {
    /// Get recent stock news for a symbol.
    Stock(StockNewsArgs),

    /// Get latest general market news.
    General(PagedArgs),

    /// Get latest FMP articles.
    Articles(PagedArgs),

    /// Get latest forex news.
    Forex(PagedArgs),

    /// Get latest crypto news.
    Crypto(PagedArgs),
}

/// Shared symbol argument.
#[derive(Debug, Args)]
pub struct SymbolArgs {
    /// Ticker symbol, forex pair, or crypto pair.
    pub symbol: String,
}

/// Shared symbol and date range arguments.
#[derive(Debug, Args)]
pub struct SymbolDateRangeArgs {
    /// Ticker symbol.
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

/// News command arguments.
#[derive(Debug, Args)]
pub struct NewsArgs {
    /// Grouped news command.
    #[command(subcommand)]
    pub command: Option<NewsCommand>,

    /// Legacy ticker symbol for `news <SYMBOL>`.
    #[arg(hide = true)]
    pub symbol: Option<String>,

    /// Maximum number of news items to return with the legacy `news <SYMBOL>` form.
    #[arg(long, hide = true)]
    pub limit: Option<u16>,
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

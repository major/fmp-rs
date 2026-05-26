//! Clap command and argument definitions.

use crate::cli::{groups, help};

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
    pub(crate) command: Command,
}

/// Supported FMP endpoint commands.
#[derive(Debug, Subcommand)]
pub(crate) enum Command {
    /// Search command.
    #[command(about = help::SEARCH_ABOUT, long_about = help::SEARCH_LONG)]
    Search {
        /// Search query.
        #[arg(help = help::SEARCH_QUERY)]
        query: String,
    },

    /// Schema dump command.
    #[command(about = help::SCHEMA_ABOUT, long_about = help::SCHEMA_LONG)]
    Schema,

    /// Top-level alias for `market quote`.
    #[command(about = help::QUOTE_ALIAS_ABOUT, long_about = help::QUOTE_ALIAS_LONG)]
    Quote(SymbolArgs),

    /// Top-level alias for `market historical`.
    #[command(
        about = help::HISTORICAL_ALIAS_ABOUT,
        long_about = help::HISTORICAL_ALIAS_LONG
    )]
    Historical(SymbolDateRangeArgs),

    /// Top-level alias for `company profile`.
    #[command(about = help::PROFILE_ALIAS_ABOUT, long_about = help::PROFILE_ALIAS_LONG)]
    Profile(SymbolArgs),

    /// Top-level alias for `calendar earnings`.
    #[command(about = help::EARNINGS_ALIAS_ABOUT, long_about = help::EARNINGS_ALIAS_LONG)]
    Earnings(DateRangeArgs),

    /// Company data command group.
    #[command(about = help::COMPANY_GROUP_ABOUT, long_about = help::COMPANY_GROUP_LONG)]
    Company {
        /// Company subcommand to run.
        #[command(subcommand)]
        command: Option<groups::company::Cmd>,
    },

    /// Market data command group.
    #[command(about = help::MARKET_GROUP_ABOUT, long_about = help::MARKET_GROUP_LONG)]
    Market {
        /// Market subcommand to run.
        #[command(subcommand)]
        command: Option<groups::market::Cmd>,
    },

    /// Fundamentals command group.
    #[command(
        about = help::FUNDAMENTALS_GROUP_ABOUT,
        long_about = help::FUNDAMENTALS_GROUP_LONG
    )]
    Fundamentals {
        /// Fundamentals subcommand to run.
        #[command(subcommand)]
        command: Option<groups::fundamentals::Cmd>,
    },

    /// Analyst command group.
    #[command(about = help::ANALYST_GROUP_ABOUT, long_about = help::ANALYST_GROUP_LONG)]
    Analyst {
        /// Analyst subcommand to run.
        #[command(subcommand)]
        command: Option<groups::analyst::Cmd>,
    },

    /// Insider command group.
    #[command(about = help::INSIDER_GROUP_ABOUT, long_about = help::INSIDER_GROUP_LONG)]
    Insider {
        /// Insider subcommand to run.
        #[command(subcommand)]
        command: Option<groups::insider::Cmd>,
    },

    /// Calendar command group.
    #[command(about = help::CALENDAR_GROUP_ABOUT, long_about = help::CALENDAR_GROUP_LONG)]
    Calendar {
        /// Calendar subcommand to run.
        #[command(subcommand)]
        command: Option<groups::calendar::Cmd>,
    },

    /// Macro command group.
    #[command(
        name = "macro",
        about = help::MACRO_GROUP_ABOUT,
        long_about = help::MACRO_GROUP_LONG
    )]
    MacroEcon {
        /// Macro subcommand to run.
        #[command(subcommand)]
        command: Option<groups::macro_econ::Cmd>,
    },

    /// Technical analysis command group.
    #[command(about = help::TECHNICAL_GROUP_ABOUT, long_about = help::TECHNICAL_GROUP_LONG)]
    Technical {
        /// Technical subcommand to run.
        #[command(subcommand)]
        command: Option<groups::technical::Cmd>,
    },

    /// SEC command group.
    #[command(about = help::SEC_GROUP_ABOUT, long_about = help::SEC_GROUP_LONG)]
    Sec {
        /// SEC subcommand to run.
        #[command(subcommand)]
        command: Option<groups::sec::Cmd>,
    },

    /// ETF command group.
    #[command(about = help::ETF_GROUP_ABOUT, long_about = help::ETF_GROUP_LONG)]
    Etf {
        /// ETF subcommand to run.
        #[command(subcommand)]
        command: Option<groups::etf::Cmd>,
    },

    /// Cryptocurrency command group.
    #[command(about = help::CRYPTO_GROUP_ABOUT, long_about = help::CRYPTO_GROUP_LONG)]
    Crypto {
        /// Cryptocurrency subcommand to run.
        #[command(subcommand)]
        command: Option<groups::crypto::Cmd>,
    },

    /// Forex command group.
    #[command(about = help::FOREX_GROUP_ABOUT, long_about = help::FOREX_GROUP_LONG)]
    Forex {
        /// Forex subcommand to run.
        #[command(subcommand)]
        command: Option<groups::forex::Cmd>,
    },

    /// News command group.
    #[command(about = help::NEWS_GROUP_ABOUT, long_about = help::NEWS_GROUP_LONG)]
    News {
        /// News subcommand to run.
        #[command(subcommand)]
        command: Option<groups::news::Cmd>,
    },
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
    #[arg(long, default_value_t = 10, help = help::LIMIT_ROWS)]
    pub limit: u16,
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
    #[arg(long, default_value_t = 10, help = help::LIMIT_ROWS)]
    pub limit: u16,
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

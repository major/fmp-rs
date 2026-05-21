//! Stable API endpoint descriptors for confirmed FMP paths.
//!
//! All endpoints are flat [`Endpoint`] values. Query shape (symbol-only,
//! date-range, annual statement, etc.) is supplied by the caller using the
//! corresponding [`FmpClient`](crate::FmpClient) shape method.

/// Default period used by annual statement-style endpoints.
pub const ANNUAL_PERIOD: &str = "annual";

/// Default row limit for annual statement-style endpoints.
pub const ANNUAL_LIMIT: u16 = 5;

/// Default item limit for news endpoints.
pub const NEWS_LIMIT: u16 = 10;

/// Default page for paginated endpoints.
pub const PAGE: u16 = 0;

/// Stable API endpoint path.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Endpoint {
    path: &'static str,
}

impl Endpoint {
    /// Creates a descriptor for an FMP stable API path.
    #[must_use]
    pub const fn new(path: &'static str) -> Self {
        Self { path }
    }

    /// Returns the stable API path.
    #[must_use]
    pub const fn path(self) -> &'static str {
        self.path
    }
}

/// Symbol search endpoint.
pub const SEARCH_SYMBOL: Endpoint = Endpoint::new("search-symbol");
/// Company profile endpoint.
pub const PROFILE: Endpoint = Endpoint::new("profile");
/// Key executives endpoint.
pub const KEY_EXECUTIVES: Endpoint = Endpoint::new("key-executives");
/// Quote endpoint.
pub const QUOTE: Endpoint = Endpoint::new("quote");
/// Historical end-of-day price endpoint.
pub const HISTORICAL_PRICE_EOD_FULL: Endpoint = Endpoint::new("historical-price-eod/full");
/// Stock peers endpoint.
pub const STOCK_PEERS: Endpoint = Endpoint::new("stock-peers");
/// Historical dividends endpoint.
pub const DIVIDENDS: Endpoint = Endpoint::new("dividends");
/// Historical stock splits endpoint.
pub const SPLITS: Endpoint = Endpoint::new("splits");
/// SEC filings search by symbol endpoint.
pub const SEC_FILINGS_SEARCH_SYMBOL: Endpoint = Endpoint::new("sec-filings-search/symbol");
/// Earnings calendar endpoint.
pub const EARNINGS_CALENDAR: Endpoint = Endpoint::new("earnings-calendar");
/// Treasury rates endpoint.
pub const TREASURY_RATES: Endpoint = Endpoint::new("treasury-rates");
/// Simple moving average technical indicator endpoint.
pub const TECHNICAL_SMA: Endpoint = Endpoint::new("technical-indicators/sma");
/// Stock price change endpoint.
pub const STOCK_PRICE_CHANGE: Endpoint = Endpoint::new("stock-price-change");
/// Shares float endpoint.
pub const SHARES_FLOAT: Endpoint = Endpoint::new("shares-float");
/// Financial report dates endpoint.
pub const FINANCIAL_REPORTS_DATES: Endpoint = Endpoint::new("financial-reports-dates");
/// Annual income statement endpoint.
pub const INCOME_STATEMENT: Endpoint = Endpoint::new("income-statement");
/// Annual income statement as reported endpoint.
pub const INCOME_STATEMENT_AS_REPORTED: Endpoint = Endpoint::new("income-statement-as-reported");
/// Annual balance sheet endpoint.
pub const BALANCE_SHEET_STATEMENT: Endpoint = Endpoint::new("balance-sheet-statement");
/// Annual cash flow statement endpoint.
pub const CASH_FLOW_STATEMENT: Endpoint = Endpoint::new("cash-flow-statement");
/// Annual financial ratios endpoint.
pub const RATIOS: Endpoint = Endpoint::new("ratios");
/// Annual key metrics endpoint.
pub const KEY_METRICS: Endpoint = Endpoint::new("key-metrics");
/// Annual income statement growth endpoint.
pub const INCOME_STATEMENT_GROWTH: Endpoint = Endpoint::new("income-statement-growth");
/// Annual balance sheet statement growth endpoint.
pub const BALANCE_SHEET_STATEMENT_GROWTH: Endpoint =
    Endpoint::new("balance-sheet-statement-growth");
/// Annual cash flow statement growth endpoint.
pub const CASH_FLOW_STATEMENT_GROWTH: Endpoint = Endpoint::new("cash-flow-statement-growth");
/// Annual enterprise values endpoint.
pub const ENTERPRISE_VALUES: Endpoint = Endpoint::new("enterprise-values");
/// Financial scores endpoint.
pub const FINANCIAL_SCORES: Endpoint = Endpoint::new("financial-scores");
/// Annual analyst estimates endpoint.
pub const ANALYST_ESTIMATES: Endpoint = Endpoint::new("analyst-estimates");
/// Analyst grades endpoint.
pub const GRADES: Endpoint = Endpoint::new("grades");
/// Price target consensus endpoint.
pub const PRICE_TARGET_CONSENSUS: Endpoint = Endpoint::new("price-target-consensus");
/// Price target summary endpoint.
pub const PRICE_TARGET_SUMMARY: Endpoint = Endpoint::new("price-target-summary");
/// Analyst grades consensus endpoint.
pub const GRADES_CONSENSUS: Endpoint = Endpoint::new("grades-consensus");
/// Stock news endpoint.
pub const STOCK_NEWS: Endpoint = Endpoint::new("news/stock");
/// General news endpoint.
pub const GENERAL_NEWS: Endpoint = Endpoint::new("news/general-latest");
/// FMP articles endpoint.
pub const FMP_ARTICLES: Endpoint = Endpoint::new("fmp-articles");
/// Forex news endpoint.
pub const FOREX_NEWS: Endpoint = Endpoint::new("news/forex-latest");
/// Crypto news endpoint.
pub const CRYPTO_NEWS: Endpoint = Endpoint::new("news/crypto-latest");

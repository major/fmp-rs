//! Shared user-facing CLI help text.

pub(crate) const CLI_ABOUT: &str =
    "Financial Modeling Prep CLI optimized for predictable JSON output.";
pub(crate) const EXIT_CODES: &str = "EXIT CODES:\n  0  Success\n  2  Usage error (bad flags or arguments)\n  3  Configuration error (missing API key or invalid base URL)\n  4  Network error (HTTP request failed)\n  5  API error (server returned an error response)\n  6  Parse error (JSON deserialization failed)\n\nParse errors use Clap's native human-readable usage text on stderr for exit code 2. Runtime errors use the JSON envelope on stderr: {\"ok\":false,...} for exit codes 3-6. To distinguish programmatically, check the exit code first, then parse stderr only for exit codes 3-6.";

pub(crate) const API_KEY: &str =
    "FMP API key. Prefer FMP_API_KEY in .env or the environment so shells do not record it.";
pub(crate) const BASE_URL: &str = "FMP stable API base URL. Override for tests or proxies.";

pub(crate) const SEARCH_ABOUT: &str =
    "Search for tradable stock symbols by ticker or company name.";
pub(crate) const SEARCH_QUERY: &str = "Ticker fragment or company name to search for.";
pub(crate) const COMPANY_PROFILE_ABOUT: &str =
    "Get company profile and reference data for a stock ticker.";
pub(crate) const COMPANY_PROFILE_LONG: &str = "Get company profile and reference data including sector, industry, description, and headquarters for a stock ticker.\n\nExamples:\n  fmp-agent company-profile AAPL\n  fmp-agent company-profile MSFT";
pub(crate) const COMPANY_EXECUTIVES_ABOUT: &str = "Get key executives for a stock ticker.";
pub(crate) const COMPANY_EXECUTIVES_LONG: &str = "Get key executives for a company including name, title, and compensation data.\n\nExamples:\n  fmp-agent company-executives AAPL";
pub(crate) const COMPANY_PEERS_ABOUT: &str = "Get peer company tickers for a stock ticker.";
pub(crate) const COMPANY_PEERS_LONG: &str = "Get a list of peer company tickers that trade in the same sector and industry.\n\nExamples:\n  fmp-agent company-peers AAPL";
pub(crate) const MARKET_STOCK_LIST_ABOUT: &str = "List supported stock symbols.";
pub(crate) const ETF_HOLDINGS_ABOUT: &str =
    "Get ETF holdings for a fund symbol; Starter accounts return an API error.";
pub(crate) const COMPANY_FINANCIAL_SCORES_ABOUT: &str =
    "Get financial quality scores for a stock ticker.";
pub(crate) const COMPANY_FINANCIAL_SCORES_LONG: &str = "Get financial quality scores for a company including Piotroski score and intrinsic value rating.\n\nExamples:\n  fmp-agent company-financial-scores AAPL";
pub(crate) const COMPANY_SHARE_FLOAT_ABOUT: &str =
    "Get share float and outstanding share data for a stock ticker.";
pub(crate) const COMPANY_SHARE_FLOAT_LONG: &str = "Get share float and outstanding share counts for a stock ticker.\n\nExamples:\n  fmp-agent company-share-float AAPL";
pub(crate) const COMPANY_RATING_ABOUT: &str =
    "Get current analyst rating consensus for a stock ticker.";
pub(crate) const COMPANY_RATING_LONG: &str = "Get the current analyst rating consensus (buy/hold/sell) for a stock ticker.\n\nExamples:\n  fmp-agent company-rating AAPL";
pub(crate) const COMPANY_HISTORICAL_RATING_ABOUT: &str =
    "Get historical company rating rows for a stock ticker.";
pub(crate) const COMPANY_HISTORICAL_RATING_LONG: &str = "Get historical company rating rows for a stock ticker. Returns up to the limit of most-recent rating snapshots.\n\nExamples:\n  fmp-agent company-historical-rating AAPL\n  fmp-agent company-historical-rating AAPL --limit 20";
pub(crate) const MARKET_QUOTE_ABOUT: &str = "Get the latest market quote for a stock ticker.";
pub(crate) const MARKET_HISTORICAL_ABOUT: &str =
    "Get historical end-of-day price bars for a stock ticker.";
pub(crate) const MARKET_DIVIDENDS_ABOUT: &str =
    "Get historical dividend events for a stock ticker.";
pub(crate) const MARKET_SPLITS_ABOUT: &str =
    "Get historical stock split events for a stock ticker.";
pub(crate) const MARKET_PRICE_CHANGE_ABOUT: &str =
    "Get price change percentages for a stock ticker.";
pub(crate) const FUNDAMENTALS_INCOME_STATEMENT_ABOUT: &str =
    "Get annual income statement rows for a stock ticker.";
pub(crate) const FUNDAMENTALS_INCOME_STATEMENT_AS_REPORTED_ABOUT: &str =
    "Get annual as-reported income statement rows for a stock ticker.";
pub(crate) const FUNDAMENTALS_BALANCE_SHEET_ABOUT: &str =
    "Get annual balance sheet rows for a stock ticker.";
pub(crate) const FUNDAMENTALS_CASH_FLOW_ABOUT: &str =
    "Get annual cash flow statement rows for a stock ticker.";
pub(crate) const FUNDAMENTALS_RATIOS_ABOUT: &str =
    "Get annual financial ratio rows for a stock ticker.";
pub(crate) const FUNDAMENTALS_METRICS_ABOUT: &str =
    "Get annual key metric rows for a stock ticker.";
pub(crate) const FUNDAMENTALS_INCOME_STATEMENT_GROWTH_ABOUT: &str =
    "Get annual income statement growth rows for a stock ticker.";
pub(crate) const FUNDAMENTALS_BALANCE_SHEET_GROWTH_ABOUT: &str =
    "Get annual balance sheet growth rows for a stock ticker.";
pub(crate) const FUNDAMENTALS_CASH_FLOW_GROWTH_ABOUT: &str =
    "Get annual cash flow growth rows for a stock ticker.";
pub(crate) const FUNDAMENTALS_ENTERPRISE_VALUES_ABOUT: &str =
    "Get annual enterprise value rows for a stock ticker.";
pub(crate) const FUNDAMENTALS_ANALYST_ESTIMATES_ABOUT: &str =
    "Get annual analyst estimate rows for a stock ticker.";
pub(crate) const FUNDAMENTALS_REPORT_DATES_ABOUT: &str =
    "Get available financial report dates for a stock ticker.";
pub(crate) const FUNDAMENTALS_ANNUAL_REPORT_FORM_ABOUT: &str =
    "Get annual report form JSON for a stock ticker and fiscal year.";
pub(crate) const ANALYST_PRICE_TARGET_CONSENSUS_ABOUT: &str =
    "Get analyst price target consensus for a stock ticker.";
pub(crate) const ANALYST_PRICE_TARGET_SUMMARY_ABOUT: &str =
    "Get analyst price target summary for a stock ticker.";
pub(crate) const ANALYST_GRADES_ABOUT: &str =
    "Get analyst grade action history for a stock ticker.";
pub(crate) const INSIDER_TRADING_LATEST_ABOUT: &str = "Get latest insider trading rows.";
pub(crate) const INSIDER_TRADING_LATEST_LONG: &str =
    "Get latest insider trading rows. Uses zero-based paging.";
pub(crate) const EARNINGS_CALENDAR_ABOUT: &str =
    "Get earnings calendar rows for an optional announcement date range.";
pub(crate) const TREASURY_RATES_ABOUT: &str = "Get treasury rate rows for an optional date range.";
pub(crate) const ECONOMIC_INDICATORS_ABOUT: &str =
    "Get economic indicator rows by indicator name and optional date range.";
pub(crate) const TECHNICAL_SMA_ABOUT: &str =
    "Get simple moving average technical indicator rows for a stock ticker.";
pub(crate) const TECHNICAL_SMA_LONG: &str = "Get simple moving average technical indicator rows for a stock ticker. Defaults to a 10-period SMA on the 1day timeframe.";
pub(crate) const SEC_FILINGS_ABOUT: &str = "Get SEC filing metadata for a stock ticker.";
pub(crate) const SEC_FILINGS_LONG: &str = "Get SEC filing metadata for a stock ticker. Defaults --from to 90 days ago when omitted because FMP requires a start date.";
pub(crate) const CRYPTO_LIST_ABOUT: &str = "List supported cryptocurrency symbols.";
pub(crate) const CRYPTO_QUOTE_ABOUT: &str = "Get the latest quote for a cryptocurrency pair.";
pub(crate) const CRYPTO_HISTORICAL_ABOUT: &str =
    "Get historical end-of-day price bars for a cryptocurrency pair.";
pub(crate) const FOREX_QUOTE_ABOUT: &str = "Get the latest quote for a forex pair.";
pub(crate) const FOREX_HISTORICAL_ABOUT: &str =
    "Get historical end-of-day price bars for a forex pair.";
pub(crate) const NEWS_STOCK_ABOUT: &str = "Get recent stock news for a stock ticker.";
pub(crate) const NEWS_STOCK_LONG: &str =
    "Get recent stock news for a stock ticker.";
pub(crate) const NEWS_GENERAL_ABOUT: &str = "Get latest general market news.";
pub(crate) const NEWS_GENERAL_LONG: &str =
    "Get latest general market news. Uses zero-based paging.";
pub(crate) const NEWS_ARTICLES_ABOUT: &str = "Get latest FMP articles.";
pub(crate) const NEWS_ARTICLES_LONG: &str = "Get latest FMP articles. Uses zero-based paging.";
pub(crate) const NEWS_FOREX_ABOUT: &str = "Get latest forex news.";
pub(crate) const NEWS_FOREX_LONG: &str = "Get latest forex news. Uses zero-based paging.";
pub(crate) const NEWS_CRYPTO_ABOUT: &str = "Get latest crypto news.";
pub(crate) const NEWS_CRYPTO_LONG: &str = "Get latest crypto news. Uses zero-based paging.";

pub(crate) const SYMBOL: &str =
    "Symbol accepted by this command, such as a stock ticker, forex pair, or crypto pair.";
pub(crate) const STOCK_SYMBOL: &str = "Stock ticker symbol, for example AAPL.";
pub(crate) const LIMIT_ROWS: &str =
    "Maximum number of rows to return.";
pub(crate) const YEAR: &str = "Fiscal year, for example 2024.";
pub(crate) const PERIOD: &str = "Fiscal period, usually FY.";
pub(crate) const FROM: &str =
    "Inclusive start date in YYYY-MM-DD format. Omit for an open-ended start when supported.";
pub(crate) const TO: &str =
    "Inclusive end date in YYYY-MM-DD format. Omit for an open-ended end when supported.";
pub(crate) const ECONOMIC_INDICATOR_NAME: &str = "Economic indicator name, for example GDP.";
pub(crate) const ANNUAL_LIMIT: &str =
    "Maximum number of annual rows to return.";
pub(crate) const PERIOD_LENGTH: &str = "Number of periods in the moving average window.";
pub(crate) const TIMEFRAME: &str = "FMP candle timeframe, for example 1day.";
pub(crate) const PAGE: &str = "Zero-based result page.";
pub(crate) const PAGE_LIMIT: &str = "Maximum number of items to return.";

//! Shared user-facing CLI help text.

pub(crate) const CLI_ABOUT: &str = "Financial Modeling Prep API CLI. Commands are organized into groups, run `fmp-agent <group>` to explore, or `fmp-agent <group> <command> [args]` to execute.\n\nGetting started (set FMP_API_KEY first):\n  fmp-agent quote AAPL\n  fmp-agent profile AAPL\n  fmp-agent market historical AAPL --from 2024-01-01\n  fmp-agent commands --grouped\n\nExamples:\n  fmp-agent doctor\n  fmp-agent market quote AAPL\n  fmp-agent company profile AAPL\n\nHelp topics:\n  fmp-agent help environment   Environment variables, .env, and config precedence\n  fmp-agent help exit-codes    Exit codes and structured stderr errors\n  fmp-agent help schema        Schema and tool-calling guidance\n  fmp-agent help examples      Representative command examples\n\nShortcuts:\n  fmp-agent doctor            Check local configuration readiness as JSON\n  fmp-agent commands           List all leaf commands\n  fmp-agent completions <shell> Generate shell completions (bash/zsh/fish/powershell)\n  fmp-agent schema             Dump CLI metadata as JSON\n  fmp-agent quote AAPL         Alias for market quote\n  fmp-agent historical AAPL    Alias for market historical\n  fmp-agent profile AAPL       Alias for company profile\n  fmp-agent earnings           Alias for calendar earnings";
pub(crate) const EXIT_CODES: &str = "EXIT CODES:\n  0  Success\n  2  Usage error (bad flags, missing arguments, or invalid dates)\n  3  Configuration error (missing API key or invalid base URL)\n  4  Network error (HTTP request failed)\n  5  API error or unavailable endpoint (server returned an error response, rate limit, or command maps only to a legacy endpoint)\n  6  Parse error (JSON deserialization failed)\n  7  Empty symbol result in --strict-empty mode\n\nClap catches bad flags and invalid dates at parse time and prints human-readable usage text on stderr with exit code 2. Runtime errors use the JSON envelope on stderr: {\"ok\":false,...} for exit codes 3-7. To distinguish programmatically, check the exit code first, then parse stderr only for exit codes 3-7. HTTP 429 uses error.kind=\"rate_limited\" so agents can retry later with backoff instead of treating it like a subscription or authentication failure. Commands that FMP only documents as legacy endpoints use error.kind=\"endpoint_unavailable\" and do not make a network request.";

pub(crate) const API_KEY: &str =
    "FMP API key. Prefer FMP_API_KEY in .env or the environment so shells do not record it.";
pub(crate) const BASE_URL: &str = "FMP stable API base URL. Override for tests or proxies.";
pub(crate) const STRICT_EMPTY: &str =
    "Treat empty symbol lookup responses as an error and suggest `fmp-agent search` for discovery.";

pub(crate) const HELP_GROUP_ABOUT: &str = "Show operational help topics.";
pub(crate) const HELP_GROUP_LONG: &str = "Show git-like help topics for common operational guidance. These topics do not require FMP_API_KEY and do not make network requests.\n\nAvailable topics:\n  environment  Environment variables, .env, and config precedence\n  exit-codes   Exit codes and structured stderr errors\n  schema       Schema and tool-calling guidance\n  examples     Representative command examples\n\nExamples:\n  fmp-agent help environment\n  fmp-agent help exit-codes\n  fmp-agent help schema\n  fmp-agent help examples";

pub(crate) const HELP_ENVIRONMENT_ABOUT: &str =
    "Explain environment variables, .env loading, and config precedence.";
pub(crate) const HELP_ENVIRONMENT_LONG: &str = "Environment and configuration\n\nThe CLI loads a .env file from the working directory before parsing arguments. Prefer FMP_API_KEY in .env or the environment so API keys do not appear in shell history.\n\nConfiguration precedence:\n  1. Command-line flags\n  2. Environment variables or .env\n  3. Built-in defaults\n\nVariables and flags:\n  FMP_API_KEY / --api-key <KEY>    Required for API commands, hidden from help values\n  FMP_BASE_URL / --base-url <URL>  Defaults to https://financialmodelingprep.com/stable/\n  RUST_LOG                         Optional log filter; -v, -vv, and -vvv also set logging\n\nMetadata commands that need no API key:\n  fmp-agent commands\n  fmp-agent completions <shell>\n  fmp-agent schema\n  fmp-agent help <topic>\n\nExamples:\n  FMP_API_KEY=your-key fmp-agent quote AAPL\n  fmp-agent --base-url http://127.0.0.1:8080/ market quote AAPL\n  fmp-agent help exit-codes";

pub(crate) const HELP_EXIT_CODES_ABOUT: &str =
    "Explain exit codes and structured runtime error stderr.";
pub(crate) const HELP_EXIT_CODES_LONG: &str = "Exit codes and stderr format\n\nExit codes:\n  0  Success\n  2  Usage error from Clap, such as bad flags, missing arguments, or invalid dates\n  3  Configuration error, such as missing API key or invalid base URL\n  4  Network error while sending the HTTP request\n  5  API error from a non-2xx response, including rate limits\n  6  Parse error while decoding JSON\n  7  Empty symbol result in --strict-empty mode\n\nStderr format:\n  Clap parse errors use human-readable usage text on stderr with exit code 2. Runtime errors use one structured JSON line on stderr for exit codes 3-7:\n  {\"ok\":false,\"error\":{\"kind\":\"<kind>\",\"message\":\"...\"}}\n\nAutomation guidance:\n  Check the exit code first. Parse stderr as JSON only for exit codes 3-7. HTTP 429 uses error.kind=\"rate_limited\" with exit code 5, so retry later with backoff instead of treating it as an authentication or subscription failure.\n\nExamples:\n  fmp-agent help exit-codes\n  fmp-agent --strict-empty quote UNKNOWN\n  fmp-agent market quote AAPL 2>error.json";

pub(crate) const HELP_SCHEMA_ABOUT: &str =
    "Explain schema introspection and tool-calling integration.";
pub(crate) const HELP_SCHEMA_LONG: &str = "Schema and tool-calling\n\nfmp-agent schema emits versioned JSON metadata and does not require FMP_API_KEY or make network requests. Use it as the source of truth for automation instead of scraping human help text.\n\nUseful discovery commands:\n  fmp-agent schema              JSON metadata for command paths, arguments, defaults, and API-key requirements\n  fmp-agent commands            Sorted leaf command paths, one per line\n  fmp-agent commands --grouped  Human-readable grouped command list\n  fmp-agent completions <shell> Shell completions generated from the current binary\n\nSchema entries include path, aliases, preferred_path, api_key_required, about, long_about, and args. Argument entries include kind, required, default, value_name, help, long, short, parser, possible_values, and multi_value.\n\nExamples:\n  fmp-agent schema | jq '.commands | length'\n  fmp-agent schema | jq '.commands[] | select(.preferred_path == \"market quote\")'\n  fmp-agent commands --grouped";

pub(crate) const HELP_EXAMPLES_ABOUT: &str = "Show representative command examples.";
pub(crate) const HELP_EXAMPLES_LONG: &str = "Representative examples\n\nDiscovery, no API key required:\n  fmp-agent help environment\n  fmp-agent commands\n  fmp-agent commands --grouped\n  fmp-agent schema | jq '.commands[0]'\n  fmp-agent completions bash\n\nCommon market and company data:\n  fmp-agent quote AAPL\n  fmp-agent market batch-quote AAPL MSFT GOOGL\n  fmp-agent market historical AAPL --from 2024-01-01 --to 2024-12-31\n  fmp-agent company profile AAPL\n  fmp-agent company peers AAPL\n\nFundamentals, analyst, and calendars:\n  fmp-agent fundamentals income-statement AAPL --limit 5\n  fmp-agent fundamentals annual-report AAPL --year 2023\n  fmp-agent analyst grades AAPL\n  fmp-agent calendar earnings --from 2024-01-01 --to 2024-03-31\n\nNews, macro, and technicals:\n  fmp-agent news stock AAPL --limit 10\n  fmp-agent macro economic-indicators GDP --from 2024-01-01\n  fmp-agent technical sma AAPL --period-length 20 --timeframe 1day\n\nExamples:\n  fmp-agent help examples\n  fmp-agent help schema";

pub(crate) const COMPANY_GROUP_ABOUT: &str = "Company data command group.";
pub(crate) const COMPANY_GROUP_LONG: &str = "Company data commands.\n\nExamples:\n  fmp-agent company\n  fmp-agent company profile AAPL\n  fmp-agent company peers AAPL\n  fmp-agent company delisted";
pub(crate) const MARKET_GROUP_ABOUT: &str = "Market data command group.";
pub(crate) const MARKET_GROUP_LONG: &str = "Market data commands.\n\nExamples:\n  fmp-agent market\n  fmp-agent market quote AAPL\n  fmp-agent market realtime-quote AAPL";
pub(crate) const FUNDAMENTALS_GROUP_ABOUT: &str = "Fundamentals command group.";
pub(crate) const FUNDAMENTALS_GROUP_LONG: &str = "Fundamentals commands.\n\nExamples:\n  fmp-agent fundamentals\n  fmp-agent fundamentals income-statement AAPL\n  fmp-agent fundamentals earnings AAPL";
pub(crate) const ANALYST_GROUP_ABOUT: &str = "Analyst command group.";
pub(crate) const ANALYST_GROUP_LONG: &str = "Analyst commands.\n\nExamples:\n  fmp-agent analyst\n  fmp-agent analyst grades AAPL\n  fmp-agent analyst price-target-summary AAPL";
pub(crate) const INSIDER_GROUP_ABOUT: &str = "Insider command group.";
pub(crate) const INSIDER_GROUP_LONG: &str = "Insider commands.\n\nExamples:\n  fmp-agent insider\n  fmp-agent insider latest --page 1 --limit 20\n  fmp-agent insider search AAPL";
pub(crate) const CALENDAR_GROUP_ABOUT: &str = "Calendar command group.";
pub(crate) const CALENDAR_GROUP_LONG: &str = "Calendar commands.\n\nExamples:\n  fmp-agent calendar\n  fmp-agent calendar earnings --from 2024-01-01 --to 2024-03-31";
pub(crate) const MACRO_GROUP_ABOUT: &str = "Macro command group.";
pub(crate) const MACRO_GROUP_LONG: &str =
    "Macro commands.\n\nExamples:\n  fmp-agent macro\n  fmp-agent macro economic-indicators GDP";
pub(crate) const TECHNICAL_GROUP_ABOUT: &str = "Technical analysis command group.";
pub(crate) const TECHNICAL_GROUP_LONG: &str = "Technical analysis commands.\n\nExamples:\n  fmp-agent technical\n  fmp-agent technical sma AAPL";
pub(crate) const SEC_GROUP_ABOUT: &str = "SEC command group.";
pub(crate) const SEC_GROUP_LONG: &str =
    "SEC commands.\n\nExamples:\n  fmp-agent sec\n  fmp-agent sec filings AAPL";
pub(crate) const ETF_GROUP_ABOUT: &str = "ETF command group.";
pub(crate) const ETF_GROUP_LONG: &str = "ETF commands.\n\nExamples:\n  fmp-agent etf\n  fmp-agent etf holdings SPY\n  fmp-agent etf list";
pub(crate) const CRYPTO_GROUP_ABOUT: &str = "Cryptocurrency command group.";
pub(crate) const CRYPTO_GROUP_LONG: &str =
    "Cryptocurrency commands.\n\nExamples:\n  fmp-agent crypto\n  fmp-agent crypto quote BTCUSD";
pub(crate) const FOREX_GROUP_ABOUT: &str = "Forex command group.";
pub(crate) const FOREX_GROUP_LONG: &str =
    "Forex commands.\n\nExamples:\n  fmp-agent forex\n  fmp-agent forex quote EURUSD";
pub(crate) const NEWS_GROUP_ABOUT: &str = "News command group.";
pub(crate) const NEWS_GROUP_LONG: &str =
    "News commands.\n\nExamples:\n  fmp-agent news\n  fmp-agent news stock AAPL";

pub(crate) const QUOTE_ALIAS_ABOUT: &str = "Get the latest market quote; alias for market quote.";
pub(crate) const QUOTE_ALIAS_LONG: &str = "Top-level alias for `market quote`. Get the latest market quote for a stock ticker including price, volume, and change data.\n\nExamples:\n  fmp-agent quote AAPL\n  fmp-agent market quote AAPL";
pub(crate) const HISTORICAL_ALIAS_ABOUT: &str =
    "Get historical end-of-day price bars; alias for market historical.";
pub(crate) const HISTORICAL_ALIAS_LONG: &str = "Top-level alias for `market historical`. Get historical end-of-day price bars for a stock ticker. Optionally filter by date range.\n\nExamples:\n  fmp-agent historical AAPL\n  fmp-agent market historical AAPL --from 2024-01-01 --to 2024-12-31";
pub(crate) const PROFILE_ALIAS_ABOUT: &str = "Get company profile data; alias for company profile.";
pub(crate) const PROFILE_ALIAS_LONG: &str = "Top-level alias for `company profile`. Get company profile and reference data including sector, industry, description, and headquarters for a stock ticker.\n\nExamples:\n  fmp-agent profile AAPL\n  fmp-agent company profile AAPL";
pub(crate) const EARNINGS_ALIAS_ABOUT: &str =
    "Get earnings calendar rows; alias for calendar earnings.";
pub(crate) const EARNINGS_ALIAS_LONG: &str = "Top-level alias for `calendar earnings`. Get earnings calendar rows for an optional announcement date range.\n\nExamples:\n  fmp-agent earnings\n  fmp-agent calendar earnings --from 2024-01-01 --to 2024-03-31";

pub(crate) const SEARCH_ABOUT: &str =
    "Search for tradable stock symbols by ticker or company name.";
pub(crate) const SEARCH_LONG: &str = "Search for tradable stock symbols by ticker fragment or company name. The query is a positional argument.\n\nExamples:\n  fmp-agent search Apple\n  fmp-agent search AAPL";
pub(crate) const SEARCH_QUERY: &str = "Ticker fragment or company name to search for.";
pub(crate) const COMPANY_PROFILE_ABOUT: &str =
    "Get company profile and reference data for a stock ticker.";
pub(crate) const COMPANY_PROFILE_LONG: &str = "Get company profile and reference data including sector, industry, description, and headquarters for a stock ticker.\n\nExamples:\n  fmp-agent company profile AAPL\n  fmp-agent company profile MSFT";
pub(crate) const COMPANY_EXECUTIVES_ABOUT: &str = "Get key executives for a stock ticker.";
pub(crate) const COMPANY_EXECUTIVES_LONG: &str = "Get key executives for a company including name, title, and compensation data.\n\nExamples:\n  fmp-agent company executives AAPL";
pub(crate) const COMPANY_PEERS_ABOUT: &str = "Get peer company tickers for a stock ticker.";
pub(crate) const COMPANY_PEERS_LONG: &str = "Get a list of peer company tickers that trade in the same sector and industry.\n\nExamples:\n  fmp-agent company peers AAPL";
pub(crate) const MARKET_STOCK_LIST_ABOUT: &str = "List supported stock symbols.";
pub(crate) const MARKET_STOCK_LIST_LONG: &str = "List all supported stock symbols. Returns a large payload with ticker, name, exchange, and type for each instrument.\n\nExamples:\n  fmp-agent market stock-list";
pub(crate) const MARKET_REALTIME_QUOTE_ABOUT: &str =
    "Get a real-time market quote for a stock ticker.";
pub(crate) const MARKET_REALTIME_QUOTE_LONG: &str = "Get a real-time market quote for a stock ticker including live price, volume, and change data.\n\nExamples:\n  fmp-agent market realtime-quote AAPL";
pub(crate) const MARKET_AFTERMARKET_QUOTE_ABOUT: &str =
    "Get aftermarket quote data for a stock ticker.";
pub(crate) const MARKET_AFTERMARKET_QUOTE_LONG: &str = "Get aftermarket (pre-market and after-hours) quote data for a stock ticker including extended-hours price, volume, and change.\n\nExamples:\n  fmp-agent market aftermarket-quote AAPL";
pub(crate) const MARKET_AFTERMARKET_TRADE_ABOUT: &str =
    "Get aftermarket trade data for a stock ticker.";
pub(crate) const MARKET_AFTERMARKET_TRADE_LONG: &str = "Get aftermarket (pre-market and after-hours) trade data for a stock ticker including extended-hours trade details.\n\nExamples:\n  fmp-agent market aftermarket-trade AAPL";
pub(crate) const ETF_HOLDINGS_ABOUT: &str =
    "Get ETF holdings for a fund symbol; Starter accounts return an API error.";
pub(crate) const ETF_HOLDINGS_LONG: &str = "Get ETF holdings for a fund symbol. Note: Starter plan accounts receive an API error for this endpoint; it is included to exercise structured error handling.\n\nExamples:\n  fmp-agent etf holdings SPY";
pub(crate) const ETF_LIST_ABOUT: &str = "List supported ETF symbols.";
pub(crate) const ETF_LIST_LONG: &str = "List all supported ETF symbols. Returns a large payload with ticker, name, and type for each instrument.\n\nExamples:\n  fmp-agent etf list";
pub(crate) const ETF_INFO_ABOUT: &str = "Get ETF information for a fund symbol.";
pub(crate) const ETF_INFO_LONG: &str = "Get ETF information for a fund symbol including expense ratio, AUM, sector breakdown, and holdings summary.\n\nExamples:\n  fmp-agent etf info SPY\n  fmp-agent etf info QQQ";
pub(crate) const COMPANY_FINANCIAL_SCORES_ABOUT: &str =
    "Get financial quality scores for a stock ticker.";
pub(crate) const COMPANY_FINANCIAL_SCORES_LONG: &str = "Get financial quality scores for a company including Piotroski score and intrinsic value rating.\n\nExamples:\n  fmp-agent company scores AAPL";
pub(crate) const COMPANY_SHARE_FLOAT_ABOUT: &str =
    "Get share float and outstanding share data for a stock ticker.";
pub(crate) const COMPANY_SHARE_FLOAT_LONG: &str = "Get share float and outstanding share counts for a stock ticker.\n\nExamples:\n  fmp-agent company float AAPL";
pub(crate) const COMPANY_RATING_ABOUT: &str =
    "Get current analyst rating consensus for a stock ticker.";
pub(crate) const COMPANY_RATING_LONG: &str = "Get the current analyst rating consensus (buy/hold/sell) for a stock ticker.\n\nExamples:\n  fmp-agent company rating AAPL";
pub(crate) const COMPANY_HISTORICAL_RATING_ABOUT: &str =
    "Get historical company rating rows for a stock ticker.";
pub(crate) const COMPANY_HISTORICAL_RATING_LONG: &str = "Get historical company rating rows for a stock ticker. Returns up to the limit of most-recent rating snapshots.\n\nExamples:\n  fmp-agent company historical-rating AAPL\n  fmp-agent company historical-rating AAPL --limit 20";
pub(crate) const COMPANY_OUTLOOK_ABOUT: &str =
    "Report that company outlook is unavailable on the stable API.";
pub(crate) const COMPANY_OUTLOOK_LONG: &str = "Report that company outlook is unavailable because FMP only documents the aggregate company outlook command as a legacy endpoint. This command returns a structured endpoint_unavailable error without making a network request; use stable commands such as company profile, fundamentals income-statement, fundamentals ratios, and company scores instead.\n\nExamples:\n  fmp-agent company outlook AAPL";
pub(crate) const COMPANY_DELISTED_ABOUT: &str = "List delisted companies.";
pub(crate) const COMPANY_DELISTED_LONG: &str = "List delisted companies with optional paging. Returns a large payload with symbol, company name, exchange, and delisting date.\n\nExamples:\n  fmp-agent company delisted\n  fmp-agent company delisted --page 0 --limit 100";
pub(crate) const MARKET_QUOTE_ABOUT: &str = "Get the latest market quote for a stock ticker.";
pub(crate) const MARKET_QUOTE_LONG: &str = "Get the latest market quote for a stock ticker including price, volume, and change data.\n\nExamples:\n  fmp-agent market quote AAPL\n  fmp-agent market quote TSLA";
pub(crate) const MARKET_HISTORICAL_ABOUT: &str =
    "Get historical end-of-day price bars for a stock ticker.";
pub(crate) const MARKET_HISTORICAL_LONG: &str = "Get historical end-of-day price bars for a stock ticker. Optionally filter by date range.\n\nExamples:\n  fmp-agent market historical AAPL\n  fmp-agent market historical AAPL --from 2024-01-01 --to 2024-12-31";
pub(crate) const MARKET_DIVIDENDS_ABOUT: &str =
    "Get historical dividend events for a stock ticker.";
pub(crate) const MARKET_DIVIDENDS_LONG: &str = "Get historical dividend events for a stock ticker.\n\nExamples:\n  fmp-agent market dividends AAPL";
pub(crate) const MARKET_SPLITS_ABOUT: &str =
    "Get historical stock split events for a stock ticker.";
pub(crate) const MARKET_SPLITS_LONG: &str = "Get historical stock split events for a stock ticker.\n\nExamples:\n  fmp-agent market splits AAPL";
pub(crate) const MARKET_BATCH_QUOTE_ABOUT: &str =
    "Get the latest market quotes for multiple stock tickers in one request.";
pub(crate) const MARKET_BATCH_QUOTE_LONG: &str = "Get the latest market quotes for multiple stock tickers in a single API request. Accepts one or more symbols as positional arguments and returns current price, volume, and change data for each.\n\nExamples:\n  fmp-agent market batch-quote AAPL\n  fmp-agent market batch-quote AAPL MSFT GOOGL";
pub(crate) const MARKET_PRICE_CHANGE_ABOUT: &str =
    "Get price change percentages for a stock ticker.";
pub(crate) const MARKET_PRICE_CHANGE_LONG: &str = "Get price change percentages for a stock ticker across multiple time periods (1D, 5D, 1M, 3M, 6M, YTD, 1Y, 3Y, 5Y, 10Y).\n\nExamples:\n  fmp-agent market price-change AAPL";
pub(crate) const FUNDAMENTALS_INCOME_STATEMENT_ABOUT: &str =
    "Get annual income statement rows for a stock ticker.";
pub(crate) const FUNDAMENTALS_INCOME_STATEMENT_LONG: &str = "Get annual income statement rows for a stock ticker. Returns up to the limit of most-recent fiscal years.\n\nExamples:\n  fmp-agent fundamentals income-statement AAPL\n  fmp-agent fundamentals income-statement AAPL --limit 10";
pub(crate) const FUNDAMENTALS_INCOME_STATEMENT_AS_REPORTED_ABOUT: &str =
    "Get annual as-reported income statement rows for a stock ticker.";
pub(crate) const FUNDAMENTALS_INCOME_STATEMENT_AS_REPORTED_LONG: &str = "Get annual as-reported income statement rows for a stock ticker as filed with the SEC.\n\nExamples:\n  fmp-agent fundamentals income-statement-as-reported AAPL";
pub(crate) const FUNDAMENTALS_BALANCE_SHEET_ABOUT: &str =
    "Get annual balance sheet rows for a stock ticker.";
pub(crate) const FUNDAMENTALS_BALANCE_SHEET_LONG: &str = "Get annual balance sheet rows for a stock ticker.\n\nExamples:\n  fmp-agent fundamentals balance-sheet AAPL\n  fmp-agent fundamentals balance-sheet MSFT --limit 3";
pub(crate) const FUNDAMENTALS_CASH_FLOW_ABOUT: &str =
    "Get annual cash flow statement rows for a stock ticker.";
pub(crate) const FUNDAMENTALS_CASH_FLOW_LONG: &str = "Get annual cash flow statement rows for a stock ticker.\n\nExamples:\n  fmp-agent fundamentals cash-flow AAPL";
pub(crate) const FUNDAMENTALS_RATIOS_ABOUT: &str =
    "Get annual financial ratio rows for a stock ticker.";
pub(crate) const FUNDAMENTALS_RATIOS_LONG: &str = "Get annual financial ratio rows for a stock ticker including P/E, P/B, debt-to-equity, and return metrics.\n\nExamples:\n  fmp-agent fundamentals ratios AAPL";
pub(crate) const FUNDAMENTALS_METRICS_ABOUT: &str =
    "Get annual key metric rows for a stock ticker.";
pub(crate) const FUNDAMENTALS_METRICS_LONG: &str = "Get annual key metric rows for a stock ticker including revenue-per-share, earnings-per-share, and free cash flow.\n\nExamples:\n  fmp-agent fundamentals metrics AAPL";
pub(crate) const FUNDAMENTALS_INCOME_STATEMENT_GROWTH_ABOUT: &str =
    "Get annual income statement growth rows for a stock ticker.";
pub(crate) const FUNDAMENTALS_INCOME_STATEMENT_GROWTH_LONG: &str = "Get annual income statement growth rows for a stock ticker showing year-over-year change percentages.\n\nExamples:\n  fmp-agent fundamentals income-statement-growth AAPL";
pub(crate) const FUNDAMENTALS_BALANCE_SHEET_GROWTH_ABOUT: &str =
    "Get annual balance sheet growth rows for a stock ticker.";
pub(crate) const FUNDAMENTALS_BALANCE_SHEET_GROWTH_LONG: &str = "Get annual balance sheet growth rows for a stock ticker.\n\nExamples:\n  fmp-agent fundamentals balance-sheet-growth AAPL";
pub(crate) const FUNDAMENTALS_CASH_FLOW_GROWTH_ABOUT: &str =
    "Get annual cash flow growth rows for a stock ticker.";
pub(crate) const FUNDAMENTALS_CASH_FLOW_GROWTH_LONG: &str = "Get annual cash flow growth rows for a stock ticker.\n\nExamples:\n  fmp-agent fundamentals cash-flow-growth AAPL";
pub(crate) const FUNDAMENTALS_ENTERPRISE_VALUES_ABOUT: &str =
    "Get annual enterprise value rows for a stock ticker.";
pub(crate) const FUNDAMENTALS_ENTERPRISE_VALUES_LONG: &str = "Get annual enterprise value rows for a stock ticker including market cap, debt, and enterprise value.\n\nExamples:\n  fmp-agent fundamentals enterprise-values AAPL";
pub(crate) const FUNDAMENTALS_ANALYST_ESTIMATES_ABOUT: &str =
    "Get annual analyst estimate rows for a stock ticker.";
pub(crate) const FUNDAMENTALS_ANALYST_ESTIMATES_LONG: &str = "Get annual analyst estimate rows for a stock ticker including revenue and EPS consensus estimates.\n\nExamples:\n  fmp-agent fundamentals analyst-estimates AAPL";
pub(crate) const FUNDAMENTALS_REPORT_DATES_ABOUT: &str =
    "Get available financial report dates for a stock ticker.";
pub(crate) const FUNDAMENTALS_REPORT_DATES_LONG: &str = "Get available financial report dates for a stock ticker showing when statements were filed.\n\nExamples:\n  fmp-agent fundamentals report-dates AAPL";
pub(crate) const FUNDAMENTALS_ANNUAL_REPORT_FORM_ABOUT: &str =
    "Get annual report form JSON for a stock ticker and fiscal year.";
pub(crate) const FUNDAMENTALS_ANNUAL_REPORT_FORM_LONG: &str = "Get annual report form JSON for a stock ticker and fiscal year. The --year flag is required.\n\nExamples:\n  fmp-agent fundamentals annual-report AAPL --year 2023\n  fmp-agent fundamentals annual-report MSFT --year 2022 --period FY";
pub(crate) const FUNDAMENTALS_EARNINGS_ABOUT: &str =
    "Get historical earnings per share data for a stock ticker.";
pub(crate) const FUNDAMENTALS_EARNINGS_LONG: &str = "Get historical earnings per share data for a stock ticker including reported date, EPS, and revenue.\n\nExamples:\n  fmp-agent fundamentals earnings AAPL";
pub(crate) const FUNDAMENTALS_STATEMENT_GROWTH_ABOUT: &str =
    "Get annual statement growth rows for a stock ticker.";
pub(crate) const FUNDAMENTALS_STATEMENT_GROWTH_LONG: &str = "Get annual statement growth rows for a stock ticker showing consolidated growth data across all financial statements.\n\nExamples:\n  fmp-agent fundamentals statement-growth AAPL";
pub(crate) const ANALYST_PRICE_TARGET_CONSENSUS_ABOUT: &str =
    "Get analyst price target consensus for a stock ticker.";
pub(crate) const ANALYST_PRICE_TARGET_CONSENSUS_LONG: &str = "Get the consensus price target for a stock ticker aggregated across analyst estimates.\n\nExamples:\n  fmp-agent analyst price-target-consensus AAPL";
pub(crate) const ANALYST_PRICE_TARGET_SUMMARY_ABOUT: &str =
    "Get analyst price target summary for a stock ticker.";
pub(crate) const ANALYST_PRICE_TARGET_SUMMARY_LONG: &str = "Get a summary of analyst price targets for a stock ticker including high, low, and median targets.\n\nExamples:\n  fmp-agent analyst price-target-summary AAPL";
pub(crate) const ANALYST_GRADES_ABOUT: &str =
    "Get analyst grade action history for a stock ticker.";
pub(crate) const ANALYST_GRADES_LONG: &str = "Get analyst grade action history for a stock ticker showing upgrades, downgrades, and initiations.\n\nExamples:\n  fmp-agent analyst grades AAPL";
pub(crate) const ANALYST_UPGRADES_DOWNGRADES_ABOUT: &str =
    "Report that analyst upgrades and downgrades is unavailable on the stable API.";
pub(crate) const ANALYST_UPGRADES_DOWNGRADES_LONG: &str = "Report that analyst upgrades and downgrades is unavailable because FMP only documents this command as a legacy endpoint. This command returns a structured endpoint_unavailable error without making a network request; use analyst grades for stable analyst grade actions instead.\n\nExamples:\n  fmp-agent analyst upgrades-downgrades AAPL";
pub(crate) const ANALYST_RATINGS_SNAPSHOT_ABOUT: &str =
    "Get a snapshot of current analyst ratings for a stock ticker.";
pub(crate) const ANALYST_RATINGS_SNAPSHOT_LONG: &str = "Get a snapshot of current analyst ratings for a stock ticker including buy, hold, and sell counts with average rating. Starter plan accounts receive symbol-limited results.\n\nExamples:\n  fmp-agent analyst ratings-snapshot AAPL";
pub(crate) const ANALYST_EARNINGS_SURPRISES_ABOUT: &str =
    "Report that symbol earnings surprises is unavailable on the stable API.";
pub(crate) const ANALYST_EARNINGS_SURPRISES_LONG: &str = "Report that symbol earnings surprises is unavailable because FMP only documents this command as a legacy endpoint. This command returns a structured endpoint_unavailable error without making a network request; no stable per-symbol earnings surprises API path is available.\n\nExamples:\n  fmp-agent analyst earnings-surprises AAPL";
pub(crate) const ANALYST_PRICE_TARGET_ABOUT: &str =
    "Report that historical analyst price targets is unavailable on the stable API.";
pub(crate) const ANALYST_PRICE_TARGET_LONG: &str = "Report that historical analyst price targets is unavailable because FMP only documents this command as a legacy endpoint. This command returns a structured endpoint_unavailable error without making a network request; use analyst price-target-consensus or analyst price-target-summary for stable price target data instead.\n\nExamples:\n  fmp-agent analyst price-target AAPL";
pub(crate) const INSIDER_TRADING_LATEST_ABOUT: &str = "Get latest insider trading rows.";
pub(crate) const INSIDER_TRADING_LATEST_LONG: &str = "Get latest insider trading rows. Uses zero-based paging.\n\nExamples:\n  fmp-agent insider latest\n  fmp-agent insider latest --page 1 --limit 20";
pub(crate) const INSIDER_TRADING_SEARCH_ABOUT: &str =
    "Search insider trading filings for a stock ticker.";
pub(crate) const INSIDER_TRADING_SEARCH_LONG: &str = "Search insider trading filings filtered by stock ticker. Returns individual filing records with reporting names, transaction types, and details.\n\nExamples:\n  fmp-agent insider search AAPL";
pub(crate) const EARNINGS_CALENDAR_ABOUT: &str =
    "Get earnings calendar rows for an optional announcement date range.";
pub(crate) const EARNINGS_CALENDAR_LONG: &str = "Get earnings calendar rows for an optional announcement date range.\n\nExamples:\n  fmp-agent calendar earnings\n  fmp-agent calendar earnings --from 2024-01-01 --to 2024-03-31";
pub(crate) const CALENDAR_MARKET_HOURS_ABOUT: &str =
    "Get current market open status and trading hours.";
pub(crate) const CALENDAR_MARKET_HOURS_LONG: &str = "Get current market open status and trading hours including whether the stock market is currently open and the next session open and close times.\n\nExamples:\n  fmp-agent calendar market-hours";
pub(crate) const TREASURY_RATES_ABOUT: &str = "Get treasury rate rows for an optional date range.";
pub(crate) const TREASURY_RATES_LONG: &str = "Get treasury rate rows for an optional date range. Returns yields for standard maturities (1M, 2M, 3M, 6M, 1Y, 2Y, 3Y, 5Y, 7Y, 10Y, 20Y, 30Y).\n\nExamples:\n  fmp-agent macro treasury-rates\n  fmp-agent macro treasury-rates --from 2024-01-01 --to 2024-12-31";
pub(crate) const MACRO_RISK_PREMIUM_ABOUT: &str = "Get the current market risk premium.";
pub(crate) const MACRO_RISK_PREMIUM_LONG: &str = "Get the current market risk premium data including equity risk premium, country risk premium, and default spread.\n\nExamples:\n  fmp-agent macro risk-premium";
pub(crate) const ECONOMIC_INDICATORS_ABOUT: &str =
    "Get economic indicator rows by indicator name and optional date range.";
pub(crate) const ECONOMIC_INDICATORS_LONG: &str = "Get economic indicator rows by indicator name and optional date range. The indicator name is a positional argument.\n\nExamples:\n  fmp-agent macro economic-indicators GDP\n  fmp-agent macro economic-indicators GDP --from 2024-01-01 --to 2024-12-31";
pub(crate) const TECHNICAL_SMA_ABOUT: &str =
    "Get simple moving average technical indicator rows for a stock ticker.";
pub(crate) const TECHNICAL_SMA_LONG: &str = "Get simple moving average technical indicator rows for a stock ticker. Defaults to a 10-period SMA on the 1day timeframe.\n\nExamples:\n  fmp-agent technical sma AAPL\n  fmp-agent technical sma AAPL --period-length 20 --timeframe 1hour";
pub(crate) const SEC_FILINGS_ABOUT: &str = "Get SEC filing metadata for a stock ticker.";
pub(crate) const SEC_FILINGS_LONG: &str = "Get SEC filing metadata for a stock ticker. Defaults --from to 90 days ago when omitted because FMP requires a start date.\n\nExamples:\n  fmp-agent sec filings AAPL\n  fmp-agent sec filings AAPL --from 2024-01-01 --to 2024-12-31";
pub(crate) const CRYPTO_LIST_ABOUT: &str = "List supported cryptocurrency symbols.";
pub(crate) const CRYPTO_LIST_LONG: &str = "List all supported cryptocurrency symbols. Returns a large payload with symbol and name for each instrument.\n\nExamples:\n  fmp-agent crypto list";
pub(crate) const CRYPTO_QUOTE_ABOUT: &str = "Get the latest quote for a cryptocurrency pair.";
pub(crate) const CRYPTO_QUOTE_LONG: &str = "Get the latest quote for a cryptocurrency pair including price and volume data.\n\nExamples:\n  fmp-agent crypto quote BTCUSD";
pub(crate) const CRYPTO_HISTORICAL_ABOUT: &str =
    "Get historical end-of-day price bars for a cryptocurrency pair.";
pub(crate) const CRYPTO_HISTORICAL_LONG: &str = "Get historical end-of-day price bars for a cryptocurrency pair. Optionally filter by date range.\n\nExamples:\n  fmp-agent crypto historical BTCUSD\n  fmp-agent crypto historical BTCUSD --from 2024-01-01 --to 2024-12-31";
pub(crate) const FOREX_QUOTE_ABOUT: &str = "Get the latest quote for a forex pair.";
pub(crate) const FOREX_QUOTE_LONG: &str =
    "Get the latest quote for a forex pair.\n\nExamples:\n  fmp-agent forex quote EURUSD";
pub(crate) const FOREX_HISTORICAL_ABOUT: &str =
    "Get historical end-of-day price bars for a forex pair.";
pub(crate) const FOREX_HISTORICAL_LONG: &str = "Get historical end-of-day price bars for a forex pair. Optionally filter by date range.\n\nExamples:\n  fmp-agent forex historical EURUSD\n  fmp-agent forex historical EURUSD --from 2024-01-01 --to 2024-12-31";
pub(crate) const NEWS_STOCK_ABOUT: &str = "Get recent stock news for a stock ticker.";
pub(crate) const NEWS_STOCK_LONG: &str = "Get recent stock news for a stock ticker.\n\nExamples:\n  fmp-agent news stock AAPL\n  fmp-agent news stock AAPL --limit 20";
pub(crate) const NEWS_GENERAL_ABOUT: &str = "Get latest general market news.";
pub(crate) const NEWS_GENERAL_LONG: &str = "Get latest general market news. Uses zero-based paging.\n\nExamples:\n  fmp-agent news general\n  fmp-agent news general --page 1 --limit 20";
pub(crate) const NEWS_ARTICLES_ABOUT: &str = "Get latest FMP articles.";
pub(crate) const NEWS_ARTICLES_LONG: &str = "Get latest FMP articles. Uses zero-based paging.\n\nExamples:\n  fmp-agent news articles\n  fmp-agent news articles --page 1 --limit 20";
pub(crate) const NEWS_FOREX_ABOUT: &str = "Get latest forex news.";
pub(crate) const NEWS_FOREX_LONG: &str = "Get latest forex news. Uses zero-based paging.\n\nExamples:\n  fmp-agent news forex\n  fmp-agent news forex --page 0 --limit 10";
pub(crate) const NEWS_CRYPTO_ABOUT: &str = "Get latest crypto news.";
pub(crate) const NEWS_CRYPTO_LONG: &str = "Get latest crypto news. Uses zero-based paging.\n\nExamples:\n  fmp-agent news crypto\n  fmp-agent news crypto --page 0 --limit 10";
pub(crate) const SCHEMA_ABOUT: &str = "Dump the CLI surface as machine-readable JSON.";
pub(crate) const SCHEMA_LONG: &str = "Dump the CLI surface as machine-readable JSON. The output format is experimental and may change between versions. schema_version 3 is the current grouped-command format.\n\nThis command does not require FMP_API_KEY and does not make any network requests.\n\nExamples:\n  fmp-agent schema\n  fmp-agent schema | jq '.commands | length'";

pub(crate) const COMMANDS_ABOUT: &str = "List all available leaf commands.";
pub(crate) const COMMANDS_LONG: &str = "List all available leaf commands. Output is sorted alphabetically by default. Pass --grouped for a human-readable listing with commands organized by category and annotated with their about text.\n\nThis command does not require FMP_API_KEY and does not make any network requests.\n\nExamples:\n  fmp-agent commands\n  fmp-agent commands | grep quote\n  fmp-agent commands --grouped";
pub(crate) const COMMANDS_GROUPED: &str =
    "Group commands by category instead of flat sorted output.";

pub(crate) const COMPLETIONS_ABOUT: &str =
    "Generate shell completions for bash, zsh, fish, or powershell.";
pub(crate) const COMPLETIONS_LONG: &str = "Generate shell completions for the specified shell. The completions script is written to stdout. Supported shells: bash, elvish, fish, powershell, zsh.\n\nCompletions are generated at runtime from the current binary and are not shipped as pre-installed files.\n\nThis command does not require FMP_API_KEY and does not make any network requests.\n\nExamples:\n  fmp-agent completions bash\n  fmp-agent completions zsh | sudo tee /usr/share/zsh/site-functions/_fmp-agent\n  source <(fmp-agent completions bash)";

pub(crate) const DOCTOR_ABOUT: &str = "Check local configuration readiness as JSON.";
pub(crate) const DOCTOR_LONG: &str = "Check whether fmp-agent is locally configured for live API calls. The report includes the binary version, sanitized base URL, API key presence, and live-connectivity status without revealing secrets.\n\nThis command does not require FMP_API_KEY and does not make any network requests, so it does not consume FMP API quota.\n\nExamples:\n  fmp-agent doctor\n  fmp-agent doctor | jq '.ok'\n  FMP_API_KEY=your-key fmp-agent doctor";

pub(crate) const SYMBOL: &str =
    "Symbol accepted by this command, such as a stock ticker, forex pair, or crypto pair.";
pub(crate) const SYMBOLS: &str = "One or more stock ticker symbols, separated by spaces.";
pub(crate) const STOCK_SYMBOL: &str = "Stock ticker symbol, for example AAPL.";
pub(crate) const LIMIT_ROWS: &str = "Maximum number of rows to return.";
pub(crate) const YEAR: &str = "Fiscal year, for example 2024.";
pub(crate) const PERIOD: &str = "Fiscal period, usually FY.";
pub(crate) const FROM: &str =
    "Inclusive start date in YYYY-MM-DD format. Omit for an open-ended start when supported.";
pub(crate) const TO: &str =
    "Inclusive end date in YYYY-MM-DD format. Omit for an open-ended end when supported.";
pub(crate) const ECONOMIC_INDICATOR_NAME: &str = "Economic indicator name, for example GDP.";
pub(crate) const ANNUAL_LIMIT: &str = "Maximum number of annual rows to return.";
pub(crate) const PERIOD_LENGTH: &str = "Number of periods in the moving average window.";
pub(crate) const TIMEFRAME: &str = "FMP candle timeframe, for example 1day.";
pub(crate) const PAGE: &str = "Zero-based result page.";
pub(crate) const PAGE_LIMIT: &str = "Maximum number of items to return.";

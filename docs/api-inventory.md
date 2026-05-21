# Financial Modeling Prep API inventory

This inventory targets the Financial Modeling Prep Starter plan that the account labels as the standard `$19/mo` plan. Public FMP pricing currently shows Starter as `$22/mo` when billed annually, so this document treats `$19/mo` as an account-specific or legacy price for the Starter tier unless the authenticated dashboard says otherwise.

## Source status

Public sources provide category-level Starter entitlements, not a complete endpoint-by-endpoint entitlement matrix. The endpoint list below comes from public FMP developer docs and sitemap output. Treat rows marked `inferred` or `unknown` as candidates that need API Viewer confirmation before the CLI exposes them as supported.

Authenticated testing was performed with the account credentials from the local `.env` file on 2026-05-21. The API key is intentionally not recorded here. The API Viewer uses stable URLs under `https://financialmodelingprep.com/stable/` and loaded `income-statement?symbol=AAPL` by default.

## Starter constraints from public pricing

| Constraint | Starter value | Source note |
| --- | --- | --- |
| API calls | 300 calls/min | Public pricing |
| Historical depth | Up to 5 years | Public pricing |
| Bandwidth | 20 GB trailing 30 days | Public pricing |
| Geographic coverage | US coverage | Public pricing |
| Clearly included categories | Profile/reference data, annual fundamentals and ratios, historical stock price data, financial market news, crypto, forex | Public pricing |
| Clearly excluded categories | 30+ years history, UK/Canada/global coverage, full fundamentals/ratios, intraday charts, technical indicators, corporate calendars, earnings call transcripts, ETF and mutual fund holdings, 13F institutional holdings, bulk and batch delivery | Public pricing |

## Status labels

| Status | Meaning |
| --- | --- |
| `explicit` | Public pricing names this data category as included in Starter. |
| `inferred` | Public pricing includes the parent category, but the exact endpoint entitlement is not public. |
| `excluded` | Public pricing names the category as not included in Starter. |
| `unknown` | Public docs list the endpoint, but public pricing does not clearly map it to Starter. |
| `confirmed` | Authenticated account probe returned HTTP 200 with usable data. |
| `denied` | Authenticated account probe returned a subscription restriction for this account. |
| `unconfirmed` | Authenticated probe used a plausible path but did not confirm access, usually because the path needs API Viewer validation. |

## Authenticated Starter probe results

These probes used representative symbols and short date ranges. A `confirmed` result means the account can call the endpoint with the tested shape, not that every parameter, symbol, exchange, or historical depth is available.

| Endpoint area | Stable path tested | Result | Response shape |
| --- | --- | --- | --- |
| Company profile | `profile?symbol=AAPL` | confirmed | Array with company profile fields such as `symbol`, `price`, `marketCap`, `beta`. |
| Symbol search | `search-symbol?query=AAPL` | confirmed | Array of matching securities with `symbol`, `name`, `currency`, `exchangeFullName`, `exchange`. |
| Stock quote | `quote?symbol=AAPL` | confirmed | Array with quote fields such as `symbol`, `name`, `price`, `changePercentage`, `volume`. |
| Historical EOD | `historical-price-eod/full?symbol=AAPL&from=2025-01-01&to=2025-01-31` | confirmed | Array of daily bars with `date`, `open`, `high`, `low`, `close`, `volume`. |
| Income statement | `income-statement?symbol=AAPL&period=annual&limit=5` | confirmed | Array of annual statements with `date`, `symbol`, `reportedCurrency`, `fiscalYear`, `period`. |
| Balance sheet | `balance-sheet-statement?symbol=AAPL&period=annual&limit=5` | confirmed | Array of annual statements with `date`, `symbol`, `reportedCurrency`, `fiscalYear`, `period`. |
| Cash flow | `cash-flow-statement?symbol=AAPL&period=annual&limit=5` | confirmed | Array of annual statements with `date`, `symbol`, `reportedCurrency`, `fiscalYear`, `period`. |
| Financial ratios | `ratios?symbol=AAPL&period=annual&limit=5` | confirmed | Array with ratio fields such as `grossProfitMargin`, `ebitMargin`, `ebitdaMargin`. |
| Key metrics | `key-metrics?symbol=AAPL&period=annual&limit=5` | confirmed | Array with metric fields such as `marketCap`, `enterpriseValue`, `evToSales`. |
| Stock news | `news/stock?symbols=AAPL&limit=5` | confirmed | Array with news fields such as `symbol`, `publishedDate`, `publisher`, `title`, `url`. |
| Crypto quote | `quote?symbol=BTCUSD` | confirmed | Array with the same quote shape as equities. |
| Forex quote | `quote?symbol=EURUSD` | confirmed | Array with the same quote shape as equities. |
| Earnings calendar | `earnings-calendar?from=2026-01-01&to=2026-01-31` | confirmed | Array with `symbol`, `date`, `epsActual`, `epsEstimated`, `revenueActual`, `revenueEstimated`. |
| Income statement growth | `income-statement-growth?symbol=AAPL&period=annual&limit=5` | confirmed | Array with growth fields such as `growthRevenue`, `growthCostOfRevenue`, `growthGrossProfit`. |
| Enterprise value | `enterprise-values?symbol=AAPL&period=annual&limit=5` | confirmed | Array with `stockPrice`, `numberOfShares`, `marketCapitalization`, `enterpriseValue`. |
| SEC filings by symbol | `sec-filings-search/symbol?symbol=AAPL&from=2024-01-01&to=2024-03-01` | confirmed | Array with `symbol`, `cik`, `filingDate`, `acceptedDate`, `formType`, `link`, `finalLink`. |
| Analyst estimates | `analyst-estimates?symbol=AAPL&period=annual&limit=5` | confirmed | Array with estimate ranges and averages such as `revenueLow`, `revenueHigh`, `revenueAvg`. |
| Price target consensus | `price-target-consensus?symbol=AAPL` | confirmed | Array with `symbol`, `targetHigh`, `targetLow`, `targetMedian`, and `targetConsensus`. |
| Price target summary | `price-target-summary?symbol=AAPL` | confirmed | Array with price target averages and counts for recent periods plus publisher data. |
| Analyst grades consensus | `grades-consensus?symbol=AAPL` | confirmed | Array with `strongBuy`, `buy`, `hold`, `sell`, `strongSell`, and `consensus`. |
| Shares float | `shares-float?symbol=AAPL` | confirmed | Array with `symbol`, `date`, `freeFloat`, `floatShares`, `outstandingShares`, and `source`. |
| Financial report dates | `financial-reports-dates?symbol=AAPL` | confirmed | Array with report metadata such as `symbol`, `fiscalYear`, `period`, and JSON/report links. |
| Stock peers | `stock-peers?symbol=AAPL` | confirmed | Array with peer `symbol`, `companyName`, `price`, `mktCap`. |
| Historical dividends | `dividends?symbol=AAPL` | confirmed | Array with `date`, `recordDate`, `paymentDate`, `adjDividend`, `dividend`, `yield`. |
| Historical splits | `splits?symbol=AAPL` | confirmed | Array with `date`, `numerator`, `denominator`, `splitType`. |
| Treasury rates | `treasury-rates?from=2025-01-01&to=2025-01-31` | confirmed | Array with date and treasury maturity columns. |
| Technical SMA | `technical-indicators/sma?symbol=AAPL&periodLength=10&timeframe=1day` | confirmed | Array with OHLCV fields and `sma`; intraday indicator access still needs separate testing. |
| Stock price change | `stock-price-change?symbol=AAPL` | confirmed | Object with percentage changes across periods such as `1D`, `5D`, `1M`, `ytd`, `1Y`, `5Y`, `10Y`, and `max`. |
| Key executives | `key-executives?symbol=AAPL` | confirmed | Array with executive fields such as `title`, `name`, `pay`, `currencyPay`, `gender`, `yearBorn`, `titleSince`, and `active`. |
| Income statement as reported | `income-statement-as-reported?symbol=AAPL&period=annual&limit=5` | confirmed | Array of annual as-reported statements with `symbol`, `fiscalYear`, `period`, `reportedCurrency`, `date`, and nested `data`. |
| Balance sheet growth | `balance-sheet-statement-growth?symbol=AAPL&period=annual&limit=5` | confirmed | Array with growth fields such as `growthCashAndCashEquivalents`, `growthShortTermInvestments`, and `growthCashAndShortTermInvestments`. |
| Cash flow growth | `cash-flow-statement-growth?symbol=AAPL&period=annual&limit=5` | confirmed | Array with growth fields such as `growthNetIncome`, `growthDepreciationAndAmortization`, and `growthDeferredIncomeTax`. |
| Financial scores | `financial-scores?symbol=AAPL` | confirmed | Array with score fields such as `altmanZScore`, `piotroskiScore`, `workingCapital`, `totalAssets`, `retainedEarnings`, and `ebit`. |
| General news | `news/general-latest?page=0&limit=1` | confirmed | Array with news fields such as `symbol`, `publishedDate`, `publisher`, `title`, `image`, `site`, `text`, and `url`. |
| FMP articles | `fmp-articles?page=0&limit=1` | confirmed | Array with article fields such as `title`, `date`, `content`, `tickers`, `image`, `link`, `author`, and `site`. |
| Forex news | `news/forex-latest?page=0&limit=1` | confirmed | Array with news fields such as `symbol`, `publishedDate`, `publisher`, `title`, `image`, `site`, `text`, and `url`. |
| Crypto news | `news/crypto-latest?page=0&limit=1` | confirmed | Array with news fields such as `symbol`, `publishedDate`, `publisher`, `title`, `image`, `site`, `text`, and `url`. |
| Analyst grades | `grades?symbol=AAPL` | confirmed | Array with grade action fields such as `symbol`, `date`, `gradingCompany`, `previousGrade`, `newGrade`, and `action`. |
| ETF holdings | `etf/holdings?symbol=SPY` | denied | HTTP 402 subscription restriction. |
| Earnings transcript | `earning-call-transcript?symbol=AAPL&year=2024&quarter=1` | denied | HTTP 402 subscription restriction. |
| Transcript dates | `earning-call-transcript-dates?symbol=AAPL` | denied | HTTP 402 subscription restriction. |
| Price target | `price-target?symbol=AAPL` | unconfirmed | Tested path returned HTTP 404 with an empty array, so the path or entitlement needs API Viewer confirmation. |
| Company outlook | `company-outlook?symbol=AAPL` | unconfirmed | Tested path returned HTTP 404 with an empty array, so the path or entitlement needs API Viewer confirmation. |
| Market open | `market-hours` | unconfirmed | Tested path returned HTTP 404 with an empty array, so the path or entitlement needs API Viewer confirmation. |

## Endpoint inventory

| Group | Endpoint doc | Starter status | CLI priority | Notes |
| --- | --- | --- | --- | --- |
| Company/reference | [Company core information](https://site.financialmodelingprep.com/developer/docs/company-core-information-api) | confirmed | high | Tested `profile?symbol=AAPL`. |
| Company/reference | [Company key stats](https://site.financialmodelingprep.com/developer/docs/companies-key-stats-free-api) | confirmed | high | Current stable docs map this legacy area to `profile?symbol=AAPL`; implemented as `company stats` aliasing the confirmed profile endpoint. |
| Company/reference | [Key executives](https://site.financialmodelingprep.com/developer/docs/key-executives-api) | confirmed | medium | Tested `key-executives?symbol=AAPL`; implemented as `company executives`. |
| Company/reference | [Company image](https://site.financialmodelingprep.com/developer/docs/company-image-api) | explicit | low | Useful metadata, not core CLI output. |
| Company/reference | [Company outlook](https://site.financialmodelingprep.com/developer/docs/company-outlook-api) | inferred | high | Aggregated endpoint, confirm exact Starter behavior. |
| Company/reference | [Stock peers](https://site.financialmodelingprep.com/developer/docs/stock-peers-api) | confirmed | medium | Tested `stock-peers?symbol=AAPL`; implemented as `company peers`. |
| Company/reference | [Symbol lookup](https://site.financialmodelingprep.com/developer/docs/stock-ticker-symbol-lookup-api) | confirmed | high | Tested `search-symbol?query=AAPL`; needed for search/discovery commands. |
| Company/reference | [Historical share float](https://site.financialmodelingprep.com/developer/docs/company-historical-share-float) | confirmed | medium | Tested `shares-float?symbol=AAPL`; implemented as `company share-float`. Historical depth likely capped at 5 years. |
| Company/reference | [All countries](https://site.financialmodelingprep.com/developer/docs/all-countries-company-information) | excluded | low | Non-US/global coverage is not Starter-safe. |
| Company/reference | [Euronext prices](https://site.financialmodelingprep.com/developer/docs/euronext-prices-api) | excluded | low | Non-US/global coverage is not Starter-safe. |
| Quote/market | [Stock market quote](https://site.financialmodelingprep.com/developer/docs/stock-market-quote-free-api) | confirmed | high | Tested `quote?symbol=AAPL`. |
| Quote/market | [Realtime stock quote](https://site.financialmodelingprep.com/developer/docs/realtime-stock-quote-api) | inferred | high | Confirm delay/real-time behavior for account. |
| Quote/market | [Quote order](https://site.financialmodelingprep.com/developer/docs/quote-order-quote) | inferred | low | Candidate after basic quote works. |
| Quote/market | [Aftermarket quote](https://site.financialmodelingprep.com/developer/docs/aftermarket-quote-quote) | unknown | low | Not clearly mapped to Starter. |
| Quote/market | [Stock price change](https://site.financialmodelingprep.com/developer/docs/stock-price-change-api) | confirmed | high | Tested `stock-price-change?symbol=AAPL`; implemented as `market price-change`. |
| Quote/market | [All realtime full prices](https://site.financialmodelingprep.com/developer/docs/all-realtime-full-prices-quote) | unknown | low | Could be broad/high-volume, confirm entitlement. |
| Historical/charts | [Historical stock data](https://site.financialmodelingprep.com/developer/docs/historical-stock-data-free-api) | confirmed | high | Tested `historical-price-eod/full?symbol=AAPL` with a January 2025 date range. |
| Historical/charts | [Daily chart](https://site.financialmodelingprep.com/developer/docs/daily-chart-charts) | confirmed | high | Current stable docs map daily EOD data to `historical-price-eod/full?symbol=AAPL`; implemented as `market daily-chart` aliasing the confirmed historical endpoint. |
| Historical/charts | [Historical stock splits](https://site.financialmodelingprep.com/developer/docs/historical-stock-splits-api) | confirmed | medium | Tested `splits?symbol=AAPL`; implemented as `market splits`; date depth still needs boundary testing. |
| Historical/charts | [Historical stock dividends](https://site.financialmodelingprep.com/developer/docs/historical-stock-dividends-api) | confirmed | medium | Tested `dividends?symbol=AAPL`; implemented as `market dividends`; date depth still needs boundary testing. |
| Fundamentals/statements | [Income statement](https://site.financialmodelingprep.com/developer/docs/financial-statement-free-api) | confirmed | high | Tested `income-statement?symbol=AAPL&period=annual&limit=5`. |
| Fundamentals/statements | [Balance sheet](https://site.financialmodelingprep.com/developer/docs/balance-sheet-statements-financial-statements) | confirmed | high | Tested `balance-sheet-statement?symbol=AAPL&period=annual&limit=5`. |
| Fundamentals/statements | [Cash flow](https://site.financialmodelingprep.com/developer/docs/cashflow-statements-financial-statements) | confirmed | high | Tested `cash-flow-statement?symbol=AAPL&period=annual&limit=5`. |
| Fundamentals/statements | [Income statement as reported](https://site.financialmodelingprep.com/developer/docs/financial-statement-as-reported-api) | confirmed | medium | Tested `income-statement-as-reported?symbol=AAPL&period=annual&limit=5`; implemented as `fundamentals income-statement-as-reported`. |
| Fundamentals/statements | [Financial report dates](https://site.financialmodelingprep.com/developer/docs/financial-reports-dates) | confirmed | medium | Tested `financial-reports-dates?symbol=AAPL`; implemented as `fundamentals report-dates`. |
| Fundamentals/statements | [Annual report form](https://site.financialmodelingprep.com/developer/docs/annual-report-form-api) | inferred | medium | Confirm entitlement and document/download shape. |
| Ratios/metrics/growth | [Key metrics](https://site.financialmodelingprep.com/developer/docs/company-key-metrics-api) | confirmed | high | Tested `key-metrics?symbol=AAPL&period=annual&limit=5`. |
| Ratios/metrics/growth | [Financial ratios](https://site.financialmodelingprep.com/developer/docs/financial-ratio-free-api) | confirmed | high | Tested `ratios?symbol=AAPL&period=annual&limit=5`. |
| Ratios/metrics/growth | [Financial statement growth](https://site.financialmodelingprep.com/developer/docs/financial-statements-growth-api) | inferred | high | Growth family likely available from tested income statement growth endpoint. |
| Ratios/metrics/growth | [Income statement growth](https://site.financialmodelingprep.com/developer/docs/income-statements-growth-api) | confirmed | medium | Tested `income-statement-growth?symbol=AAPL&period=annual&limit=5`. |
| Ratios/metrics/growth | [Balance sheet growth](https://site.financialmodelingprep.com/developer/docs/balance-sheet-statements-growth-api) | confirmed | medium | Tested `balance-sheet-statement-growth?symbol=AAPL&period=annual&limit=5`; implemented as `fundamentals balance-sheet-growth`. |
| Ratios/metrics/growth | [Cash flow statement growth](https://site.financialmodelingprep.com/developer/docs/company-financial-statement-growth-api) | confirmed | medium | Tested `cash-flow-statement-growth?symbol=AAPL&period=annual&limit=5`; implemented as `fundamentals cash-flow-growth`. |
| Ratios/metrics/growth | [Enterprise value](https://site.financialmodelingprep.com/developer/docs/company-enterprise-value-api) | confirmed | medium | Tested `enterprise-values?symbol=AAPL&period=annual&limit=5`. |
| Ratios/metrics/growth | [Financial scores](https://site.financialmodelingprep.com/developer/docs/stock-financial-scores) | confirmed | medium | Tested `financial-scores?symbol=AAPL`; implemented as `company financial-scores`. |
| News | [FMP articles](https://site.financialmodelingprep.com/developer/docs/fmp-articles-api) | confirmed | medium | Tested `fmp-articles?page=0&limit=1`; implemented as `news articles`. |
| News | [General news](https://site.financialmodelingprep.com/developer/docs/general-news-api) | confirmed | medium | Tested `news/general-latest?page=0&limit=1`; implemented as `news general`. |
| News | [Stock news](https://site.financialmodelingprep.com/developer/docs/stock-news-api) | confirmed | high | Tested `news/stock?symbols=AAPL&limit=5`; good first news command. |
| News | [Forex news](https://site.financialmodelingprep.com/developer/docs/forex-news-api) | confirmed | low | Tested `news/forex-latest?page=0&limit=1`; implemented as `news forex`. |
| News | [Crypto news](https://site.financialmodelingprep.com/developer/docs/crypto-news-api) | confirmed | low | Tested `news/crypto-latest?page=0&limit=1`; implemented as `news crypto`. |
| News | [Press releases](https://site.financialmodelingprep.com/developer/docs/press-releases-api) | denied | medium | Tested `press-releases?symbol=AAPL&page=0&limit=1`, which returned HTTP 404 with an empty array; tested `news/press-releases-latest?page=0&limit=1`, which returned HTTP 402 subscription restriction. |
| News | [Social sentiment](https://site.financialmodelingprep.com/developer/docs/social-sentiment-api) | unknown | low | Not clearly mapped to Starter. |
| Crypto/forex/commodities | [Crypto quote/list](https://site.financialmodelingprep.com/developer/docs/crypto-currency-free-api) | confirmed | medium | Tested `quote?symbol=BTCUSD`. |
| Crypto/forex/commodities | [Bitcoin price](https://site.financialmodelingprep.com/developer/docs/bitcoin-price-free-api) | explicit | low | Crypto category. |
| Crypto/forex/commodities | [Crypto historical](https://site.financialmodelingprep.com/developer/docs/cryptocurrency-historical-data-api) | explicit | medium | Historical depth may be capped. |
| Crypto/forex/commodities | [Currency exchange rate](https://site.financialmodelingprep.com/developer/docs/currency-exchange-rate-free-api) | confirmed | medium | Tested `quote?symbol=EURUSD`. |
| Crypto/forex/commodities | [Forex historical](https://site.financialmodelingprep.com/developer/docs/forex-historical-data-api) | explicit | medium | Historical depth may be capped. |
| Crypto/forex/commodities | [Full quote forex](https://site.financialmodelingprep.com/developer/docs/full-quote-forex) | explicit | low | Forex category. |
| Crypto/forex/commodities | [Commodities list](https://site.financialmodelingprep.com/developer/docs/commodities-list-commodities) | unknown | low | Commodities are not named in Starter summary. |
| Crypto/forex/commodities | [Commodities prices](https://site.financialmodelingprep.com/developer/docs/commodities-prices-api) | unknown | low | Commodities are not named in Starter summary. |
| Crypto/forex/commodities | [Commodity historical](https://site.financialmodelingprep.com/developer/docs/commodity-historical-price-api) | unknown | low | Commodities are not named in Starter summary. |
| Calendars/corporate actions | [Earnings calendar](https://site.financialmodelingprep.com/developer/docs/earnings-calendar-api) | confirmed | low | Tested `earnings-calendar?from=2026-01-01&to=2026-01-31`; implemented as `calendar earnings`; account access differs from public-pricing inference. |
| Calendars/corporate actions | [Confirmed earnings calendar](https://site.financialmodelingprep.com/developer/docs/earnings-calendar-confirmed-api) | excluded | low | Corporate calendars are excluded by pricing. |
| Calendars/corporate actions | [Earnings surprises](https://site.financialmodelingprep.com/developer/docs/earnings-surprises-api) | unknown | low | May not be calendar-only, verify. |
| Calendars/corporate actions | [Historical earnings](https://site.financialmodelingprep.com/developer/docs/earnings-historical-earnings) | unknown | low | Verify before exposing. |
| Calendars/corporate actions | [Earnings call transcript](https://site.financialmodelingprep.com/developer/docs/earning-call-transcript-api) | denied | low | Tested `earning-call-transcript?symbol=AAPL&year=2024&quarter=1`; HTTP 402 subscription restriction. |
| Calendars/corporate actions | [Transcript dates](https://site.financialmodelingprep.com/developer/docs/transcript-dates-earnings-transcripts) | denied | low | Tested `earning-call-transcript-dates?symbol=AAPL`; HTTP 402 subscription restriction. |
| Calendars/corporate actions | [Dividend calendar](https://site.financialmodelingprep.com/developer/docs/dividend-calendar-api) | excluded | low | Corporate calendars are excluded by pricing. |
| Calendars/corporate actions | [Stock split calendar](https://site.financialmodelingprep.com/developer/docs/stock-split-calendar-api) | excluded | low | Corporate calendars are excluded by pricing. |
| Calendars/corporate actions | [IPO calendar](https://site.financialmodelingprep.com/developer/docs/ipo-calendar-api) | excluded | low | Corporate calendars are excluded by pricing. |
| Calendars/corporate actions | [Market open](https://site.financialmodelingprep.com/developer/docs/is-the-market-open-api) | unknown | low | Utility endpoint, confirm entitlement. |
| SEC/filings/insider | [SEC filings](https://site.financialmodelingprep.com/developer/docs/sec-filings-api) | confirmed | medium | Tested `sec-filings-search/symbol?symbol=AAPL&from=2024-01-01&to=2024-03-01`; implemented as `filings sec`. |
| SEC/filings/insider | [SEC RSS feeds](https://site.financialmodelingprep.com/developer/docs/sec-rss-feeds-api) | unknown | low | Verify before exposing. |
| SEC/filings/insider | [All SEC RSS feeds](https://site.financialmodelingprep.com/developer/docs/sec-all-rss-feeds-api) | unknown | low | Verify before exposing. |
| SEC/filings/insider | [RSS feed 8-K](https://site.financialmodelingprep.com/developer/docs/rss-feed-8k-api) | unknown | low | Verify before exposing. |
| SEC/filings/insider | [Insider trading](https://site.financialmodelingprep.com/developer/docs/stock-insider-trading-api) | unknown | medium | Verify before exposing. |
| SEC/filings/insider | [Insider trading RSS](https://site.financialmodelingprep.com/developer/docs/insider-trading-rss-feed-api) | unknown | low | Verify before exposing. |
| SEC/filings/insider | [Senate trading](https://site.financialmodelingprep.com/developer/docs/senate-trading-api) | unknown | low | Verify before exposing. |
| SEC/filings/insider | [Senate disclosure](https://site.financialmodelingprep.com/developer/docs/senate-disclosure-api) | unknown | low | Verify before exposing. |
| SEC/filings/insider | [House disclosure](https://site.financialmodelingprep.com/developer/docs/house-disclosure-api) | unknown | low | Verify before exposing. |
| SEC/filings/insider | [Beneficial ownership](https://site.financialmodelingprep.com/developer/docs/acquisition-of-beneficial-ownership) | unknown | low | Verify before exposing. |
| SEC/filings/insider | [CIK mapper](https://site.financialmodelingprep.com/developer/docs/cik-mapper-insider-trading) | unknown | low | Verify before exposing. |
| ETFs/funds/13F | [ETF list](https://site.financialmodelingprep.com/developer/docs/etf-list-api) | unknown | low | ETF metadata may differ from ETF holdings. |
| ETFs/funds/13F | [ETF holders](https://site.financialmodelingprep.com/developer/docs/etf-holders-api) | denied | low | Tested `etf/holdings?symbol=SPY`; HTTP 402 subscription restriction. |
| ETFs/funds/13F | [Historical ETF holdings](https://site.financialmodelingprep.com/developer/docs/historical-etf-holdings-api) | excluded | low | ETF holdings are excluded by pricing. |
| ETFs/funds/13F | [ETF holdings dates](https://site.financialmodelingprep.com/developer/docs/historical-etf-holdings-available-dates-api) | excluded | low | ETF holdings are excluded by pricing. |
| ETFs/funds/13F | [ETF expense ratio](https://site.financialmodelingprep.com/developer/docs/etf-expense-ratio-api) | unknown | low | Verify before exposing. |
| ETFs/funds/13F | [ETF sector weightings](https://site.financialmodelingprep.com/developer/docs/etf-sector-weightings-api) | excluded | low | ETF holdings/weightings are not Starter-safe. |
| ETFs/funds/13F | [ETF country weightings](https://site.financialmodelingprep.com/developer/docs/etf-country-weightings-api) | excluded | low | ETF holdings/weightings are not Starter-safe. |
| ETFs/funds/13F | [ETF stock exposure](https://site.financialmodelingprep.com/developer/docs/etf-stock-exposure-api) | excluded | low | ETF holdings/weightings are not Starter-safe. |
| ETFs/funds/13F | [Mutual fund holders](https://site.financialmodelingprep.com/developer/docs/mutual-fund-holders-api) | excluded | low | Mutual fund holdings are excluded by pricing. |
| ETFs/funds/13F | [Institutional ownership](https://site.financialmodelingprep.com/developer/docs/institutional-stock-ownership-api) | excluded | low | 13F institutional holdings are excluded by pricing. |
| ETFs/funds/13F | [Form 13F](https://site.financialmodelingprep.com/developer/docs/form-13f-api) | excluded | low | 13F institutional holdings are excluded by pricing. |
| Analyst/ratings/price targets | [Financial estimates](https://site.financialmodelingprep.com/developer/docs/stable/financial-estimates) | inferred | medium | Analyst estimate availability needs API Viewer confirmation. |
| Analyst/ratings/price targets | [Analyst estimates](https://site.financialmodelingprep.com/developer/docs/analyst-estimates-api) | confirmed | medium | Tested `analyst-estimates?symbol=AAPL&period=annual&limit=5`. |
| Analyst/ratings/price targets | [Analyst recommendations](https://site.financialmodelingprep.com/developer/docs/analyst-stock-recommendations-api) | confirmed | medium | Tested `grades?symbol=AAPL`; implemented as `analyst grades`. |
| Analyst/ratings/price targets | [Price target](https://site.financialmodelingprep.com/developer/docs/price-target-api) | inferred | medium | Verify before exposing. |
| Analyst/ratings/price targets | [Price target summary](https://site.financialmodelingprep.com/developer/docs/price-target-summary-api) | confirmed | medium | Tested `price-target-summary?symbol=AAPL`; implemented as `analyst price-target-summary`. |
| Analyst/ratings/price targets | [Price target consensus](https://site.financialmodelingprep.com/developer/docs/price-target-consensus-api) | confirmed | medium | Tested `price-target-consensus?symbol=AAPL`; implemented as `analyst price-target-consensus`. |
| Analyst/ratings/price targets | [Upgrades and downgrades](https://site.financialmodelingprep.com/developer/docs/upgrades-and-downgrades-api) | inferred | medium | Verify before exposing. |
| Analyst/ratings/price targets | [Company rating](https://site.financialmodelingprep.com/developer/docs/companies-rating-free-api) | confirmed | medium | Tested `grades-consensus?symbol=AAPL`; implemented as `company rating`. |
| Analyst/ratings/price targets | [Historical rating](https://site.financialmodelingprep.com/developer/docs/historical-rating-company-information) | inferred | low | Verify before exposing. |
| Technicals | [Intraday SMA](https://site.financialmodelingprep.com/developer/docs/technical-intraday-sma) | confirmed | low | Tested daily `technical-indicators/sma?symbol=AAPL&periodLength=10&timeframe=1day`; implemented as `technical sma`; intraday access still needs testing. |
| Technicals | [Intraday EMA](https://site.financialmodelingprep.com/developer/docs/technical-intraday-ema) | excluded | low | Technical indicators are excluded by pricing. |
| Technicals | [Intraday RSI](https://site.financialmodelingprep.com/developer/docs/technical-intraday-rsi) | excluded | low | Technical indicators are excluded by pricing. |
| Technicals | [Intraday ADX](https://site.financialmodelingprep.com/developer/docs/technical-intraday-adx) | excluded | low | Technical indicators are excluded by pricing. |
| Technicals | [Intraday technicals](https://site.financialmodelingprep.com/developer/docs/technicals-intraday-api) | excluded | low | Technical indicators are excluded by pricing. |
| Bulk/batch/websocket | [Companies batch request](https://site.financialmodelingprep.com/developer/docs/companies-batch-request-free-api) | excluded | low | Bulk/batch delivery is excluded by pricing. |
| Bulk/batch/websocket | [Batch EOD prices](https://site.financialmodelingprep.com/developer/docs/batch-eod-prices) | excluded | low | Bulk/batch delivery is excluded by pricing. |
| Bulk/batch/websocket | [Bulk endpoint](https://site.financialmodelingprep.com/developer/docs/bulk-endpoint-api) | excluded | low | Bulk/batch delivery is excluded by pricing. |
| Bulk/batch/websocket | [Bulk balance statements](https://site.financialmodelingprep.com/developer/docs/bulk-balance-statemenents) | excluded | low | Bulk/batch delivery is excluded by pricing. |
| Bulk/batch/websocket | [Bulk cash flow statements](https://site.financialmodelingprep.com/developer/docs/bulk-cashflow-statemenents) | excluded | low | Bulk/batch delivery is excluded by pricing. |
| Bulk/batch/websocket | [Bulk ratios](https://site.financialmodelingprep.com/developer/docs/bulk-ratios) | excluded | low | Bulk/batch delivery is excluded by pricing. |
| Bulk/batch/websocket | [Bulk key metrics](https://site.financialmodelingprep.com/developer/docs/bulk-key-metrics) | excluded | low | Bulk/batch delivery is excluded by pricing. |
| Bulk/batch/websocket | [Bulk financial growth](https://site.financialmodelingprep.com/developer/docs/bulk-financial-growth) | excluded | low | Bulk/batch delivery is excluded by pricing. |
| Bulk/batch/websocket | [Bulk ETF holders](https://site.financialmodelingprep.com/developer/docs/bulk-etf-holders) | excluded | low | Bulk and ETF holdings are excluded by pricing. |
| Bulk/batch/websocket | [Batch quote](https://site.financialmodelingprep.com/developer/docs/batch-quote-quote) | excluded | low | Bulk/batch delivery is excluded by pricing. |
| Bulk/batch/websocket | [Batch trade](https://site.financialmodelingprep.com/developer/docs/batch-trade-quote) | excluded | low | Bulk/batch delivery is excluded by pricing. |
| Bulk/batch/websocket | [Websocket](https://site.financialmodelingprep.com/developer/docs/websocket-api) | unknown | low | Not clearly mapped to Starter. |
| Bulk/batch/websocket | [Crypto websocket](https://site.financialmodelingprep.com/developer/docs/crypto-websocket) | unknown | low | Not clearly mapped to Starter. |
| Bulk/batch/websocket | [Forex websocket](https://site.financialmodelingprep.com/developer/docs/forex-websocket) | unknown | low | Not clearly mapped to Starter. |
| Economics | [Economic indicator](https://site.financialmodelingprep.com/developer/docs/economic-indicator-api) | unknown | low | Not clearly mapped to Starter. |
| Economics | [Economic calendar](https://site.financialmodelingprep.com/developer/docs/economic-calendar-api) | excluded | low | Corporate/economic calendar access is not Starter-safe from public pricing. |
| Economics | [Treasury rates](https://site.financialmodelingprep.com/developer/docs/treasury-rates-api) | confirmed | low | Tested `treasury-rates?from=2025-01-01&to=2025-01-31`; implemented as `rates treasury`. |
| Economics | [Market risk premium](https://site.financialmodelingprep.com/developer/docs/market-risk-premium-api) | unknown | low | Not clearly mapped to Starter. |

## Recommended first CLI surface

Start with endpoints marked `explicit` and `high` priority:

| Command idea | Endpoint area | Why first |
| --- | --- | --- |
| `fmp-agent search <query>` | Symbol lookup | Needed for discovery. |
| `fmp-agent company profile <symbol>` | Company core information | Simple reference data. |
| `fmp-agent market quote <symbol>` | Stock market quote | Common CLI use case. |
| `fmp-agent market historical <symbol>` | Historical stock data or daily chart | Explicit Starter category. |
| `fmp-agent fundamentals income-statement <symbol>` | Income statement | Explicit annual fundamentals category. |
| `fmp-agent fundamentals balance-sheet <symbol>` | Balance sheet | Explicit annual fundamentals category. |
| `fmp-agent fundamentals cash-flow <symbol>` | Cash flow | Explicit annual fundamentals category. |
| `fmp-agent fundamentals ratios <symbol>` | Financial ratios | Explicit annual ratios category. |
| `fmp-agent fundamentals metrics <symbol>` | Key metrics | Explicit annual fundamentals category. |
| `fmp-agent news stock <symbol>` | Stock news | Explicit news category. |

## Verification plan

Before implementing an endpoint as supported:

1. Open the authenticated FMP API Viewer for the account.
2. Test the endpoint with the account API key.
3. Record the result as `confirmed` or `denied` in this file.
4. Capture required parameters and a representative response shape.
5. Keep CLI output as compact JSON only. Do not add table, CSV, pretty-printing, filtering, or other output-shaping options.

## Public source URLs

- https://site.financialmodelingprep.com/pricing-plans
- https://site.financialmodelingprep.com/developer/docs/pricing
- https://site.financialmodelingprep.com/developer/docs/dashboard
- https://site.financialmodelingprep.com/developer/docs/quickstart
- https://site.financialmodelingprep.com/faqs
- https://site.financialmodelingprep.com/developer/docs/changelog
- https://site.financialmodelingprep.com/sitemap.xml

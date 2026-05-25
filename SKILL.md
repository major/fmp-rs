# fmp-agent CLI command reference

Use this as a compact command reference for the unofficial `fmp-agent` CLI. The project is not affiliated with, endorsed by, or sponsored by Financial Modeling Prep.

## Install

```bash
cargo install rusty-fmp --locked
```

GitHub releases also provide cargo-dist archives and shell or PowerShell installers for supported platforms.

## Global form

```bash
fmp-agent [OPTIONS] <COMMAND>
```

Global options:

- `--api-key <API_KEY>`: FMP API key. Prefer `FMP_API_KEY` or local `.env`.
- `--base-url <BASE_URL>`: FMP stable API base URL. Defaults to `https://financialmodelingprep.com/stable/`; can also use `FMP_BASE_URL`.
- `-v`, `--verbose`: increase log verbosity. Pass once for INFO, twice (`-vv`) for DEBUG, three times (`-vvv`) for TRACE. Log output goes to stderr.
- `-h`, `--help`: print help.
- `-V`, `--version`: print version.

Running `fmp-agent` without a command prints this generic help text.

Successful command responses are the raw FMP JSON payload on one line. Normal command output goes to stdout, and runtime errors are JSON on stderr. Help and version output are human-readable text. The CLI does not provide output formatting or filtering options.

## Commands

The visible command surface is flat and domain-prefixed so agents and humans can use one command token per endpoint. Old grouped forms such as `market quote` and legacy aliases such as `quote` are rejected.

### Discovery

```bash
fmp-agent search <QUERY>
```

Search for a tradable symbol by ticker or company name.

### Company

```bash
fmp-agent company-profile <SYMBOL>
fmp-agent company-executives <SYMBOL>
fmp-agent company-peers <SYMBOL>
fmp-agent company-financial-scores <SYMBOL>
fmp-agent company-share-float <SYMBOL>
fmp-agent company-rating <SYMBOL>
fmp-agent company-historical-rating <SYMBOL> [--limit <LIMIT>]
```

Company commands cover profile/reference data, key executives, peer companies, financial scores, share float, rating consensus, and historical ratings.

### Market

```bash
fmp-agent etf-holdings <SYMBOL>
fmp-agent market-quote <SYMBOL>
fmp-agent market-historical <SYMBOL> [--from <FROM>] [--to <TO>]
fmp-agent market-dividends <SYMBOL>
fmp-agent market-splits <SYMBOL>
fmp-agent market-price-change <SYMBOL>
fmp-agent market-stock-list
```

Market commands cover ETF holdings, quotes, end-of-day price bars, dividends, splits, price change percentages, and supported stock symbols. Date ranges use inclusive `YYYY-MM-DD` values. `etf-holdings` is intentionally exposed even though Starter accounts return a subscription error, so callers can exercise the structured API-error path.

### Crypto and forex

```bash
fmp-agent crypto-list
fmp-agent crypto-quote <SYMBOL>
fmp-agent crypto-historical <SYMBOL> [--from <FROM>] [--to <TO>]
fmp-agent forex-quote <SYMBOL>
fmp-agent forex-historical <SYMBOL> [--from <FROM>] [--to <TO>]
```

Crypto and forex commands cover supported cryptocurrency symbols, full quotes, currency exchange quotes, and end-of-day price bars. Date ranges use inclusive `YYYY-MM-DD` values.

### Fundamentals

```bash
fmp-agent fundamentals-income-statement <SYMBOL> [--limit <LIMIT>]
fmp-agent fundamentals-income-statement-as-reported <SYMBOL> [--limit <LIMIT>]
fmp-agent fundamentals-balance-sheet <SYMBOL> [--limit <LIMIT>]
fmp-agent fundamentals-cash-flow <SYMBOL> [--limit <LIMIT>]
fmp-agent fundamentals-ratios <SYMBOL> [--limit <LIMIT>]
fmp-agent fundamentals-metrics <SYMBOL> [--limit <LIMIT>]
fmp-agent fundamentals-income-statement-growth <SYMBOL> [--limit <LIMIT>]
fmp-agent fundamentals-balance-sheet-growth <SYMBOL> [--limit <LIMIT>]
fmp-agent fundamentals-cash-flow-growth <SYMBOL> [--limit <LIMIT>]
fmp-agent fundamentals-enterprise-values <SYMBOL> [--limit <LIMIT>]
fmp-agent fundamentals-analyst-estimates <SYMBOL> [--limit <LIMIT>]
fmp-agent fundamentals-report-dates <SYMBOL>
fmp-agent fundamentals-annual-report-form <SYMBOL> --year <YEAR> [--period <PERIOD>]
```

Fundamentals commands return annual statement, ratio, metric, growth, enterprise value, analyst estimate, financial report date, and annual report form JSON. `--limit` sets the maximum annual rows returned for statement-like endpoints. Annual report forms default `--period` to `FY`.

### Analyst

```bash
fmp-agent analyst-price-target-consensus <SYMBOL>
fmp-agent analyst-price-target-summary <SYMBOL>
fmp-agent analyst-grades <SYMBOL>
```

Analyst commands cover price target consensus, price target summary, and analyst grade actions for a symbol.

### Calendar, rates, technicals, filings, and news

```bash
fmp-agent earnings-calendar [--from <FROM>] [--to <TO>]
fmp-agent treasury-rates [--from <FROM>] [--to <TO>]
fmp-agent economic-indicators <NAME> [--from <FROM>] [--to <TO>]
fmp-agent technical-sma <SYMBOL> [--period-length <PERIOD_LENGTH>] [--timeframe <TIMEFRAME>]
fmp-agent sec-filings <SYMBOL> [--from <FROM>] [--to <TO>]
fmp-agent insider-trading-latest [--page <PAGE>] [--limit <LIMIT>]
fmp-agent news-stock <SYMBOL> [--limit <LIMIT>]
fmp-agent news-general [--page <PAGE>] [--limit <LIMIT>]
fmp-agent news-articles [--page <PAGE>] [--limit <LIMIT>]
fmp-agent news-forex [--page <PAGE>] [--limit <LIMIT>]
fmp-agent news-crypto [--page <PAGE>] [--limit <LIMIT>]
```

Technical SMA defaults are `--period-length 10` and `--timeframe 1day`. SEC filings default `--from` to 90 days ago when omitted. Economic indicators use FMP indicator names such as `GDP`. Stock news `--limit` sets the maximum news items returned. Paginated news and insider trading commands default to page 0 and limit 10 when omitted.

## Examples

```bash
FMP_API_KEY=your-key fmp-agent market-quote AAPL
fmp-agent market-historical AAPL --from 2025-01-01 --to 2025-01-31
fmp-agent company-executives AAPL
fmp-agent company-peers AAPL
fmp-agent etf-holdings SPY
fmp-agent market-stock-list
fmp-agent market-dividends AAPL
fmp-agent market-splits AAPL
fmp-agent earnings-calendar --from 2026-01-01 --to 2026-01-31
fmp-agent treasury-rates --from 2025-01-01 --to 2025-01-31
fmp-agent crypto-list
fmp-agent crypto-quote BTCUSD
fmp-agent crypto-historical BTCUSD --from 2025-01-01 --to 2025-01-03
fmp-agent forex-quote EURUSD
fmp-agent forex-historical EURUSD --from 2025-01-01 --to 2025-01-03
fmp-agent technical-sma AAPL --period-length 10 --timeframe 1day
fmp-agent market-price-change AAPL
fmp-agent sec-filings AAPL --from 2024-01-01 --to 2024-03-01
fmp-agent economic-indicators GDP --from 2025-01-01 --to 2025-12-31
fmp-agent fundamentals-income-statement AAPL --limit 5
fmp-agent fundamentals-income-statement-as-reported AAPL --limit 5
fmp-agent fundamentals-balance-sheet AAPL --limit 5
fmp-agent fundamentals-cash-flow AAPL --limit 5
fmp-agent fundamentals-income-statement-growth AAPL --limit 5
fmp-agent fundamentals-balance-sheet-growth AAPL --limit 5
fmp-agent fundamentals-cash-flow-growth AAPL --limit 5
fmp-agent fundamentals-enterprise-values AAPL --limit 5
fmp-agent company-financial-scores AAPL
fmp-agent company-share-float AAPL
fmp-agent company-rating AAPL
fmp-agent company-historical-rating AAPL --limit 5
fmp-agent fundamentals-analyst-estimates AAPL --limit 5
fmp-agent fundamentals-report-dates AAPL
fmp-agent fundamentals-annual-report-form AAPL --year 2022
fmp-agent analyst-price-target-consensus AAPL
fmp-agent analyst-price-target-summary AAPL
fmp-agent analyst-grades AAPL
fmp-agent insider-trading-latest --page 0 --limit 10
fmp-agent news-stock AAPL --limit 10
fmp-agent news-general --page 0 --limit 10
fmp-agent news-articles --page 0 --limit 10
fmp-agent news-forex --page 0 --limit 10
fmp-agent news-crypto --page 0 --limit 10
```

In the repo, use `cargo run -- <COMMAND>` for the same arguments before installing or after cleaning the build.

## Help commands

```bash
fmp-agent --help
fmp-agent <COMMAND> --help
```

## Development

```bash
make check
make coverage
make patch-coverage
make audit
```

`make check` runs formatting, clippy, tests, and docs for both supported feature shapes: the default CLI build and the library-only `--no-default-features` build. GitHub CI mirrors these checks and verifies MSRV 1.95. The integration tests check that the README library dependency example stays aligned with `Cargo.toml`. Keep command help text in `src/cli/help.rs` so `--help`, release-generated man pages, and command reference updates stay in sync.

`make coverage` enforces 90 percent line coverage with `cargo llvm-cov`. Before opening a PR, run `make patch-coverage` to generate `lcov.info` and verify changed lines against `main` with `diff-cover`, matching the Codecov patch gate. Use `PATCH_COVERAGE_BASE=<branch>` for non-main bases or `DIFF_COVER='uvx diff-cover'` when needed.

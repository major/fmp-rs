# fmp-agent CLI command reference

Use this as a compact command reference for the unofficial `fmp-agent` CLI. The project is not affiliated with, endorsed by, or sponsored by Financial Modeling Prep.

## Global form

```bash
fmp-agent [OPTIONS] <COMMAND>
```

Global options:

- `--api-key <API_KEY>`: FMP API key. Prefer `FMP_API_KEY` or local `.env`.
- `--base-url <BASE_URL>`: FMP stable API base URL. Defaults to `https://financialmodelingprep.com/stable/`; can also use `FMP_BASE_URL`.
- `-h`, `--help`: print help.
- `-V`, `--version`: print version.

Running `fmp-agent` without a command prints this generic help text.

Output is compact JSON on one line. Normal output goes to stdout. Errors are JSON on stderr. The CLI does not provide output formatting or filtering options.

## Commands

The visible command surface is grouped by domain so agents and humans can scan the hierarchy quickly. Legacy flat commands such as `quote`, `historical`, and `income-statement` still parse for compatibility, but new usage should prefer the grouped form.

### Discovery

```bash
fmp-agent search <QUERY>
```

Search for a tradable symbol by ticker or company name.

### Company

```bash
fmp-agent company profile <SYMBOL>
fmp-agent company executives <SYMBOL>
fmp-agent company peers <SYMBOL>
fmp-agent company stats <SYMBOL>
fmp-agent company financial-scores <SYMBOL>
fmp-agent company share-float <SYMBOL>
fmp-agent company rating <SYMBOL>
```

Company commands cover profile/reference data, key executives, peer companies, profile-backed key statistics, financial scores, share float, and rating consensus.

### Market

```bash
fmp-agent market quote <SYMBOL>
fmp-agent market historical <SYMBOL> [--from <FROM>] [--to <TO>]
fmp-agent market daily-chart <SYMBOL> [--from <FROM>] [--to <TO>]
fmp-agent market dividends <SYMBOL>
fmp-agent market splits <SYMBOL>
fmp-agent market price-change <SYMBOL>
```

Market commands cover quotes, end-of-day price bars, daily chart aliases, dividends, splits, and price change percentages. Date ranges use inclusive `YYYY-MM-DD` values.

### Fundamentals

```bash
fmp-agent fundamentals income-statement <SYMBOL> [--limit <LIMIT>]
fmp-agent fundamentals income-statement-as-reported <SYMBOL> [--limit <LIMIT>]
fmp-agent fundamentals balance-sheet <SYMBOL> [--limit <LIMIT>]
fmp-agent fundamentals cash-flow <SYMBOL> [--limit <LIMIT>]
fmp-agent fundamentals ratios <SYMBOL> [--limit <LIMIT>]
fmp-agent fundamentals metrics <SYMBOL> [--limit <LIMIT>]
fmp-agent fundamentals income-statement-growth <SYMBOL> [--limit <LIMIT>]
fmp-agent fundamentals balance-sheet-growth <SYMBOL> [--limit <LIMIT>]
fmp-agent fundamentals cash-flow-growth <SYMBOL> [--limit <LIMIT>]
fmp-agent fundamentals enterprise-values <SYMBOL> [--limit <LIMIT>]
fmp-agent fundamentals analyst-estimates <SYMBOL> [--limit <LIMIT>]
fmp-agent fundamentals report-dates <SYMBOL>
```

Fundamentals commands return annual statement, ratio, metric, growth, enterprise value, analyst estimate, and financial report date rows. `--limit` sets the maximum annual rows returned for statement-like endpoints.

### Analyst

```bash
fmp-agent analyst price-target-consensus <SYMBOL>
fmp-agent analyst price-target-summary <SYMBOL>
fmp-agent analyst grades <SYMBOL>
```

Analyst commands cover price target consensus, price target summary, and analyst grade actions for a symbol.

### Calendar, rates, technicals, filings, and news

```bash
fmp-agent calendar earnings [--from <FROM>] [--to <TO>]
fmp-agent rates treasury [--from <FROM>] [--to <TO>]
fmp-agent technical sma <SYMBOL> [--period-length <PERIOD_LENGTH>] [--timeframe <TIMEFRAME>]
fmp-agent filings sec <SYMBOL> [--from <FROM>] [--to <TO>]
fmp-agent news stock <SYMBOL> [--limit <LIMIT>]
fmp-agent news general [--page <PAGE>] [--limit <LIMIT>]
fmp-agent news articles [--page <PAGE>] [--limit <LIMIT>]
fmp-agent news forex [--page <PAGE>] [--limit <LIMIT>]
fmp-agent news crypto [--page <PAGE>] [--limit <LIMIT>]
```

Technical SMA defaults are `--period-length 10` and `--timeframe 1day`. Stock news `--limit` sets the maximum news items returned. Paginated news commands default to page 0 and limit 10 when omitted.

## Examples

```bash
FMP_API_KEY=your-key fmp-agent market quote AAPL
fmp-agent market historical AAPL --from 2025-01-01 --to 2025-01-31
fmp-agent market daily-chart AAPL --from 2025-01-01 --to 2025-01-31
fmp-agent company executives AAPL
fmp-agent company stats AAPL
fmp-agent company peers AAPL
fmp-agent market dividends AAPL
fmp-agent market splits AAPL
fmp-agent calendar earnings --from 2026-01-01 --to 2026-01-31
fmp-agent rates treasury --from 2025-01-01 --to 2025-01-31
fmp-agent technical sma AAPL --period-length 10 --timeframe 1day
fmp-agent market price-change AAPL
fmp-agent filings sec AAPL --from 2024-01-01 --to 2024-03-01
fmp-agent fundamentals income-statement AAPL --limit 5
fmp-agent fundamentals income-statement-as-reported AAPL --limit 5
fmp-agent fundamentals balance-sheet AAPL --limit 5
fmp-agent fundamentals cash-flow AAPL --limit 5
fmp-agent fundamentals income-statement-growth AAPL --limit 5
fmp-agent fundamentals balance-sheet-growth AAPL --limit 5
fmp-agent fundamentals cash-flow-growth AAPL --limit 5
fmp-agent fundamentals enterprise-values AAPL --limit 5
fmp-agent company financial-scores AAPL
fmp-agent company share-float AAPL
fmp-agent company rating AAPL
fmp-agent fundamentals analyst-estimates AAPL --limit 5
fmp-agent fundamentals report-dates AAPL
fmp-agent analyst price-target-consensus AAPL
fmp-agent analyst price-target-summary AAPL
fmp-agent analyst grades AAPL
fmp-agent news stock AAPL --limit 10
fmp-agent news general --page 0 --limit 10
fmp-agent news articles --page 0 --limit 10
fmp-agent news forex --page 0 --limit 10
fmp-agent news crypto --page 0 --limit 10
```

In the repo, use `cargo run -- <COMMAND>` for the same arguments before installing or after cleaning the build.

## Help commands

```bash
fmp-agent --help
fmp-agent <GROUP> --help
fmp-agent <GROUP> <COMMAND> --help
```

## Development

```bash
make check
make coverage
make patch-coverage
make audit
```

`make check` runs formatting, clippy, tests, and docs for both supported feature shapes: the default CLI build and the library-only `--no-default-features` build. GitHub CI mirrors these checks and verifies MSRV 1.95.

`make coverage` enforces 90 percent line coverage with `cargo llvm-cov`. Before opening a PR, run `make patch-coverage` to generate `lcov.info` and verify changed lines against `main` with `diff-cover`, matching the Codecov patch gate. Use `PATCH_COVERAGE_BASE=<branch>` for non-main bases or `DIFF_COVER='uvx diff-cover'` when needed.

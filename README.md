# fmp-agent

[![Crates.io](https://img.shields.io/crates/v/rusty-fmp.svg)](https://crates.io/crates/rusty-fmp)
[![Docs.rs](https://docs.rs/rusty-fmp/badge.svg)](https://docs.rs/rusty-fmp)
[![CI](https://github.com/major/fmp-rs/actions/workflows/ci.yml/badge.svg)](https://github.com/major/fmp-rs/actions/workflows/ci.yml)
[![MSRV](https://img.shields.io/badge/MSRV-1.95-blue.svg)](https://github.com/major/fmp-rs/blob/main/Cargo.toml)
[![License](https://img.shields.io/crates/l/rusty-fmp.svg)](https://crates.io/crates/rusty-fmp)

`fmp-agent` is a Rust CLI for a Starter-plan Financial Modeling Prep account. It calls the FMP stable API and returns raw JSON payloads for shell pipelines.

This project is unofficial and is not affiliated with, endorsed by, or sponsored by Financial Modeling Prep.

## Install and setup

Install the CLI from crates.io with Cargo:

```bash
cargo install rusty-fmp --locked
```

GitHub releases also provide cargo-dist archives and shell or PowerShell installers for supported platforms.

For local development, install a Rust toolchain with Rust 1.95 or newer, then provide an API key with either `FMP_API_KEY` in the environment or a local `.env` file.

```bash
FMP_API_KEY=your-key cargo run -- market quote AAPL
```

`FMP_BASE_URL` can override the default `https://financialmodelingprep.com/stable/` base URL for proxies or tests.

## Migration from 0.3.x

0.4.0 restructures the CLI into a git-like grouped command tree. Commands now take the form `fmp-agent <group> <subcommand>` instead of a single flat hyphenated verb.

Old flat commands are no longer accepted. Use the grouped form instead:

```bash
# 0.3.x (no longer works)
fmp-agent market-quote AAPL
fmp-agent company-profile AAPL

# 0.4.0 grouped form
fmp-agent market quote AAPL
fmp-agent company profile AAPL
```

Four single-token aliases are preserved for ergonomics: `quote` (market quote), `historical` (market historical), `profile` (company profile), and `earnings` (calendar earnings). Run `fmp-agent commands` to list all available leaf commands, `fmp-agent completions <shell>` for bash/zsh/fish/powershell completions, or `fmp-agent schema` for machine-readable JSON metadata. Everything else requires the two-level `<group> <subcommand>` form.

## Commands

See [`SKILL.md`](SKILL.md) for the full command reference and examples. Commands follow a two-level grouped structure: a group name followed by a subcommand verb, such as `company profile`, `market quote`, `fundamentals income-statement`, `macro economic-indicators`, and `news stock`. The 13 groups are: `company`, `market`, `fundamentals`, `analyst`, `insider`, `calendar`, `macro`, `technical`, `sec`, `etf`, `crypto`, `forex`, and `news`.

In the repo, use `cargo run -- <GROUP> <SUBCOMMAND>` with the same arguments before installing or after cleaning the build. After `cargo build`, run the built binary as `target/debug/fmp-agent`, for example `target/debug/fmp-agent market quote AAPL`.

Successful command responses are the raw FMP JSON payload on one line for shell pipelines, and runtime errors are JSON on stderr. Help and version output are human-readable text. The CLI does not provide output formatting or filtering options.

By default, empty JSON arrays from FMP are treated as successful raw API responses. This preserves upstream semantics because an empty array can mean several valid things, such as no rows for a date range, no news in the requested window, or a symbol that FMP does not recognize. For symbol lookup commands where an empty result should stop automation, pass `--strict-empty`; the CLI exits 7 with `empty_result` on stderr and suggests `fmp-agent search <SYMBOL>`.

Pass `--verbose` / `-v` for INFO logs, `-vv` for DEBUG, or `-vvv` for TRACE. Log output goes to stderr and does not appear without the flag. The `RUST_LOG` environment variable can also control log level.

Running `fmp-agent` without a command prints the generic help text.

### Schema introspection

`fmp-agent schema` dumps CLI metadata as JSON without contacting the FMP API and without requiring `FMP_API_KEY`. The output is experimental and may change between releases. Fields include `schema_version` (currently `3`), `binary`, `version`, and `commands` (a flat array of leaf command entries). Each leaf includes `path`, `aliases`, `preferred_path`, `api_key_required`, `about`, `long_about`, and `args`. Argument entries include `kind`, `required`, `default`, `value_name`, the exact `long` and `short` flag spellings, a `parser` hint (`string`, `integer`, `bool`, `enum`, or `count`), `possible_values` for enum-like arguments, and a `multi_value` flag.

```bash
fmp-agent schema | jq '.commands | length'
```

Example leaf shape:

```json
{
  "path": ["market", "quote"],
  "aliases": ["quote"],
  "preferred_path": "market quote",
  "api_key_required": true,
  "args": [
    {
      "name": "symbol",
      "kind": "positional",
      "required": true,
      "default": null,
      "value_name": "SYMBOL",
      "long": null,
      "short": null,
      "parser": { "hint": "string" },
      "possible_values": null,
      "multi_value": false
    }
  ]
}
```

### Agent and tool-calling integration

When integrating `fmp-agent` into an LLM agent or tool-calling pipeline, use these discovery commands first (none require an API key):

```bash
fmp-agent schema              # JSON metadata: all commands, args, defaults, api_key_required
fmp-agent commands            # sorted list of leaf command paths, one per line
fmp-agent completions <shell> # shell completions (bash, elvish, fish, powershell, zsh)
```

**Command chooser by intent:** Use the command path to select the right group.

| You want to... | Use this command |
|---|---|
| Get latest price or historical bars for a ticker | `market quote`, `market batch-quote`, `market historical` |
| Get company info, executives, peers, ratings | `company profile`, `company executives`, `company peers` |
| Get income statements, balance sheets, ratios | `fundamentals income-statement`, `fundamentals balance-sheet` |
| Get analyst price targets or grades | `analyst price-target-consensus`, `analyst grades` |
| Get earnings calendar or economic data | `calendar earnings`, `macro treasury-rates` |
| Get technical indicators (SMA) | `technical sma` |
| Get SEC filings for a ticker | `sec filings` |
| Get crypto or forex quotes | `crypto quote`, `forex quote` |
| Get latest news (stock, general, crypto, forex) | `news stock`, `news general` |
| Search for a ticker by name | `search` |

**Shape-based dispatch:** Commands follow reusable argument shapes (Symbol, SymbolLimit, DateRange, Annual, etc.). The `schema` output includes per-command `args` with exact flag spellings, parser type hints, possible values for enums, and defaults so agents can construct valid invocations without scraping `--help`.

```json
{
  "name": "income-statement",
  "path": ["fundamentals", "income-statement"],
  "args": [
    { "name": "symbol", "kind": "positional", "required": true, "default": null, "long": null, "parser": { "hint": "string" }, "multi_value": false },
    { "name": "limit", "kind": "option", "required": false, "default": 5, "long": "limit", "parser": { "hint": "integer" }, "multi_value": false }
  ],
  "api_key_required": true
}
```

## Using as a library

Other Rust projects (for example an MCP server) can depend on `rusty-fmp` as an API client without pulling in the CLI by disabling default features:

```toml
[dependencies]
rusty-fmp = { default-features = false }
```

This excludes `clap` and `dotenvy` and exposes `FmpClient`, `Endpoint`, `Error`, and `Result`. The `cli` feature (enabled by default) adds the `Cli` parser and the `run` entry point used by the `fmp-agent` binary.

## Exit codes

| Code | Meaning |
|------|---------|
| 0 | Success |
| 2 | Usage error (bad flags, missing arguments, or invalid dates) |
| 3 | Configuration error (missing API key or invalid base URL) |
| 4 | Network error (HTTP request failed) |
| 5 | API error (server returned a non-2xx response, including rate limits) |
| 6 | Parse error (JSON deserialization failed) |
| 7 | Empty symbol result in `--strict-empty` mode |

Exit code 2 is produced at parse time by Clap with human-readable usage text on stderr (bad flags and invalid dates are caught before the CLI reaches runtime validation). Exit codes 3-7 are runtime errors and use structured JSON on stderr: `{"ok":false,"error":{"kind":"<kind>","message":"..."}}`.

HTTP 429 responses use exit code 5 with `error.kind` set to `rate_limited`. Agents should treat that kind as retryable later with backoff, not as a subscription, authentication, or generic API failure. Other non-429 API failures continue to use `api_error`.

## Development

```bash
make check
make coverage
make patch-coverage
make audit
make machete
```

`make check` runs formatting, clippy, tests, and docs for the supported feature shapes: the default CLI build and the library-only `--no-default-features` build. The GitHub CI workflow mirrors those checks across Linux, macOS, and Windows, with an MSRV job pinned to Rust 1.95. The integration tests also check that the library dependency example above matches the package version in `Cargo.toml`, so release bumps do not leave stale copy-paste instructions behind.

`make machete` runs [`cargo machete`](https://github.com/bnjbvr/cargo-machete) to catch unused entries in `Cargo.toml`. Install it once with `cargo install cargo-machete --locked`. The same check runs as the `machete` job in CI. The `[lints.rust]` table in `Cargo.toml` separately denies the `unused` lint group so dead code and unused imports fail `cargo build` even without the `-D warnings` flag.

`make coverage` enforces 90 percent line coverage with `cargo llvm-cov`. `make patch-coverage` generates `lcov.info` and checks changed-line coverage against `main` with `diff-cover`, matching the Codecov patch gate used for GitHub PRs. Override the comparison base with `PATCH_COVERAGE_BASE=<branch>`, lower the local threshold with `PATCH_COVERAGE_FAIL_UNDER=<percent>`, or use `DIFF_COVER='uvx diff-cover'` if `diff-cover` is not installed as a standalone command.

Run a focused test with `cargo test <test_name>`, for example `cargo test search_sends_expected_query`.

Releases are driven by [release-plz](https://release-plz.dev). The `release-plz` workflow keeps a release PR open on `main` that bumps `Cargo.toml` and regenerates `CHANGELOG.md` from conventional commits via git-cliff. Merging that PR pushes a `v<version>` tag, which triggers `release.yml` to build multi-platform binaries with cargo-dist and publish to crates.io via OIDC trusted publishing - no stored API token required. Configure crates.io Trusted Publishing once in the crate settings, pointing to `release.yml`.

## Endpoint coverage

Before adding a new CLI endpoint, check `docs/api-inventory.md`. The CLI should expose endpoints only after account access is confirmed there, except for intentionally denied commands such as `etf holdings` that exist to exercise structured API-error handling. The inventory should be updated when new probes are performed.

New endpoints are added by registering an `Endpoint` constant in `src/endpoint.rs`, adding command help text in `src/cli/help.rs`, and dispatching through the shape-based methods on `FmpClient` (`endpoint`, `query`, `by_symbol`, `by_symbol_limit`, `by_symbol_date_range`, `by_date_range`, `by_name_date_range`, `annual`, `annual_report_form`, `technical`, `news`, `paged`) and the matching `run_*` helper in `src/cli/dispatch.rs`. There are no per-endpoint client wrappers.

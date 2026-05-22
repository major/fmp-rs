# Agent notes

Keep `AGENTS.md`, `README.md`, and `SKILL.md` updated with code changes. Stale docs are worse than missing docs in this repo.

## Project shape

- Rust 2024 crate package named `rusty-fmp`, MSRV 1.95 from `Cargo.toml`; it builds the CLI binary as `fmp-agent`.
- The crate has a default `cli` feature that pulls in `clap` and `dotenvy` and enables the `cli` module. Library-only consumers depend with `default-features = false` and get just `FmpClient`, `Endpoint`, `Error`, and `Result`. The `fmp-agent` binary has `required-features = ["cli"]`.
- `src/main.rs` is intentionally thin: load `.env`, parse Clap, call `rusty_fmp::run`, render structured JSON errors to stderr.
- `src/lib.rs` denies missing docs. Always re-exports `FmpClient`, `Endpoint`, `Error`, `Result`. Behind the `cli` feature it also exposes the `cli` module and re-exports `Cli` and `run`.
- `src/endpoint.rs` owns the public `Endpoint` descriptors and shared defaults (`ANNUAL_PERIOD`, `ANNUAL_LIMIT`, `NEWS_LIMIT`, `PAGE`). Add only confirmed paths from `docs/api-inventory.md` there.
- `src/cli/` owns command parsing and stdout JSON shape. `mod.rs` is the public entry point for `Cli` and `run`, `args.rs` owns Clap command/argument definitions, `commands.rs` owns command dispatch, `dispatch.rs` owns shape helpers (`run_endpoint`, `run_query`, `run_by_symbol`, `run_by_symbol_date_range`, `run_by_date_range`, `run_annual`, `run_technical_sma`, `run_news`, `run_paged`), `output.rs` owns compact success rendering, and `tests.rs` keeps CLI parser/dispatch tests. User-facing commands are grouped under domain subcommands (`company`, `market`, `fundamentals`, `analyst`, `calendar`, `rates`, `technical`, `filings`, `crypto`, `forex`, `news`), with hidden legacy flat aliases for compatibility. Successful CLI output is the raw FMP JSON payload on one line and no output formatting or filtering options are available.
- `src/client.rs` exposes shape-based methods on `FmpClient` (`endpoint`, `query`, `by_symbol`, `by_symbol_date_range`, `by_date_range`, `annual`, `technical`, `news`, `paged`). Add new endpoints by adding an `Endpoint` constant and calling the matching shape method; do not add named per-endpoint wrappers. Keep API keys out of errors and test output. The client emits `log` traces at `DEBUG` for each HTTP request (URL with apikey redacted) and response status, and at `WARN` for non-2xx responses.
- `build.rs` generates `man/fmp-agent.1` via `clap_mangen` when the `cli` feature is active; skipped for library-only builds. The `man/` directory is gitignored and included in cargo-dist releases via `dist-workspace.toml`.
- `docs/api-inventory.md` is the source of truth for Starter-plan endpoint access. Confirm or update that file before exposing new endpoints.
- This project is unofficial and not affiliated with Financial Modeling Prep. Do not imply endorsement or sponsorship in docs, CLI text, or errors.

## Commands

- Full local check: `make check`.
- Format: `cargo fmt --check` or `cargo fmt`.
- Lint: `cargo clippy --all-targets --all-features -- -D warnings`.
- Lint library-only build: `cargo clippy --lib --no-default-features -- -D warnings`.
- Test everything: `cargo test`.
- Test supported feature shapes: `cargo test --all-features` and `cargo test --lib --no-default-features`.
- Run one test: `cargo test <test_name>`, for example `cargo test search_sends_expected_query`.
- Build: `cargo build`.
- Docs: `cargo doc --all-features --no-deps` and `cargo doc --no-default-features --no-deps` validate crate docs against `#![deny(missing_docs)]`.
- Coverage: `make coverage` runs `cargo llvm-cov --workspace --all-features --fail-under-lines 90`.
- Patch coverage: `make patch-coverage` generates `lcov.info` and runs `diff-cover` against `PATCH_COVERAGE_BASE ?= main` with `PATCH_COVERAGE_FAIL_UNDER ?= 100`. Use `DIFF_COVER='uvx diff-cover'` when the standalone command is not installed.
- Audit: `make audit` runs `cargo audit`.
- Unused dependencies: `make machete` runs `cargo machete` (install with `cargo install cargo-machete --locked`).
- Local CLI smoke test after building: `FMP_API_KEY=<key> cargo run -- market quote AAPL` or `target/debug/fmp-agent market quote AAPL`; never commit `.env`.

## Automation

- GitHub CI lives in `.github/workflows/ci.yml` and mirrors `make check` across supported feature shapes. Keep the default CLI build and library-only `--no-default-features` checks green.
- Codecov uses `codecov.yml` for 90 percent project coverage and 100 percent patch coverage. The CI coverage job uploads `lcov.info` only when `CODECOV_TOKEN` is available.
- Security audit lives in `.github/workflows/audit.yml` and runs on `Cargo.toml`/`Cargo.lock` changes plus a daily schedule.
- Unused-dependency check runs as the `machete` job in `.github/workflows/ci.yml` via `cargo machete`.
- `Cargo.toml` has a `[lints.rust]` table that denies the `unused` lint group declaratively, so `cargo build` and `cargo check` also reject unused imports, unused variables, and dead code without needing `-D warnings`. Per-item `#[allow(...)]` still works because the group is set with `priority = -1`.
- Release automation uses `release-plz.toml`, `cliff.toml`, `dist-workspace.toml`, `.github/workflows/cd.yml`, and `.github/workflows/release.yml`.
- CodeRabbit uses `.coderabbit.yaml`. Keep its path instructions aligned with this repo's JSON CLI, library-only feature support, and endpoint inventory rules.
- `rust-toolchain.toml` pins Rust 1.95 for local consistency with the MSRV workflow.

## Implementation notes

- Successful CLI output is the raw FMP JSON payload on one line. Failures in `main.rs` use `{ ok: false, error: { kind, message } }` on stderr.
- Clap already reads `FMP_API_KEY` and `FMP_BASE_URL` from the environment, and `main.rs` loads `.env` with `dotenvy` before parsing.
- `FmpClient::with_base_url` exists for tests and proxies. Tests should use `httpmock` rather than the live FMP API.
- Statement-like annual endpoints currently default `limit` to 5; stock and paginated news endpoints default `limit` to 10; paginated endpoints default `page` to 0. SEC filings default `--from` to 90 days ago (via `jiff`) when the user omits it, because the FMP API requires `from` for this endpoint.
- Annual endpoints include statements, as-reported statements, statement growth, enterprise values, and analyst estimates. Symbol-only reference/metric endpoints include profile, key executives, peers, dividends, splits, price change, financial scores, share float, report dates, price target consensus/summary, analyst grades, and rating consensus.
- For new endpoints, preserve the pattern: add an `Endpoint` constant in `src/endpoint.rs`, add a grouped CLI subcommand in `src/cli/args.rs`, and dispatch through the matching shape helper from `src/cli/dispatch.rs`. Avoid bespoke per-endpoint client methods.
- Reuse the shape-specific helpers in `src/client.rs` and `src/cli/dispatch.rs` for no-parameter, symbol-only, symbol plus date range, date-range-only, annual, technical indicator, stock news, and paginated commands. Do not duplicate query assembly.
- Do not add commands for endpoints marked `denied`, `excluded`, `unknown`, or `unconfirmed` in `docs/api-inventory.md` without first verifying account access and updating the inventory.
- The `Error` enum has an `exit_code()` method that returns distinct `ExitCode` values per variant: 0 success, 2 usage error (`MissingArgument`), 3 config error (`MissingApiKey`/`InvalidBaseUrl`), 4 network error (`Http`), 5 API error (`Api`), 6 parse error (`Json`). The exit codes are documented in `--help` via `after_help` on `Cli`.
- The `cli` feature adds `--verbose` / `-v` (INFO), `-vv` (DEBUG), `-vvv` (TRACE) via `clap-verbosity-flag`. `env_logger` is initialized in `main.rs` after CLI parsing using `cli.verbose.log_level_filter()`. Log output goes to stderr. Do not log API key values in any log statement.
- `human_panic::setup_panic!()` is the first call in `main()` to catch panics before `dotenvy` or Clap run.
- Integration tests in `tests/` use `assert_cmd::Command::cargo_bin("fmp-agent")` and `predicates` for assertions; `httpmock` provides mock HTTP servers.
- Run `cargo build --all-features` to generate `man/fmp-agent.1` from `build.rs`. Do NOT commit the generated `man/` directory; it is gitignored.

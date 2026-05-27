# Agent notes

Keep `AGENTS.md`, `README.md`, and `SKILL.md` updated with code changes. Stale docs are worse than missing docs in this repo.

## Project shape

- Rust 2024 crate package named `rusty-fmp`, MSRV 1.95 from `Cargo.toml`; it builds the CLI binary as `fmp-agent`.
- The crate has a default `cli` feature that pulls in `clap` and `dotenvy` and enables the `cli` module. Library-only consumers depend with `default-features = false` and get just `FmpClient`, `Endpoint`, `Error`, and `Result`. The `fmp-agent` binary has `required-features = ["cli"]`.
- `src/main.rs` is intentionally thin: load `.env`, parse Clap, call `rusty_fmp::run`, render structured JSON errors to stderr.
- `src/lib.rs` denies missing docs. Always re-exports `FmpClient`, `Endpoint`, `Error`, `Result`. Behind the `cli` feature it also exposes the `cli` module and re-exports only the documented CLI entry points: `Cli` and `run`.
- `src/endpoint.rs` owns the public `Endpoint` descriptors and shared defaults (`ANNUAL_PERIOD`, `ANNUAL_LIMIT`, `NEWS_LIMIT`, `PAGE`). Add only confirmed paths from `docs/api-inventory.md` there, except intentionally denied commands like `etf-holdings` that exercise structured API-error handling.
- `src/cli/` owns command parsing and stdout JSON shape. `mod.rs` is the public entry point for `Cli` and `run`, `args.rs` owns the top-level `Command` enum with group variants, `help.rs` owns shared user-facing help strings used by Clap, `commands.rs` owns command dispatch, `dispatch.rs` owns shape helpers (`run_endpoint`, `run_query`, `run_by_symbol`, `run_by_symbol_limit`, `run_by_symbol_date_range`, `run_by_date_range`, `run_by_name_date_range`, `run_annual`, `run_annual_report_form`, `run_technical_sma`, `run_news`, `run_paged`), `output.rs` owns compact success rendering, and `tests.rs` keeps CLI parser/dispatch tests. User-facing commands are two-level grouped subcommands: a group name followed by a subcommand verb (e.g., `market quote AAPL`, `company profile AAPL`, `fundamentals income-statement AAPL`, `analyst grades AAPL`, `macro economic-indicators`, `technical sma AAPL`, `news stock AAPL`). Group modules live under `src/cli/groups/<name>.rs`. Four top-level aliases (`quote`, `historical`, `profile`, `earnings`) provide shortcuts to common leaf commands. Successful CLI output is the raw FMP JSON payload on one line and no output formatting or filtering options are available.
- `src/client.rs` exposes shape-based methods on `FmpClient` (`endpoint`, `query`, `by_symbol`, `by_symbol_limit`, `by_symbol_date_range`, `by_date_range`, `by_name_date_range`, `annual`, `annual_report_form`, `technical`, `news`, `paged`). Add new endpoints by adding an `Endpoint` constant and calling the matching shape method; do not add named per-endpoint wrappers. Keep API keys out of errors and test output. The client emits `log` traces at `DEBUG` for each HTTP request (URL with apikey redacted) and response status, and at `WARN` for non-2xx responses.
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
- Release automation is split across release-plz and cargo-dist. `release-plz.toml` plus `.github/workflows/release-plz.yml` (jobs `release-plz-pr` and `release-plz-release`) open and maintain the release PR (version bump in `Cargo.toml`, regenerated `CHANGELOG.md` via `cliff.toml`); merging that PR pushes a `v<version>` tag using `RELEASE_PLZ_TOKEN` so the tag triggers downstream workflows. The tag fires `.github/workflows/release.yml` (cargo-dist), which uses `dist-workspace.toml` to build artifacts, create the GitHub Release, and publish to crates.io via OIDC trusted publishing. `release-plz.toml` sets `publish = false` and `git_release_enable = false` so the two systems do not race. There is no longer a manual `/make-release` step; releases happen entirely by merging the release-plz PR. The end-to-end flow:

```text
   conventional commits land on main
                  |
                  v
   +-----------------------------------+
   | release-plz-pr (push to main)     |
   |  - bumps Cargo.toml version       |
   |  - regenerates CHANGELOG.md       |
   |    via cliff.toml                 |
   +-----------------+-----------------+
                     |
                     v
            release PR opened
            (chore: release v<ver>,
             branch release-plz-*)
                     |
                     v   (human merges)
   +-----------------------------------+
   | release-plz-release               |
   |  - pushes v<ver> annotated tag    |
   |    via RELEASE_PLZ_TOKEN          |
   +-----------------+-----------------+
                     |
                     v   (tag push event)
   +-----------------------------------+
    | release.yml (cargo-dist)          |
    |  - builds binaries (5 targets)    |
    |  - creates GitHub Release         |
    |  - publishes to crates.io via     |
    |    OIDC trusted publishing        |
   +-----------------------------------+
```

- CodeRabbit uses `.coderabbit.yaml`. Keep its path instructions aligned with this repo's JSON CLI, library-only feature support, and endpoint inventory rules.
- `rust-toolchain.toml` pins Rust 1.95 for local consistency with the MSRV workflow.

## Review guidelines

- Codex PR reviews should follow the repository rules above plus the reviewer-focused guidance in `.github/copilot-instructions.md`.
- Treat `.coderabbit.yaml` as CodeRabbit-specific configuration, but keep its path instructions aligned with these rules when review policy changes.

## Implementation notes

- Successful CLI output is the raw FMP JSON payload on one line. Failures in `main.rs` use `{ ok: false, error: { kind, message } }` on stderr.
- Clap already reads `FMP_API_KEY` and `FMP_BASE_URL` from the environment, and `main.rs` loads `.env` with `dotenvy` before parsing.
- `FmpClient::with_base_url` exists for tests and proxies. Tests should use `httpmock` rather than the live FMP API.
- Statement-like annual endpoints currently default `limit` to 5; stock and paginated news endpoints default `limit` to 10; paginated endpoints default `page` to 0. SEC filings default `--from` to 90 days ago (via `jiff`) when the user omits it, because the FMP API requires `from` for this endpoint.
- Annual endpoints include statements, as-reported statements, statement growth, enterprise values, analyst estimates, and annual report form JSON. Symbol-only reference/metric endpoints include profile, key executives, peers, ETF holdings, dividends, splits, price change, financial scores, share float, report dates, price target consensus/summary, analyst grades, and rating consensus. Historical ratings use the symbol plus limit shape.
- For new endpoints, preserve the pattern: add an `Endpoint` constant in `src/endpoint.rs`, add centralized help strings in `src/cli/help.rs`, add a `Cmd` variant in the appropriate group module under `src/cli/groups/<name>.rs`, and dispatch through the matching shape helper from `src/cli/dispatch.rs`. Avoid bespoke per-endpoint client methods.
- Each new endpoint belongs to exactly one group; refer to `docs/api-inventory.md` to confirm Starter-plan access before adding it.
- Reuse the shape-specific helpers in `src/client.rs` and `src/cli/dispatch.rs` for no-parameter, symbol-only, symbol plus limit, symbol plus date range, date-range-only, named date range, annual, annual report form, technical indicator, stock news, and paginated commands. Do not duplicate query assembly.
- Do not add commands for endpoints marked `excluded`, `unknown`, or `unconfirmed` in `docs/api-inventory.md` without first verifying account access and updating the inventory. Denied endpoints may be added only when the command is explicitly intended to test structured subscription-error handling.
- The `Error` enum has an `exit_code()` method that returns distinct `ExitCode` values per variant: 0 success, 2 usage error (`MissingArgument`), 3 config error (`MissingApiKey`/`InvalidBaseUrl`), 4 network error (`Http`), 5 API error (`Api`), 6 parse error (`Json`). The exit codes are documented in `--help` via `after_help` on `Cli`.
- The `cli` feature adds `--verbose` / `-v` (INFO), `-vv` (DEBUG), `-vvv` (TRACE) via a manual `u8` count flag mapped to `log::LevelFilter`. `env_logger` is initialized in `main.rs` after CLI parsing using the mapped filter. Log output goes to stderr. Do not log API key values in any log statement.
- `human_panic::setup_panic!()` is the first call in `main()` to catch panics before `dotenvy` or Clap run.
- Integration tests in `tests/` use `assert_cmd::Command::cargo_bin("fmp-agent")` and `predicates` for assertions; `httpmock` provides mock HTTP servers.
- `tests/readme.rs` guards documentation drift by checking that the README library dependency example uses the current `Cargo.toml` package version.
- `help_heading` on `Subcommand` enum variants is NOT supported in Clap 4.6. The top-level `Commands:` listing in `--help` shows group names only; subcommand help appears when the user runs `fmp-agent <group> --help`. Do NOT add `#[command(help_heading = "...")]` to top-level `Command` enum variants; it compiles but has no effect and creates misleading expectations.
- Static numeric defaults are expressed via Clap `default_value_t` on the struct field, not via prose "Defaults to N when omitted" in help strings. Clap renders `[default: N]` automatically in `--help`. The exception is SEC filings `--from`, which stays as a runtime `jiff` computation in `src/cli/commands.rs` because the default is relative to the current date.
- `fmp-agent schema` dumps CLI metadata as versioned JSON (`schema_version: 2`, experimental). It bypasses the api_key validation block in `src/cli/mod.rs::run()` via an early-return branch that runs before `MissingApiKey` is checked, so `schema` works without any API key configured. Bare-group invocations (a group name with no subcommand) also bypass the api_key check and print the group's help text with exit 0, so users can explore available subcommands without configuring a key.
- When adding a new `Cmd` variant to a group module, you MUST also add it to `tests/cli_drift.rs::COMMANDS` (using the two-level path, e.g., `["market", "quote"]`) and supply both `about` and `long_about` (with an `Examples:` block) in `src/cli/help.rs`. The `cli_drift` test fails if any variant is missing from the constant list or if either help field is absent.

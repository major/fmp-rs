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
FMP_API_KEY=your-key cargo run -- market-quote AAPL
```

`FMP_BASE_URL` can override the default `https://financialmodelingprep.com/stable/` base URL for proxies or tests.

## Commands

See [`SKILL.md`](SKILL.md) for the full command reference and examples. Commands are flat, domain-prefixed verbs such as `company-profile`, `market-quote`, `fundamentals-income-statement`, `economic-indicators`, and `news-stock`. Old grouped commands and legacy aliases are rejected.

In the repo, use `cargo run -- <COMMAND>` with the same arguments before installing or after cleaning the build. After `cargo build`, run the built binary as `target/debug/fmp-agent`, for example `target/debug/fmp-agent market-quote AAPL`.

Successful command responses are the raw FMP JSON payload on one line for shell pipelines, and runtime errors are JSON on stderr. Help and version output are human-readable text. The CLI does not provide output formatting or filtering options.

Pass `--verbose` / `-v` for INFO logs, `-vv` for DEBUG, or `-vvv` for TRACE. Log output goes to stderr and does not appear without the flag. The `RUST_LOG` environment variable can also control log level.

After installation, `man fmp-agent` shows the man page. Build locally with `cargo build --all-features`, which writes `man/fmp-agent.1` (gitignored, not committed).

Running `fmp-agent` without a command prints the generic help text.

## Using as a library

Other Rust projects (for example an MCP server) can depend on `rusty-fmp` as an API client without pulling in the CLI by disabling default features:

```toml
[dependencies]
rusty-fmp = { version = "0.3.0", default-features = false }
```

This excludes `clap` and `dotenvy` and exposes `FmpClient`, `Endpoint`, `Error`, and `Result`. The `cli` feature (enabled by default) adds the `Cli` parser and the `run` entry point used by the `fmp-agent` binary.

## Exit codes

| Code | Meaning |
|------|---------|
| 0 | Success |
| 2 | Usage error (bad flags or arguments) |
| 3 | Configuration error (missing API key or invalid base URL) |
| 4 | Network error (HTTP request failed) |
| 5 | API error (server returned a non-2xx response) |
| 6 | Parse error (JSON deserialization failed) |

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

Releases use a tag-push workflow: run `/make-release` in opencode to bump the version, regenerate the changelog with git-cliff, and push a signed tag. The tag triggers `release.yml`, which builds multi-platform binaries via cargo-dist and publishes to crates.io via OIDC trusted publishing - no stored API token required. Configure crates.io Trusted Publishing once in the crate settings, pointing to `release.yml`.

## Endpoint coverage

Before adding a new CLI endpoint, check `docs/api-inventory.md`. The CLI should expose endpoints only after account access is confirmed there, and the inventory should be updated when new probes are performed.

New endpoints are added by registering an `Endpoint` constant in `src/endpoint.rs` and dispatching through the shape-based methods on `FmpClient` (`endpoint`, `query`, `by_symbol`, `by_symbol_limit`, `by_symbol_date_range`, `by_date_range`, `by_name_date_range`, `annual`, `annual_report_form`, `technical`, `news`, `paged`) and the matching `run_*` helper in `src/cli/dispatch.rs`. There are no per-endpoint client wrappers.

# fmp-agent

[![Crates.io](https://img.shields.io/crates/v/rusty-fmp.svg)](https://crates.io/crates/rusty-fmp)
[![Docs.rs](https://docs.rs/rusty-fmp/badge.svg)](https://docs.rs/rusty-fmp)
[![CI](https://github.com/major/fmp-rs/actions/workflows/ci.yml/badge.svg)](https://github.com/major/fmp-rs/actions/workflows/ci.yml)
[![MSRV](https://img.shields.io/badge/MSRV-1.95-blue.svg)](https://github.com/major/fmp-rs/blob/main/Cargo.toml)
[![License](https://img.shields.io/crates/l/rusty-fmp.svg)](https://crates.io/crates/rusty-fmp)

`fmp-agent` is a Rust CLI for a Starter-plan Financial Modeling Prep account. It calls the FMP stable API and returns predictable JSON for shell pipelines.

This project is unofficial and is not affiliated with, endorsed by, or sponsored by Financial Modeling Prep.

## Setup

Install a Rust toolchain with Rust 1.95 or newer, then provide an API key with either `FMP_API_KEY` in the environment or a local `.env` file.

```bash
FMP_API_KEY=your-key cargo run -- market quote AAPL
```

`FMP_BASE_URL` can override the default `https://financialmodelingprep.com/stable/` base URL for proxies or tests.

## Commands

See [`SKILL.md`](SKILL.md) for the full command reference and examples. Commands are grouped by domain, including `company`, `market`, `fundamentals`, `analyst`, `calendar`, `rates`, `technical`, `filings`, and `news`. Legacy flat commands still parse for compatibility, but new usage should prefer the grouped form.

In the repo, use `cargo run -- <GROUP> <COMMAND>` with the same arguments before installing or after cleaning the build. After `cargo build`, run the built binary as `target/debug/fmp-agent`, for example `target/debug/fmp-agent market quote AAPL`.

Output is always compact JSON on one line for shell pipelines. The CLI does not provide output formatting or filtering options.

## Using as a library

Other Rust projects (for example an MCP server) can depend on `rusty-fmp` as an API client without pulling in the CLI by disabling default features:

```toml
[dependencies]
rusty-fmp = { version = "0.1", default-features = false }
```

This excludes `clap` and `dotenvy` and exposes `FmpClient`, `Endpoint`, `Error`, and `Result`. The `cli` feature (enabled by default) adds the `Cli` parser and the `run` entry point used by the `fmp-agent` binary.

## Development

```bash
make check
make coverage
make patch-coverage
make audit
make machete
```

`make check` runs formatting, clippy, tests, and docs for the supported feature shapes: the default CLI build and the library-only `--no-default-features` build. The GitHub CI workflow mirrors those checks across Linux, macOS, and Windows, with an MSRV job pinned to Rust 1.95.

`make machete` runs [`cargo machete`](https://github.com/bnjbvr/cargo-machete) to catch unused entries in `Cargo.toml`. Install it once with `cargo install cargo-machete --locked`. The same check runs as the `machete` job in CI. The `[lints.rust]` table in `Cargo.toml` separately denies the `unused` lint group so dead code and unused imports fail `cargo build` even without the `-D warnings` flag.

`make coverage` enforces 90 percent line coverage with `cargo llvm-cov`. `make patch-coverage` generates `lcov.info` and checks changed-line coverage against `main` with `diff-cover`, matching the Codecov patch gate used for GitHub PRs. Override the comparison base with `PATCH_COVERAGE_BASE=<branch>`, lower the local threshold with `PATCH_COVERAGE_FAIL_UNDER=<percent>`, or use `DIFF_COVER='uvx diff-cover'` if `diff-cover` is not installed as a standalone command.

Run a focused test with `cargo test <test_name>`, for example `cargo test search_sends_expected_query`.

Releases use release-plz for release PRs and crates.io publishing, git-cliff for changelog generation, and cargo-dist for tagged binary releases. The first crates.io release still needs to be published manually; after that, configure crates.io Trusted Publishing for `.github/workflows/cd.yml`.

## Endpoint coverage

Before adding a new CLI endpoint, check `docs/api-inventory.md`. The CLI should expose endpoints only after account access is confirmed there, and the inventory should be updated when new probes are performed.

New endpoints are added by registering an `Endpoint` constant in `src/endpoint.rs` and dispatching through the shape-based methods on `FmpClient` (`query`, `by_symbol`, `by_symbol_date_range`, `by_date_range`, `annual`, `technical`, `news`, `paged`) and the matching `run_*` helper in `src/cli/dispatch.rs`. There are no per-endpoint client wrappers.

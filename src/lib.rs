//! Library implementation for the `rusty-fmp` crate.
//!
//! The HTTP client (`FmpClient`), endpoint descriptors, and error types are always available
//! so other Rust projects (for example an MCP server) can consume the API without pulling in
//! the CLI. The `cli` module is gated behind the default `cli` feature and exposes only the
//! `Cli` parser type plus `run` entry point used by the bundled `fmp-agent` binary.
//!
//! Library-only consumers should disable default features:
//!
//! ```toml
//! rusty-fmp = { version = "0.4.0", default-features = false }
//! ```
//!
//! This crate emits log records via the [`log`] crate. Callers control output
//! by installing a compatible logger (e.g., `env_logger` in the `cli` feature).

#![deny(missing_docs)]

#[cfg(feature = "cli")]
pub mod cli;
pub mod client;
pub mod endpoint;
pub mod error;

#[cfg(feature = "cli")]
pub use crate::cli::{Cli, run};
pub use crate::client::FmpClient;
pub use crate::endpoint::Endpoint;
pub use crate::error::{Error, Result};

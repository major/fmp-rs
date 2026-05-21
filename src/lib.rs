//! Library implementation for the `fmp` crate.
//!
//! The HTTP client (`FmpClient`), endpoint descriptors, and error types are always available
//! so other Rust projects (for example an MCP server) can consume the API without pulling in
//! the CLI. The `cli` module and its re-exports are gated behind the default `cli` feature and
//! used by the bundled `fmp-agent` binary.
//!
//! Library-only consumers should disable default features:
//!
//! ```toml
//! fmp = { version = "0.1", default-features = false }
//! ```

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

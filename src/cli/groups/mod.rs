//! Per-group nested command modules.
//!
//! Each group exposes a `Cmd` subcommand enum and an async `dispatch` function
//! called from `src/cli/commands.rs::execute`.

pub mod analyst;
pub mod calendar;
pub mod company;
pub mod crypto;
pub mod etf;
pub mod forex;
pub mod fundamentals;
pub mod insider;
pub mod macro_econ;
pub mod market;
pub mod news;
pub mod sec;
pub mod technical;

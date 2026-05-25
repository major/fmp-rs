//! Generates the `fmp-agent` man page for release packaging.

use std::env;
use std::error::Error;
use std::fs::{self, File};
use std::path::PathBuf;

use clap::CommandFactory;
use rusty_fmp::Cli;

fn main() -> Result<(), Box<dyn Error>> {
    let path = env::args_os()
        .nth(1)
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("target/generated-man/man/fmp-agent.1"));

    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }

    let mut file = File::create(path)?;
    clap_mangen::Man::new(Cli::command()).render(&mut file)?;

    Ok(())
}

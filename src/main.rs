use std::io::{self, Write};
use std::process::ExitCode;

use clap::error::ErrorKind;
use clap::{CommandFactory, Parser};
use rusty_fmp::{Cli, run};

#[tokio::main]
async fn main() -> ExitCode {
    human_panic::setup_panic!();
    dotenvy::dotenv().ok();

    let cli = match Cli::try_parse() {
        Ok(cli) => cli,
        Err(error) if error.kind() == ErrorKind::MissingSubcommand && no_cli_args() => {
            let mut command = Cli::command();
            command.set_bin_name("fmp-agent");
            let help = command.render_help();
            if let Err(e) = write!(io::stdout(), "{help}")
                && e.kind() != io::ErrorKind::BrokenPipe
            {
                panic!("failed printing to stdout: {e}");
            }
            return ExitCode::SUCCESS;
        }
        Err(error) => error.exit(),
    };

    let level_filter = match cli.verbose {
        0 => log::LevelFilter::Off,
        1 => log::LevelFilter::Info,
        2 => log::LevelFilter::Debug,
        _ => log::LevelFilter::Trace,
    };
    env_logger::Builder::from_env(
        env_logger::Env::default().default_filter_or(level_filter.to_string()),
    )
    .init();

    match run(cli).await {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            let body = serde_json::json!({
                "ok": false,
                "error": {
                    "kind": error.kind(),
                    "message": error.to_string(),
                }
            });

            eprintln!(
                "{}",
                serde_json::to_string(&body).unwrap_or_else(|_| body.to_string())
            );
            error.exit_code()
        }
    }
}

fn no_cli_args() -> bool {
    std::env::args_os().nth(1).is_none()
}

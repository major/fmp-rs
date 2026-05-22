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
            print!("{}", command.render_help());
            return ExitCode::SUCCESS;
        }
        Err(error) => error.exit(),
    };

    let level_filter = cli.verbose.log_level_filter();
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

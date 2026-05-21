use std::process::ExitCode;

use clap::error::ErrorKind;
use clap::{CommandFactory, Parser};
use rusty_fmp::{Cli, run};

#[tokio::main]
async fn main() -> ExitCode {
    dotenvy::dotenv().ok();

    let cli = match Cli::try_parse() {
        Ok(cli) => cli,
        Err(error)
            if error.kind() == ErrorKind::MissingSubcommand && std::env::args_os().len() == 1 =>
        {
            let mut command = Cli::command();
            command.set_bin_name("fmp-agent");
            print!("{}", command.render_help());
            return ExitCode::SUCCESS;
        }
        Err(error) => error.exit(),
    };

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
            ExitCode::FAILURE
        }
    }
}

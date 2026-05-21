use std::process::ExitCode;

use fmp::{Cli, run};

#[tokio::main]
async fn main() -> ExitCode {
    dotenvy::dotenv().ok();

    let cli = <Cli as clap::Parser>::parse();

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

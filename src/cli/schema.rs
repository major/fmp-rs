//! CLI schema introspection for the `schema` subcommand.

use clap::CommandFactory;
use serde_json::{Value, json};

use super::args::Cli;

/// Builds a JSON snapshot of the CLI surface by introspecting the Clap command tree.
pub(super) fn schema_payload() -> Value {
    let cmd = Cli::command();

    let commands: Vec<Value> = cmd
        .get_subcommands()
        .map(|sub| {
            let name = sub.get_name().to_owned();
            let about = sub.get_about().map(|s| s.to_string());
            let long_about = sub.get_long_about().map(|s| s.to_string());

            let args: Vec<Value> = sub
                .get_arguments()
                .filter(|a| !a.is_hide_set())
                .map(|a| {
                    let kind = if a.is_positional() {
                        "positional"
                    } else if a.get_num_args().is_some_and(|r| r.max_values() > 0) {
                        "option"
                    } else {
                        "flag"
                    };
                    let default_val = a
                        .get_default_values()
                        .first()
                        .and_then(|v| v.to_str())
                        .map(|s| json!(s));
                    let help_text = a.get_help().map(|s| s.to_string());
                    json!({
                        "name": a.get_id().as_str(),
                        "kind": kind,
                        "required": a.is_required_set(),
                        "default": default_val,
                        "help": help_text,
                    })
                })
                .collect();

            json!({
                "name": name,
                "about": about,
                "long_about": long_about,
                "args": args,
            })
        })
        .collect();

    json!({
        "schema_version": 1,
        "binary": "fmp-agent",
        "version": env!("CARGO_PKG_VERSION"),
        "commands": commands,
    })
}

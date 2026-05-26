//! CLI schema introspection for the `schema` subcommand.

use clap::CommandFactory;
use serde_json::{Value, json};

use super::args::Cli;

/// Mapping from top-level alias name to its canonical grouped path.
const ALIAS_PREFERRED: &[(&str, &str)] = &[
    ("quote", "market quote"),
    ("historical", "market historical"),
    ("profile", "company profile"),
    ("earnings", "calendar earnings"),
];

/// Builds a versioned JSON snapshot of the CLI surface by recursively walking the
/// Clap command tree.
///
/// Schema v2 exposes group names, leaf command paths, aliases, preferred canonical
/// paths, and per-command `api_key_required` flags.
pub(super) fn schema_payload() -> Value {
    let cmd = Cli::command();

    let mut groups: Vec<String> = Vec::new();
    let mut commands: Vec<Value> = Vec::new();

    for sub in cmd.get_subcommands() {
        let name = sub.get_name();
        let children: Vec<_> = sub.get_subcommands().collect();

        if children.is_empty() {
            // Top-level leaf: search, schema, or an alias.
            let path = vec![name.to_owned()];
            let preferred = preferred_path_for(name);
            let api_key_required = name != "schema";

            commands.push(leaf_to_json(sub, path, preferred, &[], api_key_required));
        } else {
            // Group: collect group name and emit each child as a leaf.
            groups.push(name.to_owned());

            for child in &children {
                let child_name = child.get_name();
                let path = vec![name.to_owned(), child_name.to_owned()];
                let preferred = format!("{name} {child_name}");
                let aliases = aliases_for(name, child_name);

                commands.push(leaf_to_json(child, path, preferred, &aliases, true));
            }
        }
    }

    json!({
        "schema_version": 2,
        "binary": "fmp-agent",
        "version": env!("CARGO_PKG_VERSION"),
        "groups": groups,
        "commands": commands,
    })
}

/// Serializes a single leaf command into a JSON value with v1-compatible arg specs.
fn leaf_to_json(
    sub: &clap::Command,
    path: Vec<String>,
    preferred_path: String,
    aliases: &[String],
    api_key_required: bool,
) -> Value {
    let name = sub.get_name().to_owned();
    let about = sub.get_about().map(|s| s.to_string());
    let long_about = sub.get_long_about().map(|s| s.to_string());

    let args: Vec<Value> = sub
        .get_arguments()
        .filter(|a| !a.is_hide_set())
        .map(|a| {
            let kind = if a.is_positional() {
                "positional"
            } else {
                "option"
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
        "path": path,
        "about": about,
        "long_about": long_about,
        "args": args,
        "aliases": aliases,
        "preferred_path": preferred_path,
        "api_key_required": api_key_required,
    })
}

/// Returns the preferred canonical path for a top-level command.
///
/// Aliases (quote, historical, profile, earnings) map to their canonical grouped
/// path. All other top-level leaves return their own name.
fn preferred_path_for(name: &str) -> String {
    for &(alias, canonical) in ALIAS_PREFERRED {
        if alias == name {
            return canonical.to_owned();
        }
    }
    name.to_owned()
}

/// Returns aliases for a canonical grouped command.
///
/// When a group child has a matching top-level alias, the alias name is returned.
/// Otherwise the result is empty.
fn aliases_for(group: &str, child: &str) -> Vec<String> {
    let canonical = format!("{group} {child}");
    for &(alias, preferred) in ALIAS_PREFERRED {
        if preferred == canonical {
            return vec![alias.to_owned()];
        }
    }
    Vec::new()
}

//! CLI schema introspection for the `schema` subcommand.

use clap::CommandFactory;
use clap::builder::PossibleValue;
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
            let api_key_required = name != "schema" && name != "commands" && name != "completions";

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
        "schema_version": 3,
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
            let default_str = a.get_default_values().first().and_then(|v| v.to_str());
            let default_val = default_str.map(|s| {
                // Emit numeric defaults as JSON numbers so agents don't need
                // to coerce strings themselves.
                if let Ok(n) = s.parse::<u64>() {
                    json!(n)
                } else {
                    json!(s)
                }
            });
            let value_name = a
                .get_value_names()
                .and_then(|names| names.first())
                .map(|n| json!(n.as_str()));
            let help_text = a.get_help().map(|s| s.to_string());
            let long_flag = a.get_long().map(|s| json!(s));
            let short_flag = a.get_short().map(|c| json!(c.to_string()));
            let parser = parser_hint(a, default_str);
            let possible_vals: Option<Vec<Value>> = possible_values_json(&a.get_possible_values());
            let multi = matches!(a.get_action(), clap::ArgAction::Append)
                || a.get_num_args().is_some_and(|r| r.max_values() > 1);
            json!({
                "name": a.get_id().as_str(),
                "kind": kind,
                "required": a.is_required_set(),
                "default": default_val,
                "value_name": value_name,
                "help": help_text,
                "long": long_flag,
                "short": short_flag,
                "parser": parser,
                "possible_values": possible_vals,
                "multi_value": multi,
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

/// Derives a parser type hint from a Clap argument's action, possible values,
/// and default value.
fn parser_hint(a: &clap::Arg, default_str: Option<&str>) -> Value {
    let action = a.get_action();

    if matches!(action, clap::ArgAction::SetTrue | clap::ArgAction::SetFalse) {
        return json!({ "hint": "bool" });
    }
    if matches!(action, clap::ArgAction::Count) {
        return json!({ "hint": "count" });
    }
    if !a.get_possible_values().is_empty() {
        return json!({ "hint": "enum" });
    }
    if default_str.is_some_and(|s| s.parse::<u64>().is_ok()) {
        return json!({ "hint": "integer" });
    }

    json!({ "hint": "string" })
}

/// Converts Clap possible values into a JSON array of `{ name, help }` objects,
/// or `None` when the list is empty.
fn possible_values_json(values: &[PossibleValue]) -> Option<Vec<Value>> {
    if values.is_empty() {
        return None;
    }
    Some(
        values
            .iter()
            .filter(|v| !v.is_hide_set())
            .map(|v| {
                let name = v.get_name();
                let help = v.get_help().map(|s| s.to_string());
                if let Some(help) = help {
                    json!({ "name": name, "help": help })
                } else {
                    json!({ "name": name, "help": null })
                }
            })
            .collect(),
    )
}

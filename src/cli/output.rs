//! CLI output envelope types and rendering.

use serde::Serialize;
use serde_json::Value;

use crate::error::Result;

#[derive(Debug)]
pub(super) struct CommandPayload {
    pub(super) endpoint: &'static str,
    pub(super) query: Value,
    pub(super) data: Value,
}

impl CommandPayload {
    pub(super) fn new(endpoint: &'static str, query: Value, data: Value) -> Self {
        Self {
            endpoint,
            query,
            data,
        }
    }
}

#[derive(Debug, Serialize)]
struct OutputBody {
    ok: bool,
    endpoint: &'static str,
    query: Value,
    data: Value,
}

pub(super) fn render_output(payload: CommandPayload) -> Result<String> {
    let output = OutputBody {
        ok: true,
        endpoint: payload.endpoint,
        query: payload.query,
        data: payload.data,
    };

    Ok(serde_json::to_string(&output)?)
}

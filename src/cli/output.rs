//! CLI output rendering.

use serde_json::Value;

use crate::error::Result;

#[derive(Debug)]
pub(super) struct CommandPayload {
    #[cfg(test)]
    pub(super) endpoint: &'static str,
    #[cfg(test)]
    pub(super) query: Value,
    pub(super) data: Value,
}

impl CommandPayload {
    #[cfg(test)]
    pub(super) fn new(endpoint: &'static str, query: Value, data: Value) -> Self {
        Self {
            endpoint,
            query,
            data,
        }
    }

    #[cfg(not(test))]
    pub(super) fn new(_endpoint: &'static str, _query: Value, data: Value) -> Self {
        Self { data }
    }
}

pub(super) fn render_output(payload: CommandPayload) -> Result<Option<String>> {
    Ok(Some(serde_json::to_string(&payload.data)?))
}

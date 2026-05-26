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
    silent: bool,
}

impl CommandPayload {
    #[cfg(test)]
    pub(super) fn new(endpoint: &'static str, query: Value, data: Value) -> Self {
        Self {
            endpoint,
            query,
            data,
            silent: false,
        }
    }

    #[cfg(not(test))]
    pub(super) fn new(_endpoint: &'static str, _query: Value, data: Value) -> Self {
        Self {
            data,
            silent: false,
        }
    }

    pub(super) fn silent() -> Self {
        Self {
            #[cfg(test)]
            endpoint: "",
            #[cfg(test)]
            query: Value::Null,
            data: Value::Null,
            silent: true,
        }
    }
}

pub(super) fn render_output(payload: CommandPayload) -> Result<Option<String>> {
    if payload.silent {
        Ok(None)
    } else {
        Ok(Some(serde_json::to_string(&payload.data)?))
    }
}

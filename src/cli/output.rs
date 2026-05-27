//! CLI output rendering.

use serde_json::Value;

use crate::error::{Error, Result};

#[derive(Debug)]
pub(super) struct EmptyResultContext {
    endpoint: &'static str,
    symbol: String,
    search_query: String,
}

#[derive(Debug)]
pub(super) struct CommandPayload {
    #[cfg(test)]
    pub(super) endpoint: &'static str,
    #[cfg(test)]
    pub(super) query: Value,
    pub(super) data: Value,
    empty_result_context: Option<EmptyResultContext>,
}

impl CommandPayload {
    #[cfg(test)]
    pub(super) fn new(endpoint: &'static str, query: Value, data: Value) -> Self {
        Self {
            endpoint,
            query,
            data,
            empty_result_context: None,
        }
    }

    #[cfg(not(test))]
    pub(super) fn new(_endpoint: &'static str, _query: Value, data: Value) -> Self {
        Self {
            data,
            empty_result_context: None,
        }
    }

    pub(super) fn symbol_lookup(
        mut self,
        endpoint: &'static str,
        symbol: impl Into<String>,
    ) -> Self {
        let symbol = symbol.into();
        self.empty_result_context = Some(EmptyResultContext {
            endpoint,
            search_query: symbol.clone(),
            symbol,
        });
        self
    }

    pub(super) fn symbol_lookup_with_search_query(
        mut self,
        endpoint: &'static str,
        symbol: impl Into<String>,
        search_query: impl Into<String>,
    ) -> Self {
        self.empty_result_context = Some(EmptyResultContext {
            endpoint,
            symbol: symbol.into(),
            search_query: search_query.into(),
        });
        self
    }

    pub(super) fn reject_strict_empty(&self) -> Result<()> {
        if let Some(context) = &self.empty_result_context
            && is_empty_payload(&self.data)
        {
            return Err(Error::EmptyResult {
                symbol: context.symbol.clone(),
                search_query: context.search_query.clone(),
                endpoint: context.endpoint,
            });
        }

        Ok(())
    }
}

fn is_empty_payload(data: &Value) -> bool {
    match data {
        Value::Null => true,
        Value::Array(values) => values.is_empty(),
        _ => false,
    }
}

pub(super) fn render_output(payload: CommandPayload) -> Result<Option<String>> {
    Ok(Some(serde_json::to_string(&payload.data)?))
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::CommandPayload;

    #[test]
    fn strict_empty_rejects_null_symbol_lookup() {
        let payload = CommandPayload::new("/quote", json!({ "symbol": "FAKE" }), json!(null))
            .symbol_lookup("/quote", "FAKE");

        let error = payload.reject_strict_empty().unwrap_err();

        assert_eq!(error.kind(), "empty_result");
        assert!(error.to_string().contains("fmp-agent search FAKE"));
    }

    #[test]
    fn batch_symbol_lookup_suggests_first_symbol_search() {
        let payload =
            CommandPayload::new("/quote", json!({ "symbols": ["FAKE", "NOPE"] }), json!([]))
                .symbol_lookup_with_search_query("/quote", "FAKE,NOPE", "FAKE");

        let message = payload.reject_strict_empty().unwrap_err().to_string();

        assert!(message.contains("symbol FAKE,NOPE"));
        assert!(message.contains("fmp-agent search FAKE"));
        assert!(!message.contains("fmp-agent search FAKE,NOPE"));
    }

    #[test]
    fn strict_empty_allows_non_empty_symbol_lookup() {
        let payload = CommandPayload::new(
            "/quote",
            json!({ "symbol": "AAPL" }),
            json!({ "symbol": "AAPL" }),
        )
        .symbol_lookup("/quote", "AAPL");

        payload.reject_strict_empty().unwrap();
    }

    #[test]
    fn strict_empty_allows_empty_untagged_payload() {
        let payload = CommandPayload::new("/search-symbol", json!({ "query": "FAKE" }), json!([]));

        payload.reject_strict_empty().unwrap();
    }
}

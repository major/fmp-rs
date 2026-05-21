//! Shape-based FMP API dispatch helpers.

use serde_json::json;

use crate::client::FmpClient;
use crate::endpoint::{ANNUAL_PERIOD, Endpoint, NEWS_LIMIT, PAGE};
use crate::error::Result;

use super::output::CommandPayload;

pub(super) async fn run_endpoint(client: &FmpClient, endpoint: Endpoint) -> Result<CommandPayload> {
    let data = client.endpoint(endpoint).await?;
    Ok(CommandPayload::new(endpoint.path(), json!({}), data))
}

pub(super) async fn run_query(
    client: &FmpClient,
    endpoint: Endpoint,
    query: &str,
) -> Result<CommandPayload> {
    let data = client.query(endpoint, query).await?;
    Ok(CommandPayload::new(
        endpoint.path(),
        json!({ "query": query }),
        data,
    ))
}

pub(super) async fn run_by_symbol(
    client: &FmpClient,
    endpoint: Endpoint,
    symbol: &str,
) -> Result<CommandPayload> {
    let data = client.by_symbol(endpoint, symbol).await?;
    Ok(CommandPayload::new(
        endpoint.path(),
        json!({ "symbol": symbol }),
        data,
    ))
}

pub(super) async fn run_by_symbol_date_range(
    client: &FmpClient,
    endpoint: Endpoint,
    symbol: &str,
    from: &Option<String>,
    to: &Option<String>,
) -> Result<CommandPayload> {
    let data = client
        .by_symbol_date_range(endpoint, symbol, from.as_deref(), to.as_deref())
        .await?;
    Ok(CommandPayload::new(
        endpoint.path(),
        json!({ "symbol": symbol, "from": from, "to": to }),
        data,
    ))
}

pub(super) async fn run_by_date_range(
    client: &FmpClient,
    endpoint: Endpoint,
    from: &Option<String>,
    to: &Option<String>,
) -> Result<CommandPayload> {
    let data = client
        .by_date_range(endpoint, from.as_deref(), to.as_deref())
        .await?;
    Ok(CommandPayload::new(
        endpoint.path(),
        json!({ "from": from, "to": to }),
        data,
    ))
}

pub(super) async fn run_annual(
    client: &FmpClient,
    endpoint: Endpoint,
    symbol: &str,
    limit: Option<u16>,
) -> Result<CommandPayload> {
    let data = client.annual(endpoint, symbol, limit).await?;
    Ok(CommandPayload::new(
        endpoint.path(),
        json!({ "symbol": symbol, "period": ANNUAL_PERIOD, "limit": limit }),
        data,
    ))
}

pub(super) async fn run_technical_sma(
    client: &FmpClient,
    endpoint: Endpoint,
    symbol: &str,
    period_length: u16,
    timeframe: &str,
) -> Result<CommandPayload> {
    let data = client
        .technical(endpoint, symbol, period_length, timeframe)
        .await?;
    Ok(CommandPayload::new(
        endpoint.path(),
        json!({
            "symbol": symbol,
            "periodLength": period_length,
            "timeframe": timeframe,
        }),
        data,
    ))
}

pub(super) async fn run_news(
    client: &FmpClient,
    endpoint: Endpoint,
    symbol: &str,
    limit: Option<u16>,
) -> Result<CommandPayload> {
    let data = client.news(endpoint, symbol, limit).await?;
    Ok(CommandPayload::new(
        endpoint.path(),
        json!({ "symbol": symbol, "limit": limit }),
        data,
    ))
}

pub(super) async fn run_paged(
    client: &FmpClient,
    endpoint: Endpoint,
    page: Option<u16>,
    limit: Option<u16>,
) -> Result<CommandPayload> {
    let data = client.paged(endpoint, page, limit).await?;
    let page = page.unwrap_or(PAGE);
    let limit = limit.unwrap_or(NEWS_LIMIT);
    Ok(CommandPayload::new(
        endpoint.path(),
        json!({ "page": page, "limit": limit }),
        data,
    ))
}

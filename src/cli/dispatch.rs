//! Shape-based FMP API dispatch helpers.

use serde_json::json;

use crate::client::FmpClient;
use crate::endpoint::{ANNUAL_PERIOD, Endpoint};
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
    .map(|payload| payload.symbol_lookup(endpoint.path(), symbol))
}

pub(super) async fn run_by_symbols(
    client: &FmpClient,
    endpoint: Endpoint,
    symbols: &[String],
) -> Result<CommandPayload> {
    let data = client.by_symbols(endpoint, symbols).await?;
    Ok(CommandPayload::new(
        endpoint.path(),
        json!({ "symbols": symbols }),
        data,
    ))
    .map(|payload| {
        payload.symbol_lookup_with_search_query(
            endpoint.path(),
            symbols.join(","),
            symbols.first().cloned().unwrap_or_default(),
        )
    })
}

pub(super) async fn run_by_symbol_limit(
    client: &FmpClient,
    endpoint: Endpoint,
    symbol: &str,
    limit: u16,
) -> Result<CommandPayload> {
    let data = client
        .by_symbol_limit(endpoint, symbol, Some(limit))
        .await?;
    Ok(CommandPayload::new(
        endpoint.path(),
        json!({ "symbol": symbol, "limit": limit }),
        data,
    ))
    .map(|payload| payload.symbol_lookup(endpoint.path(), symbol))
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
    .map(|payload| payload.symbol_lookup(endpoint.path(), symbol))
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

pub(super) async fn run_by_name_date_range(
    client: &FmpClient,
    endpoint: Endpoint,
    name: &str,
    from: &Option<String>,
    to: &Option<String>,
) -> Result<CommandPayload> {
    let data = client
        .by_name_date_range(endpoint, name, from.as_deref(), to.as_deref())
        .await?;
    Ok(CommandPayload::new(
        endpoint.path(),
        json!({ "name": name, "from": from, "to": to }),
        data,
    ))
}

pub(super) async fn run_annual(
    client: &FmpClient,
    endpoint: Endpoint,
    symbol: &str,
    limit: u16,
) -> Result<CommandPayload> {
    let data = client.annual(endpoint, symbol, Some(limit)).await?;
    Ok(CommandPayload::new(
        endpoint.path(),
        json!({ "symbol": symbol, "period": ANNUAL_PERIOD, "limit": limit }),
        data,
    ))
    .map(|payload| payload.symbol_lookup(endpoint.path(), symbol))
}

pub(super) async fn run_annual_report_form(
    client: &FmpClient,
    endpoint: Endpoint,
    symbol: &str,
    year: u16,
    period: &str,
) -> Result<CommandPayload> {
    let data = client
        .annual_report_form(endpoint, symbol, year, period)
        .await?;
    Ok(CommandPayload::new(
        endpoint.path(),
        json!({ "symbol": symbol, "year": year, "period": period }),
        data,
    ))
    .map(|payload| payload.symbol_lookup(endpoint.path(), symbol))
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
    .map(|payload| payload.symbol_lookup(endpoint.path(), symbol))
}

pub(super) async fn run_news(
    client: &FmpClient,
    endpoint: Endpoint,
    symbol: &str,
    limit: u16,
) -> Result<CommandPayload> {
    let data = client.news(endpoint, symbol, Some(limit)).await?;
    Ok(CommandPayload::new(
        endpoint.path(),
        json!({ "symbol": symbol, "limit": limit }),
        data,
    ))
    .map(|payload| payload.symbol_lookup(endpoint.path(), symbol))
}

pub(super) async fn run_paged(
    client: &FmpClient,
    endpoint: Endpoint,
    page: u16,
    limit: u16,
) -> Result<CommandPayload> {
    let data = client.paged(endpoint, Some(page), Some(limit)).await?;
    Ok(CommandPayload::new(
        endpoint.path(),
        json!({ "page": page, "limit": limit }),
        data,
    ))
}

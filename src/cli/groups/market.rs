//! Market data command group.

use clap::Subcommand;
use serde_json::Value;

use crate::cli::args::{SymbolArgs, SymbolDateRangeArgs, SymbolsArgs};
use crate::cli::commands::{dispatch_historical_eod, dispatch_quote};
use crate::cli::dispatch::{run_by_symbol, run_by_symbols, run_endpoint};
use crate::cli::help;
use crate::cli::output::CommandPayload;
use crate::client::FmpClient;
use crate::error::Error;
use crate::error::Result;

/// Market data subcommands.
#[derive(Debug, Subcommand)]
pub(crate) enum Cmd {
    /// Get the latest market quote for a stock ticker.
    #[command(about = help::MARKET_QUOTE_ABOUT, long_about = help::MARKET_QUOTE_LONG)]
    Quote(SymbolArgs),

    /// Get historical end-of-day price bars for a stock ticker.
    #[command(about = help::MARKET_HISTORICAL_ABOUT, long_about = help::MARKET_HISTORICAL_LONG)]
    Historical(SymbolDateRangeArgs),

    /// Get historical dividend events for a stock ticker.
    #[command(about = help::MARKET_DIVIDENDS_ABOUT, long_about = help::MARKET_DIVIDENDS_LONG)]
    Dividends(SymbolArgs),

    /// Get historical stock split events for a stock ticker.
    #[command(about = help::MARKET_SPLITS_ABOUT, long_about = help::MARKET_SPLITS_LONG)]
    Splits(SymbolArgs),

    /// Get the latest market quotes for multiple stock tickers.
    #[command(
        about = help::MARKET_BATCH_QUOTE_ABOUT,
        long_about = help::MARKET_BATCH_QUOTE_LONG
    )]
    BatchQuote(SymbolsArgs),

    /// Get price change percentages for one or more stock tickers.
    #[command(about = help::MARKET_PRICE_CHANGE_ABOUT, long_about = help::MARKET_PRICE_CHANGE_LONG)]
    PriceChange(SymbolsArgs),

    /// List supported stock symbols.
    #[command(about = help::MARKET_STOCK_LIST_ABOUT, long_about = help::MARKET_STOCK_LIST_LONG)]
    StockList,

    /// Get a real-time market quote for a stock ticker.
    #[command(
        about = help::MARKET_REALTIME_QUOTE_ABOUT,
        long_about = help::MARKET_REALTIME_QUOTE_LONG
    )]
    RealtimeQuote(SymbolArgs),

    /// Get aftermarket quote data for a stock ticker.
    #[command(
        about = help::MARKET_AFTERMARKET_QUOTE_ABOUT,
        long_about = help::MARKET_AFTERMARKET_QUOTE_LONG
    )]
    AftermarketQuote(SymbolArgs),

    /// Get aftermarket trade data for a stock ticker.
    #[command(
        about = help::MARKET_AFTERMARKET_TRADE_ABOUT,
        long_about = help::MARKET_AFTERMARKET_TRADE_LONG
    )]
    AftermarketTrade(SymbolArgs),
}

/// Dispatch a market subcommand.
pub(crate) async fn dispatch(client: &FmpClient, cmd: &Cmd) -> Result<CommandPayload> {
    match cmd {
        Cmd::Quote(args) => dispatch_quote(client, args).await,
        Cmd::Historical(args) => dispatch_historical_eod(client, args).await,
        Cmd::Dividends(args) => {
            run_by_symbol(client, crate::endpoint::DIVIDENDS, &args.symbol).await
        }
        Cmd::Splits(args) => run_by_symbol(client, crate::endpoint::SPLITS, &args.symbol).await,
        Cmd::BatchQuote(args) => {
            run_by_symbols(client, crate::endpoint::BATCH_QUOTE, &args.symbols).await
        }
        Cmd::PriceChange(args) => run_price_change(client, &args.symbols).await,
        Cmd::StockList => run_endpoint(client, crate::endpoint::STOCK_LIST).await,
        Cmd::RealtimeQuote(args) => {
            run_by_symbol(client, crate::endpoint::REALTIME_QUOTE, &args.symbol).await
        }
        Cmd::AftermarketQuote(args) => {
            run_by_symbol(client, crate::endpoint::AFTERMARKET_QUOTE, &args.symbol).await
        }
        Cmd::AftermarketTrade(args) => {
            run_by_symbol(client, crate::endpoint::AFTERMARKET_TRADE, &args.symbol).await
        }
    }
}

async fn run_price_change(client: &FmpClient, symbols: &[String]) -> Result<CommandPayload> {
    let data = client
        .by_symbol_list(crate::endpoint::STOCK_PRICE_CHANGE, symbols)
        .await?;
    let payload = CommandPayload::new(
        crate::endpoint::STOCK_PRICE_CHANGE.path(),
        serde_json::json!({ "symbol": symbols.join(",") }),
        data,
    )
    .symbol_lookup_with_search_query(
        crate::endpoint::STOCK_PRICE_CHANGE.path(),
        symbols.join(","),
        symbols.first().cloned().unwrap_or_default(),
    );
    reject_missing_price_change_symbols(symbols, &payload.data)?;
    Ok(payload)
}

fn reject_missing_price_change_symbols(symbols: &[String], data: &Value) -> Result<()> {
    let Value::Array(rows) = data else {
        if symbols.len() == 1 {
            return Ok(());
        }

        return Err(Error::Api {
            status: 200,
            message: format!(
                "stock-price-change response for multiple requested symbols must be an array; got {}",
                value_type_name(data)
            ),
        });
    };

    if rows.is_empty() && symbols.len() == 1 {
        return Ok(());
    }

    let missing_symbols = symbols
        .iter()
        .filter(|symbol| !rows.iter().any(|row| row_symbol_matches(row, symbol)))
        .map(String::as_str)
        .collect::<Vec<_>>();

    if !missing_symbols.is_empty() {
        return Err(Error::Api {
            status: 200,
            message: format!(
                "stock-price-change response did not include requested symbol(s): {}",
                missing_symbols.join(",")
            ),
        });
    }

    Ok(())
}

fn row_symbol_matches(row: &Value, symbol: &str) -> bool {
    row.get("symbol")
        .and_then(Value::as_str)
        .is_some_and(|row_symbol| row_symbol.eq_ignore_ascii_case(symbol))
}

fn value_type_name(value: &Value) -> &'static str {
    match value {
        Value::Null => "null",
        Value::Bool(_) => "boolean",
        Value::Number(_) => "number",
        Value::String(_) => "string",
        Value::Array(_) => "array",
        Value::Object(_) => "object",
    }
}

#[cfg(test)]
mod tests {
    use serde_json::{Value, json};

    use super::{reject_missing_price_change_symbols, value_type_name};

    #[test]
    fn price_change_validation_allows_single_symbol_object() {
        reject_missing_price_change_symbols(&["AAPL".to_owned()], &json!({ "symbol": "AAPL" }))
            .unwrap();
    }

    #[test]
    fn price_change_validation_rejects_multi_symbol_object() {
        let error = reject_missing_price_change_symbols(
            &["AAPL".to_owned(), "MSFT".to_owned()],
            &json!({ "symbol": "AAPL" }),
        )
        .unwrap_err();

        assert_eq!(error.kind(), "api_error");
        assert!(error.to_string().contains("multiple requested symbols"));
        assert!(error.to_string().contains("object"));
    }

    #[test]
    fn value_type_name_describes_json_types() {
        assert_eq!(value_type_name(&Value::Null), "null");
        assert_eq!(value_type_name(&json!(true)), "boolean");
        assert_eq!(value_type_name(&json!(1)), "number");
        assert_eq!(value_type_name(&json!("AAPL")), "string");
        assert_eq!(value_type_name(&json!([])), "array");
        assert_eq!(value_type_name(&json!({})), "object");
    }

    #[test]
    fn price_change_validation_allows_requested_symbols_in_order() {
        reject_missing_price_change_symbols(
            &["ALAB".to_owned(), "CLS".to_owned()],
            &json!([{ "symbol": "ALAB" }, { "symbol": "CLS" }]),
        )
        .unwrap();
    }

    #[test]
    fn price_change_validation_allows_single_symbol_empty_array() {
        reject_missing_price_change_symbols(&["NOPE".to_owned()], &json!([])).unwrap();
    }

    #[test]
    fn price_change_validation_rejects_partial_multi_symbol_response() {
        let error = reject_missing_price_change_symbols(
            &["ALAB".to_owned(), "NOPE".to_owned(), "BAD".to_owned()],
            &json!([{ "symbol": "ALAB" }]),
        )
        .unwrap_err();

        assert_eq!(error.kind(), "api_error");
        assert!(error.to_string().contains("NOPE"));
        assert!(error.to_string().contains("BAD"));
    }
}

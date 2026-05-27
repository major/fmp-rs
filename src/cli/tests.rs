use clap::Parser;
use httpmock::Method::GET;
use httpmock::MockServer;
use serde_json::{Value, json};

use crate::client::FmpClient;

use super::args::{
    AnnualArgs, AnnualReportFormArgs, Cli, Command, DateRangeArgs, NameDateRangeArgs, PagedArgs,
    StockNewsArgs, SymbolArgs, SymbolDateRangeArgs, SymbolLimitArgs, TechnicalSmaArgs,
};
use super::commands::execute;
use super::groups;
use super::output::{CommandPayload, render_output};

fn test_client(server: &MockServer) -> FmpClient {
    FmpClient::with_base_url("test-key", format!("{}/", server.base_url())).unwrap()
}

fn symbol(symbol: &str) -> SymbolArgs {
    SymbolArgs {
        symbol: symbol.to_owned(),
    }
}

fn symbol_limit(symbol: &str, limit: u16) -> SymbolLimitArgs {
    SymbolLimitArgs {
        symbol: symbol.to_owned(),
        limit,
    }
}

fn annual(symbol: &str) -> AnnualArgs {
    AnnualArgs {
        symbol: symbol.to_owned(),
        limit: 5,
    }
}

fn annual_report(symbol: &str) -> AnnualReportFormArgs {
    AnnualReportFormArgs {
        symbol: symbol.to_owned(),
        year: 2022,
        period: "FY".to_owned(),
    }
}

fn date_range() -> DateRangeArgs {
    DateRangeArgs {
        from: Some("2025-01-01".to_owned()),
        to: Some("2025-01-31".to_owned()),
    }
}

fn symbol_date_range(symbol: &str) -> SymbolDateRangeArgs {
    SymbolDateRangeArgs {
        symbol: symbol.to_owned(),
        from: Some("2025-01-01".to_owned()),
        to: Some("2025-01-31".to_owned()),
    }
}

fn optional_symbol_date_range(symbol: &str) -> SymbolDateRangeArgs {
    SymbolDateRangeArgs {
        symbol: symbol.to_owned(),
        from: None,
        to: None,
    }
}

fn name_date_range(name: &str) -> NameDateRangeArgs {
    NameDateRangeArgs {
        name: name.to_owned(),
        from: Some("2025-01-01".to_owned()),
        to: Some("2025-12-31".to_owned()),
    }
}

fn paged() -> PagedArgs {
    PagedArgs { page: 0, limit: 3 }
}

fn stock_news(symbol: &str) -> StockNewsArgs {
    StockNewsArgs {
        symbol: symbol.to_owned(),
        limit: 10,
    }
}

#[test]
fn print_group_help_unknown_group_returns_err() {
    let result = super::print_group_help("nonexistent-group");

    assert!(result.is_err());
}

#[test]
fn print_group_help_market_returns_ok() {
    let result = super::print_group_help("market");

    assert!(result.is_ok());
}

#[test]
fn parses_bare_group_as_none() {
    let cli = Cli::parse_from(["fmp", "market"]);

    assert!(matches!(cli.command, Command::Market { command: None }));
}

#[test]
fn parses_grouped_commands() {
    let quote = Cli::parse_from(["fmp", "market", "quote", "AAPL"]);
    assert!(matches!(
        quote.command,
        Command::Market {
            command: Some(groups::market::Cmd::Quote(_))
        }
    ));

    let fundamentals = Cli::parse_from(["fmp", "fundamentals", "income-statement", "AAPL"]);
    assert!(matches!(
        fundamentals.command,
        Command::Fundamentals {
            command: Some(groups::fundamentals::Cmd::Income(_))
        }
    ));

    let macro_cmd = Cli::parse_from(["fmp", "macro", "economic-indicators", "GDP"]);
    assert!(matches!(
        macro_cmd.command,
        Command::MacroEcon {
            command: Some(groups::macro_econ::Cmd::EconomicIndicators(_))
        }
    ));

    let news = Cli::parse_from(["fmp", "news", "stock", "AAPL", "--limit", "3"]);
    let Command::News {
        command: Some(groups::news::Cmd::Stock(args)),
    } = news.command
    else {
        panic!("expected grouped stock news command");
    };
    assert_eq!(args.symbol, "AAPL");
    assert_eq!(args.limit, 3);
}

#[test]
fn parses_top_level_aliases() {
    assert!(matches!(
        Cli::parse_from(["fmp", "quote", "AAPL"]).command,
        Command::Quote(_)
    ));
    assert!(matches!(
        Cli::parse_from(["fmp", "historical", "AAPL"]).command,
        Command::Historical(_)
    ));
    assert!(matches!(
        Cli::parse_from(["fmp", "profile", "AAPL"]).command,
        Command::Profile(_)
    ));
    assert!(matches!(
        Cli::parse_from(["fmp", "earnings"]).command,
        Command::Earnings(_)
    ));
}

#[test]
fn rejects_flat_command_names() {
    let cases: &[&[&str]] = &[
        &["fmp", "company-profile", "AAPL"],
        &["fmp", "market-quote", "AAPL"],
        &["fmp", "news-stock", "AAPL"],
        &["fmp", "technical-sma", "AAPL"],
        &["fmp", "daily-chart", "AAPL"],
    ];

    for args in cases {
        assert!(
            Cli::try_parse_from(*args).is_err(),
            "flat command should be rejected: {}",
            args[1..].join(" ")
        );
    }
}

#[test]
fn render_output_returns_compact_json_payload() {
    let payload = CommandPayload::new(
        "quote",
        json!({ "symbol": "AAPL" }),
        json!([{ "symbol": "AAPL", "price": 200.0, "volume": 1000 }]),
    );

    let output = render_output(payload).unwrap().unwrap();

    assert!(!output.contains('\n'));

    let output: Value = serde_json::from_str(&output).unwrap();
    assert_eq!(
        output,
        json!([{ "symbol": "AAPL", "price": 200.0, "volume": 1000 }])
    );
}

#[tokio::test]
async fn execute_alias_commands_use_endpoint_descriptors() {
    let server = MockServer::start_async().await;
    let cases = [
        (
            "quote",
            Command::Quote(symbol("AAPL")),
            json!({ "symbol": "AAPL" }),
        ),
        (
            "historical-price-eod/full",
            Command::Historical(symbol_date_range("AAPL")),
            json!({ "symbol": "AAPL", "from": "2025-01-01", "to": "2025-01-31" }),
        ),
        (
            "profile",
            Command::Profile(symbol("AAPL")),
            json!({ "symbol": "AAPL" }),
        ),
        (
            "earnings-calendar",
            Command::Earnings(date_range()),
            json!({ "from": "2025-01-01", "to": "2025-01-31" }),
        ),
    ];
    let mut _mocks = Vec::new();

    for (path, _, _) in &cases {
        _mocks.push(
            server
                .mock_async(|when, then| {
                    when.method(GET)
                        .path(format!("/{path}"))
                        .query_param("apikey", "test-key");
                    then.status(200).json_body(json!([{ "ok": true }]));
                })
                .await,
        );
    }

    let client = test_client(&server);
    for (expected_path, command, expected_query) in cases {
        let payload = execute(&client, &command).await.unwrap();

        assert_eq!(payload.endpoint, expected_path);
        assert_eq!(payload.query, expected_query);
        assert_eq!(payload.data, json!([{ "ok": true }]));
    }
}

#[tokio::test]
async fn execute_grouped_commands_use_endpoint_descriptors() {
    let server = MockServer::start_async().await;
    let cases = vec![
        (
            "search-symbol",
            Command::Search {
                query: "Apple".to_owned(),
            },
            json!({ "query": "Apple" }),
        ),
        (
            "profile",
            Command::Company {
                command: Some(groups::company::Cmd::Profile(symbol("AAPL"))),
            },
            json!({ "symbol": "AAPL" }),
        ),
        (
            "key-executives",
            Command::Company {
                command: Some(groups::company::Cmd::Executives(symbol("AAPL"))),
            },
            json!({ "symbol": "AAPL" }),
        ),
        (
            "stock-peers",
            Command::Company {
                command: Some(groups::company::Cmd::Peers(symbol("AAPL"))),
            },
            json!({ "symbol": "AAPL" }),
        ),
        (
            "financial-scores",
            Command::Company {
                command: Some(groups::company::Cmd::Scores(symbol("AAPL"))),
            },
            json!({ "symbol": "AAPL" }),
        ),
        (
            "shares-float",
            Command::Company {
                command: Some(groups::company::Cmd::Float(symbol("AAPL"))),
            },
            json!({ "symbol": "AAPL" }),
        ),
        (
            "grades-consensus",
            Command::Company {
                command: Some(groups::company::Cmd::Rating(symbol("AAPL"))),
            },
            json!({ "symbol": "AAPL" }),
        ),
        (
            "ratings-historical",
            Command::Company {
                command: Some(groups::company::Cmd::HistoricalRating(symbol_limit(
                    "AAPL", 3,
                ))),
            },
            json!({ "symbol": "AAPL", "limit": 3 }),
        ),
        (
            "quote",
            Command::Market {
                command: Some(groups::market::Cmd::Quote(symbol("AAPL"))),
            },
            json!({ "symbol": "AAPL" }),
        ),
        (
            "historical-price-eod/full",
            Command::Market {
                command: Some(groups::market::Cmd::Historical(symbol_date_range("AAPL"))),
            },
            json!({ "symbol": "AAPL", "from": "2025-01-01", "to": "2025-01-31" }),
        ),
        (
            "dividends",
            Command::Market {
                command: Some(groups::market::Cmd::Dividends(symbol("AAPL"))),
            },
            json!({ "symbol": "AAPL" }),
        ),
        (
            "splits",
            Command::Market {
                command: Some(groups::market::Cmd::Splits(symbol("AAPL"))),
            },
            json!({ "symbol": "AAPL" }),
        ),
        (
            "stock-price-change",
            Command::Market {
                command: Some(groups::market::Cmd::PriceChange(symbol("AAPL"))),
            },
            json!({ "symbol": "AAPL" }),
        ),
        (
            "stock-list",
            Command::Market {
                command: Some(groups::market::Cmd::StockList),
            },
            json!({}),
        ),
        (
            "income-statement",
            Command::Fundamentals {
                command: Some(groups::fundamentals::Cmd::Income(annual("AAPL"))),
            },
            json!({ "symbol": "AAPL", "period": "annual", "limit": 5 }),
        ),
        (
            "income-statement-as-reported",
            Command::Fundamentals {
                command: Some(groups::fundamentals::Cmd::IncomeAsReported(annual("AAPL"))),
            },
            json!({ "symbol": "AAPL", "period": "annual", "limit": 5 }),
        ),
        (
            "balance-sheet-statement",
            Command::Fundamentals {
                command: Some(groups::fundamentals::Cmd::BalanceSheet(annual("AAPL"))),
            },
            json!({ "symbol": "AAPL", "period": "annual", "limit": 5 }),
        ),
        (
            "cash-flow-statement",
            Command::Fundamentals {
                command: Some(groups::fundamentals::Cmd::CashFlow(annual("AAPL"))),
            },
            json!({ "symbol": "AAPL", "period": "annual", "limit": 5 }),
        ),
        (
            "ratios",
            Command::Fundamentals {
                command: Some(groups::fundamentals::Cmd::Ratios(annual("AAPL"))),
            },
            json!({ "symbol": "AAPL", "period": "annual", "limit": 5 }),
        ),
        (
            "key-metrics",
            Command::Fundamentals {
                command: Some(groups::fundamentals::Cmd::Metrics(annual("AAPL"))),
            },
            json!({ "symbol": "AAPL", "period": "annual", "limit": 5 }),
        ),
        (
            "income-statement-growth",
            Command::Fundamentals {
                command: Some(groups::fundamentals::Cmd::IncomeGrowth(annual("AAPL"))),
            },
            json!({ "symbol": "AAPL", "period": "annual", "limit": 5 }),
        ),
        (
            "balance-sheet-statement-growth",
            Command::Fundamentals {
                command: Some(groups::fundamentals::Cmd::BalanceSheetGrowth(annual(
                    "AAPL",
                ))),
            },
            json!({ "symbol": "AAPL", "period": "annual", "limit": 5 }),
        ),
        (
            "cash-flow-statement-growth",
            Command::Fundamentals {
                command: Some(groups::fundamentals::Cmd::CashFlowGrowth(annual("AAPL"))),
            },
            json!({ "symbol": "AAPL", "period": "annual", "limit": 5 }),
        ),
        (
            "enterprise-values",
            Command::Fundamentals {
                command: Some(groups::fundamentals::Cmd::EnterpriseValues(annual("AAPL"))),
            },
            json!({ "symbol": "AAPL", "period": "annual", "limit": 5 }),
        ),
        (
            "analyst-estimates",
            Command::Fundamentals {
                command: Some(groups::fundamentals::Cmd::Estimates(annual("AAPL"))),
            },
            json!({ "symbol": "AAPL", "period": "annual", "limit": 5 }),
        ),
        (
            "financial-reports-dates",
            Command::Fundamentals {
                command: Some(groups::fundamentals::Cmd::ReportDates(symbol("AAPL"))),
            },
            json!({ "symbol": "AAPL" }),
        ),
        (
            "financial-reports-json",
            Command::Fundamentals {
                command: Some(groups::fundamentals::Cmd::AnnualReport(annual_report(
                    "AAPL",
                ))),
            },
            json!({ "symbol": "AAPL", "year": 2022, "period": "FY" }),
        ),
        (
            "price-target-consensus",
            Command::Analyst {
                command: Some(groups::analyst::Cmd::PriceTargetConsensus(symbol("AAPL"))),
            },
            json!({ "symbol": "AAPL" }),
        ),
        (
            "price-target-summary",
            Command::Analyst {
                command: Some(groups::analyst::Cmd::PriceTargetSummary(symbol("AAPL"))),
            },
            json!({ "symbol": "AAPL" }),
        ),
        (
            "grades",
            Command::Analyst {
                command: Some(groups::analyst::Cmd::Grades(symbol("AAPL"))),
            },
            json!({ "symbol": "AAPL" }),
        ),
        (
            "insider-trading/latest",
            Command::Insider {
                command: Some(groups::insider::Cmd::Latest(paged())),
            },
            json!({ "page": 0, "limit": 3 }),
        ),
        (
            "earnings-calendar",
            Command::Calendar {
                command: Some(groups::calendar::Cmd::Earnings(date_range())),
            },
            json!({ "from": "2025-01-01", "to": "2025-01-31" }),
        ),
        (
            "treasury-rates",
            Command::MacroEcon {
                command: Some(groups::macro_econ::Cmd::TreasuryRates(date_range())),
            },
            json!({ "from": "2025-01-01", "to": "2025-01-31" }),
        ),
        (
            "economic-indicators",
            Command::MacroEcon {
                command: Some(groups::macro_econ::Cmd::EconomicIndicators(
                    name_date_range("GDP"),
                )),
            },
            json!({ "name": "GDP", "from": "2025-01-01", "to": "2025-12-31" }),
        ),
        (
            "technical-indicators/sma",
            Command::Technical {
                command: Some(groups::technical::Cmd::Sma(TechnicalSmaArgs {
                    symbol: "AAPL".to_owned(),
                    period_length: 10,
                    timeframe: "1day".to_owned(),
                })),
            },
            json!({ "symbol": "AAPL", "periodLength": 10, "timeframe": "1day" }),
        ),
        (
            "sec-filings-search/symbol",
            Command::Sec {
                command: Some(groups::sec::Cmd::Filings(symbol_date_range("AAPL"))),
            },
            json!({ "symbol": "AAPL", "from": "2025-01-01", "to": "2025-01-31" }),
        ),
        (
            "etf/holdings",
            Command::Etf {
                command: Some(groups::etf::Cmd::Holdings(symbol("SPY"))),
            },
            json!({ "symbol": "SPY" }),
        ),
        (
            "cryptocurrency-list",
            Command::Crypto {
                command: Some(groups::crypto::Cmd::List),
            },
            json!({}),
        ),
        (
            "quote",
            Command::Crypto {
                command: Some(groups::crypto::Cmd::Quote(symbol("BTCUSD"))),
            },
            json!({ "symbol": "BTCUSD" }),
        ),
        (
            "historical-price-eod/full",
            Command::Crypto {
                command: Some(groups::crypto::Cmd::Historical(symbol_date_range("BTCUSD"))),
            },
            json!({ "symbol": "BTCUSD", "from": "2025-01-01", "to": "2025-01-31" }),
        ),
        (
            "quote",
            Command::Forex {
                command: Some(groups::forex::Cmd::Quote(symbol("EURUSD"))),
            },
            json!({ "symbol": "EURUSD" }),
        ),
        (
            "historical-price-eod/full",
            Command::Forex {
                command: Some(groups::forex::Cmd::Historical(symbol_date_range("EURUSD"))),
            },
            json!({ "symbol": "EURUSD", "from": "2025-01-01", "to": "2025-01-31" }),
        ),
        (
            "news/stock",
            Command::News {
                command: Some(groups::news::Cmd::Stock(stock_news("AAPL"))),
            },
            json!({ "symbol": "AAPL", "limit": 10 }),
        ),
        (
            "news/general-latest",
            Command::News {
                command: Some(groups::news::Cmd::General(paged())),
            },
            json!({ "page": 0, "limit": 3 }),
        ),
        (
            "fmp-articles",
            Command::News {
                command: Some(groups::news::Cmd::Articles(paged())),
            },
            json!({ "page": 0, "limit": 3 }),
        ),
        (
            "news/forex-latest",
            Command::News {
                command: Some(groups::news::Cmd::Forex(paged())),
            },
            json!({ "page": 0, "limit": 3 }),
        ),
        (
            "news/crypto-latest",
            Command::News {
                command: Some(groups::news::Cmd::Crypto(paged())),
            },
            json!({ "page": 0, "limit": 3 }),
        ),
    ];

    let mut _mocks = Vec::new();
    for (path, _, _) in &cases {
        _mocks.push(
            server
                .mock_async(|when, then| {
                    when.method(GET)
                        .path(format!("/{path}"))
                        .query_param("apikey", "test-key");
                    then.status(200).json_body(json!([{ "ok": true }]));
                })
                .await,
        );
    }

    let client = test_client(&server);
    for (expected_path, command, expected_query) in cases {
        let payload = execute(&client, &command).await.unwrap();

        assert_eq!(payload.endpoint, expected_path);
        assert_eq!(payload.query, expected_query);
        assert_eq!(payload.data, json!([{ "ok": true }]));
    }
}

#[tokio::test]
async fn sec_filings_defaults_from_when_omitted() {
    let server = MockServer::start_async().await;
    server
        .mock_async(|when, then| {
            when.method(GET)
                .path("/sec-filings-search/symbol")
                .query_param("symbol", "AAPL")
                .query_param("apikey", "test-key");
            then.status(200).json_body(json!([{ "ok": true }]));
        })
        .await;

    let command = Command::Sec {
        command: Some(groups::sec::Cmd::Filings(optional_symbol_date_range(
            "AAPL",
        ))),
    };
    let payload = execute(&test_client(&server), &command).await.unwrap();

    assert!(
        payload.query["from"].is_string(),
        "from should be defaulted to 90 days ago"
    );
    assert!(payload.query["to"].is_null());
}

#[tokio::test]
#[should_panic(expected = "Schema command is handled before execute() in run()")]
async fn schema_arm_in_execute_panics() {
    let client = FmpClient::with_base_url("dummy", "http://localhost:1/").unwrap();
    execute(&client, &Command::Schema).await.unwrap();
}

#[tokio::test]
#[should_panic(expected = "Commands command is handled before execute() in run()")]
async fn commands_arm_in_execute_panics() {
    let client = FmpClient::with_base_url("dummy", "http://localhost:1/").unwrap();
    execute(&client, &Command::Commands { grouped: false })
        .await
        .unwrap();
}

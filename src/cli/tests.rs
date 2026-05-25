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
use super::output::{CommandPayload, render_output};

fn test_client(server: &MockServer) -> FmpClient {
    FmpClient::with_base_url("test-key", format!("{}/", server.base_url())).unwrap()
}

fn symbol(symbol: &str) -> SymbolArgs {
    SymbolArgs {
        symbol: symbol.to_owned(),
    }
}

fn symbol_limit(symbol: &str) -> SymbolLimitArgs {
    SymbolLimitArgs {
        symbol: symbol.to_owned(),
        limit: None,
    }
}

fn annual_report(symbol: &str) -> AnnualReportFormArgs {
    AnnualReportFormArgs {
        symbol: symbol.to_owned(),
        year: 2022,
        period: "FY".to_owned(),
    }
}

fn name_date_range(name: &str) -> NameDateRangeArgs {
    NameDateRangeArgs {
        name: name.to_owned(),
        from: Some("2025-01-01".to_owned()),
        to: Some("2025-12-31".to_owned()),
    }
}

fn annual(symbol: &str) -> AnnualArgs {
    AnnualArgs {
        symbol: symbol.to_owned(),
        limit: 5,
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

fn stock_news(symbol: &str) -> StockNewsArgs {
    StockNewsArgs {
        symbol: symbol.to_owned(),
        limit: None,
    }
}

fn paged() -> PagedArgs {
    PagedArgs { page: 0, limit: 3 }
}

#[test]
fn parses_historical_command() {
    let cli = Cli::parse_from([
        "fmp",
        "--api-key",
        "test-key",
        "market-historical",
        "AAPL",
        "--from",
        "2025-01-01",
        "--to",
        "2025-01-31",
    ]);

    assert!(matches!(cli.command, Command::MarketHistorical(_)));
}

#[test]
fn render_output_returns_compact_json_payload() {
    let payload = CommandPayload::new(
        "quote",
        json!({ "symbol": "AAPL" }),
        json!([{ "symbol": "AAPL", "price": 200.0, "volume": 1000 }]),
    );

    let output = render_output(payload).unwrap();

    assert!(!output.contains('\n'));

    let output: Value = serde_json::from_str(&output).unwrap();

    assert_eq!(
        output,
        json!([{ "symbol": "AAPL", "price": 200.0, "volume": 1000 }])
    );
}

#[test]
fn rejects_legacy_historical_alias() {
    let result = Cli::try_parse_from([
        "fmp",
        "--api-key",
        "test-key",
        "historical",
        "AAPL",
        "--from",
        "2025-01-01",
        "--to",
        "2025-01-31",
    ]);

    assert!(result.is_err());
}

#[test]
fn parses_technical_sma_defaults() {
    let cli = Cli::parse_from(["fmp", "--api-key", "test-key", "technical-sma", "AAPL"]);

    let Command::TechnicalSma(args) = cli.command else {
        panic!("expected technical SMA command");
    };
    assert_eq!(args.period_length, 10);
    assert_eq!(args.timeframe, "1day");
}

#[test]
fn parses_grouped_news_command() {
    let cli = Cli::parse_from([
        "fmp",
        "--api-key",
        "test-key",
        "news-stock",
        "AAPL",
        "--limit",
        "3",
    ]);

    let Command::NewsStock(args) = cli.command else {
        panic!("expected stock news command");
    };
    assert_eq!(args.symbol, "AAPL");
    assert_eq!(args.limit, Some(3));
}

#[test]
fn parses_paginated_news_commands() {
    let cases = [
        ("news-general", "general"),
        ("news-articles", "articles"),
        ("news-forex", "forex"),
        ("news-crypto", "crypto"),
    ];

    for (command, expected) in cases {
        let cli = Cli::parse_from([
            "fmp",
            "--api-key",
            "test-key",
            command,
            "--page",
            "0",
            "--limit",
            "3",
        ]);

        let args = match cli.command {
            Command::NewsGeneral(args) if expected == "general" => args,
            Command::NewsArticles(args) if expected == "articles" => args,
            Command::NewsForex(args) if expected == "forex" => args,
            Command::NewsCrypto(args) if expected == "crypto" => args,
            _ => panic!("unexpected news command"),
        };
        assert_eq!(args.page, 0);
        assert_eq!(args.limit, 3);
    }
}

#[test]
fn parses_new_endpoint_commands() {
    let stock_list = Cli::parse_from(["fmp", "--api-key", "test-key", "market-stock-list"]);
    assert!(matches!(stock_list.command, Command::MarketStockList));

    let rating = Cli::parse_from([
        "fmp",
        "--api-key",
        "test-key",
        "company-historical-rating",
        "AAPL",
        "--limit",
        "3",
    ]);
    let Command::CompanyHistoricalRating(args) = rating.command else {
        panic!("expected historical rating command");
    };
    assert_eq!(args.symbol, "AAPL");
    assert_eq!(args.limit, Some(3));

    let annual_report = Cli::parse_from([
        "fmp",
        "--api-key",
        "test-key",
        "fundamentals-annual-report-form",
        "AAPL",
        "--year",
        "2022",
    ]);
    let Command::FundamentalsAnnualReportForm(args) = annual_report.command else {
        panic!("expected annual report form command");
    };
    assert_eq!(args.period, "FY");

    let insider = Cli::parse_from([
        "fmp",
        "--api-key",
        "test-key",
        "insider-trading-latest",
        "--limit",
        "3",
    ]);
    assert!(matches!(insider.command, Command::InsiderTradingLatest(_)));

    let economics = Cli::parse_from([
        "fmp",
        "--api-key",
        "test-key",
        "economic-indicators",
        "GDP",
        "--from",
        "2025-01-01",
        "--to",
        "2025-12-31",
    ]);
    assert!(matches!(economics.command, Command::EconomicIndicators(_)));
}

#[test]
fn parses_new_symbol_commands() {
    let cases = [
        (
            [
                "fmp",
                "--api-key",
                "test-key",
                "company-share-float",
                "AAPL",
            ],
            "company-share-float",
        ),
        (
            ["fmp", "--api-key", "test-key", "company-rating", "AAPL"],
            "company-rating",
        ),
        (
            ["fmp", "--api-key", "test-key", "etf-holdings", "SPY"],
            "etf-holdings",
        ),
        (
            [
                "fmp",
                "--api-key",
                "test-key",
                "fundamentals-report-dates",
                "AAPL",
            ],
            "fundamentals-report-dates",
        ),
        (
            [
                "fmp",
                "--api-key",
                "test-key",
                "analyst-price-target-consensus",
                "AAPL",
            ],
            "analyst-price-target-consensus",
        ),
        (
            [
                "fmp",
                "--api-key",
                "test-key",
                "analyst-price-target-summary",
                "AAPL",
            ],
            "analyst-price-target-summary",
        ),
        (
            ["fmp", "--api-key", "test-key", "analyst-grades", "AAPL"],
            "analyst-grades",
        ),
    ];

    for (args, expected) in cases {
        let cli = Cli::parse_from(args);

        match expected {
            "company-share-float" => assert!(matches!(cli.command, Command::CompanyShareFloat(_))),
            "company-rating" => assert!(matches!(cli.command, Command::CompanyRating(_))),
            "etf-holdings" => assert!(matches!(cli.command, Command::EtfHoldings(_))),
            "fundamentals-report-dates" => {
                assert!(matches!(cli.command, Command::FundamentalsReportDates(_)));
            }
            "analyst-price-target-consensus" => {
                assert!(matches!(
                    cli.command,
                    Command::AnalystPriceTargetConsensus(_)
                ));
            }
            "analyst-price-target-summary" => {
                assert!(matches!(cli.command, Command::AnalystPriceTargetSummary(_)));
            }
            "analyst-grades" => assert!(matches!(cli.command, Command::AnalystGrades(_))),
            _ => unreachable!(),
        }
    }
}

#[test]
fn parses_crypto_and_forex_commands() {
    let cases: &[(&[&str], &str)] = &[
        (
            &["fmp", "--api-key", "test-key", "crypto-list"],
            "crypto-list",
        ),
        (
            &["fmp", "--api-key", "test-key", "crypto-quote", "BTCUSD"],
            "crypto-quote",
        ),
        (
            &[
                "fmp",
                "--api-key",
                "test-key",
                "crypto-historical",
                "BTCUSD",
            ],
            "crypto-historical",
        ),
        (
            &["fmp", "--api-key", "test-key", "forex-quote", "EURUSD"],
            "forex-quote",
        ),
        (
            &["fmp", "--api-key", "test-key", "forex-historical", "EURUSD"],
            "forex-historical",
        ),
    ];

    for (args, expected) in cases {
        let cli = Cli::parse_from(*args);

        match *expected {
            "crypto-list" => assert!(matches!(cli.command, Command::CryptoList)),
            "crypto-quote" => assert!(matches!(cli.command, Command::CryptoQuote(_))),
            "crypto-historical" => assert!(matches!(cli.command, Command::CryptoHistorical(_))),
            "forex-quote" => assert!(matches!(cli.command, Command::ForexQuote(_))),
            "forex-historical" => assert!(matches!(cli.command, Command::ForexHistorical(_))),
            _ => unreachable!(),
        }
    }
}

#[test]
fn rejects_legacy_news_alias() {
    let result = Cli::try_parse_from([
        "fmp",
        "--api-key",
        "test-key",
        "news",
        "AAPL",
        "--limit",
        "3",
    ]);

    assert!(result.is_err());
}

#[tokio::test]
async fn execute_annual_command_uses_endpoint_descriptor() {
    let server = MockServer::start_async().await;
    let cases = [
        (
            "income-statement",
            Command::FundamentalsIncomeStatement(annual("AAPL")),
        ),
        (
            "income-statement-as-reported",
            Command::FundamentalsIncomeStatementAsReported(annual("AAPL")),
        ),
        (
            "balance-sheet-statement-growth",
            Command::FundamentalsBalanceSheetGrowth(annual("AAPL")),
        ),
        (
            "cash-flow-statement-growth",
            Command::FundamentalsCashFlowGrowth(annual("AAPL")),
        ),
    ];
    let mut mocks = Vec::new();

    for (path, _) in &cases {
        mocks.push(
            server
                .mock_async(|when, then| {
                    when.method(GET)
                        .path(format!("/{path}"))
                        .query_param("symbol", "AAPL")
                        .query_param("period", "annual")
                        .query_param("limit", "5")
                        .query_param("apikey", "test-key");
                    then.status(200).json_body(json!([{ "symbol": "AAPL" }]));
                })
                .await,
        );
    }

    let client = test_client(&server);
    for (expected_path, command) in cases {
        let payload = execute(&client, &command).await.unwrap();

        assert_eq!(payload.endpoint, expected_path);
        assert_eq!(
            payload.query,
            json!({ "symbol": "AAPL", "period": "annual", "limit": 5 })
        );
        assert_eq!(payload.data[0]["symbol"], "AAPL");
    }

    for mock in mocks {
        mock.assert_async().await;
    }
}

#[tokio::test]
async fn execute_symbol_commands_use_endpoint_descriptors() {
    let server = MockServer::start_async().await;
    let cases = [
        ("profile", Command::CompanyProfile(symbol("AAPL"))),
        ("stock-peers", Command::CompanyPeers(symbol("AAPL"))),
        (
            "ratings-historical",
            Command::CompanyHistoricalRating(SymbolLimitArgs {
                limit: Some(3),
                ..symbol_limit("AAPL")
            }),
        ),
        ("key-executives", Command::CompanyExecutives(symbol("AAPL"))),
        ("etf/holdings", Command::EtfHoldings(symbol("AAPL"))),
        ("dividends", Command::MarketDividends(symbol("AAPL"))),
        ("splits", Command::MarketSplits(symbol("AAPL"))),
        (
            "stock-price-change",
            Command::MarketPriceChange(symbol("AAPL")),
        ),
        (
            "financial-scores",
            Command::CompanyFinancialScores(symbol("AAPL")),
        ),
        ("shares-float", Command::CompanyShareFloat(symbol("AAPL"))),
        ("grades-consensus", Command::CompanyRating(symbol("AAPL"))),
        (
            "financial-reports-dates",
            Command::FundamentalsReportDates(symbol("AAPL")),
        ),
        (
            "price-target-consensus",
            Command::AnalystPriceTargetConsensus(symbol("AAPL")),
        ),
        (
            "price-target-summary",
            Command::AnalystPriceTargetSummary(symbol("AAPL")),
        ),
        ("grades", Command::AnalystGrades(symbol("AAPL"))),
    ];
    let mut mocks = Vec::new();
    for (path, _) in &cases {
        mocks.push(
            server
                .mock_async(|when, then| {
                    when.method(GET)
                        .path(format!("/{path}"))
                        .query_param("symbol", "AAPL")
                        .query_param("apikey", "test-key");
                    then.status(200).json_body(json!([{ "symbol": "AAPL" }]));
                })
                .await,
        );
    }

    let client = test_client(&server);
    for (expected_path, command) in cases {
        let payload = execute(&client, &command).await.unwrap();

        assert_eq!(payload.endpoint, expected_path);
        if expected_path == "ratings-historical" {
            assert_eq!(payload.query, json!({ "symbol": "AAPL", "limit": 3 }));
        } else {
            assert_eq!(payload.query, json!({ "symbol": "AAPL" }));
        }
        assert_eq!(payload.data[0]["symbol"], "AAPL");
    }

    for mock in mocks {
        mock.assert_async().await;
    }
}

#[tokio::test]
async fn execute_crypto_and_forex_commands_use_endpoint_descriptors() {
    let server = MockServer::start_async().await;
    let list_mock = server
        .mock_async(|when, then| {
            when.method(GET)
                .path("/cryptocurrency-list")
                .query_param("apikey", "test-key");
            then.status(200).json_body(json!([{ "symbol": "BTCUSD" }]));
        })
        .await;
    let quote_cases = [
        ("BTCUSD", Command::CryptoQuote(symbol("BTCUSD"))),
        ("EURUSD", Command::ForexQuote(symbol("EURUSD"))),
    ];
    let historical_cases = [
        (
            "BTCUSD",
            Command::CryptoHistorical(optional_symbol_date_range("BTCUSD")),
        ),
        (
            "EURUSD",
            Command::ForexHistorical(optional_symbol_date_range("EURUSD")),
        ),
    ];
    let mut mocks = Vec::new();

    for (symbol, _) in &quote_cases {
        mocks.push(
            server
                .mock_async(|when, then| {
                    when.method(GET)
                        .path("/quote")
                        .query_param("symbol", *symbol)
                        .query_param("apikey", "test-key");
                    then.status(200).json_body(json!([{ "symbol": symbol }]));
                })
                .await,
        );
    }
    for (symbol, _) in &historical_cases {
        mocks.push(
            server
                .mock_async(|when, then| {
                    when.method(GET)
                        .path("/historical-price-eod/full")
                        .query_param("symbol", *symbol)
                        .query_param("apikey", "test-key");
                    then.status(200).json_body(json!([{ "symbol": symbol }]));
                })
                .await,
        );
    }

    let client = test_client(&server);
    let payload = execute(&client, &Command::CryptoList).await.unwrap();
    assert_eq!(payload.endpoint, "cryptocurrency-list");
    assert_eq!(payload.query, json!({}));
    assert_eq!(payload.data[0]["symbol"], "BTCUSD");

    for (symbol, command) in quote_cases {
        let payload = execute(&client, &command).await.unwrap();
        assert_eq!(payload.endpoint, "quote");
        assert_eq!(payload.query, json!({ "symbol": symbol }));
        assert_eq!(payload.data[0]["symbol"], symbol);
    }

    for (symbol, command) in historical_cases {
        let payload = execute(&client, &command).await.unwrap();
        assert_eq!(payload.endpoint, "historical-price-eod/full");
        assert_eq!(
            payload.query,
            json!({ "symbol": symbol, "from": null, "to": null })
        );
        assert_eq!(payload.data[0]["symbol"], symbol);
    }

    list_mock.assert_async().await;
    for mock in mocks {
        mock.assert_async().await;
    }
}

#[tokio::test]
async fn execute_date_range_only_commands_use_endpoint_descriptors() {
    let server = MockServer::start_async().await;
    let endpoints = ["earnings-calendar", "treasury-rates"];
    let mut mocks = Vec::new();

    for endpoint in endpoints {
        mocks.push(
            server
                .mock_async(|when, then| {
                    when.method(GET)
                        .path(format!("/{endpoint}"))
                        .query_param("from", "2025-01-01")
                        .query_param("to", "2025-01-31")
                        .query_param("apikey", "test-key");
                    then.status(200)
                        .json_body(json!([{ "date": "2025-01-01" }]));
                })
                .await,
        );
    }

    let client = test_client(&server);
    let commands = [
        Command::EarningsCalendar(date_range()),
        Command::TreasuryRates(date_range()),
    ];

    for command in commands {
        let payload = execute(&client, &command).await.unwrap();

        assert!(matches!(
            payload.endpoint,
            "earnings-calendar" | "treasury-rates"
        ));
        assert_eq!(
            payload.query,
            json!({ "from": "2025-01-01", "to": "2025-01-31" })
        );
        assert_eq!(payload.data[0]["date"], "2025-01-01");
    }

    for mock in mocks {
        mock.assert_async().await;
    }
}

#[tokio::test]
async fn execute_technical_sma_uses_endpoint_descriptor() {
    let server = MockServer::start_async().await;
    let mock = server
        .mock_async(|when, then| {
            when.method(GET)
                .path("/technical-indicators/sma")
                .query_param("symbol", "AAPL")
                .query_param("periodLength", "10")
                .query_param("timeframe", "1day")
                .query_param("apikey", "test-key");
            then.status(200)
                .json_body(json!([{ "symbol": "AAPL", "sma": 200.0 }]));
        })
        .await;
    let command = Command::TechnicalSma(TechnicalSmaArgs {
        symbol: "AAPL".to_owned(),
        period_length: 10,
        timeframe: "1day".to_owned(),
    });

    let payload = execute(&test_client(&server), &command).await.unwrap();

    mock.assert_async().await;
    assert_eq!(payload.endpoint, "technical-indicators/sma");
    assert_eq!(
        payload.query,
        json!({ "symbol": "AAPL", "periodLength": 10, "timeframe": "1day" })
    );
    assert_eq!(payload.data[0]["symbol"], "AAPL");
}

#[tokio::test]
async fn execute_news_commands_use_endpoint_descriptor() {
    let server = MockServer::start_async().await;
    let mocks = ["AAPL", "MSFT"];

    for symbol in mocks {
        server
            .mock_async(|when, then| {
                when.method(GET)
                    .path("/news/stock")
                    .query_param("symbols", symbol)
                    .query_param("limit", "3")
                    .query_param("apikey", "test-key");
                then.status(200).json_body(json!([{ "symbol": symbol }]));
            })
            .await;
    }

    let client = test_client(&server);
    let commands = [
        Command::NewsStock(StockNewsArgs {
            limit: Some(3),
            ..stock_news("AAPL")
        }),
        Command::NewsStock(StockNewsArgs {
            symbol: "MSFT".to_owned(),
            limit: Some(3),
        }),
    ];

    for command in commands {
        let payload = execute(&client, &command).await.unwrap();

        assert_eq!(payload.endpoint, "news/stock");
        assert_eq!(payload.query["limit"], 3);
    }
}

#[tokio::test]
async fn execute_paginated_news_commands_use_endpoint_descriptors() {
    let server = MockServer::start_async().await;
    let cases = [
        ("news/general-latest", 0, 3, Command::NewsGeneral(paged())),
        ("fmp-articles", 0, 3, Command::NewsArticles(paged())),
        ("news/forex-latest", 0, 3, Command::NewsForex(paged())),
        ("news/crypto-latest", 0, 3, Command::NewsCrypto(paged())),
        (
            "insider-trading/latest",
            0,
            3,
            Command::InsiderTradingLatest(paged()),
        ),
        (
            "news/general-latest",
            0,
            10,
            Command::NewsGeneral(PagedArgs { page: 0, limit: 10 }),
        ),
    ];
    let mut mocks = Vec::new();

    for (path, page, limit, _) in &cases {
        mocks.push(
            server
                .mock_async(|when, then| {
                    when.method(GET)
                        .path(format!("/{path}"))
                        .query_param("page", page.to_string())
                        .query_param("limit", limit.to_string())
                        .query_param("apikey", "test-key");
                    then.status(200)
                        .json_body(json!([{ "title": "Market news" }]));
                })
                .await,
        );
    }

    let client = test_client(&server);
    for (expected_path, expected_page, expected_limit, command) in cases {
        let payload = execute(&client, &command).await.unwrap();

        assert_eq!(payload.endpoint, expected_path);
        assert_eq!(
            payload.query,
            json!({ "page": expected_page, "limit": expected_limit })
        );
        assert_eq!(payload.data[0]["title"], "Market news");
    }

    for mock in mocks {
        mock.assert_async().await;
    }
}

#[tokio::test]
async fn execute_new_endpoint_shapes_use_endpoint_descriptors() {
    let server = MockServer::start_async().await;
    let stock_list = server
        .mock_async(|when, then| {
            when.method(GET)
                .path("/stock-list")
                .query_param("apikey", "test-key");
            then.status(200).json_body(json!([{ "symbol": "AAPL" }]));
        })
        .await;
    let annual_report_mock = server
        .mock_async(|when, then| {
            when.method(GET)
                .path("/financial-reports-json")
                .query_param("symbol", "AAPL")
                .query_param("year", "2022")
                .query_param("period", "FY")
                .query_param("apikey", "test-key");
            then.status(200).json_body(json!({ "symbol": "AAPL" }));
        })
        .await;
    let economics = server
        .mock_async(|when, then| {
            when.method(GET)
                .path("/economic-indicators")
                .query_param("name", "GDP")
                .query_param("from", "2025-01-01")
                .query_param("to", "2025-12-31")
                .query_param("apikey", "test-key");
            then.status(200).json_body(json!([{ "name": "GDP" }]));
        })
        .await;

    let client = test_client(&server);
    let stock_payload = execute(&client, &Command::MarketStockList).await.unwrap();
    assert_eq!(stock_payload.endpoint, "stock-list");
    assert_eq!(stock_payload.query, json!({}));
    assert_eq!(stock_payload.data[0]["symbol"], "AAPL");

    let report_payload = execute(
        &client,
        &Command::FundamentalsAnnualReportForm(annual_report("AAPL")),
    )
    .await
    .unwrap();
    assert_eq!(report_payload.endpoint, "financial-reports-json");
    assert_eq!(
        report_payload.query,
        json!({ "symbol": "AAPL", "year": 2022, "period": "FY" })
    );

    let economics_payload = execute(
        &client,
        &Command::EconomicIndicators(name_date_range("GDP")),
    )
    .await
    .unwrap();
    assert_eq!(economics_payload.endpoint, "economic-indicators");
    assert_eq!(
        economics_payload.query,
        json!({ "name": "GDP", "from": "2025-01-01", "to": "2025-12-31" })
    );

    stock_list.assert_async().await;
    annual_report_mock.assert_async().await;
    economics.assert_async().await;
}

#[tokio::test]
async fn execute_flat_commands_use_endpoint_descriptors() {
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
            Command::CompanyProfile(symbol("AAPL")),
            json!({ "symbol": "AAPL" }),
        ),
        (
            "key-executives",
            Command::CompanyExecutives(symbol("AAPL")),
            json!({ "symbol": "AAPL" }),
        ),
        (
            "quote",
            Command::MarketQuote(symbol("AAPL")),
            json!({ "symbol": "AAPL" }),
        ),
        (
            "historical-price-eod/full",
            Command::MarketHistorical(symbol_date_range("AAPL")),
            json!({ "symbol": "AAPL", "from": "2025-01-01", "to": "2025-01-31" }),
        ),
        (
            "stock-peers",
            Command::CompanyPeers(symbol("AAPL")),
            json!({ "symbol": "AAPL" }),
        ),
        ("stock-list", Command::MarketStockList, json!({})),
        (
            "etf/holdings",
            Command::EtfHoldings(symbol("SPY")),
            json!({ "symbol": "SPY" }),
        ),
        (
            "dividends",
            Command::MarketDividends(symbol("AAPL")),
            json!({ "symbol": "AAPL" }),
        ),
        (
            "splits",
            Command::MarketSplits(symbol("AAPL")),
            json!({ "symbol": "AAPL" }),
        ),
        (
            "earnings-calendar",
            Command::EarningsCalendar(date_range()),
            json!({ "from": "2025-01-01", "to": "2025-01-31" }),
        ),
        (
            "treasury-rates",
            Command::TreasuryRates(date_range()),
            json!({ "from": "2025-01-01", "to": "2025-01-31" }),
        ),
        (
            "technical-indicators/sma",
            Command::TechnicalSma(TechnicalSmaArgs {
                symbol: "AAPL".to_owned(),
                period_length: 10,
                timeframe: "1day".to_owned(),
            }),
            json!({ "symbol": "AAPL", "periodLength": 10, "timeframe": "1day" }),
        ),
        (
            "stock-price-change",
            Command::MarketPriceChange(symbol("AAPL")),
            json!({ "symbol": "AAPL" }),
        ),
        (
            "sec-filings-search/symbol",
            Command::SecFilings(symbol_date_range("AAPL")),
            json!({ "symbol": "AAPL", "from": "2025-01-01", "to": "2025-01-31" }),
        ),
        (
            "income-statement",
            Command::FundamentalsIncomeStatement(annual("AAPL")),
            json!({ "symbol": "AAPL", "period": "annual", "limit": 5 }),
        ),
        (
            "income-statement-as-reported",
            Command::FundamentalsIncomeStatementAsReported(annual("AAPL")),
            json!({ "symbol": "AAPL", "period": "annual", "limit": 5 }),
        ),
        (
            "balance-sheet-statement",
            Command::FundamentalsBalanceSheet(annual("AAPL")),
            json!({ "symbol": "AAPL", "period": "annual", "limit": 5 }),
        ),
        (
            "cash-flow-statement",
            Command::FundamentalsCashFlow(annual("AAPL")),
            json!({ "symbol": "AAPL", "period": "annual", "limit": 5 }),
        ),
        (
            "ratios",
            Command::FundamentalsRatios(annual("AAPL")),
            json!({ "symbol": "AAPL", "period": "annual", "limit": 5 }),
        ),
        (
            "key-metrics",
            Command::FundamentalsMetrics(annual("AAPL")),
            json!({ "symbol": "AAPL", "period": "annual", "limit": 5 }),
        ),
        (
            "income-statement-growth",
            Command::FundamentalsIncomeStatementGrowth(annual("AAPL")),
            json!({ "symbol": "AAPL", "period": "annual", "limit": 5 }),
        ),
        (
            "balance-sheet-statement-growth",
            Command::FundamentalsBalanceSheetGrowth(annual("AAPL")),
            json!({ "symbol": "AAPL", "period": "annual", "limit": 5 }),
        ),
        (
            "cash-flow-statement-growth",
            Command::FundamentalsCashFlowGrowth(annual("AAPL")),
            json!({ "symbol": "AAPL", "period": "annual", "limit": 5 }),
        ),
        (
            "enterprise-values",
            Command::FundamentalsEnterpriseValues(annual("AAPL")),
            json!({ "symbol": "AAPL", "period": "annual", "limit": 5 }),
        ),
        (
            "financial-reports-json",
            Command::FundamentalsAnnualReportForm(annual_report("AAPL")),
            json!({ "symbol": "AAPL", "year": 2022, "period": "FY" }),
        ),
        (
            "financial-scores",
            Command::CompanyFinancialScores(symbol("AAPL")),
            json!({ "symbol": "AAPL" }),
        ),
        (
            "ratings-historical",
            Command::CompanyHistoricalRating(symbol_limit("AAPL")),
            json!({ "symbol": "AAPL", "limit": null }),
        ),
        (
            "insider-trading/latest",
            Command::InsiderTradingLatest(paged()),
            json!({ "page": 0, "limit": 3 }),
        ),
        (
            "economic-indicators",
            Command::EconomicIndicators(name_date_range("GDP")),
            json!({ "name": "GDP", "from": "2025-01-01", "to": "2025-12-31" }),
        ),
        (
            "analyst-estimates",
            Command::FundamentalsAnalystEstimates(annual("AAPL")),
            json!({ "symbol": "AAPL", "period": "annual", "limit": 5 }),
        ),
        (
            "news/stock",
            Command::NewsStock(stock_news("AAPL")),
            json!({ "symbol": "AAPL", "limit": null }),
        ),
    ];
    for (path, _, _) in &cases {
        server
            .mock_async(|when, then| {
                when.method(GET)
                    .path(format!("/{path}"))
                    .query_param("apikey", "test-key");
                then.status(200).json_body(json!([{ "ok": true }]));
            })
            .await;
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
async fn execute_date_range_commands_use_endpoint_descriptors() {
    let server = MockServer::start_async().await;
    let cases = [
        (
            "historical-price-eod/full",
            "AAPL",
            Command::MarketHistorical(symbol_date_range("AAPL")),
        ),
        (
            "historical-price-eod/full",
            "MSFT",
            Command::MarketHistorical(symbol_date_range("MSFT")),
        ),
        (
            "sec-filings-search/symbol",
            "IBM",
            Command::SecFilings(symbol_date_range("IBM")),
        ),
        (
            "historical-price-eod/full",
            "BTCUSD",
            Command::CryptoHistorical(symbol_date_range("BTCUSD")),
        ),
        (
            "historical-price-eod/full",
            "EURUSD",
            Command::ForexHistorical(symbol_date_range("EURUSD")),
        ),
    ];
    let mut mocks = Vec::new();

    for (path, symbol, _) in &cases {
        let symbol = *symbol;
        mocks.push(
            server
                .mock_async(|when, then| {
                    when.method(GET)
                        .path(format!("/{path}"))
                        .query_param("symbol", symbol)
                        .query_param("from", "2025-01-01")
                        .query_param("to", "2025-01-31")
                        .query_param("apikey", "test-key");
                    then.status(200).json_body(json!([{ "symbol": symbol }]));
                })
                .await,
        );
    }

    let client = test_client(&server);
    for (expected_path, symbol, command) in cases {
        let payload = execute(&client, &command).await.unwrap();

        assert_eq!(payload.endpoint, expected_path);
        assert_eq!(
            payload.query,
            json!({ "symbol": symbol, "from": "2025-01-01", "to": "2025-01-31" })
        );
        assert_eq!(payload.data[0]["symbol"], symbol);
    }

    for mock in mocks {
        mock.assert_async().await;
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

    let client = test_client(&server);
    let command = Command::SecFilings(optional_symbol_date_range("AAPL"));
    let payload = execute(&client, &command).await.unwrap();

    assert!(
        payload.query["from"].is_string(),
        "from should be defaulted to 90 days ago"
    );
    assert!(payload.query["to"].is_null());
}

#[tokio::test]
async fn sec_filings_legacy_alias_defaults_from_when_omitted() {
    let server = MockServer::start_async().await;
    server
        .mock_async(|when, then| {
            when.method(GET)
                .path("/sec-filings-search/symbol")
                .query_param("symbol", "TSLA")
                .query_param("apikey", "test-key");
            then.status(200).json_body(json!([{ "ok": true }]));
        })
        .await;

    let client = test_client(&server);
    let command = Command::SecFilings(optional_symbol_date_range("TSLA"));
    let payload = execute(&client, &command).await.unwrap();

    assert!(
        payload.query["from"].is_string(),
        "from should be defaulted to 90 days ago"
    );
    assert!(payload.query["to"].is_null());
}

#[test]
fn parses_flat_commands() {
    let cases: &[&[&str]] = &[
        &["fmp", "--api-key", "test-key", "search", "Apple"],
        &["fmp", "--api-key", "test-key", "company-profile", "AAPL"],
        &["fmp", "--api-key", "test-key", "company-executives", "AAPL"],
        &["fmp", "--api-key", "test-key", "company-peers", "AAPL"],
        &["fmp", "--api-key", "test-key", "market-stock-list"],
        &["fmp", "--api-key", "test-key", "etf-holdings", "SPY"],
        &[
            "fmp",
            "--api-key",
            "test-key",
            "company-financial-scores",
            "AAPL",
        ],
        &[
            "fmp",
            "--api-key",
            "test-key",
            "company-share-float",
            "AAPL",
        ],
        &["fmp", "--api-key", "test-key", "company-rating", "AAPL"],
        &[
            "fmp",
            "--api-key",
            "test-key",
            "company-historical-rating",
            "AAPL",
        ],
        &["fmp", "--api-key", "test-key", "market-quote", "AAPL"],
        &[
            "fmp",
            "--api-key",
            "test-key",
            "market-historical",
            "AAPL",
            "--from",
            "2025-01-01",
            "--to",
            "2025-01-31",
        ],
        &["fmp", "--api-key", "test-key", "market-dividends", "AAPL"],
        &["fmp", "--api-key", "test-key", "market-splits", "AAPL"],
        &[
            "fmp",
            "--api-key",
            "test-key",
            "market-price-change",
            "AAPL",
        ],
        &[
            "fmp",
            "--api-key",
            "test-key",
            "fundamentals-income-statement",
            "AAPL",
        ],
        &[
            "fmp",
            "--api-key",
            "test-key",
            "fundamentals-income-statement-as-reported",
            "AAPL",
        ],
        &[
            "fmp",
            "--api-key",
            "test-key",
            "fundamentals-balance-sheet",
            "AAPL",
        ],
        &[
            "fmp",
            "--api-key",
            "test-key",
            "fundamentals-cash-flow",
            "AAPL",
        ],
        &[
            "fmp",
            "--api-key",
            "test-key",
            "fundamentals-ratios",
            "AAPL",
        ],
        &[
            "fmp",
            "--api-key",
            "test-key",
            "fundamentals-metrics",
            "AAPL",
        ],
        &[
            "fmp",
            "--api-key",
            "test-key",
            "fundamentals-income-statement-growth",
            "AAPL",
        ],
        &[
            "fmp",
            "--api-key",
            "test-key",
            "fundamentals-balance-sheet-growth",
            "AAPL",
        ],
        &[
            "fmp",
            "--api-key",
            "test-key",
            "fundamentals-cash-flow-growth",
            "AAPL",
        ],
        &[
            "fmp",
            "--api-key",
            "test-key",
            "fundamentals-enterprise-values",
            "AAPL",
        ],
        &[
            "fmp",
            "--api-key",
            "test-key",
            "fundamentals-analyst-estimates",
            "AAPL",
        ],
        &[
            "fmp",
            "--api-key",
            "test-key",
            "fundamentals-report-dates",
            "AAPL",
        ],
        &[
            "fmp",
            "--api-key",
            "test-key",
            "fundamentals-annual-report-form",
            "AAPL",
            "--year",
            "2022",
        ],
        &[
            "fmp",
            "--api-key",
            "test-key",
            "analyst-price-target-consensus",
            "AAPL",
        ],
        &[
            "fmp",
            "--api-key",
            "test-key",
            "analyst-price-target-summary",
            "AAPL",
        ],
        &["fmp", "--api-key", "test-key", "analyst-grades", "AAPL"],
        &["fmp", "--api-key", "test-key", "insider-trading-latest"],
        &["fmp", "--api-key", "test-key", "economic-indicators", "GDP"],
        &[
            "fmp",
            "--api-key",
            "test-key",
            "earnings-calendar",
            "--from",
            "2025-01-01",
            "--to",
            "2025-01-31",
        ],
        &[
            "fmp",
            "--api-key",
            "test-key",
            "treasury-rates",
            "--from",
            "2025-01-01",
            "--to",
            "2025-01-31",
        ],
        &["fmp", "--api-key", "test-key", "technical-sma", "AAPL"],
        &["fmp", "--api-key", "test-key", "sec-filings", "AAPL"],
        &["fmp", "--api-key", "test-key", "crypto-list"],
        &["fmp", "--api-key", "test-key", "crypto-quote", "BTCUSD"],
        &[
            "fmp",
            "--api-key",
            "test-key",
            "crypto-historical",
            "BTCUSD",
        ],
        &["fmp", "--api-key", "test-key", "forex-quote", "EURUSD"],
        &["fmp", "--api-key", "test-key", "forex-historical", "EURUSD"],
        &["fmp", "--api-key", "test-key", "news-stock", "AAPL"],
        &["fmp", "--api-key", "test-key", "news-general"],
        &["fmp", "--api-key", "test-key", "news-articles"],
        &["fmp", "--api-key", "test-key", "news-forex"],
        &["fmp", "--api-key", "test-key", "news-crypto"],
    ];

    for args in cases {
        let result = Cli::try_parse_from(*args);
        assert!(
            result.is_ok(),
            "flat command should parse: {}",
            args[3..].join(" ")
        );
    }
}

#[test]
fn rejects_old_grouped_commands() {
    let cases: &[&[&str]] = &[
        &["fmp", "--api-key", "test-key", "company", "profile", "AAPL"],
        &["fmp", "--api-key", "test-key", "market", "quote", "AAPL"],
        &[
            "fmp",
            "--api-key",
            "test-key",
            "fundamentals",
            "income-statement",
            "AAPL",
        ],
        &["fmp", "--api-key", "test-key", "news", "stock", "AAPL"],
        &["fmp", "--api-key", "test-key", "technical", "sma", "AAPL"],
        &[
            "fmp",
            "--api-key",
            "test-key",
            "filings",
            "sec-filings",
            "AAPL",
        ],
        &["fmp", "--api-key", "test-key", "analyst", "grades", "AAPL"],
    ];

    for args in cases {
        let result = Cli::try_parse_from(*args);
        assert!(
            result.is_err(),
            "old grouped command should be rejected: {}",
            args[3..].join(" ")
        );
    }
}

#[test]
fn rejects_old_aliases() {
    let cases: &[&[&str]] = &[
        &["fmp", "--api-key", "test-key", "profile", "AAPL"],
        &["fmp", "--api-key", "test-key", "quote", "AAPL"],
        &["fmp", "--api-key", "test-key", "historical", "AAPL"],
        &["fmp", "--api-key", "test-key", "daily-chart", "AAPL"],
        &["fmp", "--api-key", "test-key", "company-stats", "AAPL"],
        &["fmp", "--api-key", "test-key", "key-executives", "AAPL"],
        &["fmp", "--api-key", "test-key", "news", "AAPL"],
        &["fmp", "--api-key", "test-key", "stock-news", "AAPL"],
    ];

    for args in cases {
        let result = Cli::try_parse_from(*args);
        assert!(
            result.is_err(),
            "old alias should be rejected: {}",
            args[3..].join(" ")
        );
    }
}

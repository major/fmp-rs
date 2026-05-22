use clap::Parser;
use httpmock::Method::GET;
use httpmock::MockServer;
use serde_json::{Value, json};

use super::commands::execute;
use super::output::{CommandPayload, render_output};
use super::*;

fn test_client(server: &MockServer) -> FmpClient {
    FmpClient::with_base_url("test-key", format!("{}/", server.base_url())).unwrap()
}

fn symbol(symbol: &str) -> SymbolArgs {
    SymbolArgs {
        symbol: symbol.to_owned(),
    }
}

fn annual(symbol: &str) -> AnnualArgs {
    AnnualArgs {
        symbol: symbol.to_owned(),
        limit: None,
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
    PagedArgs {
        page: Some(0),
        limit: Some(3),
    }
}

#[test]
fn parses_historical_command() {
    let cli = Cli::parse_from([
        "fmp",
        "--api-key",
        "test-key",
        "market",
        "historical",
        "AAPL",
        "--from",
        "2025-01-01",
        "--to",
        "2025-01-31",
    ]);

    assert!(matches!(
        cli.command,
        Command::Market(MarketCommand::Historical(_))
    ));
}

#[test]
fn render_output_returns_compact_json_envelope() {
    let payload = CommandPayload::new(
        "quote",
        json!({ "symbol": "AAPL" }),
        json!([{ "symbol": "AAPL", "price": 200.0, "volume": 1000 }]),
    );

    let output = render_output(payload).unwrap();

    assert!(!output.contains('\n'));

    let output: Value = serde_json::from_str(&output).unwrap();

    assert_eq!(output["ok"], true);
    assert_eq!(output["endpoint"], "quote");
    assert_eq!(output["query"], json!({ "symbol": "AAPL" }));
    assert_eq!(
        output["data"],
        json!([{ "symbol": "AAPL", "price": 200.0, "volume": 1000 }])
    );
}

#[test]
fn parses_legacy_historical_alias() {
    let cli = Cli::parse_from([
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

    assert!(matches!(cli.command, Command::Historical(_)));
}

#[test]
fn parses_technical_sma_defaults() {
    let cli = Cli::parse_from(["fmp", "--api-key", "test-key", "technical", "sma", "AAPL"]);

    let Command::Technical(TechnicalCommand::Sma(args)) = cli.command else {
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
        "news",
        "stock",
        "AAPL",
        "--limit",
        "3",
    ]);

    let Command::News(NewsArgs {
        command: Some(NewsCommand::Stock(args)),
        symbol: None,
        limit: None,
    }) = cli.command
    else {
        panic!("expected grouped news command");
    };
    assert_eq!(args.symbol, "AAPL");
    assert_eq!(args.limit, Some(3));
}

#[test]
fn parses_paginated_news_commands() {
    let cases = [
        ("general", "general"),
        ("articles", "articles"),
        ("forex", "forex"),
        ("crypto", "crypto"),
    ];

    for (subcommand, expected) in cases {
        let cli = Cli::parse_from([
            "fmp",
            "--api-key",
            "test-key",
            "news",
            subcommand,
            "--page",
            "0",
            "--limit",
            "3",
        ]);

        let Command::News(NewsArgs {
            command: Some(command),
            symbol: None,
            limit: None,
        }) = cli.command
        else {
            panic!("expected grouped news command");
        };

        let args = match command {
            NewsCommand::General(args) if expected == "general" => args,
            NewsCommand::Articles(args) if expected == "articles" => args,
            NewsCommand::Forex(args) if expected == "forex" => args,
            NewsCommand::Crypto(args) if expected == "crypto" => args,
            _ => panic!("unexpected news command"),
        };
        assert_eq!(args.page, Some(0));
        assert_eq!(args.limit, Some(3));
    }
}

#[test]
fn parses_new_symbol_commands() {
    let cases = [
        (
            [
                "fmp",
                "--api-key",
                "test-key",
                "company",
                "share-float",
                "AAPL",
            ],
            "company share-float",
        ),
        (
            ["fmp", "--api-key", "test-key", "company", "rating", "AAPL"],
            "company rating",
        ),
        (
            [
                "fmp",
                "--api-key",
                "test-key",
                "fundamentals",
                "report-dates",
                "AAPL",
            ],
            "fundamentals report-dates",
        ),
        (
            [
                "fmp",
                "--api-key",
                "test-key",
                "analyst",
                "price-target-consensus",
                "AAPL",
            ],
            "analyst price-target-consensus",
        ),
        (
            [
                "fmp",
                "--api-key",
                "test-key",
                "analyst",
                "price-target-summary",
                "AAPL",
            ],
            "analyst price-target-summary",
        ),
        (
            ["fmp", "--api-key", "test-key", "analyst", "grades", "AAPL"],
            "analyst grades",
        ),
    ];

    for (args, expected) in cases {
        let cli = Cli::parse_from(args);

        match expected {
            "company share-float" => {
                assert!(matches!(
                    cli.command,
                    Command::Company(CompanyCommand::ShareFloat(_))
                ));
            }
            "company rating" => {
                assert!(matches!(
                    cli.command,
                    Command::Company(CompanyCommand::Rating(_))
                ));
            }
            "fundamentals report-dates" => {
                assert!(matches!(
                    cli.command,
                    Command::Fundamentals(FundamentalsCommand::ReportDates(_))
                ));
            }
            "analyst price-target-consensus" => {
                assert!(matches!(
                    cli.command,
                    Command::Analyst(AnalystCommand::PriceTargetConsensus(_))
                ));
            }
            "analyst price-target-summary" => {
                assert!(matches!(
                    cli.command,
                    Command::Analyst(AnalystCommand::PriceTargetSummary(_))
                ));
            }
            "analyst grades" => {
                assert!(matches!(
                    cli.command,
                    Command::Analyst(AnalystCommand::Grades(_))
                ));
            }
            _ => unreachable!(),
        }
    }
}

#[test]
fn parses_crypto_and_forex_commands() {
    let cases: &[(&[&str], &str)] = &[
        (
            &["fmp", "--api-key", "test-key", "crypto", "list"],
            "crypto list",
        ),
        (
            &["fmp", "--api-key", "test-key", "crypto", "quote", "BTCUSD"],
            "crypto quote",
        ),
        (
            &[
                "fmp",
                "--api-key",
                "test-key",
                "crypto",
                "historical",
                "BTCUSD",
            ],
            "crypto historical",
        ),
        (
            &["fmp", "--api-key", "test-key", "forex", "quote", "EURUSD"],
            "forex quote",
        ),
        (
            &[
                "fmp",
                "--api-key",
                "test-key",
                "forex",
                "historical",
                "EURUSD",
            ],
            "forex historical",
        ),
    ];

    for (args, expected) in cases {
        let cli = Cli::parse_from(*args);

        match *expected {
            "crypto list" => assert!(matches!(cli.command, Command::Crypto(CryptoCommand::List))),
            "crypto quote" => assert!(matches!(
                cli.command,
                Command::Crypto(CryptoCommand::Quote(_))
            )),
            "crypto historical" => assert!(matches!(
                cli.command,
                Command::Crypto(CryptoCommand::Historical(_))
            )),
            "forex quote" => assert!(matches!(
                cli.command,
                Command::Forex(ForexCommand::Quote(_))
            )),
            "forex historical" => assert!(matches!(
                cli.command,
                Command::Forex(ForexCommand::Historical(_))
            )),
            _ => unreachable!(),
        }
    }
}

#[test]
fn parses_legacy_news_alias() {
    let cli = Cli::parse_from([
        "fmp",
        "--api-key",
        "test-key",
        "news",
        "AAPL",
        "--limit",
        "3",
    ]);

    let Command::News(NewsArgs {
        command: None,
        symbol: Some(symbol),
        limit,
    }) = cli.command
    else {
        panic!("expected legacy news command");
    };
    assert_eq!(symbol, "AAPL");
    assert_eq!(limit, Some(3));
}

#[tokio::test]
async fn execute_annual_command_uses_endpoint_descriptor() {
    let server = MockServer::start_async().await;
    let cases = [
        (
            "income-statement",
            Command::Fundamentals(FundamentalsCommand::IncomeStatement(annual("AAPL"))),
        ),
        (
            "income-statement-as-reported",
            Command::Fundamentals(FundamentalsCommand::IncomeStatementAsReported(annual(
                "AAPL",
            ))),
        ),
        (
            "balance-sheet-statement-growth",
            Command::Fundamentals(FundamentalsCommand::BalanceSheetGrowth(annual("AAPL"))),
        ),
        (
            "cash-flow-statement-growth",
            Command::Fundamentals(FundamentalsCommand::CashFlowGrowth(annual("AAPL"))),
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
            json!({ "symbol": "AAPL", "period": "annual", "limit": null })
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
        (
            "profile",
            Command::Company(CompanyCommand::Stats(symbol("AAPL"))),
        ),
        (
            "stock-peers",
            Command::Company(CompanyCommand::Peers(symbol("AAPL"))),
        ),
        (
            "key-executives",
            Command::Company(CompanyCommand::Executives(symbol("AAPL"))),
        ),
        (
            "dividends",
            Command::Market(MarketCommand::Dividends(symbol("AAPL"))),
        ),
        (
            "splits",
            Command::Market(MarketCommand::Splits(symbol("AAPL"))),
        ),
        (
            "stock-price-change",
            Command::Market(MarketCommand::PriceChange(symbol("AAPL"))),
        ),
        (
            "financial-scores",
            Command::Company(CompanyCommand::FinancialScores(symbol("AAPL"))),
        ),
        (
            "shares-float",
            Command::Company(CompanyCommand::ShareFloat(symbol("AAPL"))),
        ),
        (
            "grades-consensus",
            Command::Company(CompanyCommand::Rating(symbol("AAPL"))),
        ),
        (
            "financial-reports-dates",
            Command::Fundamentals(FundamentalsCommand::ReportDates(symbol("AAPL"))),
        ),
        (
            "price-target-consensus",
            Command::Analyst(AnalystCommand::PriceTargetConsensus(symbol("AAPL"))),
        ),
        (
            "price-target-summary",
            Command::Analyst(AnalystCommand::PriceTargetSummary(symbol("AAPL"))),
        ),
        (
            "grades",
            Command::Analyst(AnalystCommand::Grades(symbol("AAPL"))),
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
        assert_eq!(payload.query, json!({ "symbol": "AAPL" }));
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
        (
            "BTCUSD",
            Command::Crypto(CryptoCommand::Quote(symbol("BTCUSD"))),
        ),
        (
            "EURUSD",
            Command::Forex(ForexCommand::Quote(symbol("EURUSD"))),
        ),
    ];
    let historical_cases = [
        (
            "BTCUSD",
            Command::Crypto(CryptoCommand::Historical(optional_symbol_date_range(
                "BTCUSD",
            ))),
        ),
        (
            "EURUSD",
            Command::Forex(ForexCommand::Historical(optional_symbol_date_range(
                "EURUSD",
            ))),
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
    let payload = execute(&client, &Command::Crypto(CryptoCommand::List))
        .await
        .unwrap();
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
        Command::Calendar(CalendarCommand::Earnings(date_range())),
        Command::Rates(RatesCommand::Treasury(date_range())),
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
    let command = Command::Technical(TechnicalCommand::Sma(TechnicalSmaArgs {
        symbol: "AAPL".to_owned(),
        period_length: 10,
        timeframe: "1day".to_owned(),
    }));

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
        Command::News(NewsArgs {
            command: Some(NewsCommand::Stock(StockNewsArgs {
                limit: Some(3),
                ..stock_news("AAPL")
            })),
            symbol: None,
            limit: None,
        }),
        Command::News(NewsArgs {
            command: None,
            symbol: Some("MSFT".to_owned()),
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
        (
            "news/general-latest",
            0,
            3,
            Command::News(NewsArgs {
                command: Some(NewsCommand::General(paged())),
                symbol: None,
                limit: None,
            }),
        ),
        (
            "fmp-articles",
            0,
            3,
            Command::News(NewsArgs {
                command: Some(NewsCommand::Articles(paged())),
                symbol: None,
                limit: None,
            }),
        ),
        (
            "news/forex-latest",
            0,
            3,
            Command::News(NewsArgs {
                command: Some(NewsCommand::Forex(paged())),
                symbol: None,
                limit: None,
            }),
        ),
        (
            "news/crypto-latest",
            0,
            3,
            Command::News(NewsArgs {
                command: Some(NewsCommand::Crypto(paged())),
                symbol: None,
                limit: None,
            }),
        ),
        (
            "news/general-latest",
            0,
            10,
            Command::News(NewsArgs {
                command: Some(NewsCommand::General(PagedArgs {
                    page: None,
                    limit: None,
                })),
                symbol: None,
                limit: None,
            }),
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
async fn execute_legacy_aliases_use_endpoint_descriptors() {
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
            Command::Profile(symbol("AAPL")),
            json!({ "symbol": "AAPL" }),
        ),
        (
            "key-executives",
            Command::KeyExecutives(symbol("AAPL")),
            json!({ "symbol": "AAPL" }),
        ),
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
            "historical-price-eod/full",
            Command::DailyChart(symbol_date_range("MSFT")),
            json!({ "symbol": "MSFT", "from": "2025-01-01", "to": "2025-01-31" }),
        ),
        (
            "stock-peers",
            Command::StockPeers(symbol("AAPL")),
            json!({ "symbol": "AAPL" }),
        ),
        (
            "dividends",
            Command::Dividends(symbol("AAPL")),
            json!({ "symbol": "AAPL" }),
        ),
        (
            "splits",
            Command::Splits(symbol("AAPL")),
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
            Command::StockPriceChange(symbol("AAPL")),
            json!({ "symbol": "AAPL" }),
        ),
        (
            "sec-filings-search/symbol",
            Command::SecFilings(symbol_date_range("AAPL")),
            json!({ "symbol": "AAPL", "from": "2025-01-01", "to": "2025-01-31" }),
        ),
        (
            "income-statement",
            Command::IncomeStatement(annual("AAPL")),
            json!({ "symbol": "AAPL", "period": "annual", "limit": null }),
        ),
        (
            "income-statement-as-reported",
            Command::IncomeStatementAsReported(annual("AAPL")),
            json!({ "symbol": "AAPL", "period": "annual", "limit": null }),
        ),
        (
            "balance-sheet-statement",
            Command::BalanceSheet(annual("AAPL")),
            json!({ "symbol": "AAPL", "period": "annual", "limit": null }),
        ),
        (
            "cash-flow-statement",
            Command::CashFlow(annual("AAPL")),
            json!({ "symbol": "AAPL", "period": "annual", "limit": null }),
        ),
        (
            "ratios",
            Command::Ratios(annual("AAPL")),
            json!({ "symbol": "AAPL", "period": "annual", "limit": null }),
        ),
        (
            "key-metrics",
            Command::Metrics(annual("AAPL")),
            json!({ "symbol": "AAPL", "period": "annual", "limit": null }),
        ),
        (
            "profile",
            Command::CompanyStats(symbol("MSFT")),
            json!({ "symbol": "MSFT" }),
        ),
        (
            "income-statement-growth",
            Command::IncomeStatementGrowth(annual("AAPL")),
            json!({ "symbol": "AAPL", "period": "annual", "limit": null }),
        ),
        (
            "balance-sheet-statement-growth",
            Command::BalanceSheetGrowth(annual("AAPL")),
            json!({ "symbol": "AAPL", "period": "annual", "limit": null }),
        ),
        (
            "cash-flow-statement-growth",
            Command::CashFlowGrowth(annual("AAPL")),
            json!({ "symbol": "AAPL", "period": "annual", "limit": null }),
        ),
        (
            "enterprise-values",
            Command::EnterpriseValues(annual("AAPL")),
            json!({ "symbol": "AAPL", "period": "annual", "limit": null }),
        ),
        (
            "financial-scores",
            Command::FinancialScores(symbol("AAPL")),
            json!({ "symbol": "AAPL" }),
        ),
        (
            "analyst-estimates",
            Command::AnalystEstimates(annual("AAPL")),
            json!({ "symbol": "AAPL", "period": "annual", "limit": null }),
        ),
        (
            "news/stock",
            Command::StockNews(stock_news("AAPL")),
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
            Command::Market(MarketCommand::Historical(symbol_date_range("AAPL"))),
        ),
        (
            "historical-price-eod/full",
            "MSFT",
            Command::Market(MarketCommand::DailyChart(symbol_date_range("MSFT"))),
        ),
        (
            "sec-filings-search/symbol",
            "IBM",
            Command::Filings(FilingsCommand::SecFilings(symbol_date_range("IBM"))),
        ),
        (
            "historical-price-eod/full",
            "BTCUSD",
            Command::Crypto(CryptoCommand::Historical(symbol_date_range("BTCUSD"))),
        ),
        (
            "historical-price-eod/full",
            "EURUSD",
            Command::Forex(ForexCommand::Historical(symbol_date_range("EURUSD"))),
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
    let command = Command::Filings(FilingsCommand::SecFilings(optional_symbol_date_range(
        "AAPL",
    )));
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

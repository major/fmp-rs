//! HTTP client for Financial Modeling Prep stable endpoints.
//!
//! The client exposes a small set of *shape*-based methods. Each method
//! corresponds to a common FMP query shape (no parameters, symbol only, date
//! range, annual statement, etc.) and accepts an [`Endpoint`] descriptor from
//! [`crate::endpoint`].
//!
//! # Errors
//!
//! All request methods return [`Error`] when:
//!
//! - the HTTP request fails before producing a response,
//! - the API returns a non-2xx status, or
//! - the response body is not valid JSON.
//!
//! # Logging
//!
//! The client emits [`log`] crate traces at `DEBUG` level for each HTTP
//! request (URL with API key redacted) and response status, and at `WARN`
//! level for non-2xx API responses. Enable a log subscriber such as
//! `env_logger` to surface these traces.

use std::fmt;

use reqwest::{StatusCode, Url};
use serde_json::Value;

use crate::endpoint::{ANNUAL_LIMIT, ANNUAL_PERIOD, Endpoint, NEWS_LIMIT, PAGE};
use crate::error::{Error, Result};

const DEFAULT_BASE_URL: &str = "https://financialmodelingprep.com/stable/";

/// Async client for Financial Modeling Prep stable API endpoints.
#[derive(Clone)]
pub struct FmpClient {
    http: reqwest::Client,
    base_url: Url,
    api_key: String,
}

impl fmt::Debug for FmpClient {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("FmpClient")
            .field("http", &self.http)
            .field("base_url", &self.base_url)
            .field("api_key", &"<redacted>")
            .finish()
    }
}

impl FmpClient {
    /// Builds a client using the default stable FMP base URL.
    ///
    /// # Errors
    ///
    /// Returns [`Error::InvalidBaseUrl`] only if the compiled-in default URL is invalid.
    pub fn new(api_key: impl Into<String>) -> Result<Self> {
        Self::with_base_url(api_key, DEFAULT_BASE_URL)
    }

    /// Builds a client using a custom base URL, primarily for tests.
    ///
    /// # Errors
    ///
    /// Returns [`Error::InvalidBaseUrl`] if `base_url` cannot be parsed, uses
    /// an unsupported scheme, contains credentials, or contains query or
    /// fragment components.
    pub fn with_base_url(api_key: impl Into<String>, base_url: impl AsRef<str>) -> Result<Self> {
        let base_url = normalize_base_url(base_url.as_ref())?;

        Ok(Self {
            http: reqwest::Client::new(),
            base_url,
            api_key: api_key.into(),
        })
    }

    /// Calls `endpoint` without endpoint-specific query parameters.
    ///
    /// # Errors
    ///
    /// See [module-level errors](self#errors).
    pub async fn endpoint(&self, endpoint: Endpoint) -> Result<Value> {
        self.get(endpoint, &[]).await
    }

    /// Calls `endpoint` with a free-text `query` parameter (`?query=...`).
    ///
    /// # Errors
    ///
    /// See [module-level errors](self#errors).
    pub async fn query(&self, endpoint: Endpoint, query: &str) -> Result<Value> {
        self.get(endpoint, &[("query", query)]).await
    }

    /// Calls `endpoint` with a `?symbol=...` parameter.
    ///
    /// # Errors
    ///
    /// See [module-level errors](self#errors).
    pub async fn by_symbol(&self, endpoint: Endpoint, symbol: &str) -> Result<Value> {
        self.get(endpoint, &[("symbol", symbol)]).await
    }

    /// Calls `endpoint` with `?symbol=...&limit=...` (limit defaults to
    /// [`NEWS_LIMIT`]).
    ///
    /// # Errors
    ///
    /// See [module-level errors](self#errors).
    pub async fn by_symbol_limit(
        &self,
        endpoint: Endpoint,
        symbol: &str,
        limit: Option<u16>,
    ) -> Result<Value> {
        let limit = limit.unwrap_or(NEWS_LIMIT).to_string();
        self.get(endpoint, &[("symbol", symbol), ("limit", &limit)])
            .await
    }

    /// Calls `endpoint` with `?symbol=...` plus optional `from` and `to` dates.
    ///
    /// # Errors
    ///
    /// See [module-level errors](self#errors).
    pub async fn by_symbol_date_range(
        &self,
        endpoint: Endpoint,
        symbol: &str,
        from: Option<&str>,
        to: Option<&str>,
    ) -> Result<Value> {
        let mut params = vec![("symbol", symbol)];
        push_opt(&mut params, "from", from);
        push_opt(&mut params, "to", to);
        self.get(endpoint, &params).await
    }

    /// Calls `endpoint` with optional `from` and `to` dates.
    ///
    /// # Errors
    ///
    /// See [module-level errors](self#errors).
    pub async fn by_date_range(
        &self,
        endpoint: Endpoint,
        from: Option<&str>,
        to: Option<&str>,
    ) -> Result<Value> {
        let mut params = Vec::new();
        push_opt(&mut params, "from", from);
        push_opt(&mut params, "to", to);
        self.get(endpoint, &params).await
    }

    /// Calls `endpoint` with a required `name` plus optional `from` and `to` dates.
    ///
    /// # Errors
    ///
    /// See [module-level errors](self#errors).
    pub async fn by_name_date_range(
        &self,
        endpoint: Endpoint,
        name: &str,
        from: Option<&str>,
        to: Option<&str>,
    ) -> Result<Value> {
        let mut params = vec![("name", name)];
        push_opt(&mut params, "from", from);
        push_opt(&mut params, "to", to);
        self.get(endpoint, &params).await
    }

    /// Calls a statement-style `endpoint` with `symbol`, fixed `period=annual`,
    /// and `limit` (defaulting to [`ANNUAL_LIMIT`]).
    ///
    /// # Errors
    ///
    /// See [module-level errors](self#errors).
    pub async fn annual(
        &self,
        endpoint: Endpoint,
        symbol: &str,
        limit: Option<u16>,
    ) -> Result<Value> {
        let limit = limit.unwrap_or(ANNUAL_LIMIT).to_string();
        self.get(
            endpoint,
            &[
                ("symbol", symbol),
                ("period", ANNUAL_PERIOD),
                ("limit", &limit),
            ],
        )
        .await
    }

    /// Calls an annual report form `endpoint` with `symbol`, `year`, and fiscal `period`.
    ///
    /// # Errors
    ///
    /// See [module-level errors](self#errors).
    pub async fn annual_report_form(
        &self,
        endpoint: Endpoint,
        symbol: &str,
        year: u16,
        period: &str,
    ) -> Result<Value> {
        let year = year.to_string();
        self.get(
            endpoint,
            &[("symbol", symbol), ("year", &year), ("period", period)],
        )
        .await
    }

    /// Calls a technical indicator `endpoint` with `symbol`, `periodLength`,
    /// and `timeframe`.
    ///
    /// # Errors
    ///
    /// See [module-level errors](self#errors).
    pub async fn technical(
        &self,
        endpoint: Endpoint,
        symbol: &str,
        period_length: u16,
        timeframe: &str,
    ) -> Result<Value> {
        let period_length = period_length.to_string();
        self.get(
            endpoint,
            &[
                ("symbol", symbol),
                ("periodLength", &period_length),
                ("timeframe", timeframe),
            ],
        )
        .await
    }

    /// Calls a news `endpoint` with `?symbols=...&limit=...` (limit defaults to
    /// [`NEWS_LIMIT`]).
    ///
    /// # Errors
    ///
    /// See [module-level errors](self#errors).
    pub async fn news(
        &self,
        endpoint: Endpoint,
        symbol: &str,
        limit: Option<u16>,
    ) -> Result<Value> {
        let limit = limit.unwrap_or(NEWS_LIMIT).to_string();
        self.get(endpoint, &[("symbols", symbol), ("limit", &limit)])
            .await
    }

    /// Calls a paginated `endpoint` with `?page=...&limit=...` (`page`
    /// defaults to [`PAGE`] and `limit` defaults to [`NEWS_LIMIT`]).
    ///
    /// # Errors
    ///
    /// See [module-level errors](self#errors).
    pub async fn paged(
        &self,
        endpoint: Endpoint,
        page: Option<u16>,
        limit: Option<u16>,
    ) -> Result<Value> {
        let page = page.unwrap_or(PAGE).to_string();
        let limit = limit.unwrap_or(NEWS_LIMIT).to_string();
        self.get(endpoint, &[("page", &page), ("limit", &limit)])
            .await
    }

    async fn get(&self, endpoint: Endpoint, params: &[(&str, &str)]) -> Result<Value> {
        let url = self.build_url(endpoint.path(), params)?;
        log::debug!("GET {}", self.redact_url(&url));
        let response = self.http.get(url).send().await?;
        let status = response.status();
        log::debug!("response status: {status}");
        let body = response.text().await?;

        if !status.is_success() {
            let summary = summarize_body(&body);
            log::warn!("API error {}: {}", status, summary);
            return Err(Error::Api {
                status: status.as_u16(),
                message: summary,
            });
        }

        Ok(serde_json::from_str(&body)?)
    }

    fn build_url(&self, path: &str, params: &[(&str, &str)]) -> Result<Url> {
        let mut url = self
            .base_url
            .join(path)
            .map_err(|_| Error::InvalidBaseUrl(self.base_url.to_string()))?;
        {
            let mut query = url.query_pairs_mut();
            for (key, value) in params {
                query.append_pair(key, value);
            }
            query.append_pair("apikey", &self.api_key);
        }
        Ok(url)
    }

    fn redact_url(&self, url: &Url) -> Url {
        let mut display = url.clone();
        let pairs: Vec<(String, String)> = url
            .query_pairs()
            .map(|(k, v)| {
                if k == "apikey" {
                    (k.into_owned(), "***".to_owned())
                } else {
                    (k.into_owned(), v.into_owned())
                }
            })
            .collect();
        display.set_query(None);
        {
            let mut query = display.query_pairs_mut();
            for (k, v) in &pairs {
                query.append_pair(k, v);
            }
        }
        display
    }
}

fn push_opt<'a>(params: &mut Vec<(&'a str, &'a str)>, key: &'a str, value: Option<&'a str>) {
    if let Some(value) = value {
        params.push((key, value));
    }
}

fn normalize_base_url(base_url: &str) -> Result<Url> {
    let mut url =
        Url::parse(base_url).map_err(|_| Error::InvalidBaseUrl("<invalid URL>".to_owned()))?;

    if !matches!(url.scheme(), "http" | "https")
        || !url.username().is_empty()
        || url.password().is_some()
    {
        return Err(Error::InvalidBaseUrl(sanitize_base_url(&url)));
    }

    if url.query().is_some() || url.fragment().is_some() {
        return Err(Error::InvalidBaseUrl(sanitize_base_url(&url)));
    }

    if !url.path().ends_with('/') {
        let mut path = url.path().to_owned();
        path.push('/');
        url.set_path(&path);
    }

    Ok(url)
}

fn sanitize_base_url(url: &Url) -> String {
    let mut sanitized = url.clone();
    let _ = sanitized.set_username("");
    let _ = sanitized.set_password(None);
    sanitized.set_query(None);
    sanitized.set_fragment(None);
    sanitized.to_string()
}

fn summarize_body(body: &str) -> String {
    let trimmed = body.trim();
    if trimmed.is_empty() {
        return StatusCode::INTERNAL_SERVER_ERROR
            .canonical_reason()
            .unwrap_or("empty response")
            .to_owned();
    }

    trimmed.chars().take(240).collect()
}

#[cfg(test)]
mod tests {
    use httpmock::Method::GET;
    use httpmock::MockServer;
    use serde_json::json;

    use super::*;
    use crate::endpoint::{
        ANALYST_ESTIMATES, BALANCE_SHEET_STATEMENT, BALANCE_SHEET_STATEMENT_GROWTH,
        CASH_FLOW_STATEMENT, CASH_FLOW_STATEMENT_GROWTH, CRYPTO_NEWS, CRYPTOCURRENCY_LIST,
        DIVIDENDS, EARNINGS_CALENDAR, ECONOMIC_INDICATORS, ENTERPRISE_VALUES, ETF_HOLDINGS,
        FINANCIAL_REPORTS_DATES, FINANCIAL_REPORTS_JSON, FINANCIAL_SCORES, FMP_ARTICLES,
        FOREX_NEWS, GENERAL_NEWS, GRADES, GRADES_CONSENSUS, HISTORICAL_PRICE_EOD_FULL,
        INCOME_STATEMENT, INCOME_STATEMENT_AS_REPORTED, INCOME_STATEMENT_GROWTH,
        INSIDER_TRADING_LATEST, KEY_EXECUTIVES, KEY_METRICS, PRICE_TARGET_CONSENSUS,
        PRICE_TARGET_SUMMARY, PROFILE, QUOTE, RATINGS_HISTORICAL, RATIOS, SEARCH_SYMBOL,
        SEC_FILINGS_SEARCH_SYMBOL, SHARES_FLOAT, SPLITS, STOCK_LIST, STOCK_PEERS,
        STOCK_PRICE_CHANGE, TECHNICAL_SMA, TREASURY_RATES,
    };

    /// Returns a client wired to `server`'s base URL with `api_key=test-key`.
    fn test_client(server: &MockServer) -> FmpClient {
        FmpClient::with_base_url("test-key", format!("{}/", server.base_url())).unwrap()
    }

    #[test]
    fn debug_output_redacts_api_key() {
        let client = FmpClient::with_base_url("secret-key", DEFAULT_BASE_URL).unwrap();

        let output = format!("{client:?}");

        assert!(output.contains("<redacted>"));
        assert!(!output.contains("secret-key"));
    }

    #[test]
    fn with_base_url_adds_trailing_slash() {
        let client = FmpClient::with_base_url("test-key", "https://example.com/stable").unwrap();
        let url = client.build_url("quote", &[]).unwrap();

        assert_eq!(
            url.as_str(),
            "https://example.com/stable/quote?apikey=test-key"
        );
    }

    #[test]
    fn with_base_url_preserves_existing_trailing_slash() {
        let client = FmpClient::with_base_url("test-key", "https://example.com/stable/").unwrap();
        let url = client.build_url("quote", &[]).unwrap();

        assert_eq!(
            url.as_str(),
            "https://example.com/stable/quote?apikey=test-key"
        );
    }

    #[test]
    fn with_base_url_rejects_query_and_fragment() {
        let query_error =
            FmpClient::with_base_url("test-key", "https://example.com/stable?token=secret-token")
                .unwrap_err()
                .to_string();
        assert!(query_error.contains("https://example.com/stable"));
        assert!(!query_error.contains("secret-token"));

        let fragment_error =
            FmpClient::with_base_url("test-key", "https://example.com/stable#secret-fragment")
                .unwrap_err()
                .to_string();
        assert!(fragment_error.contains("https://example.com/stable"));
        assert!(!fragment_error.contains("secret-fragment"));
    }

    #[test]
    fn with_base_url_rejects_credentials_and_unsupported_schemes() {
        let credentials_error = FmpClient::with_base_url(
            "test-key",
            "https://user:secret-password@example.com/stable",
        )
        .unwrap_err()
        .to_string();
        assert!(credentials_error.contains("https://example.com/stable"));
        assert!(!credentials_error.contains("secret-password"));

        assert!(matches!(
            FmpClient::with_base_url("test-key", "ftp://example.com/stable"),
            Err(Error::InvalidBaseUrl(_))
        ));
    }

    #[tokio::test]
    async fn endpoint_sends_expected_request() {
        let server = MockServer::start_async().await;
        let endpoints = [CRYPTOCURRENCY_LIST, STOCK_LIST];
        let mut mocks = Vec::new();

        for endpoint in endpoints {
            mocks.push(
                server
                    .mock_async(|when, then| {
                        when.method(GET)
                            .path(format!("/{}", endpoint.path()))
                            .query_param("apikey", "test-key");
                        then.status(200).json_body(json!([{ "ok": true }]));
                    })
                    .await,
            );
        }

        let client = test_client(&server);
        for endpoint in endpoints {
            let value = client.endpoint(endpoint).await.unwrap();
            assert_eq!(value[0]["ok"], true);
        }

        for mock in mocks {
            mock.assert_async().await;
        }
    }

    #[tokio::test]
    async fn query_sends_expected_request() {
        let server = MockServer::start_async().await;
        let mock = server
            .mock_async(|when, then| {
                when.method(GET)
                    .path("/search-symbol")
                    .query_param("query", "AAPL")
                    .query_param("apikey", "test-key");
                then.status(200)
                    .json_body(json!([{ "symbol": "AAPL", "name": "Apple Inc." }]));
            })
            .await;

        let value = test_client(&server)
            .query(SEARCH_SYMBOL, "AAPL")
            .await
            .unwrap();

        mock.assert_async().await;
        assert_eq!(value[0]["symbol"], "AAPL");
    }

    #[tokio::test]
    async fn by_symbol_sends_expected_requests() {
        let server = MockServer::start_async().await;
        let endpoints = [
            PROFILE,
            KEY_EXECUTIVES,
            QUOTE,
            STOCK_PEERS,
            ETF_HOLDINGS,
            DIVIDENDS,
            SPLITS,
            STOCK_PRICE_CHANGE,
            FINANCIAL_SCORES,
            SHARES_FLOAT,
            GRADES_CONSENSUS,
            FINANCIAL_REPORTS_DATES,
            GRADES,
            PRICE_TARGET_CONSENSUS,
            PRICE_TARGET_SUMMARY,
        ];
        let mut mocks = Vec::new();

        for endpoint in endpoints {
            mocks.push(
                server
                    .mock_async(|when, then| {
                        when.method(GET)
                            .path(format!("/{}", endpoint.path()))
                            .query_param("symbol", "AAPL")
                            .query_param("apikey", "test-key");
                        then.status(200).json_body(json!([{ "symbol": "AAPL" }]));
                    })
                    .await,
            );
        }

        let client = test_client(&server);
        for endpoint in endpoints {
            client.by_symbol(endpoint, "AAPL").await.unwrap();
        }

        for mock in mocks {
            mock.assert_async().await;
        }
    }

    #[tokio::test]
    async fn by_symbol_limit_sends_expected_request() {
        let server = MockServer::start_async().await;
        let mock = server
            .mock_async(|when, then| {
                when.method(GET)
                    .path("/ratings-historical")
                    .query_param("symbol", "AAPL")
                    .query_param("limit", "10")
                    .query_param("apikey", "test-key");
                then.status(200).json_body(json!([{ "symbol": "AAPL" }]));
            })
            .await;

        let value = test_client(&server)
            .by_symbol_limit(RATINGS_HISTORICAL, "AAPL", None)
            .await
            .unwrap();

        mock.assert_async().await;
        assert_eq!(value[0]["symbol"], "AAPL");
    }

    #[tokio::test]
    async fn by_date_range_sends_expected_requests() {
        let server = MockServer::start_async().await;
        let endpoints = [EARNINGS_CALENDAR, TREASURY_RATES];
        let mut mocks = Vec::new();

        for endpoint in endpoints {
            mocks.push(
                server
                    .mock_async(|when, then| {
                        when.method(GET)
                            .path(format!("/{}", endpoint.path()))
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
        for endpoint in endpoints {
            client
                .by_date_range(endpoint, Some("2025-01-01"), Some("2025-01-31"))
                .await
                .unwrap();
        }

        for mock in mocks {
            mock.assert_async().await;
        }
    }

    #[tokio::test]
    async fn by_name_date_range_sends_expected_request() {
        let server = MockServer::start_async().await;
        let mock = server
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

        let value = test_client(&server)
            .by_name_date_range(
                ECONOMIC_INDICATORS,
                "GDP",
                Some("2025-01-01"),
                Some("2025-12-31"),
            )
            .await
            .unwrap();

        mock.assert_async().await;
        assert_eq!(value[0]["name"], "GDP");
    }

    #[tokio::test]
    async fn technical_sends_expected_request() {
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

        let value = test_client(&server)
            .technical(TECHNICAL_SMA, "AAPL", 10, "1day")
            .await
            .unwrap();

        mock.assert_async().await;
        assert_eq!(value[0]["symbol"], "AAPL");
    }

    #[tokio::test]
    async fn by_symbol_date_range_sends_expected_requests() {
        let server = MockServer::start_async().await;
        let endpoints = [HISTORICAL_PRICE_EOD_FULL, SEC_FILINGS_SEARCH_SYMBOL];
        let mut mocks = Vec::new();

        for endpoint in endpoints {
            mocks.push(
                server
                    .mock_async(|when, then| {
                        when.method(GET)
                            .path(format!("/{}", endpoint.path()))
                            .query_param("symbol", "AAPL")
                            .query_param("from", "2025-01-01")
                            .query_param("to", "2025-01-31")
                            .query_param("apikey", "test-key");
                        then.status(200).json_body(json!([{ "symbol": "AAPL" }]));
                    })
                    .await,
            );
        }

        let client = test_client(&server);
        for endpoint in endpoints {
            client
                .by_symbol_date_range(endpoint, "AAPL", Some("2025-01-01"), Some("2025-01-31"))
                .await
                .unwrap();
        }

        for mock in mocks {
            mock.assert_async().await;
        }
    }

    #[tokio::test]
    async fn annual_endpoints_send_expected_requests() {
        let server = MockServer::start_async().await;
        let endpoints = [
            INCOME_STATEMENT,
            INCOME_STATEMENT_AS_REPORTED,
            BALANCE_SHEET_STATEMENT,
            CASH_FLOW_STATEMENT,
            RATIOS,
            KEY_METRICS,
            INCOME_STATEMENT_GROWTH,
            BALANCE_SHEET_STATEMENT_GROWTH,
            CASH_FLOW_STATEMENT_GROWTH,
            ENTERPRISE_VALUES,
            ANALYST_ESTIMATES,
        ];
        let mut mocks = Vec::new();

        for endpoint in endpoints {
            mocks.push(
                server
                    .mock_async(|when, then| {
                        when.method(GET)
                            .path(format!("/{}", endpoint.path()))
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
        for endpoint in endpoints {
            client.annual(endpoint, "AAPL", None).await.unwrap();
        }

        for mock in mocks {
            mock.assert_async().await;
        }
    }

    #[tokio::test]
    async fn annual_report_form_sends_expected_request() {
        let server = MockServer::start_async().await;
        let mock = server
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

        let value = test_client(&server)
            .annual_report_form(FINANCIAL_REPORTS_JSON, "AAPL", 2022, "FY")
            .await
            .unwrap();

        mock.assert_async().await;
        assert_eq!(value["symbol"], "AAPL");
    }

    #[tokio::test]
    async fn paged_endpoints_send_expected_requests() {
        let server = MockServer::start_async().await;
        let endpoints = [
            GENERAL_NEWS,
            FMP_ARTICLES,
            FOREX_NEWS,
            CRYPTO_NEWS,
            INSIDER_TRADING_LATEST,
        ];
        let mut mocks = Vec::new();

        for endpoint in endpoints {
            mocks.push(
                server
                    .mock_async(|when, then| {
                        when.method(GET)
                            .path(format!("/{}", endpoint.path()))
                            .query_param("page", "0")
                            .query_param("limit", "10")
                            .query_param("apikey", "test-key");
                        then.status(200)
                            .json_body(json!([{ "title": "Market news" }]));
                    })
                    .await,
            );
        }

        let client = test_client(&server);
        for endpoint in endpoints {
            client.paged(endpoint, None, None).await.unwrap();
        }

        for mock in mocks {
            mock.assert_async().await;
        }
    }

    #[tokio::test]
    async fn api_errors_are_redacted_and_structured() {
        let server = MockServer::start_async().await;
        server
            .mock_async(|when, then| {
                when.method(GET).path("/quote");
                then.status(402)
                    .body("Restricted Endpoint: upgrade required");
            })
            .await;
        let client =
            FmpClient::with_base_url("secret-key", format!("{}/", server.base_url())).unwrap();

        let error = client.by_symbol(QUOTE, "AAPL").await.unwrap_err();

        assert_eq!(error.kind(), "api_error");
        assert!(!error.to_string().contains("secret-key"));
        assert!(error.to_string().contains("402"));
    }

    #[tokio::test]
    async fn transport_errors_do_not_include_request_url_or_api_key() {
        let client = FmpClient::with_base_url("secret-key", "http://127.0.0.1:9/").unwrap();

        let error = client
            .annual(BALANCE_SHEET_STATEMENT, "AAPL", None)
            .await
            .unwrap_err();
        let message = error.to_string();

        assert_eq!(error.kind(), "http_error");
        assert!(!message.contains("secret-key"));
        assert!(!message.contains("apikey"));
        assert!(!message.contains("127.0.0.1:9"));
    }
}

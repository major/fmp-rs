//! Error types for the FMP CLI.

use std::process::ExitCode;

/// Convenient result alias for this crate.
pub type Result<T> = std::result::Result<T, Error>;

/// Errors returned by the FMP CLI and client.
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum Error {
    /// The API key was not provided.
    #[error("missing FMP API key; set FMP_API_KEY or pass --api-key")]
    MissingApiKey,

    /// The configured base URL is invalid.
    #[error("invalid FMP base URL: {0}")]
    InvalidBaseUrl(String),

    /// A required CLI argument was not provided.
    #[error("missing required CLI argument: {0}")]
    MissingArgument(&'static str),

    /// The API returned a non-successful response.
    #[error("FMP API request failed with HTTP {status}: {message}")]
    Api {
        /// HTTP status code returned by the API.
        status: u16,
        /// Redacted response body or status message.
        message: String,
    },

    /// The API rate-limited the request.
    #[error("FMP API request was rate limited with HTTP {status}: {message}")]
    RateLimited {
        /// HTTP status code returned by the API.
        status: u16,
        /// Redacted response body or status message.
        message: String,
    },

    /// A strict symbol lookup returned no data.
    #[error(
        "empty result for symbol {symbol} from {endpoint}; try `fmp-agent search {search_query}` to verify the symbol, or rerun without --strict-empty to keep the raw FMP response"
    )]
    EmptyResult {
        /// Symbol or symbol list used for the lookup.
        symbol: String,
        /// Query suggested for discovery with `fmp-agent search`.
        search_query: String,
        /// API endpoint path that returned the empty payload.
        endpoint: &'static str,
    },

    /// A CLI command maps to an endpoint that FMP no longer documents for the stable API.
    #[error("endpoint {endpoint} is unavailable: {message}")]
    EndpointUnavailable {
        /// Endpoint path or command identifier that is unavailable.
        endpoint: &'static str,
        /// User-facing explanation of the unavailable endpoint.
        message: &'static str,
    },

    /// The HTTP client failed before receiving a usable response.
    #[error("HTTP request failed: {0}")]
    Http(reqwest::Error),

    /// JSON serialization failed while rendering output.
    #[error("failed to render JSON output: {0}")]
    Json(#[from] serde_json::Error),
}

impl From<reqwest::Error> for Error {
    fn from(error: reqwest::Error) -> Self {
        Self::Http(error.without_url())
    }
}

impl Error {
    /// Returns a stable machine-readable error kind.
    #[must_use]
    pub const fn kind(&self) -> &'static str {
        match self {
            Self::MissingApiKey => "missing_api_key",
            Self::InvalidBaseUrl(_) => "invalid_base_url",
            Self::MissingArgument(_) => "missing_argument",
            Self::Api { .. } => "api_error",
            Self::RateLimited { .. } => "rate_limited",
            Self::EmptyResult { .. } => "empty_result",
            Self::EndpointUnavailable { .. } => "endpoint_unavailable",
            Self::Http(_) => "http_error",
            Self::Json(_) => "json_error",
        }
    }

    /// Returns a process exit code for this error.
    ///
    /// Codes follow a simple convention:
    /// - `2` - usage/argument error (missing required CLI argument)
    /// - `3` - configuration error (missing API key, invalid base URL)
    /// - `4` - network/HTTP error
    /// - `5` - API error (server returned an error response, including rate limits)
    /// - `6` - parse error (JSON deserialization failed)
    /// - `7` - empty symbol result in strict mode
    #[must_use]
    pub fn exit_code(&self) -> ExitCode {
        match self {
            Self::MissingArgument(_) => ExitCode::from(2),
            Self::MissingApiKey | Self::InvalidBaseUrl(_) => ExitCode::from(3),
            Self::Http(_) => ExitCode::from(4),
            Self::Api { .. } | Self::RateLimited { .. } | Self::EndpointUnavailable { .. } => {
                ExitCode::from(5)
            }
            Self::Json(_) => ExitCode::from(6),
            Self::EmptyResult { .. } => ExitCode::from(7),
        }
    }
}

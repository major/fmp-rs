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

    /// A date value was not in `YYYY-MM-DD` format.
    #[error("invalid date '{0}': expected YYYY-MM-DD")]
    InvalidDate(String),

    /// The API returned a non-successful response.
    #[error("FMP API request failed with HTTP {status}: {message}")]
    Api {
        /// HTTP status code returned by the API.
        status: u16,
        /// Redacted response body or status message.
        message: String,
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
            Self::InvalidDate(_) => "invalid_date",
            Self::Api { .. } => "api_error",
            Self::Http(_) => "http_error",
            Self::Json(_) => "json_error",
        }
    }

    /// Returns a process exit code for this error.
    ///
    /// Codes follow a simple convention:
    /// - `2` - usage/argument error (aligns with clap convention)
    /// - `3` - configuration error (missing API key, invalid base URL)
    /// - `4` - network/HTTP error
    /// - `5` - API error (server returned an error response)
    /// - `6` - parse error (JSON deserialization failed)
    #[must_use]
    pub fn exit_code(&self) -> ExitCode {
        match self {
            Self::MissingArgument(_) | Self::InvalidDate(_) => ExitCode::from(2),
            Self::MissingApiKey | Self::InvalidBaseUrl(_) => ExitCode::from(3),
            Self::Http(_) => ExitCode::from(4),
            Self::Api { .. } => ExitCode::from(5),
            Self::Json(_) => ExitCode::from(6),
        }
    }
}

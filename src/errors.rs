//! Error Handling Module
//!
//! This module defines custom error types for the bot detector library, allowing for better error
//! handling and propagation.

use pcre2::Error as Pcre2Error;
use std::{fmt, io};

/// Custom error type for the bot detector
#[derive(Debug)]
pub enum BotDetectorError {
    /// Error related to IO operations, such as reading a file.
    Io(io::Error),

    /// Error related to JSON parsing.
    JsonParse(serde_json::Error),

    /// Error related to compiling the regex pattern.
    RegexCompile(Pcre2Error),
}

impl fmt::Display for BotDetectorError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BotDetectorError::Io(e) => write!(f, "IO error: {e}"),
            BotDetectorError::JsonParse(e) => write!(f, "JSON Parse error: {e}"),
            BotDetectorError::RegexCompile(e) => write!(f, "Regex compilation error: {e}"),
        }
    }
}

impl From<io::Error> for BotDetectorError {
    fn from(value: io::Error) -> Self {
        BotDetectorError::Io(value)
    }
}

impl From<serde_json::Error> for BotDetectorError {
    fn from(value: serde_json::Error) -> Self {
        BotDetectorError::JsonParse(value)
    }
}

impl From<Pcre2Error> for BotDetectorError {
    fn from(value: Pcre2Error) -> Self {
        BotDetectorError::RegexCompile(value)
    }
}

impl BotDetectorError {
    /// Returns a human-readable error message.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use my_bot_checker::errors::BotDetectorError;
    /// let error = BotDetectorError::Io(std::io::Error::new(std::io::ErrorKind::Other, "on error"));
    /// assert_eq!(error.error_message(), "IO error: on error");
    /// ```
    #[must_use]
    pub fn error_message(&self) -> String {
        format!("{self}")
    }
}

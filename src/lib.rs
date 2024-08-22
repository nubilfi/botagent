//! Bot Detector Library
//!
//! This library provides functionality to detect bot user agents using regular expressions.
//!
//! It reads patterns from a JSON File, compiles them into a regex, and checks user agents
//! against these patterns.

pub mod errors;
pub mod pattern;

use crate::errors::BotDetectorError;
use once_cell::sync::OnceCell;
use pcre2::bytes::Regex;

static REGEX: OnceCell<Regex> = OnceCell::new();

/// Initialize the global regex pattern, only done once.
///
/// # Arguments
///
/// * `json_path` - Path to the JSON file containing patterns.
///
/// # Returns
///
/// Returns a reference to the compiled `Regex` or a `BotDetectorError` if something goes wrong.
///
/// # Errors
///
/// This function will return an error if the JSON file cannot be read, parsed, or if the regex
/// pattern cannot be compiled.
///
/// # Panics
///
/// Will panic if `generate_pattern` process failed.
///
/// # Example
///
/// ```no_run
/// # use botagent::init_pattern;
/// let regex = init_pattern("patterns.json").unwrap();
/// ```
pub fn init_pattern(json_path: &str) -> Result<&'static Regex, BotDetectorError> {
    Ok(
        REGEX.get_or_init(|| match pattern::generate_pattern(json_path) {
            Ok(regex) => regex,
            Err(e) => {
                panic!("Error detected: {:?}", e.error_message());
            }
        }),
    )
}

/// Check if the given user agent includes a bot pattern.
///
/// # Arguments
///
/// * `user_agent` - The user agent string to be checked.
/// * `json_path` - Path to the JSON file containing bot patterns.
///
/// # Returns
///
/// Returns `true` if the user agent matches any bot pattern, otherwise `false`.
///
/// # Errors
///
/// Returns a `BotDetectorError` if there's an issue with reading the patterns or compiling the regex.
///
/// # Example
///
/// ```no_run
/// # use botagent::is_bot;
/// let is_bot = is_bot("Mozilla/5.0 (compatible; Googlebot/2.1; +http://www.google.com/bot.html)", "patterns.json").unwrap();
/// assert!(is_bot);
/// ```
pub fn is_bot(user_agent: &str, json_path: &str) -> Result<bool, BotDetectorError> {
    let regex = init_pattern(json_path)?;
    Ok(regex.is_match(user_agent.as_bytes()).unwrap_or(false))
}

/// Find the first non-empty capture group match of a bot pattern in the user agent string.
///
/// # Arguments
///
/// * `user_agent` - The user agent string to be checked.
/// * `json_path` - Path to the JSON file containing bot patterns.
///
/// # Returns
///
/// Returns `Some(String)` with the first matched capture group or `None` if no match is found.
///
/// # Errors
///
/// Returns a `BotDetectorError` if there's an issue with reading the patterns or compiling the regex.
///
/// # Example
///
/// ```no_run
/// # use botagent::is_bot_match;
/// let matched_pattern = is_bot_match("Mozilla/5.0 (compatible; Googlebot/2.1; +http://www.google.com/bot.html)", "patterns.json").unwrap();
/// assert_eq!(matched_pattern, Some("Googlebot".to_string()));
/// ```
pub fn is_bot_match(user_agent: &str, json_path: &str) -> Result<Option<String>, BotDetectorError> {
    let regex = init_pattern(json_path)?;

    if let Ok(Some(caps)) = regex.captures(user_agent.as_bytes()) {
        if let Some(matched) = caps.get(0) {
            return Ok(Some(
                String::from_utf8_lossy(matched.as_bytes()).to_string(),
            ));
        }
    }

    Ok(None)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::NamedTempFile;

    fn create_temp_patterns_file(patterns: &[&str]) -> NamedTempFile {
        let file = NamedTempFile::new().expect("Failed to create temp file");
        let pattern_list = serde_json::to_string(&patterns).expect("Failed to serialze patterns");

        fs::write(file.path(), pattern_list).expect("failed to write to temp file");

        file
    }

    #[test]
    fn test_is_bot() {
        let bot_user_agent =
            "Mozilla/5.0 (compatible; Googlebot/2.1; +http://www.google.com/bot.html)";
        let temp_file = create_temp_patterns_file(&["Googlebot"]);

        assert!(is_bot(bot_user_agent, temp_file.path().to_str().unwrap()).unwrap());
    }

    #[test]
    fn test_is_bot_match() {
        let bot_user_agent =
            "Mozilla/5.0 (compatible; Googlebot/2.1; +http://www.google.com/bot.html)";
        let temp_file = create_temp_patterns_file(&["Googlebot"]);

        assert_eq!(
            is_bot_match(bot_user_agent, temp_file.path().to_str().unwrap()).unwrap(),
            Some("Googlebot".to_string())
        );
    }

    #[test]
    fn test_invalid_inputs() {
        let temp_file = create_temp_patterns_file(&["Googlebot"]);

        assert!(!is_bot("", temp_file.path().to_str().unwrap()).unwrap());
        assert_eq!(
            is_bot_match("", temp_file.path().to_str().unwrap()).unwrap(),
            None
        );
    }
}

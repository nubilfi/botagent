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
use serde::Deserialize;
use std::fs;

static REGEX: OnceCell<Regex> = OnceCell::new();

#[derive(Debug, Deserialize)]
struct List(Vec<String>);

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

/// Check if the given user agent matches any patterns in the provided JSON file.
///
/// # Arguments
///
/// * `user_agent` - The user agent string to be checked.
/// * `json_path` - Path to the JSON file containing the bot patterns.
///
/// # Returns
///
/// Returns a `Result<Vec<String>, BotDetectorError>`. If successful, returns a vector of matching patterns as strings. If there is an error reading the JSON file or compiling the regex patterns, returns a `BotDetectorError`.
///
/// # Errors
///
/// Returns a `BotDetectorError` if there's an issue with reading the patterns from the JSON file or compiling the regex patterns.
///
/// # Example
///
/// ```no_run
/// # use botagent::is_bot_matches;
/// let matches = is_bot_matches("Mozilla/5.0 (compatible; Googlebot/2.1; +http://www.google.com/bot.html)", "patterns.json").unwrap();
/// assert!(matches.contains(&"Googlebot".to_string()));
/// ```
pub fn is_bot_matches(user_agent: &str, json_path: &str) -> Result<Vec<String>, BotDetectorError> {
    let patterns_json = fs::read_to_string(json_path)?;
    let patterns: List = serde_json::from_str(&patterns_json)?;

    let matches = patterns
        .0
        .iter()
        .filter_map(|pattern| {
            let regex = Regex::new(format!("(?i){pattern}").as_str()).ok()?;

            if regex.is_match(user_agent.as_bytes()).unwrap_or(false) {
                Some(pattern.clone())
            } else {
                None
            }
        })
        .collect();

    Ok(matches)
}

/// Check if the given user agent matches any bot pattern and return the matching pattern.
///
/// # Arguments
///
/// * `user_agent` - The user agent string to be checked.
/// * `json_path` - Path to the JSON file containing bot patterns.
///
/// # Returns
///
/// Returns `Some(pattern)` if the user agent matches any bot pattern, where `pattern` is the matching pattern string.
/// Returns `None` if no patterns match the user agent.
///
/// # Errors
///
/// Returns a `BotDetectorError` if there's an issue with reading the patterns, deserializing the JSON, or compiling the regex.
///
/// # Example
///
/// ```no_run
/// # use botagent::is_bot_pattern;
/// let pattern = is_bot_pattern("Mozilla/5.0 (compatible; Googlebot/2.1; +http://www.google.com/bot.html)", "patterns.json").unwrap();
/// assert_eq!(pattern, Some("Googlebot/2.1".to_string()));
/// ```
pub fn is_bot_pattern(
    user_agent: &str,
    json_path: &str,
) -> Result<Option<String>, BotDetectorError> {
    let patterns_json = fs::read_to_string(json_path)?;
    let patterns: List = serde_json::from_str(&patterns_json)?;

    for pattern in patterns.0 {
        let regex = Regex::new(&pattern)?;

        if regex.is_match(user_agent.as_bytes())? {
            return Ok(Some(pattern));
        }
    }

    Ok(None)
}

/// Check which bot patterns from the given JSON file match the user agent.
///
/// # Arguments
///
/// * `user_agent` - The user agent string to be checked.
/// * `json_path` - Path to the JSON file containing bot patterns.
///
/// # Returns
///
/// Returns a `Vec<String>` containing all bot patterns from the JSON file that match the user agent.
/// If no patterns match, an empty vector is returned.
///
/// # Errors
///
/// Returns a `BotDetectorError` if there's an issue with reading the patterns, parsing the JSON, or compiling any regex.
///
/// # Example
///
/// ```no_run
/// # use botagent::is_bot_patterns;
/// let matching_patterns = is_bot_patterns("Mozilla/5.0 (compatible; Googlebot/2.1; +http://www.google.com/bot.html)", "patterns.json").unwrap();
/// assert!(matching_patterns.contains(&"Googlebot/2.1".to_string()));
/// ```
pub fn is_bot_patterns(user_agent: &str, json_path: &str) -> Result<Vec<String>, BotDetectorError> {
    let patterns_json = fs::read_to_string(json_path)?;
    let patterns: List = serde_json::from_str(&patterns_json)?;

    let matching_patterns: Vec<String> = patterns
        .0
        .into_iter()
        .filter_map(|pattern| {
            let regex = Regex::new(&pattern).ok()?;
            if regex.is_match(user_agent.as_bytes()).ok()? {
                Some(pattern)
            } else {
                None
            }
        })
        .collect();

    Ok(matching_patterns)
}

/// Creates a closure that checks if a user agent string matches a custom regex pattern
///
/// # Arguments
///
/// * `custom_pattern` - A `Regex` object that represents the custom pattern to be used for matching.
/// # Returns
///
/// Returns a closure that takes a user agent string as input and returns `true` if the user agent matches the
/// custom regex pattern and is not empty, or `false` otherwise.
///
/// ```no_run
/// # use pcre2::bytes::Regex;
/// # use botagent::create_is_bot;
/// let pattern = Regex::new(r"Googlebot").unwrap();
/// let custom_bot = create_is_bot(pattern);
/// assert!(custom_bot("Mozilla/5.0 (compatible; Googlebot/2.1; +http://www.google.com/bot.html)"));
/// ```
pub fn create_is_bot(custom_pattern: Regex) -> impl Fn(&str) -> bool {
    move |user_agent: &str| -> bool {
        !user_agent.is_empty()
            && custom_pattern
                .is_match(user_agent.as_bytes())
                .unwrap_or(false)
    }
}

#[cfg(test)]
mod features {
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

        // bot_user_agent string is recognised as bot
        assert!(is_bot(bot_user_agent, temp_file.path().to_str().unwrap()).unwrap());
    }

    #[test]
    fn test_is_bot_match() {
        let bot_user_agent =
            "Mozilla/5.0 (compatible; Googlebot/2.1; +http://www.google.com/bot.html)";
        let temp_file = create_temp_patterns_file(&["Googlebot"]);

        // find pattern in bot_user_agent string
        assert_eq!(
            is_bot_match(bot_user_agent, temp_file.path().to_str().unwrap()).unwrap(),
            Some("Googlebot".to_string())
        );
    }

    #[test]
    fn test_is_bot_matches() {
        let bot_user_agent =
            "Mozilla/5.0 (compatible; Googlebot/2.1; +http://www.google.com/bot.html)";
        let temp_file = create_temp_patterns_file(&["Google", "Googlebot", "bot", "http"]);

        let matches = is_bot_matches(bot_user_agent, temp_file.path().to_str().unwrap()).unwrap();

        // find all patterns in bot_user_agent string
        assert!(matches.contains(&"Google".to_string()));
        assert_eq!(matches.len(), 4);
    }

    #[test]
    fn test_is_bot_pattern() {
        let bot_user_agent =
            "Mozilla/5.0 (compatible; Googlebot/2.1; +http://www.google.com/bot.html)";
        let expected_pattern = r"(?<! (?:channel/|google/))google(?!(app|/google| pixel))";

        let temp_file = create_temp_patterns_file(&[expected_pattern]);

        let result = is_bot_pattern(bot_user_agent, temp_file.path().to_str().unwrap())
            .expect("Failed to execute is_bot_pattern");

        assert_eq!(result, Some(expected_pattern.to_string()));
    }

    #[test]
    fn test_is_bot_patterns() {
        let bot_user_agent =
            "Mozilla/5.0 (compatible; Googlebot/2.1; +http://www.google.com/bot.html)";
        let patterns = [
            r"(?<! (?:channel/|google/))google(?!(app|/google| pixel))",
            r"(?<! cu)bots?(?:\b|_)",
            r"(?<!(?:lib))http",
            r"\.com",
        ];

        let temp_file = create_temp_patterns_file(&patterns);
        let result = is_bot_patterns(bot_user_agent, temp_file.path().to_str().unwrap())
            .expect("Failed to execute is_bot_patterns");

        for pattern in patterns.iter() {
            assert!(result.contains(&pattern.to_string()));
        }

        assert_eq!(result.len(), 4);
    }

    #[test]
    fn test_create_is_bot() {
        let bot_user_agent =
            "Mozilla/5.0 (compatible; Googlebot/2.1; +http://www.google.com/bot.html)";
        let custom_pattern = Regex::new(r"bot").unwrap();
        let custom_is_bot = create_is_bot(custom_pattern);

        // create custom_is_bot function with custom pattern
        assert!(custom_is_bot(bot_user_agent));
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

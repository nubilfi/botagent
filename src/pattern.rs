//! Pattern Generation Module
//!
//! This module is responsible for reading bot patterns from a JSON file and compiling them
//! into a regulax expression that can be used to match user agents.

use crate::errors::BotDetectorError;
use pcre2::bytes::{Regex as RegexBytes, RegexBuilder};
use serde::Deserialize;
use std::fs;

#[derive(Debug, Deserialize)]
struct PatternList(Vec<String>);

/// Read patterns from a JSON file and generates a regex pattern.
///
/// # Arguments
///
/// * `json_path` - Path to the JSON file containing patterns.
///
/// # Returns
///
/// Returns a compiled `RegexBytes` object or an error if something goes wrong.
///
/// # Errors
///
/// This function will return an error if the JSON file cannot be read, parsed, or if the regex
/// pattern cannot be compiled.
///
/// # Example
///
/// ```no_run
/// # use botagent::pattern::generate_pattern;
/// let regex = generate_pattern("patterns.json").unwrap();
/// ```
#[allow(clippy::module_name_repetitions)]
pub fn generate_pattern(json_path: &str) -> Result<RegexBytes, BotDetectorError> {
    // read and parse the JSON file
    let patterns_json = fs::read_to_string(json_path)?;
    let patterns: PatternList = serde_json::from_str(&patterns_json)?;

    let pattern_str = patterns.0.join("|");
    let regex = RegexBuilder::new().caseless(true).build(&pattern_str)?;

    Ok(regex)
}

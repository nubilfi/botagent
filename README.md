# botagent

[![version](https://img.shields.io/crates/v/botagent?color=blue&logo=rust&style=flat-square)](https://crates.io/crates/botagent)
[![Build Status](https://github.com/nubilfi/botagent/actions/workflows/rust.yml/badge.svg)](https://github.com/nubilfi/botagent/actions?branch=main)
[![Documentation](https://docs.rs/botagent/badge.svg)](https://docs.rs/botagent/latest/botagent/)

[![codecov](https://codecov.io/gh/nubilfi/botagent/graph/badge.svg?token=SRGOFSB31Q)](https://codecov.io/gh/nubilfi/botagent)

**botagent** is a Rust library for detecting bot user agents using regular expressions. It reads patterns from a JSON file, compiles them into a regex, and checks user agents against these patterns.

## Features

- **Bot Detection**: Identify whether a given user agent string matches known bot patterns.
- **Customizable**: Use your own bot patterns by providing a JSON file.
- **Efficient**: Uses the `pcre2` crate for high-performance regex matching.
- **Error Handling**: Robust error handling with detailed messages.

## Installation

To use this library in your project, add the following to your `Cargo.toml`:

```toml
[dependencies]
botagent = "0.1"
```

Or by run `cargo add botagent` command.

## Usage
### Check if User Agent is a Bot

To check if a given user agent string matches any known bot patterns:

```rust
use botagent::is_bot;

fn main() {
    let is_bot = is_bot("Mozilla/5.0 (compatible; Googlebot/2.1; +http://www.google.com/bot.html)", "patterns.json").unwrap();
    println!("Is bot: {}", is_bot);
}
```

### Find the Matching Bot Pattern

If you want to know which bot pattern matched the user agent:

```rust
use botagent::is_bot_match;

fn main() {
    let matched_pattern = is_bot_match("Mozilla/5.0 (compatible; Googlebot/2.1; +http://www.google.com/bot.html)", "patterns.json").unwrap();
    if let Some(pattern) = matched_pattern {
        println!("Matched bot pattern: {}", pattern);
    } else {
        println!("No bot pattern matched.");
    }
}
```

## Patterns File Format

The bot patterns are stored in a JSON file, with each pattern being a regular expression string. Here is an example `patterns.json`:

```json
[
  "(?<! (?:channel/|google/))google(?!(app|/google| pixel))",
  "(?<! cu)bots?(?:\\b|_)"
]
```

Each string in the array is a pattern that will be compiled into a single regular expression to match against user agent strings.

## Running Tests

To run the tests, you can use the following command:

```bash
cargo test
```

This will also run the documentation tests to ensure that all code examples in the documentation are correct.

## Accuracy

This library follows the original logic and pattern list from [isbot](https://github.com/omrilotan/isbot) and it uses a regular expression that matches bots and only bots. I'll try to ensure it aligns closely with the original logic, but tailored to work seamlessly in a Rust environment, feel free to reach out with any feedback or suggestions!

## Credits

This library was inspired by and largely based on the work from [isbot](https://github.com/omrilotan/isbot). The original TypeScript code was rewritten in Rust.


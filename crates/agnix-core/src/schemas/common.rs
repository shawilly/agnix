//! Shared schema parsing helpers.

use serde::de::DeserializeOwned;
use serde_json::Value;

/// Parse error with optional line/column metadata.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseError {
    pub message: String,
    pub line: usize,
    pub column: usize,
}

impl ParseError {
    pub fn new(message: impl Into<String>, line: usize, column: usize) -> Self {
        Self {
            message: message.into(),
            line,
            column,
        }
    }

    pub fn from_json(err: serde_json::Error) -> Self {
        Self::new(err.to_string(), err.line(), err.column())
    }
}

/// Parse JSON once into raw value and typed schema.
///
/// On typed conversion failure, this retries typed parsing from the original
/// string to recover best-effort line/column metadata.
pub fn parse_json_with_raw<T: DeserializeOwned>(
    content: &str,
) -> (Option<T>, Option<ParseError>, Option<Value>) {
    let raw = match serde_json::from_str::<Value>(content) {
        Ok(value) => value,
        Err(err) => return (None, Some(ParseError::from_json(err)), None),
    };

    match serde_json::from_value::<T>(raw.clone()) {
        Ok(parsed) => (Some(parsed), None, Some(raw)),
        Err(err) => {
            let parse_error = serde_json::from_str::<T>(content)
                .err()
                .map(ParseError::from_json)
                .unwrap_or_else(|| ParseError::new(err.to_string(), 0, 0));
            (None, Some(parse_error), Some(raw))
        }
    }
}

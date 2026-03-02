//! Kiro MCP schema helpers.
//!
//! Covers `.kiro/settings/mcp.json` and power-local `mcp.json`.

use crate::schemas::common::{ParseError, parse_json_with_raw};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

/// Parsed Kiro MCP document.
#[allow(dead_code)] // schema-level API; consumed by validator layer
#[derive(Debug, Clone)]
pub struct ParsedKiroMcpConfig {
    pub config: Option<KiroMcpConfig>,
    pub parse_error: Option<ParseError>,
    pub raw_value: Option<Value>,
}

/// Kiro MCP config file schema.
#[allow(dead_code)] // schema-level API; consumed by validator layer
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct KiroMcpConfig {
    pub mcp_servers: Option<HashMap<String, KiroMcpServerConfig>>,
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

/// Per-server MCP config in Kiro context.
#[allow(dead_code)] // schema-level API; consumed by validator layer
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct KiroMcpServerConfig {
    pub command: Option<String>,
    pub args: Option<Vec<String>>,
    pub url: Option<String>,
    pub env: Option<HashMap<String, String>>,
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

/// Parse Kiro MCP JSON into typed schema and raw value.
#[allow(dead_code)] // schema-level API; consumed by validator layer
pub fn parse_kiro_mcp_config(content: &str) -> ParsedKiroMcpConfig {
    let (config, parse_error, raw_value) = parse_json_with_raw::<KiroMcpConfig>(content);
    ParsedKiroMcpConfig {
        config,
        parse_error,
        raw_value,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_valid_kiro_mcp_config() {
        let parsed = parse_kiro_mcp_config(
            r#"{
  "mcpServers": {
    "local": {
      "command": "node",
      "args": ["server.js"]
    },
    "remote": {
      "url": "https://example.com/mcp"
    }
  }
}"#,
        );

        assert!(parsed.parse_error.is_none());
        let config = parsed.config.expect("config should parse");
        assert_eq!(
            config.mcp_servers.as_ref().map(|servers| servers.len()),
            Some(2)
        );
    }

    #[test]
    fn parse_invalid_kiro_mcp_config_reports_error_location() {
        let parsed = parse_kiro_mcp_config(r#"{"mcpServers":[}"#);
        let error = parsed.parse_error.expect("expected parse error");
        assert!(error.line > 0);
        assert!(error.column > 0);
    }
}

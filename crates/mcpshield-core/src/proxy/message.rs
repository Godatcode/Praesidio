use serde::{Deserialize, Serialize};

/// JSON-RPC message types used in MCP protocol
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JsonRpcMessage {
    pub jsonrpc: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub method: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub params: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<JsonRpcError>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JsonRpcError {
    pub code: i64,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
}

impl JsonRpcMessage {
    pub fn is_request(&self) -> bool {
        self.method.is_some() && self.id.is_some()
    }

    pub fn is_notification(&self) -> bool {
        self.method.is_some() && self.id.is_none()
    }

    pub fn is_response(&self) -> bool {
        self.result.is_some() || self.error.is_some()
    }

    /// Check if this is a tools/list response
    pub fn is_tools_list_response(&self) -> bool {
        self.is_response()
            && self
                .result
                .as_ref()
                .and_then(|r| r.get("tools"))
                .is_some()
    }

    /// Check if this is a tools/call request
    pub fn is_tool_call(&self) -> bool {
        self.method.as_deref() == Some("tools/call")
    }

    /// Extract tool name from a tools/call request
    pub fn tool_call_name(&self) -> Option<&str> {
        if !self.is_tool_call() {
            return None;
        }
        self.params
            .as_ref()
            .and_then(|p| p.get("name"))
            .and_then(|n| n.as_str())
    }

    /// Extract tools from a tools/list response
    pub fn extract_tools(&self) -> Vec<McpTool> {
        let tools_value = match self.result.as_ref().and_then(|r| r.get("tools")) {
            Some(v) => v,
            None => return Vec::new(),
        };

        let tools_array = match tools_value.as_array() {
            Some(a) => a,
            None => return Vec::new(),
        };

        tools_array
            .iter()
            .filter_map(|t| {
                let name = t.get("name")?.as_str()?.to_string();
                let description = t
                    .get("description")
                    .and_then(|d| d.as_str())
                    .unwrap_or("")
                    .to_string();
                let input_schema = t
                    .get("inputSchema")
                    .cloned()
                    .unwrap_or(serde_json::Value::Null);

                Some(McpTool {
                    name,
                    description,
                    input_schema,
                })
            })
            .collect()
    }

    /// Extract text content from a tool result
    pub fn extract_tool_result_text(&self) -> Option<String> {
        let result = self.result.as_ref()?;
        let content = result.get("content")?.as_array()?;

        let texts: Vec<&str> = content
            .iter()
            .filter_map(|c| {
                if c.get("type")?.as_str()? == "text" {
                    c.get("text")?.as_str()
                } else {
                    None
                }
            })
            .collect();

        if texts.is_empty() {
            None
        } else {
            Some(texts.join("\n"))
        }
    }

    /// Create an error response for a given request ID
    pub fn error_response(id: serde_json::Value, code: i64, message: &str) -> Self {
        Self {
            jsonrpc: "2.0".to_string(),
            id: Some(id),
            method: None,
            params: None,
            result: None,
            error: Some(JsonRpcError {
                code,
                message: message.to_string(),
                data: None,
            }),
        }
    }
}

#[derive(Debug, Clone)]
pub struct McpTool {
    pub name: String,
    pub description: String,
    pub input_schema: serde_json::Value,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_tool_call() {
        let msg: JsonRpcMessage = serde_json::from_str(r#"{
            "jsonrpc": "2.0",
            "id": 1,
            "method": "tools/call",
            "params": {
                "name": "read_file",
                "arguments": {"path": "/tmp/test"}
            }
        }"#).unwrap();

        assert!(msg.is_tool_call());
        assert_eq!(msg.tool_call_name(), Some("read_file"));
    }

    #[test]
    fn test_extract_tools() {
        let msg: JsonRpcMessage = serde_json::from_str(r#"{
            "jsonrpc": "2.0",
            "id": 1,
            "result": {
                "tools": [
                    {
                        "name": "read_file",
                        "description": "Read a file",
                        "inputSchema": {"type": "object"}
                    }
                ]
            }
        }"#).unwrap();

        let tools = msg.extract_tools();
        assert_eq!(tools.len(), 1);
        assert_eq!(tools[0].name, "read_file");
    }

    #[test]
    fn test_extract_tool_result_text() {
        let msg: JsonRpcMessage = serde_json::from_str(r#"{
            "jsonrpc": "2.0",
            "id": 1,
            "result": {
                "content": [
                    {"type": "text", "text": "Hello world"}
                ]
            }
        }"#).unwrap();

        assert_eq!(msg.extract_tool_result_text(), Some("Hello world".to_string()));
    }
}

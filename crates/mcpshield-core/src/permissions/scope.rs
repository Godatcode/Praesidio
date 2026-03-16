use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ToolScope {
    ReadOnly,
    ReadWrite,
    Blocked,
    RequireConfirmation,
}

impl ToolScope {
    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "read-only" | "readonly" | "read" => ToolScope::ReadOnly,
            "read-write" | "readwrite" | "rw" => ToolScope::ReadWrite,
            "blocked" | "block" | "deny" => ToolScope::Blocked,
            "confirm" | "require-confirmation" => ToolScope::RequireConfirmation,
            _ => ToolScope::ReadOnly, // Default to most restrictive
        }
    }
}

/// Known write operations that should be blocked in read-only mode
pub const WRITE_TOOL_PATTERNS: &[&str] = &[
    "write",
    "create",
    "delete",
    "remove",
    "update",
    "edit",
    "modify",
    "push",
    "put",
    "post",
    "patch",
    "insert",
    "drop",
    "truncate",
    "execute",
    "run",
    "exec",
    "mv",
    "move",
    "rename",
];

/// Check if a tool name looks like a write operation
pub fn is_write_tool(tool_name: &str) -> bool {
    let name_lower = tool_name.to_lowercase();
    WRITE_TOOL_PATTERNS
        .iter()
        .any(|p| name_lower.contains(p))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scope_from_str() {
        assert_eq!(ToolScope::from_str("read-only"), ToolScope::ReadOnly);
        assert_eq!(ToolScope::from_str("read-write"), ToolScope::ReadWrite);
        assert_eq!(ToolScope::from_str("blocked"), ToolScope::Blocked);
        assert_eq!(ToolScope::from_str("unknown"), ToolScope::ReadOnly);
    }

    #[test]
    fn test_write_detection() {
        assert!(is_write_tool("write_file"));
        assert!(is_write_tool("create_issue"));
        assert!(is_write_tool("delete_repository"));
        assert!(!is_write_tool("read_file"));
        assert!(!is_write_tool("list_directory"));
        assert!(!is_write_tool("search_files"));
    }
}

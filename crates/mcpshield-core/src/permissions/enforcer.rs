use crate::config::ServerConfig;
use crate::detection::severity::{Finding, Severity};

use super::scope::{self, ToolScope};

/// Result of permission check
#[derive(Debug, Clone, PartialEq)]
pub enum PermissionResult {
    /// Tool call is allowed
    Allowed,
    /// Tool call is blocked — do not forward to server
    Blocked(String),
    /// Tool call requires confirmation — log warning but allow
    RequireConfirmation(String),
}

/// Check if a tool call is permitted based on server configuration
pub fn check_permission(
    server_name: &str,
    tool_name: &str,
    server_config: Option<&ServerConfig>,
) -> (PermissionResult, Option<Finding>) {
    let config = match server_config {
        Some(c) => c,
        None => {
            // No specific config — allow by default
            return (PermissionResult::Allowed, None);
        }
    };

    // Check if tool is explicitly blocked
    if config.blocked_tools.iter().any(|t| t == tool_name) {
        let finding = Finding {
            severity: Severity::High,
            category: "permission".to_string(),
            server: server_name.to_string(),
            tool: Some(tool_name.to_string()),
            title: "Tool call blocked by policy".to_string(),
            description: format!(
                "Tool '{}' is in the blocked list for server '{}'",
                tool_name, server_name
            ),
            recommendation: "This tool has been explicitly blocked in your MCPShield config"
                .to_string(),
        };
        return (
            PermissionResult::Blocked(format!("Tool '{}' is blocked by policy", tool_name)),
            Some(finding),
        );
    }

    // Check if tool requires confirmation
    if config
        .require_confirmation
        .iter()
        .any(|t| t == tool_name)
    {
        let finding = Finding {
            severity: Severity::Medium,
            category: "permission".to_string(),
            server: server_name.to_string(),
            tool: Some(tool_name.to_string()),
            title: "Tool call requires confirmation".to_string(),
            description: format!(
                "Tool '{}' requires confirmation for server '{}'",
                tool_name, server_name
            ),
            recommendation: "Review this tool call before allowing".to_string(),
        };
        return (
            PermissionResult::RequireConfirmation(format!(
                "Tool '{}' requires confirmation",
                tool_name
            )),
            Some(finding),
        );
    }

    // If allowed_tools is specified and non-empty, only allow listed tools
    if !config.allowed_tools.is_empty() && !config.allowed_tools.iter().any(|t| t == tool_name) {
        let finding = Finding {
            severity: Severity::High,
            category: "permission".to_string(),
            server: server_name.to_string(),
            tool: Some(tool_name.to_string()),
            title: "Tool not in allowlist".to_string(),
            description: format!(
                "Tool '{}' is not in the allowed list for server '{}'",
                tool_name, server_name
            ),
            recommendation: "Add this tool to allowed_tools if it should be permitted".to_string(),
        };
        return (
            PermissionResult::Blocked(format!("Tool '{}' is not in allowlist", tool_name)),
            Some(finding),
        );
    }

    // Check scope — if read-only, block write operations
    let tool_scope = ToolScope::from_str(&config.scope);
    if tool_scope == ToolScope::ReadOnly && scope::is_write_tool(tool_name) {
        let finding = Finding {
            severity: Severity::High,
            category: "permission".to_string(),
            server: server_name.to_string(),
            tool: Some(tool_name.to_string()),
            title: "Write operation blocked (read-only scope)".to_string(),
            description: format!(
                "Tool '{}' appears to be a write operation but server '{}' is in read-only mode",
                tool_name, server_name
            ),
            recommendation:
                "Change server scope to 'read-write' if write operations should be allowed"
                    .to_string(),
        };
        return (
            PermissionResult::Blocked(format!(
                "Write tool '{}' blocked in read-only mode",
                tool_name
            )),
            Some(finding),
        );
    }

    if tool_scope == ToolScope::Blocked {
        let finding = Finding {
            severity: Severity::High,
            category: "permission".to_string(),
            server: server_name.to_string(),
            tool: Some(tool_name.to_string()),
            title: "Server is fully blocked".to_string(),
            description: format!("All tools for server '{}' are blocked", server_name),
            recommendation: "Change server scope to allow tool calls".to_string(),
        };
        return (
            PermissionResult::Blocked(format!("Server '{}' is fully blocked", server_name)),
            Some(finding),
        );
    }

    (PermissionResult::Allowed, None)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_config(scope: &str, blocked: Vec<&str>, allowed: Vec<&str>) -> ServerConfig {
        ServerConfig {
            scope: scope.to_string(),
            blocked_tools: blocked.into_iter().map(String::from).collect(),
            allowed_tools: allowed.into_iter().map(String::from).collect(),
            require_confirmation: vec![],
            block_unknown: false,
        }
    }

    #[test]
    fn test_allowed() {
        let config = make_config("read-write", vec![], vec![]);
        let (result, _) = check_permission("server", "read_file", Some(&config));
        assert_eq!(result, PermissionResult::Allowed);
    }

    #[test]
    fn test_blocked_tool() {
        let config = make_config("read-write", vec!["delete_repo"], vec![]);
        let (result, finding) = check_permission("server", "delete_repo", Some(&config));
        assert!(matches!(result, PermissionResult::Blocked(_)));
        assert!(finding.is_some());
    }

    #[test]
    fn test_read_only_blocks_write() {
        let config = make_config("read-only", vec![], vec![]);
        let (result, _) = check_permission("server", "write_file", Some(&config));
        assert!(matches!(result, PermissionResult::Blocked(_)));
    }

    #[test]
    fn test_read_only_allows_read() {
        let config = make_config("read-only", vec![], vec![]);
        let (result, _) = check_permission("server", "read_file", Some(&config));
        assert_eq!(result, PermissionResult::Allowed);
    }

    #[test]
    fn test_allowlist() {
        let config = make_config("read-write", vec![], vec!["read_file", "list_dir"]);
        let (result, _) = check_permission("server", "search", Some(&config));
        assert!(matches!(result, PermissionResult::Blocked(_)));
        let (result, _) = check_permission("server", "read_file", Some(&config));
        assert_eq!(result, PermissionResult::Allowed);
    }
}

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    #[serde(default)]
    pub global: GlobalConfig,
    #[serde(default)]
    pub scan: ScanConfig,
    #[serde(default)]
    pub proxy: ProxyConfig,
    #[serde(default)]
    pub servers: HashMap<String, ServerConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlobalConfig {
    #[serde(default = "default_log_level")]
    pub log_level: String,
    #[serde(default = "default_audit_dir")]
    pub audit_dir: String,
    #[serde(default = "default_pin_file")]
    pub pin_file: String,
    #[serde(default = "default_true")]
    pub block_on_critical: bool,
    #[serde(default = "default_true")]
    pub alert_on_warning: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanConfig {
    #[serde(default = "default_true")]
    pub check_unicode: bool,
    #[serde(default = "default_true")]
    pub check_tool_poisoning: bool,
    #[serde(default = "default_true")]
    pub check_credential_leaks: bool,
    #[serde(default = "default_true")]
    pub check_cross_server: bool,
    #[serde(default = "default_max_description_length")]
    pub max_description_length: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProxyConfig {
    #[serde(default = "default_true")]
    pub filter_outputs: bool,
    #[serde(default = "default_true")]
    pub detect_exfiltration: bool,
    #[serde(default = "default_rate_limit")]
    pub rate_limit_per_tool: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    #[serde(default = "default_scope")]
    pub scope: String,
    #[serde(default)]
    pub allowed_tools: Vec<String>,
    #[serde(default)]
    pub blocked_tools: Vec<String>,
    #[serde(default)]
    pub require_confirmation: Vec<String>,
    #[serde(default)]
    pub block_unknown: bool,
}

fn default_log_level() -> String {
    "info".to_string()
}
fn default_audit_dir() -> String {
    "~/.mcpshield/logs".to_string()
}
fn default_pin_file() -> String {
    "~/.mcpshield/pins.json".to_string()
}
fn default_true() -> bool {
    true
}
fn default_max_description_length() -> usize {
    500
}
fn default_rate_limit() -> u32 {
    100
}
fn default_scope() -> String {
    "read-only".to_string()
}

impl Default for GlobalConfig {
    fn default() -> Self {
        Self {
            log_level: default_log_level(),
            audit_dir: default_audit_dir(),
            pin_file: default_pin_file(),
            block_on_critical: true,
            alert_on_warning: true,
        }
    }
}

impl Default for ScanConfig {
    fn default() -> Self {
        Self {
            check_unicode: true,
            check_tool_poisoning: true,
            check_credential_leaks: true,
            check_cross_server: true,
            max_description_length: 500,
        }
    }
}

impl Default for ProxyConfig {
    fn default() -> Self {
        Self {
            filter_outputs: true,
            detect_exfiltration: true,
            rate_limit_per_tool: 100,
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            global: GlobalConfig::default(),
            scan: ScanConfig::default(),
            proxy: ProxyConfig::default(),
            servers: HashMap::new(),
        }
    }
}

impl Config {
    pub fn load(path: Option<&Path>) -> Self {
        if let Some(p) = path {
            if p.exists() {
                if let Ok(contents) = std::fs::read_to_string(p) {
                    if let Ok(config) = toml::from_str(&contents) {
                        return config;
                    }
                }
            }
        }

        // Try default path
        let default_path = Self::default_config_path();
        if default_path.exists() {
            if let Ok(contents) = std::fs::read_to_string(&default_path) {
                if let Ok(config) = toml::from_str(&contents) {
                    return config;
                }
            }
        }

        Config::default()
    }

    pub fn default_config_path() -> PathBuf {
        dirs::home_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join(".mcpshield")
            .join("config.toml")
    }

    /// Expand ~ in paths to the actual home directory
    pub fn expand_path(path: &str) -> PathBuf {
        if path.starts_with("~/") {
            if let Some(home) = dirs::home_dir() {
                return home.join(&path[2..]);
            }
        }
        PathBuf::from(path)
    }

    pub fn audit_dir(&self) -> PathBuf {
        Self::expand_path(&self.global.audit_dir)
    }

    pub fn pin_file(&self) -> PathBuf {
        Self::expand_path(&self.global.pin_file)
    }
}

/// Auto-discover MCP configuration files across known client paths
pub fn discover_mcp_configs() -> Vec<(String, PathBuf)> {
    let mut configs = Vec::new();

    let home = match dirs::home_dir() {
        Some(h) => h,
        None => return configs,
    };

    let candidates: Vec<(&str, PathBuf)> = vec![
        // Claude Desktop
        #[cfg(target_os = "macos")]
        (
            "Claude Desktop",
            home.join("Library/Application Support/Claude/claude_desktop_config.json"),
        ),
        #[cfg(target_os = "linux")]
        (
            "Claude Desktop",
            home.join(".config/Claude/claude_desktop_config.json"),
        ),
        // Cursor
        ("Cursor", home.join(".cursor/mcp.json")),
        // Claude Code (user)
        ("Claude Code (user)", home.join(".claude/settings.json")),
        // Claude Code (project)
        (
            "Claude Code (project)",
            PathBuf::from(".claude/settings.json"),
        ),
        // VS Code
        ("VS Code", home.join(".vscode/mcp.json")),
        // Windsurf
        ("Windsurf", home.join(".windsurf/mcp.json")),
    ];

    for (name, path) in candidates {
        if path.exists() {
            configs.push((name.to_string(), path));
        }
    }

    configs
}

/// Parse an MCP config JSON to extract server definitions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpServerDef {
    pub name: String,
    pub command: Option<String>,
    pub args: Option<Vec<String>>,
    pub url: Option<String>,
    pub env: Option<HashMap<String, String>>,
}

pub fn parse_mcp_config(path: &Path) -> Vec<McpServerDef> {
    let contents = match std::fs::read_to_string(path) {
        Ok(c) => c,
        Err(_) => return Vec::new(),
    };

    let json: serde_json::Value = match serde_json::from_str(&contents) {
        Ok(v) => v,
        Err(_) => return Vec::new(),
    };

    let mut servers = Vec::new();

    // Handle "mcpServers" key (Claude Desktop format)
    let servers_obj = json
        .get("mcpServers")
        .or_else(|| json.get("servers"))
        .and_then(|v| v.as_object());

    if let Some(obj) = servers_obj {
        for (name, def) in obj {
            let command = def.get("command").and_then(|v| v.as_str()).map(String::from);
            let args = def.get("args").and_then(|v| {
                v.as_array().map(|arr| {
                    arr.iter()
                        .filter_map(|a| a.as_str().map(String::from))
                        .collect()
                })
            });
            let url = def.get("url").and_then(|v| v.as_str()).map(String::from);
            let env = def.get("env").and_then(|v| {
                v.as_object().map(|obj| {
                    obj.iter()
                        .filter_map(|(k, v)| v.as_str().map(|s| (k.clone(), s.to_string())))
                        .collect()
                })
            });

            servers.push(McpServerDef {
                name: name.clone(),
                command,
                args,
                url,
                env,
            });
        }
    }

    servers
}

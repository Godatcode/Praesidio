use std::process;

use mcpshield_core::audit::AuditLogger;
use mcpshield_core::config::Config;

pub fn run(config: &Config, block_writes: bool, server_cmd: Option<String>) {
    let server_cmd = match server_cmd {
        Some(cmd) => cmd,
        None => {
            eprintln!("{}  Proxy requires --server-cmd to specify the MCP server to proxy.", "❌");
            eprintln!("   Example: mcpshield proxy --server-cmd 'npx -y @modelcontextprotocol/server-filesystem /tmp'");
            process::exit(1);
        }
    };

    let mut config = config.clone();
    if block_writes {
        for (_name, server_config) in config.servers.iter_mut() {
            server_config.scope = "read-only".to_string();
        }
    }

    eprintln!("{}  MCPShield proxy starting...", "🛡️");
    eprintln!("   Server: {}", server_cmd);
    eprintln!("   Block writes: {}", block_writes);

    let rt = tokio::runtime::Runtime::new().expect("Failed to create tokio runtime");
    rt.block_on(async {
        let audit_logger = AuditLogger::new(&config.audit_dir())
            .expect("Failed to initialize audit logger");

        if let Err(e) = mcpshield_core::proxy::stdio_proxy::start_stdio_proxy(
            &server_cmd, &config, &audit_logger, "proxied-server",
        ).await {
            eprintln!("{}  Proxy error: {}", "❌", e);
            process::exit(1);
        }
    });
}

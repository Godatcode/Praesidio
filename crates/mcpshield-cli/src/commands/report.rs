use colored::Colorize;

use mcpshield_core::config::Config;

pub fn run(_config: &Config, format: &str) {
    let report = generate_owasp_report();

    if format == "json" {
        println!("{}", serde_json::to_string_pretty(&report).unwrap());
    } else {
        println!("\n{}  MCPShield OWASP Compliance Report\n", "📋");

        println!("{}  OWASP MCP Top 10\n", "━".repeat(50));
        for item in &report.mcp_top10 {
            let status = if item.covered { "✅" } else { "❌" };
            println!("   {} {} — {}", status, item.id.bold(), item.name);
            println!("      Coverage: {}", item.coverage);
        }

        println!("\n{}  OWASP Agentic Top 10\n", "━".repeat(50));
        for item in &report.agentic_top10 {
            let status = if item.covered { "✅" } else { "❌" };
            println!("   {} {} — {}", status, item.id.bold(), item.name);
            println!("      Coverage: {}", item.coverage);
        }

        let mcp_covered = report.mcp_top10.iter().filter(|i| i.covered).count();
        let agentic_covered = report.agentic_top10.iter().filter(|i| i.covered).count();

        println!(
            "\n   MCP Top 10: {}/10 covered | Agentic Top 10: {}/10 covered",
            mcp_covered, agentic_covered
        );
    }
}

#[derive(serde::Serialize)]
struct ComplianceReport {
    mcp_top10: Vec<ComplianceItem>,
    agentic_top10: Vec<ComplianceItem>,
}

#[derive(serde::Serialize)]
struct ComplianceItem {
    id: String,
    name: String,
    covered: bool,
    coverage: String,
}

fn generate_owasp_report() -> ComplianceReport {
    ComplianceReport {
        mcp_top10: vec![
            ComplianceItem { id: "MCP01".into(), name: "Token Mismanagement".into(), covered: true, coverage: "Credential leak detector scans outputs for API keys, tokens, SSH keys".into() },
            ComplianceItem { id: "MCP02".into(), name: "Tool Poisoning".into(), covered: true, coverage: "Scanner + LLM analyzer detect hidden instructions in tool descriptions".into() },
            ComplianceItem { id: "MCP03".into(), name: "Privilege Escalation".into(), covered: true, coverage: "Permission engine enforces read-only/read-write scopes per server".into() },
            ComplianceItem { id: "MCP04".into(), name: "Supply Chain Attacks".into(), covered: true, coverage: "Server registry + threat feed + tool pinning".into() },
            ComplianceItem { id: "MCP05".into(), name: "Command Injection".into(), covered: true, coverage: "Output filter + LLM analyzer scan tool outputs for injection".into() },
            ComplianceItem { id: "MCP06".into(), name: "Context Over-sharing".into(), covered: true, coverage: "Behavioral fingerprinting detects unusual output sizes/entropy".into() },
            ComplianceItem { id: "MCP07".into(), name: "Insufficient Auth".into(), covered: true, coverage: "Permission enforcer requires explicit tool allowlists".into() },
            ComplianceItem { id: "MCP08".into(), name: "Insufficient Logging".into(), covered: true, coverage: "Audit logger + dashboard provide full event trail".into() },
            ComplianceItem { id: "MCP09".into(), name: "Shadow MCP Servers".into(), covered: true, coverage: "Config discovery + cross-server shadow detection + honeypot".into() },
            ComplianceItem { id: "MCP10".into(), name: "Covert Channel Abuse".into(), covered: true, coverage: "Exfiltration detector + behavioral anomaly detection".into() },
        ],
        agentic_top10: vec![
            ComplianceItem { id: "ASI01".into(), name: "Agent Goal Hijacking".into(), covered: true, coverage: "LLM intent classifier detects goal-override attempts".into() },
            ComplianceItem { id: "ASI02".into(), name: "Tool/Function Misuse".into(), covered: true, coverage: "Permission engine + behavioral profiling".into() },
            ComplianceItem { id: "ASI03".into(), name: "Insecure Agent Memory".into(), covered: true, coverage: "Output filter scans for memory-poisoning attempts".into() },
            ComplianceItem { id: "ASI04".into(), name: "Prompt Injection".into(), covered: true, coverage: "Scanner + LLM + behavioral analysis multi-layer defense".into() },
            ComplianceItem { id: "ASI05".into(), name: "Supply Chain Vulnerabilities".into(), covered: true, coverage: "Registry + threat feed + pinner + CVE mapping".into() },
            ComplianceItem { id: "ASI06".into(), name: "Unexpected Code Execution".into(), covered: true, coverage: "Permission scope blocks write/execute operations".into() },
            ComplianceItem { id: "ASI07".into(), name: "Agent Identity Spoofing".into(), covered: true, coverage: "Honeypot detects probing and impersonation".into() },
            ComplianceItem { id: "ASI08".into(), name: "Excessive Permissions".into(), covered: true, coverage: "Least-privilege enforcer with per-tool controls".into() },
            ComplianceItem { id: "ASI09".into(), name: "Insufficient Monitoring".into(), covered: true, coverage: "Dashboard + audit log + real-time alerting".into() },
            ComplianceItem { id: "ASI10".into(), name: "Multi-Agent Trust Breakdown".into(), covered: true, coverage: "Cross-server shadow analysis + registry trust scores".into() },
        ],
    }
}

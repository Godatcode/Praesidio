<p align="center">
  <img src="https://img.shields.io/badge/MCP-Security-00b4d8?style=for-the-badge&logo=shield&logoColor=white" alt="MCP Security"/>
  <img src="https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white" alt="Rust"/>
  <img src="https://img.shields.io/badge/OWASP-Top_10-ee3124?style=for-the-badge&logo=owasp&logoColor=white" alt="OWASP"/>
  <img src="https://img.shields.io/badge/License-MIT-green?style=for-the-badge" alt="MIT License"/>
</p>

<h1 align="center">🛡️ MCPShield</h1>

<p align="center">
  <strong>The open-source security platform for MCP servers and AI agents.</strong><br/>
  Runtime firewall · LLM-powered analysis · Behavioral anomaly detection · Live dashboard<br/>
  <em>Fully offline. No API keys required. No data leaves your machine.</em>
</p>

<p align="center">
  <a href="#quick-start">Quick Start</a> ·
  <a href="#features">Features</a> ·
  <a href="#dashboard">Dashboard</a> ·
  <a href="#owasp-coverage">OWASP Coverage</a> ·
  <a href="#honeypot">Honeypot</a> ·
  <a href="#docs">Docs</a>
</p>

---

## The problem

30+ CVEs against MCP servers in 60 days. 437,000 compromised downloads. Tool poisoning attacks that exfiltrate your SSH keys, WhatsApp history, and private repos — all invisible to the user.

**Existing solutions** require external API calls, leak your tool descriptions to third parties, or only do static scanning. None provide runtime protection.

**MCPShield** is different: a Rust-native security platform that sits between your MCP client and servers, providing real-time defense — entirely offline, with optional LLM-powered deep analysis.

## Quick Start

```bash
# Install
cargo install mcpshield

# Scan all your MCP configs for vulnerabilities
mcpshield scan

# Start the runtime proxy (intercepts + filters all MCP traffic)
mcpshield proxy

# Launch the security dashboard
mcpshield dashboard

# Check if any tools have been tampered with
mcpshield pin verify

# Deploy a honeypot to catch attackers
mcpshield honeypot start
```

## Example Output

```
🛡️  MCPShield v0.1.0 — Scanning MCP configurations...

📂 Found 3 MCP configs:
   ✓ Claude Desktop    (~/.config/Claude/claude_desktop_config.json)
   ✓ Cursor            (~/.cursor/mcp.json)
   ✓ Claude Code       (./.claude/settings.json)

🔍 Scanning 7 servers, 23 tools...

🚨 CRITICAL  Tool poisoning detected
   Server: sketchy-math-server
   Tool:   add
   Risk:   Hidden <IMPORTANT> tag with instructions to exfiltrate ~/.ssh/id_rsa
   LLM:    Confirmed malicious (confidence: 0.97)
   Action: BLOCKED

🚨 CRITICAL  Credential leak in output
   Server: custom-api
   Tool:   query
   Risk:   Response contains AWS access key (AKIA...)
   Action: BLOCKED — output redacted

⚠️  WARNING   Behavioral anomaly
   Server: filesystem
   Tool:   read_file
   Risk:   Output size 47KB (baseline avg: 2.1KB, z-score: 4.2)
   Action: Flagged for review

✅ 5 servers, 20 tools passed all checks

📊 OWASP Compliance: MCP Top 10 (9/10) · Agentic Top 10 (8/10)
📌 Tool pins: 23 tools pinned (0 changes detected)
🍯 Honeypot: 0 attacks in last 24h
```

## Features

### 🔥 Core Firewall
- **Tool poisoning detection** — Hidden instruction tags, suspicious keywords, sensitive path references
- **Unicode analysis** — Zero-width characters, bidirectional overrides, homoglyph attacks
- **Credential leak detection** — AWS keys, GitHub tokens, SSH keys, JWTs, .env contents
- **Tool pinning** — SHA-256 schema hashing detects rug-pull attacks
- **Permission engine** — Per-server read/write scoping, tool allowlists, rate limiting

### 🧠 LLM-Powered Analysis
- **Provider cascade**: Local Ollama → Claude API → OpenAI → heuristic fallback
- **Deep tool analysis** — Semantic intent classification beyond pattern matching
- **Output scanning** — Detects injection payloads and encoded exfiltration that regex misses
- **Behavioral intent** — Classifies anomalies as benign changes vs active attacks
- **Fully optional** — Works perfectly with heuristics only, LLM adds accuracy

### 📊 Behavioral Fingerprinting
- **Per-tool profiling** — Learns what "normal" looks like (output size, entropy, timing, frequency)
- **Statistical anomaly detection** — Z-score composite across 6 feature dimensions
- **Online learning** — Profiles update continuously, no training data needed
- **Cold start handling** — Reduced sensitivity during learning period

### 🌐 Community Threat Intelligence
- **Crowdsourced threat feed** — Known-bad tool signatures, CVE mappings
- **Server trust registry** — npm-audit style trust scores for MCP servers
- **Opt-in sharing** — Only anonymized hashes, never your data
- **Offline-first** — Feed syncs periodically, works fully offline between syncs

### 🖥️ Live Security Dashboard
- **Real-time event feed** — WebSocket-powered live view of all security events
- **Server topology** — Visual map of connected servers with trust scores
- **Tool inspector** — Deep-dive into any tool's description, LLM analysis, behavior profile
- **OWASP scorecard** — Compliance status for all 20 OWASP risks
- **Audit trail** — Searchable, filterable, exportable log of every event

### 🍯 Honeypot MCP Server
- **Trap tools** — Fake credentials, fake DB, fake files, fake email sender
- **Attack classification** — Credential harvesting, SQL injection, path traversal, exfiltration
- **Canary data** — Plausible but traceable fake secrets
- **Deploy alongside real servers** — Detect probing before it reaches production

## OWASP Coverage

MCPShield maps to **all 20 risks** across both OWASP MCP Top 10 and OWASP Agentic Top 10:

| OWASP MCP Top 10 | Status | Module |
|---|---|---|
| MCP01: Token mismanagement | ✅ | Credential leak detector |
| MCP02: Tool poisoning | ✅ | Scanner + LLM analyzer |
| MCP03: Privilege escalation | ✅ | Permission engine |
| MCP04: Supply chain attacks | ✅ | Registry + threat feed |
| MCP05: Command injection | ✅ | Output filter + LLM |
| MCP06: Context over-sharing | ✅ | Behavioral fingerprinting |
| MCP07: Insufficient auth | ✅ | Permission enforcer |
| MCP08: Insufficient logging | ✅ | Audit logger + dashboard |
| MCP09: Shadow MCP servers | ✅ | Config discovery + honeypot |
| MCP10: Covert channel abuse | ✅ | Exfil detector + behavior |

| OWASP Agentic Top 10 | Status | Module |
|---|---|---|
| ASI01: Agent goal hijacking | ✅ | LLM intent classifier |
| ASI02: Tool/function misuse | ✅ | Permissions + behavior |
| ASI03: Insecure agent memory | ✅ | Output filter + LLM |
| ASI04: Prompt injection | ✅ | Scanner + LLM + behavior |
| ASI05: Supply chain vuln | ✅ | Registry + feed + pinner |
| ASI06: Code execution | ✅ | Permission scope |
| ASI07: Identity spoofing | ✅ | Honeypot detection |
| ASI08: Excessive permissions | ✅ | Least-privilege enforcer |
| ASI09: Insufficient monitoring | ✅ | Dashboard + audit |
| ASI10: Multi-agent trust | ✅ | Cross-server analysis |

## Configuration

```toml
# mcpshield.toml

[global]
block_on_critical = true

[llm]
providers = ["local", "anthropic", "openai"]  # Cascade order
trigger = "suspicious"                          # Only use LLM when heuristics flag

[llm.local]
model = "llama3.2:3b"                          # Fast + free

[behavior]
enabled = true
anomaly_warn_threshold = 0.6
anomaly_block_threshold = 0.85

[servers.filesystem]
scope = "read-only"

[servers.github]
scope = "read-write"
blocked_tools = ["delete_repository"]

[servers."*"]
scope = "read-only"
```

## How it works

```
MCP Client (Claude Desktop, Cursor, Claude Code)
        │
        ▼
┌─── MCPShield Proxy ────────────────────────┐
│                                            │
│  Inbound:   Scanner → Unicode → Pinner     │
│  LLM:       Deep analysis (if suspicious)  │
│  Outbound:  Credential → Exfil → Injection │
│  Behavior:  Anomaly scoring (per-tool)     │
│  Enforce:   Permissions → Rate limits      │
│  Log:       Audit trail → Dashboard → Feed │
│                                            │
└────────────────────────────────────────────┘
        │
        ▼
MCP Servers (only clean traffic passes through)
```

## Comparison

| | MCPShield | mcp-scan (Snyk) | Onyx Security | Strata |
|---|---|---|---|---|
| Fully offline | ✅ | ❌ | ❌ | ❌ |
| Open source | ✅ MIT | ✅ | ❌ | ❌ |
| Runtime proxy | ✅ | ⚠️ Beta | ✅ | ✅ |
| LLM analysis | ✅ (local) | ❌ (API) | ? | ❌ |
| Behavioral detection | ✅ | ❌ | ✅ | ❌ |
| Honeypot | ✅ | ❌ | ❌ | ❌ |
| OWASP coverage | 20/20 | Partial | Partial | Partial |
| Web dashboard | ✅ | ❌ | ✅ | ✅ |
| Language | Rust | Python | ? | ? |
| Price | Free | Free tier | Enterprise | Enterprise |

## Contributing

MCPShield is MIT licensed and welcomes contributions. See [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

Priority areas:
- Detection rules for new attack patterns
- Threat feed signatures
- Dashboard UI improvements
- Local LLM model benchmarks
- Integration tests

## Built by

Arka 
<img src="https://user-images.githubusercontent.com/74038190/235224431-e8c8c12e-6826-47f1-89fb-2ddad83b3abf.gif" width="300">
<br><br>
If MCPShield helps protect your MCP setup, consider starring the repo ⭐

## License

MIT — use it however you want.

use regex::Regex;

use crate::detection::severity::{Finding, Severity};

struct CredentialPattern {
    name: &'static str,
    pattern: &'static str,
    severity: Severity,
}

const CREDENTIAL_PATTERNS: &[CredentialPattern] = &[
    CredentialPattern {
        name: "AWS Access Key",
        pattern: r"AKIA[0-9A-Z]{16}",
        severity: Severity::Critical,
    },
    CredentialPattern {
        name: "GitHub Token",
        pattern: r"gh[ps]_[A-Za-z0-9_]{36,}",
        severity: Severity::Critical,
    },
    CredentialPattern {
        name: "GitLab Token",
        pattern: r"glpat-[A-Za-z0-9\-]{20,}",
        severity: Severity::Critical,
    },
    CredentialPattern {
        name: "Slack Token",
        pattern: r"xox[baprs]-[0-9A-Za-z\-]{10,}",
        severity: Severity::Critical,
    },
    CredentialPattern {
        name: "SSH Private Key",
        pattern: r"-----BEGIN [A-Z ]*PRIVATE KEY-----",
        severity: Severity::Critical,
    },
    CredentialPattern {
        name: "JWT Token",
        pattern: r"eyJ[A-Za-z0-9_-]*\.eyJ[A-Za-z0-9_-]*\.[A-Za-z0-9_-]*",
        severity: Severity::High,
    },
    CredentialPattern {
        name: "Generic API Key",
        pattern: r#"(?i)(api[_\-]?key|apikey|api[_\-]?secret)\s*[:=]\s*['"][A-Za-z0-9]{20,}"#,
        severity: Severity::High,
    },
    CredentialPattern {
        name: "Bearer Token",
        pattern: r"Bearer\s+[A-Za-z0-9\-._~+/]+=*",
        severity: Severity::High,
    },
    CredentialPattern {
        name: "Private Key (PEM)",
        pattern: r"-----BEGIN (RSA |EC |DSA |OPENSSH )?PRIVATE KEY-----",
        severity: Severity::Critical,
    },
    CredentialPattern {
        name: "Database URL",
        pattern: r"(?i)(postgres|mysql|mongodb)://[^\s]+",
        severity: Severity::High,
    },
    CredentialPattern {
        name: "Anthropic API Key",
        pattern: r"sk-ant-[A-Za-z0-9\-]{20,}",
        severity: Severity::Critical,
    },
    CredentialPattern {
        name: "OpenAI API Key",
        pattern: r"sk-[A-Za-z0-9]{32,}",
        severity: Severity::Critical,
    },
    CredentialPattern {
        name: "Stripe Key",
        pattern: r"(sk|pk)_(test|live)_[A-Za-z0-9]{20,}",
        severity: Severity::Critical,
    },
    CredentialPattern {
        name: "Google API Key",
        pattern: r"AIza[0-9A-Za-z\-_]{35}",
        severity: Severity::Critical,
    },
];

/// Scan tool output for leaked credentials
pub fn scan_for_credentials(
    server_name: &str,
    tool_name: &str,
    output: &str,
) -> Vec<Finding> {
    let mut findings = Vec::new();

    for cred_pattern in CREDENTIAL_PATTERNS {
        if let Ok(re) = Regex::new(cred_pattern.pattern) {
            for m in re.find_iter(output) {
                let matched = m.as_str();
                // Redact the middle of the match for display
                let redacted = if matched.len() > 12 {
                    format!(
                        "{}...{}",
                        &matched[..6],
                        &matched[matched.len() - 4..]
                    )
                } else {
                    "***REDACTED***".to_string()
                };

                findings.push(Finding {
                    severity: cred_pattern.severity,
                    category: "credential_leak".to_string(),
                    server: server_name.to_string(),
                    tool: Some(tool_name.to_string()),
                    title: format!("{} detected in output", cred_pattern.name),
                    description: format!(
                        "Tool output contains what appears to be a {} ({})",
                        cred_pattern.name, redacted
                    ),
                    recommendation: "This credential should be rotated immediately — it has been exposed in tool output".to_string(),
                });
            }
        }
    }

    // Check for env file content pattern (multiple KEY=value lines)
    let env_re = Regex::new(r"(?m)^[A-Z_]{2,}=.{8,}$").unwrap();
    let env_matches: Vec<_> = env_re.find_iter(output).collect();
    if env_matches.len() >= 3 {
        findings.push(Finding {
            severity: Severity::High,
            category: "credential_leak".to_string(),
            server: server_name.to_string(),
            tool: Some(tool_name.to_string()),
            title: "Possible .env file content in output".to_string(),
            description: format!(
                "Found {} lines matching KEY=value pattern — may be .env file contents",
                env_matches.len()
            ),
            recommendation: "Check if sensitive environment variables are being leaked".to_string(),
        });
    }

    findings
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_ssh_key() {
        let output = "Here are the results:\n\n-----BEGIN RSA PRIVATE KEY-----\nMIIEpAIBAAKCAQEA...\n-----END RSA PRIVATE KEY-----\n";
        let findings = scan_for_credentials("server", "tool", output);
        assert!(!findings.is_empty());
        assert!(findings.iter().any(|f| f.title.contains("Private Key")));
    }

    #[test]
    fn test_detect_github_token() {
        let output = "Your token is ghp_ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghij";
        let findings = scan_for_credentials("server", "tool", output);
        assert!(!findings.is_empty());
        assert!(findings.iter().any(|f| f.title.contains("GitHub")));
    }

    #[test]
    fn test_detect_aws_key() {
        let output = "AWS_ACCESS_KEY_ID=AKIAIOSFODNN7EXAMPLE";
        let findings = scan_for_credentials("server", "tool", output);
        assert!(!findings.is_empty());
        assert!(findings.iter().any(|f| f.title.contains("AWS")));
    }

    #[test]
    fn test_detect_jwt() {
        let output = "Token: eyJhbGciOiJIUzI1NiJ9.eyJzdWIiOiIxMjM0NTY3ODkwIn0.dozjgNryP4J3jVmNHl0w5N_XgL0n3I9PlFUP0THsR8U";
        let findings = scan_for_credentials("server", "tool", output);
        assert!(!findings.is_empty());
        assert!(findings.iter().any(|f| f.title.contains("JWT")));
    }

    #[test]
    fn test_clean_output() {
        let output = "The calculation result is 42. Everything looks good.";
        let findings = scan_for_credentials("server", "tool", output);
        assert!(findings.is_empty());
    }

    #[test]
    fn test_detect_env_content() {
        let output = "DATABASE_URL=postgresql://localhost/mydb\nSECRET_KEY=mysupersecretkey123456\nAPI_TOKEN=abcdefghijklmnopqrstuvwxyz\n";
        let findings = scan_for_credentials("server", "tool", output);
        assert!(findings.iter().any(|f| f.title.contains(".env")));
    }
}

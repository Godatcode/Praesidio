use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Trust score for an MCP server package
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerTrustScore {
    pub server_name: String,
    pub package_source: String,
    pub publisher: String,
    pub first_seen: DateTime<Utc>,
    pub total_scans: u64,
    pub clean_scans: u64,
    pub flagged_scans: u64,
    pub known_cves: Vec<String>,
    pub trust_score: f64,
    pub last_updated: DateTime<Utc>,
    pub verified_publisher: bool,
}

impl ServerTrustScore {
    /// Calculate trust score based on available data
    pub fn calculate_score(&mut self) {
        if self.total_scans == 0 {
            self.trust_score = 0.5; // Unknown
            return;
        }

        // Base: ratio of clean scans
        let mut score = self.clean_scans as f64 / self.total_scans as f64;

        // Penalty for unpatched CVEs
        let unpatched_cves = self.known_cves.len() as f64;
        score -= unpatched_cves * 0.2;

        // Bonus for verified publisher
        if self.verified_publisher {
            score += 0.1;
        }

        // Bonus for age > 6 months with no issues
        let age_days = (Utc::now() - self.first_seen).num_days();
        if age_days > 180 && self.flagged_scans == 0 {
            score += 0.05;
        }

        // Clamp to [0.0, 1.0]
        self.trust_score = score.clamp(0.0, 1.0);
    }

    /// Human-readable trust level
    pub fn trust_level(&self) -> &str {
        match self.trust_score {
            s if s >= 0.8 => "HIGH",
            s if s >= 0.5 => "MEDIUM",
            s if s >= 0.3 => "LOW",
            _ => "DANGEROUS",
        }
    }

    /// Is this score confident (enough data)?
    pub fn is_confident(&self) -> bool {
        self.total_scans >= 50
    }
}

/// Registry of server trust scores
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ServerRegistry {
    pub servers: Vec<ServerTrustScore>,
}

impl ServerRegistry {
    /// Look up a server by name
    pub fn lookup(&self, name: &str) -> Option<&ServerTrustScore> {
        self.servers.iter().find(|s| s.server_name == name)
    }

    /// Search servers by partial name
    pub fn search(&self, query: &str) -> Vec<&ServerTrustScore> {
        let query_lower = query.to_lowercase();
        self.servers
            .iter()
            .filter(|s| s.server_name.to_lowercase().contains(&query_lower))
            .collect()
    }

    /// Load from file
    pub fn load(path: &std::path::Path) -> Self {
        if path.exists() {
            std::fs::read_to_string(path)
                .ok()
                .and_then(|s| serde_json::from_str(&s).ok())
                .unwrap_or_default()
        } else {
            Self::default()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trust_score_calculation() {
        let mut score = ServerTrustScore {
            server_name: "test-server".into(),
            package_source: "npm".into(),
            publisher: "test-org".into(),
            first_seen: Utc::now(),
            total_scans: 100,
            clean_scans: 95,
            flagged_scans: 5,
            known_cves: vec![],
            trust_score: 0.0,
            last_updated: Utc::now(),
            verified_publisher: true,
        };
        score.calculate_score();
        assert!(score.trust_score > 0.9);
        assert_eq!(score.trust_level(), "HIGH");
    }

    #[test]
    fn test_low_trust_with_cves() {
        let mut score = ServerTrustScore {
            server_name: "sketchy".into(),
            package_source: "npm".into(),
            publisher: "unknown".into(),
            first_seen: Utc::now(),
            total_scans: 10,
            clean_scans: 5,
            flagged_scans: 5,
            known_cves: vec!["CVE-2026-0001".into(), "CVE-2026-0002".into()],
            trust_score: 0.0,
            last_updated: Utc::now(),
            verified_publisher: false,
        };
        score.calculate_score();
        assert!(score.trust_score < 0.5);
    }
}

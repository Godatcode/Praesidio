use mcpshield_core::detection::severity::{Finding, Severity};

use crate::features::ToolBehaviorFeatures;
use crate::profile::ToolProfile;
use crate::BehaviorConfig;

/// Result of anomaly detection
#[derive(Debug, Clone)]
pub struct AnomalyResult {
    pub score: f64,
    pub is_anomalous: bool,
    pub should_block: bool,
    pub description: String,
}

/// Check a tool call against its behavioral profile
pub fn check_anomaly(
    profile: &ToolProfile,
    features: &ToolBehaviorFeatures,
    config: &BehaviorConfig,
) -> AnomalyResult {
    if !profile.is_mature(config.learning_period_calls) {
        return AnomalyResult {
            score: 0.0,
            is_anomalous: false,
            should_block: false,
            description: format!(
                "Learning phase ({}/{} observations)",
                profile.observation_count, config.learning_period_calls
            ),
        };
    }

    let score = profile.anomaly_score(features, &config.sensitivity);
    let is_anomalous = score >= config.anomaly_warn_threshold;
    let should_block = score >= config.anomaly_block_threshold;

    let description = if should_block {
        format!(
            "CRITICAL anomaly (score: {:.2}) — output size: {}B (baseline: {:.0}±{:.0}), entropy: {:.2} (baseline: {:.2}±{:.2})",
            score,
            features.output_size_bytes,
            profile.output_size.mean,
            profile.output_size.stddev,
            features.output_entropy,
            profile.output_entropy.mean,
            profile.output_entropy.stddev,
        )
    } else if is_anomalous {
        format!(
            "Anomaly detected (score: {:.2}) — deviates from baseline",
            score
        )
    } else {
        format!("Normal behavior (score: {:.2})", score)
    };

    AnomalyResult {
        score,
        is_anomalous,
        should_block,
        description,
    }
}

/// Convert anomaly result to a Finding for the audit system
pub fn anomaly_to_finding(
    server_name: &str,
    tool_name: &str,
    result: &AnomalyResult,
) -> Option<Finding> {
    if !result.is_anomalous {
        return None;
    }

    Some(Finding {
        severity: if result.should_block {
            Severity::Critical
        } else {
            Severity::Medium
        },
        category: "behavior_anomaly".to_string(),
        server: server_name.to_string(),
        tool: Some(tool_name.to_string()),
        title: format!("Behavioral anomaly (score: {:.2})", result.score),
        description: result.description.clone(),
        recommendation: if result.should_block {
            "Tool output significantly deviates from baseline — blocking recommended".to_string()
        } else {
            "Monitor this tool — behavior is unusual compared to baseline".to_string()
        },
    })
}

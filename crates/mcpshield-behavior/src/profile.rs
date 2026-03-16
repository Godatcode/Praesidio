use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::baseline::GaussianEstimate;
use crate::features::ToolBehaviorFeatures;
use crate::SensitivityConfig;

/// Behavioral profile for a single tool
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolProfile {
    pub server_name: String,
    pub tool_name: String,
    pub observation_count: u64,
    pub first_seen: DateTime<Utc>,
    pub last_updated: DateTime<Utc>,

    // Statistical baselines
    pub output_size: GaussianEstimate,
    pub response_time: GaussianEstimate,
    pub output_entropy: GaussianEstimate,
    pub base64_density: GaussianEstimate,
    pub call_frequency: GaussianEstimate,
}

impl ToolProfile {
    pub fn new(server_name: &str, tool_name: &str) -> Self {
        let now = Utc::now();
        Self {
            server_name: server_name.to_string(),
            tool_name: tool_name.to_string(),
            observation_count: 0,
            first_seen: now,
            last_updated: now,
            output_size: GaussianEstimate::new(),
            response_time: GaussianEstimate::new(),
            output_entropy: GaussianEstimate::new(),
            base64_density: GaussianEstimate::new(),
            call_frequency: GaussianEstimate::new(),
        }
    }

    /// Update profile with a new observation
    pub fn observe(&mut self, features: &ToolBehaviorFeatures) {
        self.observation_count += 1;
        self.last_updated = Utc::now();

        self.output_size.observe(features.output_size_bytes as f64);
        self.response_time.observe(features.response_time_ms as f64);
        self.output_entropy.observe(features.output_entropy);
        self.base64_density.observe(features.base64_density);
        self.call_frequency
            .observe(features.call_frequency_per_min);
    }

    /// Calculate composite anomaly score (0.0 = normal, 1.0 = extreme outlier)
    pub fn anomaly_score(
        &self,
        features: &ToolBehaviorFeatures,
        sensitivity: &SensitivityConfig,
    ) -> f64 {
        if self.observation_count < 2 {
            return 0.0; // Can't detect anomalies without baseline
        }

        let scores = [
            self.output_size.z_score(features.output_size_bytes as f64)
                * 0.20
                * sensitivity.weight("output_size")
                * self.output_size.confidence(),
            self.output_entropy.z_score(features.output_entropy)
                * 0.20
                * sensitivity.weight("entropy")
                * self.output_entropy.confidence(),
            self.base64_density.z_score(features.base64_density)
                * 0.20
                * sensitivity.weight("base64_density")
                * self.base64_density.confidence(),
            self.response_time.z_score(features.response_time_ms as f64)
                * 0.10
                * sensitivity.weight("response_time")
                * self.response_time.confidence(),
            self.call_frequency
                .z_score(features.call_frequency_per_min)
                * 0.15
                * sensitivity.weight("call_frequency")
                * self.call_frequency.confidence(),
            // Burst detection bonus
            if features.burst_score > 3.0 {
                0.15
            } else {
                0.0
            },
        ];

        // Normalize: divide by max possible score to keep in [0, 1]
        let raw: f64 = scores.iter().sum();
        // Apply sigmoid-like normalization for extreme values
        normalize_score(raw)
    }

    /// Is the profile mature enough for reliable anomaly detection?
    pub fn is_mature(&self, min_observations: u64) -> bool {
        self.observation_count >= min_observations
    }

    /// Human-readable baseline description
    pub fn baseline_description(&self) -> String {
        format!(
            "avg output: {:.0}B (±{:.0}), avg response: {:.0}ms, avg entropy: {:.2}, avg b64 density: {:.3}",
            self.output_size.mean,
            self.output_size.stddev,
            self.response_time.mean,
            self.output_entropy.mean,
            self.base64_density.mean
        )
    }
}

/// Normalize raw anomaly score to [0.0, 1.0] using a sigmoid-like curve
fn normalize_score(raw: f64) -> f64 {
    if raw <= 0.0 {
        return 0.0;
    }
    // tanh gives a smooth mapping: 0→0, 1→0.76, 2→0.96, 3→0.995
    raw.tanh().min(1.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_features(size: u64, entropy: f64, base64: f64) -> ToolBehaviorFeatures {
        ToolBehaviorFeatures {
            output_size_bytes: size,
            response_time_ms: 50,
            output_entropy: entropy,
            output_charset_ratio: 0.95,
            base64_density: base64,
            json_depth: 2,
            field_count: 5,
            contains_code_patterns: false,
            contains_path_patterns: false,
            time_since_last_call_ms: 5000,
            call_frequency_per_min: 2.0,
            burst_score: 0.0,
        }
    }

    #[test]
    fn test_profile_learning() {
        let mut profile = ToolProfile::new("server", "tool");
        let sensitivity = SensitivityConfig::default();

        // Build baseline with normal observations (natural variation)
        let sizes: Vec<u64> = vec![180, 220, 195, 210, 200, 190, 215, 185, 205, 200,
                                    195, 210, 200, 190, 215, 185, 205, 200, 195, 210];
        let entropies: Vec<f64> = vec![3.8, 4.2, 3.9, 4.1, 4.0, 3.85, 4.15, 3.95, 4.05, 4.0,
                                        3.9, 4.1, 4.0, 3.85, 4.15, 3.95, 4.05, 4.0, 3.9, 4.1];
        for i in 0..20 {
            let features = make_features(sizes[i], entropies[i], 0.01);
            profile.observe(&features);
        }

        // Normal observation (within baseline range) should have low anomaly score
        let normal = make_features(200, 4.0, 0.01);
        let score = profile.anomaly_score(&normal, &sensitivity);
        assert!(score < 0.3, "Normal observation scored too high: {}", score);

        // Anomalous observation (huge output + high entropy + high base64)
        let anomalous = make_features(50000, 7.8, 0.9);
        let score = profile.anomaly_score(&anomalous, &sensitivity);
        assert!(
            score > 0.5,
            "Anomalous observation scored too low: {}",
            score
        );
    }

    #[test]
    fn test_cold_start() {
        let profile = ToolProfile::new("server", "tool");
        let sensitivity = SensitivityConfig::default();
        let features = make_features(50000, 7.8, 0.9);
        // With no baseline, anomaly score should be 0
        assert_eq!(profile.anomaly_score(&features, &sensitivity), 0.0);
    }

    #[test]
    fn test_maturity() {
        let mut profile = ToolProfile::new("server", "tool");
        assert!(!profile.is_mature(20));
        for _ in 0..20 {
            profile.observe(&make_features(100, 4.0, 0.0));
        }
        assert!(profile.is_mature(20));
    }
}

use serde::{Deserialize, Serialize};

/// Gaussian estimate using Welford's online algorithm
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GaussianEstimate {
    pub mean: f64,
    pub stddev: f64,
    pub min_observed: f64,
    pub max_observed: f64,
    pub sample_count: u64,
    // Internal state for Welford's algorithm
    m2: f64,
}

impl GaussianEstimate {
    pub fn new() -> Self {
        Self {
            mean: 0.0,
            stddev: 0.0,
            min_observed: f64::MAX,
            max_observed: f64::MIN,
            sample_count: 0,
            m2: 0.0,
        }
    }

    /// Update the estimate with a new observation (Welford's online algorithm)
    pub fn observe(&mut self, value: f64) {
        self.sample_count += 1;
        self.min_observed = self.min_observed.min(value);
        self.max_observed = self.max_observed.max(value);

        let delta = value - self.mean;
        self.mean += delta / self.sample_count as f64;
        let delta2 = value - self.mean;
        self.m2 += delta * delta2;

        if self.sample_count > 1 {
            self.stddev = (self.m2 / (self.sample_count - 1) as f64).sqrt();
        }
    }

    /// Calculate z-score for a new value
    /// Returns 0.0 if not enough samples or stddev is 0
    pub fn z_score(&self, value: f64) -> f64 {
        if self.sample_count < 2 || self.stddev < f64::EPSILON {
            return 0.0;
        }
        ((value - self.mean) / self.stddev).abs()
    }

    /// How confident are we in this estimate? (0.0 to 1.0)
    pub fn confidence(&self) -> f64 {
        // Ramp up confidence as we get more samples
        let n = self.sample_count as f64;
        (n / 20.0).min(1.0) // Full confidence after 20 observations
    }
}

impl Default for GaussianEstimate {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_welford_basic() {
        let mut est = GaussianEstimate::new();
        for v in &[10.0, 12.0, 11.0, 13.0, 9.0] {
            est.observe(*v);
        }
        assert_eq!(est.sample_count, 5);
        assert!((est.mean - 11.0).abs() < 0.01);
        assert!(est.stddev > 0.0);
    }

    #[test]
    fn test_z_score() {
        let mut est = GaussianEstimate::new();
        for v in &[100.0, 102.0, 98.0, 101.0, 99.0, 100.0, 103.0, 97.0, 100.0, 101.0] {
            est.observe(*v);
        }
        // A value very close to mean should have low z-score
        assert!(est.z_score(100.0) < 1.0);
        // A value far from mean should have high z-score
        assert!(est.z_score(200.0) > 3.0);
    }

    #[test]
    fn test_confidence_ramp() {
        let mut est = GaussianEstimate::new();
        assert_eq!(est.confidence(), 0.0);

        for i in 0..10 {
            est.observe(i as f64);
        }
        assert!((est.confidence() - 0.5).abs() < 0.01);

        for i in 10..20 {
            est.observe(i as f64);
        }
        assert!((est.confidence() - 1.0).abs() < 0.01);
    }

    #[test]
    fn test_min_max() {
        let mut est = GaussianEstimate::new();
        est.observe(5.0);
        est.observe(15.0);
        est.observe(10.0);
        assert_eq!(est.min_observed, 5.0);
        assert_eq!(est.max_observed, 15.0);
    }
}

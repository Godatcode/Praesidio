/// Features extracted from a single tool call observation
#[derive(Debug, Clone)]
pub struct ToolBehaviorFeatures {
    // Response characteristics
    pub output_size_bytes: u64,
    pub response_time_ms: u64,
    pub output_entropy: f64,
    pub output_charset_ratio: f64,
    pub base64_density: f64,

    // Structural features
    pub json_depth: u32,
    pub field_count: u32,
    pub contains_code_patterns: bool,
    pub contains_path_patterns: bool,

    // Temporal features
    pub time_since_last_call_ms: u64,
    pub call_frequency_per_min: f64,
    pub burst_score: f64,
}

impl ToolBehaviorFeatures {
    /// Extract features from a tool output string and timing info
    pub fn extract(
        output: &str,
        response_time_ms: u64,
        time_since_last_call_ms: u64,
        call_frequency_per_min: f64,
    ) -> Self {
        let output_bytes = output.as_bytes();

        Self {
            output_size_bytes: output_bytes.len() as u64,
            response_time_ms,
            output_entropy: shannon_entropy(output_bytes),
            output_charset_ratio: ascii_ratio(output_bytes),
            base64_density: base64_density(output),
            json_depth: json_nesting_depth(output),
            field_count: count_json_fields(output),
            contains_code_patterns: has_code_patterns(output),
            contains_path_patterns: has_path_patterns(output),
            time_since_last_call_ms,
            call_frequency_per_min,
            burst_score: if call_frequency_per_min > 10.0 {
                call_frequency_per_min / 10.0
            } else {
                0.0
            },
        }
    }
}

/// Calculate Shannon entropy of byte data (0.0 = uniform, ~8.0 = random)
fn shannon_entropy(data: &[u8]) -> f64 {
    if data.is_empty() {
        return 0.0;
    }

    let mut freq = [0u64; 256];
    for &byte in data {
        freq[byte as usize] += 1;
    }

    let len = data.len() as f64;
    let mut entropy = 0.0;
    for &count in &freq {
        if count > 0 {
            let p = count as f64 / len;
            entropy -= p * p.log2();
        }
    }
    entropy
}

/// Ratio of ASCII printable characters to total
fn ascii_ratio(data: &[u8]) -> f64 {
    if data.is_empty() {
        return 1.0;
    }
    let ascii_count = data.iter().filter(|&&b| b.is_ascii_graphic() || b == b' ' || b == b'\n' || b == b'\t').count();
    ascii_count as f64 / data.len() as f64
}

/// Estimate fraction of output that looks like base64
fn base64_density(text: &str) -> f64 {
    if text.is_empty() {
        return 0.0;
    }

    let mut base64_chars = 0u64;
    for ch in text.chars() {
        if ch.is_ascii_alphanumeric() || ch == '+' || ch == '/' || ch == '=' {
            base64_chars += 1;
        }
    }

    // Only count as base64-like if there are long runs without spaces/newlines
    let max_run = text
        .split(|c: char| c.is_whitespace())
        .map(|s| s.len())
        .max()
        .unwrap_or(0);

    if max_run > 40 {
        base64_chars as f64 / text.len() as f64
    } else {
        0.0
    }
}

/// Estimate JSON nesting depth
fn json_nesting_depth(text: &str) -> u32 {
    let mut max_depth = 0u32;
    let mut depth = 0u32;
    for ch in text.chars() {
        match ch {
            '{' | '[' => {
                depth += 1;
                max_depth = max_depth.max(depth);
            }
            '}' | ']' => {
                depth = depth.saturating_sub(1);
            }
            _ => {}
        }
    }
    max_depth
}

/// Count JSON-like fields (keys followed by colons)
fn count_json_fields(text: &str) -> u32 {
    text.matches("\":").count() as u32
}

/// Check for common code patterns
fn has_code_patterns(text: &str) -> bool {
    let patterns = [
        "function ", "def ", "class ", "import ", "require(",
        "const ", "let ", "var ", "fn ", "pub fn",
    ];
    patterns.iter().any(|p| text.contains(p))
}

/// Check for file path patterns
fn has_path_patterns(text: &str) -> bool {
    let patterns = ["/home/", "/Users/", "/etc/", "/var/", "C:\\", "~/."];
    patterns.iter().any(|p| text.contains(p))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shannon_entropy() {
        let low = shannon_entropy(b"aaaaaaaaaa");
        let high = shannon_entropy(b"abcdefghijklmnop");
        assert!(low < high);
    }

    #[test]
    fn test_ascii_ratio() {
        assert!((ascii_ratio(b"Hello world") - 1.0).abs() < 0.01);
        assert!(ascii_ratio(&[0xFF, 0xFE, 0x00, 0x01]) < 0.5);
    }

    #[test]
    fn test_base64_density() {
        let normal = "Hello, this is a normal text output.";
        let b64 = "SGVsbG8gV29ybGQhIFRoaXMgaXMgYSBsb25nIGJhc2U2NCBlbmNvZGVkIHN0cmluZyB0aGF0IHNob3VsZCBiZSBkZXRlY3RlZA==";
        assert!(base64_density(b64) > base64_density(normal));
    }

    #[test]
    fn test_json_depth() {
        assert_eq!(json_nesting_depth(r#"{"a":{"b":{"c":1}}}"#), 3);
        assert_eq!(json_nesting_depth("no json here"), 0);
    }

    #[test]
    fn test_feature_extraction() {
        let features = ToolBehaviorFeatures::extract(
            "Hello world, this is a test output",
            50,
            5000,
            2.0,
        );
        assert!(features.output_entropy > 0.0);
        assert!(features.output_charset_ratio > 0.9);
        assert!(!features.contains_code_patterns);
    }
}

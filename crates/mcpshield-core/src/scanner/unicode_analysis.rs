use crate::detection::severity::{Finding, Severity};

#[derive(Debug, Clone)]
pub struct UnicodeAnomaly {
    pub position: usize,
    pub codepoint: u32,
    pub category: UnicodeCategory,
    pub explanation: String,
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnicodeCategory {
    ZeroWidth,
    BidiOverride,
    Homoglyph,
    InvisibleFormatting,
    TagCharacter,
    SuspiciousWhitespace,
}

impl std::fmt::Display for UnicodeCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UnicodeCategory::ZeroWidth => write!(f, "zero-width character"),
            UnicodeCategory::BidiOverride => write!(f, "bidirectional override"),
            UnicodeCategory::Homoglyph => write!(f, "homoglyph"),
            UnicodeCategory::InvisibleFormatting => write!(f, "invisible formatting"),
            UnicodeCategory::TagCharacter => write!(f, "tag character"),
            UnicodeCategory::SuspiciousWhitespace => write!(f, "suspicious whitespace"),
        }
    }
}

/// Analyze text for hidden or deceptive Unicode characters
pub fn analyze_unicode(text: &str) -> Vec<UnicodeAnomaly> {
    let mut anomalies = Vec::new();

    for (pos, ch) in text.char_indices() {
        let cp = ch as u32;

        // Zero-width characters
        if matches!(cp, 0x200B | 0x200C | 0x200D | 0xFEFF) {
            anomalies.push(UnicodeAnomaly {
                position: pos,
                codepoint: cp,
                category: UnicodeCategory::ZeroWidth,
                explanation: format!(
                    "Zero-width character U+{:04X} at position {} — can hide content",
                    cp, pos
                ),
            });
        }

        // Bidirectional overrides
        if matches!(cp, 0x202A..=0x202E | 0x2066..=0x2069) {
            anomalies.push(UnicodeAnomaly {
                position: pos,
                codepoint: cp,
                category: UnicodeCategory::BidiOverride,
                explanation: format!(
                    "Bidirectional override U+{:04X} at position {} — can reverse text display direction",
                    cp, pos
                ),
            });
        }

        // Invisible formatting characters
        if matches!(cp, 0x00AD | 0x034F | 0x061C | 0x180E) {
            anomalies.push(UnicodeAnomaly {
                position: pos,
                codepoint: cp,
                category: UnicodeCategory::InvisibleFormatting,
                explanation: format!(
                    "Invisible formatting character U+{:04X} at position {}",
                    cp, pos
                ),
            });
        }

        // Tag characters (deprecated but renderable)
        if matches!(cp, 0xE0001..=0xE007F) {
            anomalies.push(UnicodeAnomaly {
                position: pos,
                codepoint: cp,
                category: UnicodeCategory::TagCharacter,
                explanation: format!(
                    "Tag character U+{:04X} at position {} — deprecated Unicode tag",
                    cp, pos
                ),
            });
        }

        // Suspicious whitespace (various-width spaces beyond normal space/tab/newline)
        if matches!(cp, 0x2000..=0x200A | 0x205F | 0x3000) {
            anomalies.push(UnicodeAnomaly {
                position: pos,
                codepoint: cp,
                category: UnicodeCategory::SuspiciousWhitespace,
                explanation: format!(
                    "Non-standard whitespace U+{:04X} at position {} — may be used to evade pattern matching",
                    cp, pos
                ),
            });
        }

        // Common homoglyphs (Cyrillic that looks like Latin)
        if is_cyrillic_homoglyph(ch) {
            anomalies.push(UnicodeAnomaly {
                position: pos,
                codepoint: cp,
                category: UnicodeCategory::Homoglyph,
                explanation: format!(
                    "Cyrillic homoglyph '{}' (U+{:04X}) at position {} — visually identical to Latin character",
                    ch, cp, pos
                ),
            });
        }
    }

    anomalies
}

/// Check if a character is a Cyrillic homoglyph of a Latin character
fn is_cyrillic_homoglyph(ch: char) -> bool {
    matches!(
        ch,
        'а' | // U+0430 looks like 'a'
        'е' | // U+0435 looks like 'e'
        'о' | // U+043E looks like 'o'
        'р' | // U+0440 looks like 'p'
        'с' | // U+0441 looks like 'c'
        'у' | // U+0443 looks like 'y'
        'х' | // U+0445 looks like 'x'
        'А' | // U+0410 looks like 'A'
        'В' | // U+0412 looks like 'B'
        'Е' | // U+0415 looks like 'E'
        'К' | // U+041A looks like 'K'
        'М' | // U+041C looks like 'M'
        'Н' | // U+041D looks like 'H'
        'О' | // U+041E looks like 'O'
        'Р' | // U+0420 looks like 'P'
        'С' | // U+0421 looks like 'C'
        'Т' | // U+0422 looks like 'T'
        'Х'   // U+0425 looks like 'X'
    )
}

/// Convert unicode anomalies into findings
pub fn scan_unicode(
    server_name: &str,
    tool_name: &str,
    text: &str,
) -> Vec<Finding> {
    let anomalies = analyze_unicode(text);
    if anomalies.is_empty() {
        return Vec::new();
    }

    let mut findings = Vec::new();

    // Group by category
    let zero_width: Vec<_> = anomalies
        .iter()
        .filter(|a| a.category == UnicodeCategory::ZeroWidth)
        .collect();
    let bidi: Vec<_> = anomalies
        .iter()
        .filter(|a| a.category == UnicodeCategory::BidiOverride)
        .collect();
    let homoglyphs: Vec<_> = anomalies
        .iter()
        .filter(|a| a.category == UnicodeCategory::Homoglyph)
        .collect();
    let other: Vec<_> = anomalies
        .iter()
        .filter(|a| {
            !matches!(
                a.category,
                UnicodeCategory::ZeroWidth
                    | UnicodeCategory::BidiOverride
                    | UnicodeCategory::Homoglyph
            )
        })
        .collect();

    if !zero_width.is_empty() {
        findings.push(Finding {
            severity: Severity::Critical,
            category: "unicode".to_string(),
            server: server_name.to_string(),
            tool: Some(tool_name.to_string()),
            title: format!("{} zero-width characters detected", zero_width.len()),
            description: "Zero-width characters can hide malicious instructions that are invisible to users but processed by LLMs".to_string(),
            recommendation: "Remove this tool — zero-width characters in descriptions are a strong attack indicator".to_string(),
        });
    }

    if !bidi.is_empty() {
        findings.push(Finding {
            severity: Severity::Critical,
            category: "unicode".to_string(),
            server: server_name.to_string(),
            tool: Some(tool_name.to_string()),
            title: format!("{} bidirectional override characters detected", bidi.len()),
            description: "Bidi overrides can make text display in reverse, hiding the true content".to_string(),
            recommendation: "Remove this tool — bidi overrides are used in trojan source attacks".to_string(),
        });
    }

    if !homoglyphs.is_empty() {
        findings.push(Finding {
            severity: Severity::High,
            category: "unicode".to_string(),
            server: server_name.to_string(),
            tool: Some(tool_name.to_string()),
            title: format!("{} homoglyph characters detected", homoglyphs.len()),
            description: "Homoglyphs are characters that look identical to ASCII but are different Unicode codepoints — used to evade pattern matching".to_string(),
            recommendation: "Review tool description for character substitution attacks".to_string(),
        });
    }

    if !other.is_empty() {
        findings.push(Finding {
            severity: Severity::Medium,
            category: "unicode".to_string(),
            server: server_name.to_string(),
            tool: Some(tool_name.to_string()),
            title: format!(
                "{} suspicious Unicode characters detected",
                other.len()
            ),
            description: other
                .iter()
                .map(|a| a.explanation.clone())
                .collect::<Vec<_>>()
                .join("; "),
            recommendation: "Review tool description for hidden formatting characters".to_string(),
        });
    }

    findings
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zero_width_detection() {
        let text = "Normal text\u{200B}with hidden\u{200C}chars";
        let anomalies = analyze_unicode(text);
        assert_eq!(anomalies.len(), 2);
        assert!(anomalies.iter().all(|a| a.category == UnicodeCategory::ZeroWidth));
    }

    #[test]
    fn test_bidi_detection() {
        let text = "Hello \u{202E}dlrow";
        let anomalies = analyze_unicode(text);
        assert_eq!(anomalies.len(), 1);
        assert_eq!(anomalies[0].category, UnicodeCategory::BidiOverride);
    }

    #[test]
    fn test_homoglyph_detection() {
        // Using Cyrillic 'а' (U+0430) instead of Latin 'a'
        let text = "reаd_file"; // 'а' is Cyrillic
        let anomalies = analyze_unicode(text);
        assert_eq!(anomalies.len(), 1);
        assert_eq!(anomalies[0].category, UnicodeCategory::Homoglyph);
    }

    #[test]
    fn test_clean_ascii() {
        let text = "Read the contents of a file at the given path.";
        let anomalies = analyze_unicode(text);
        assert!(anomalies.is_empty());
    }

    #[test]
    fn test_scan_unicode_findings() {
        let text = "Add numbers\u{200B}together";
        let findings = scan_unicode("test-server", "add", text);
        assert!(!findings.is_empty());
        assert!(findings.iter().any(|f| f.severity == Severity::Critical));
    }
}

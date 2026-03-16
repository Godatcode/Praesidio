use std::collections::HashMap;

use crate::logger::HoneypotEvent;

/// Summary report of honeypot activity
#[derive(Debug, serde::Serialize)]
pub struct HoneypotReport {
    pub total_events: usize,
    pub by_type: HashMap<String, usize>,
    pub by_tool: HashMap<String, usize>,
    pub top_indicators: Vec<(String, usize)>,
}

impl HoneypotReport {
    /// Generate a report from a list of honeypot events
    pub fn from_events(events: &[HoneypotEvent]) -> Self {
        let mut by_type: HashMap<String, usize> = HashMap::new();
        let mut by_tool: HashMap<String, usize> = HashMap::new();
        let mut indicator_counts: HashMap<String, usize> = HashMap::new();

        for event in events {
            let type_name = format!("{:?}", event.classification);
            *by_type.entry(type_name).or_default() += 1;
            *by_tool.entry(event.tool_called.clone()).or_default() += 1;

            for indicator in &event.indicators {
                *indicator_counts.entry(indicator.clone()).or_default() += 1;
            }
        }

        let mut top_indicators: Vec<_> = indicator_counts.into_iter().collect();
        top_indicators.sort_by(|a, b| b.1.cmp(&a.1));
        top_indicators.truncate(10);

        Self {
            total_events: events.len(),
            by_type,
            by_tool,
            top_indicators,
        }
    }
}

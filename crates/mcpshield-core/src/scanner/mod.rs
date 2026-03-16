pub mod cross_server;
pub mod pattern_matcher;
pub mod tool_poisoning;
pub mod unicode_analysis;

use crate::detection::severity::Finding;
use cross_server::ToolInfo;

/// Run all scanners on a single tool's description
pub fn scan_tool(
    server_name: &str,
    tool_name: &str,
    description: &str,
) -> Vec<Finding> {
    let mut findings = Vec::new();

    // Tool poisoning analysis
    findings.extend(tool_poisoning::scan_tool_description(
        server_name,
        tool_name,
        description,
    ));

    // Unicode analysis
    findings.extend(unicode_analysis::scan_unicode(
        server_name,
        tool_name,
        description,
    ));

    // Known injection pattern matching
    findings.extend(pattern_matcher::scan_patterns(
        server_name,
        tool_name,
        description,
    ));

    findings
}

/// Run cross-server shadow analysis across all tools
pub fn scan_cross_server(tools: &[ToolInfo]) -> Vec<Finding> {
    cross_server::detect_cross_server_shadows(tools)
}

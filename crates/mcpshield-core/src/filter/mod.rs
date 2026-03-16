pub mod credential_leak;
pub mod exfil_detector;
pub mod injection_filter;

use crate::detection::severity::Finding;

/// Run all output filters on tool output
pub fn filter_output(
    server_name: &str,
    tool_name: &str,
    output: &str,
) -> Vec<Finding> {
    let mut findings = Vec::new();

    findings.extend(credential_leak::scan_for_credentials(
        server_name,
        tool_name,
        output,
    ));

    findings.extend(injection_filter::detect_output_injection(
        server_name,
        tool_name,
        output,
    ));

    findings.extend(exfil_detector::detect_exfiltration(
        server_name,
        tool_name,
        output,
    ));

    findings
}

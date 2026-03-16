use colored::Colorize;

use mcpshield_core::config::Config;
use mcpshield_threat_feed::registry::ServerRegistry;

use crate::RegistryAction;

pub fn run(config: &Config, action: RegistryAction) {
    let data_dir = config.audit_dir().parent().unwrap_or(&config.audit_dir()).to_path_buf();
    let registry_path = data_dir.join("server_trust").join("registry.json");
    let registry = ServerRegistry::load(&registry_path);

    match action {
        RegistryAction::Check { server_name } => {
            match registry.lookup(&server_name) {
                Some(score) => {
                    let trust_color = match score.trust_level() {
                        "HIGH" => "HIGH".green(),
                        "MEDIUM" => "MEDIUM".yellow(),
                        "LOW" => "LOW".red(),
                        _ => "DANGEROUS".red().bold(),
                    };

                    println!("{}  {}", "📦", server_name.bold());
                    println!("   Publisher: {}{}", score.publisher,
                        if score.verified_publisher { " (verified ✅)" } else { " ⚠️" });
                    println!("   Trust score: {:.2} / 1.0 ({})", score.trust_score, trust_color);
                    println!("   Total scans: {}", score.total_scans);
                    if !score.known_cves.is_empty() {
                        println!("   Known CVEs: {}", score.known_cves.join(", "));
                    }
                    println!("   Confident: {}", if score.is_confident() { "Yes" } else { "No (< 50 scans)" });
                }
                None => {
                    println!("{}  Server '{}' not found in registry.", "⚠️", server_name);
                    println!("   This server has not been scanned by the community yet.");
                }
            }
        }
        RegistryAction::Search { query } => {
            let results = registry.search(&query);
            if results.is_empty() {
                println!("No servers found matching '{}'", query);
                return;
            }
            println!("{}  Found {} server(s):\n", "🔍", results.len());
            for score in results {
                println!(
                    "   {} — trust: {:.2} ({}) — {} scans",
                    score.server_name.bold(),
                    score.trust_score,
                    score.trust_level(),
                    score.total_scans
                );
            }
        }
    }
}

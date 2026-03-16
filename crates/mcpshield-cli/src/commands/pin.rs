use std::process;

use colored::Colorize;

use mcpshield_core::config::Config;
use mcpshield_core::pinner::store::PinStore;

use crate::PinAction;

pub fn run(config: &Config, action: PinAction) {
    let mut store = PinStore::load(&config.pin_file());

    match action {
        PinAction::List => {
            let pins = store.list();
            if pins.is_empty() {
                println!("No pinned tools. Run 'mcpshield scan' to pin current tool schemas.");
                return;
            }
            println!("{}  Pinned tools ({}):\n", "📌", pins.len());
            for pin in pins {
                println!(
                    "   {} :: {} (first seen: {}, last verified: {})",
                    pin.server_name.cyan(),
                    pin.tool_name.bold(),
                    pin.first_seen.format("%Y-%m-%d %H:%M"),
                    pin.last_verified.format("%Y-%m-%d %H:%M")
                );
            }
        }
        PinAction::Verify => {
            println!("{}  Verifying pinned tools...\n", "🔍");
            println!("Run 'mcpshield scan' to verify all pinned tools against current server state.");
        }
        PinAction::Reset => {
            store.reset();
            if let Err(e) = store.save(&config.pin_file()) {
                eprintln!("Error saving pin store: {}", e);
                process::exit(1);
            }
            println!("{}  All pins have been reset.", "✓");
        }
    }
}

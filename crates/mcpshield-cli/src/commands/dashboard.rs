pub fn run(port: u16) {
    println!("{}  MCPShield Dashboard", "📊");
    println!("   Starting on http://127.0.0.1:{}", port);
    println!("   Run 'cd dashboard && npm run dev' for the frontend.");
    println!("   Or run the API server: 'cargo run -p mcpshield-server'");
    println!("\n   Dashboard is served by the mcpshield-server crate.");
}

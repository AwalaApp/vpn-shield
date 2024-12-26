use anyhow::Result;
use awala_vpn_shield::app::server;
use std::env;

#[tokio::main]
async fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    match args.get(1).map(|s| s.as_str()) {
        Some("serve") => {
            server::run_server().await?;
        }
        Some(cmd) => {
            eprintln!("Unknown command: {}", cmd);
            std::process::exit(1);
        }
        None => {
            eprintln!("Usage: {} serve", args[0]);
            std::process::exit(1);
        }
    }

    Ok(())
}

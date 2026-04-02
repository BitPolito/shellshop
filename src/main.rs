//! ShellShop: SSH-first terminal shopping TUI.

mod app;
mod host_key;
mod ssh_server;

use std::path::PathBuf;
use std::sync::Arc;

use russh::server::Server;

struct CliConfig {
    host: String,
    port: u16,
    key_path: PathBuf,
}

impl Default for CliConfig {
    fn default() -> Self {
        Self {
            host: "0.0.0.0".to_string(),
            port: 22,
            key_path: PathBuf::from("host_key"),
        }
    }
}

fn parse_cli(args: &[String]) -> CliConfig {
    let mut c = CliConfig::default();
    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--help" | "-h" => {
                eprintln!(
                    "\
ShellShop — SSH server (default) or local TUI

USAGE:
    shellshop [OPTIONS]
    shellshop --local

OPTIONS:
    --host <ADDR>     Bind address (default: 0.0.0.0)
    --port <PORT>     Listen port (default: 22)
    --key-path <PATH> Host private key file (default: ./host_key)
    --local, -l       Run TUI on this terminal instead of SSH server
    -h, --help        This help
"
                );
                std::process::exit(0);
            }
            "--local" | "-l" => {
                // handled in main before parsing continues
                i += 1;
            }
            "--host" => {
                if let Some(v) = args.get(i + 1) {
                    c.host = v.clone();
                    i += 1;
                }
                i += 1;
            }
            "--port" => {
                if let Some(v) = args.get(i + 1).and_then(|s| s.parse().ok()) {
                    c.port = v;
                    i += 1;
                }
                i += 1;
            }
            "--key-path" => {
                if let Some(v) = args.get(i + 1) {
                    c.key_path = PathBuf::from(v);
                    i += 1;
                }
                i += 1;
            }
            other => {
                eprintln!("Unknown argument: {other}");
                i += 1;
            }
        }
    }
    c
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    if args.iter().any(|a| a == "--local" || a == "-l") {
        return app::App::run_local().map_err(|e| anyhow::anyhow!("{e}"));
    }

    let config = parse_cli(&args);
    let key_path = config.key_path.clone();

    let host_key = host_key::load_or_generate(&key_path)?;
    let ssh_config = Arc::new(ssh_server::server_config(host_key));

    eprintln!(
        "ShellShop SSH listening on {}:{} (host key: {})",
        config.host,
        config.port,
        key_path.display()
    );

    let mut server = ssh_server::ShellShopServer::default();
    server
        .run_on_address(ssh_config, (config.host.as_str(), config.port))
        .await?;
    Ok(())
}

use clap::Parser;
use std::{net::SocketAddr, path::PathBuf, process::Stdio, time::Duration};
use anyhow::Context;
use tokio::{io::AsyncWriteExt, net::TcpStream, process::{Child, Command}};
use texforge_server::{config::ServerConfig, start};

#[derive(Parser)]
#[command(name = "texforge")]
#[command(about = "A local-first LaTeX IDE")]
struct Cli {
    #[arg(default_value = ".")]
    path: PathBuf,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let config = ServerConfig::from_project_root(cli.path)?;
    let api_url = config.url();
    let frontend_url = config.frontend_url.clone();

    println!("TexForge project: {}", config.project_root.display());
    println!("API server: {api_url}");
    println!("Editor: {frontend_url}");

    let server = start(config).await?;
    let mut frontend = start_frontend()?;

    if wait_for_frontend("127.0.0.1:5173").await {
        match webbrowser::open(&frontend_url) {
            Ok(()) => println!("Browser opened: {frontend_url}"),
            Err(_) => eprintln!("Could not open the browser. Open this URL manually: {frontend_url}"),
        }
    } else {
        eprintln!("Frontend did not become available. Open this URL manually: {frontend_url}");
    }

    tokio::select! {
        result = server => result??,
        result = frontend.wait() => {
            result.context("frontend process failed")?;
        }
        _ = tokio::signal::ctrl_c() => {
            frontend.kill().await.context("failed to stop frontend")?;
        }
    }

    Ok(())
}

fn start_frontend() -> anyhow::Result<Child> {
    let web_root = std::env::var_os("TEXFORGE_WEB_ROOT")
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../web"));
    let package_manager = if cfg!(target_os = "windows") { "pnpm.cmd" } else { "pnpm" };

    Command::new(package_manager)
        .args(["dev", "--host", "127.0.0.1"])
        .current_dir(web_root)
        .stdin(Stdio::null())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
        .context("failed to start the frontend with pnpm")
}

async fn wait_for_frontend(address: &str) -> bool {
    let address: SocketAddr = match address.parse() {
        Ok(address) => address,
        Err(_) => return false,
    };

    for _ in 0..50 {
        if let Ok(mut stream) = TcpStream::connect(address).await {
            let _ = stream.write_all(b"GET / HTTP/1.1\r\nHost: localhost\r\nConnection: close\r\n\r\n").await;
            return true;
        }
        tokio::time::sleep(Duration::from_millis(100)).await;
    }

    false
}
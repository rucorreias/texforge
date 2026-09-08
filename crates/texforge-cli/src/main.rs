use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "texforge")]
#[command(about = "A local-first LaTeX IDE")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,

    #[arg(default_value = ".")]
    path: PathBuf,
}

#[derive(Subcommand)]
enum Commands {
    Init,
    Build,
    Watch,
}

fn main() {
    let cli = Cli::parse();

    println!("TexForge");
    println!("Project: {}", cli.path.display());
}
use clap::Parser;
use client::Client;
use server::server::Result;

mod client;
mod components;

#[derive(clap::Parser)]
#[command(version, about = "Real time multi-user chat client", long_about = None)]
struct Cli {
    #[arg(long, short, help = "User identifier in the connection")]
    user: String,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let mut client = Client::build("127.0.0.1:8080", cli.user)?;
    client
        .run(&mut ratatui::init())
        .map_err(|_| ratatui::restore())?;
    ratatui::restore();
    Ok(())
}

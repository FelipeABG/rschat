use clap::Parser;
use client::Client;
use server::server::Result;

mod client;
mod widgets;

#[derive(clap::Parser)]
#[command(version, about = "Real time multi-user chat client", long_about = None)]
struct Cli {
    #[arg(long, short, help = "User identifier in the connection")]
    user: String,

    #[arg(long, short, help = "Server address connect")]
    address: String,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let mut client = Client::build(cli.address, cli.user)?;
    client
        .run(&mut ratatui::init())
        .map_err(|_| ratatui::restore())?;
    ratatui::restore();
    Ok(())
}

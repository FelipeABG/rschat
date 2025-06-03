use clap::Parser;
use server::server::{Result, Server};

#[derive(clap::Parser)]
#[command(version, about = "Real time multi-user chat backend", long_about = None)]
struct Cli {
    #[arg(
        long,
        short,
        help = "Network PORT used by the server", 
        default_value_t = String::from("127.0.0.1:8080")
    )]
    address: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    let mut server = Server::build(cli.address).await?;
    server.run().await?;
    Ok(())
}

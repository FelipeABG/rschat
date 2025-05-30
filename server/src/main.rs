use clap::Parser;
use server::Result;
use server::Server;

pub mod event;
mod macros;
mod server;

#[derive(clap::Parser)]
#[command(version, about = "Real time multi-user chat backend", long_about = None)]
struct Cli {
    #[arg(
        long,
        short,
        help = "Network PORT used by the server", 
        value_parser = clap::value_parser!(u16).range(0..),
        default_value_t = 8080
    )]
    port: u16,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let mut addr = String::from("127.0.0.1:");
    //push the port to the addr
    addr.push_str(&cli.port.to_string());
    let mut server = Server::build(addr)?;
    server.run()?;
    Ok(())
}

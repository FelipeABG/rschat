use server::Result;
use server::Server;

mod macros;
mod server;

const ADDR: &str = "127.0.0.1:8080";

fn main() -> Result<()> {
    let server = Server::build(ADDR)?;

    server.run();

    Ok(())
}

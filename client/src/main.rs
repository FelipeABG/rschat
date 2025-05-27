use client::Client;
use client::Result;

mod client;

fn main() -> Result<()> {
    let mut term = ratatui::init();
    let client = Client::new().run(&mut term);
    ratatui::restore();
    client
}

mod macros;

use colored::Colorize;
use crossterm::{cursor, queue, terminal::Clear};
use std::io::Write;

type Result<T> = std::result::Result<T, ()>;

fn main() -> Result<()> {
    let mut stdout = std::io::stdout();
    let (width, heigth) =
        crossterm::terminal::size().map_err(|err| error!("Failed to get terminal size: {err}"))?;

    queue!(
        stdout,
        Clear(crossterm::terminal::ClearType::All),
        cursor::MoveTo(width / 2, heigth / 2),
    )
    .map_err(|err| error!("Failed to clear screen: {err}"))?;

    stdout.write_all(b"Test").unwrap();
    let _ = stdout.flush();

    loop {}
}

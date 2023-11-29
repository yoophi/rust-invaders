use std::error::Error;
use std::io;
use crossterm::{ExecutableCommand, terminal};
use crossterm::cursor::Hide;
use crossterm::terminal::EnterAlternateScreen;

fn main() -> Result<(), Box<dyn Error>> {
    println!("Hello, world!");

    // terminal
    let mut stdout = io::stdout();
    terminal::enable_raw_mode()?;
    stdout.execute(EnterAlternateScreen)?;
    stdout.execute(Hide)?;

    // cleanup
    terminal::disable_raw_mode()?;

    Ok(())
}

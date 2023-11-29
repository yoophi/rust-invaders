use std::error::Error;
use std::io;
use std::time::Duration;
use crossterm::{event, ExecutableCommand, terminal};
use crossterm::cursor::Hide;
use crossterm::event::{Event, KeyCode};
use crossterm::terminal::EnterAlternateScreen;

fn main() -> Result<(), Box<dyn Error>> {
    println!("Hello, world!");

    // terminal
    let mut stdout = io::stdout();
    terminal::enable_raw_mode()?;
    stdout.execute(EnterAlternateScreen)?;
    stdout.execute(Hide)?;

    // game loop
    'gameloop: loop {
        while event::poll(Duration::default())? {
            if let Event::Key(key_event) = event::read()? {
                match key_event.code {
                    KeyCode::Esc | KeyCode::Char('q') => {
                        break 'gameloop;
                    }
                    _ => {}
                }
            }
        }
    }

    // cleanup
    terminal::disable_raw_mode()?;

    Ok(())
}

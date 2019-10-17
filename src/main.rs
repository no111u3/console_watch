use std::{io, io::Write};

use tui::backend::TermionBackend;
use tui::Terminal;

use termion::{event::Key, input::TermRead, raw::IntoRawMode, screen::AlternateScreen};

use console_watch::app::Events;
use console_watch::watch::App;

fn main() -> Result<(), io::Error> {
    let stdout = io::stdout().into_raw_mode()?;
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let _events: Events<Key> = Events::new(|_tx| || {}, |_tx| || {});

    let app = App::new("Watch");
    let keys = io::stdin().keys();

    app.draw(&mut terminal)?;

    for key in keys.filter_map(Result::ok) {
        use Key::*;

        app.draw(&mut terminal)?;

        match key {
            Char('q') | Esc => break,
            _ => {}
        }
    }
    io::stdout().flush().ok();

    Ok(())
}

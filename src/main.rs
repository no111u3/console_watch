use std::io;

use tui::Terminal;
use tui::backend::TermionBackend;

use termion::raw::IntoRawMode;

fn main() -> Result<(), io::Error> {
    let stdout = io::stdout().into_raw_mode()?;
    let backend = TermionBackend::new(stdout);
    let mut _terminal = Terminal::new(backend)?;
    
    Ok(())
}

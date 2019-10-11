pub mod events;

use std::io;

pub use tui::backend::Backend;
pub use tui::Terminal;

pub use crate::app::events::{Event, Events};

pub trait App {
    fn draw<B>(&mut self, terminal: &mut Terminal<B>) -> Result<(), io::Error>
    where
        B: Backend;

    fn event_hanle<I>(&mut self, event: Event<I>) -> Result<(), io::Error>;

    fn proccess_event<B, I>(
        &mut self,
        terminal: &mut Terminal<B>,
        events: &Events<I>,
    ) -> Result<(), io::Error>
    where
        B: Backend,
        I: std::marker::Send,
    {
        self.draw(terminal)?;

        while let Ok(event) = events.next() {
            self.event_hanle(event)?;
        }

        Ok(())
    }
}

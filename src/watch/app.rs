use std::io;

use tui::backend::Backend;
use tui::style::{Color, Style};
use tui::widgets::{Paragraph, Text, Widget};
use tui::Terminal;

pub struct App<'a> {
    pub title: &'a str,
}

impl<'a> App<'a> {
    pub fn new(title: &'a str) -> App<'a> {
        App { title }
    }

    pub fn draw<B: Backend>(&self, terminal: &mut Terminal<B>) -> Result<(), io::Error> {
        terminal.draw(|mut f| {
            let standard = Style {
                fg: Color::Black,
                bg: Color::Yellow,
                ..Default::default()
            };
            let size = f.size();
            Paragraph::new([Text::Styled(self.title.into(), standard)].iter()).render(&mut f, size);
        })
    }
}

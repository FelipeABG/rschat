use crate::client::Mode;
use ratatui::prelude::Stylize;
use ratatui::{text::Line, widgets::Widget};

pub struct Help<'a> {
    mode: &'a Mode,
}

impl<'a> Help<'a> {
    pub fn new(mode: &'a Mode) -> Self {
        Self { mode }
    }
}

impl<'a> Widget for Help<'a> {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let line = match *self.mode {
            Mode::NormalMode => Line::from(vec![
                "Press ".into(),
                "q".bold().cyan(),
                " to exit, ".into(),
                "a".bold().cyan(),
                " to start editing message.".into(),
            ]),
            Mode::InsertMode => Line::from(vec![
                "Press ".into(),
                "ESC".bold().cyan(),
                " to stop editing, ".into(),
                "ENTER".bold().cyan(),
                " to send message".into(),
            ]),
        };

        line.render(area, buf);
    }
}

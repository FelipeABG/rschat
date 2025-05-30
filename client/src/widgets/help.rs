use crate::client::Mode;
use ratatui::prelude::Stylize;
use ratatui::{text::Line, widgets::Widget};
use std::cell::RefCell;
use std::rc::Rc;

pub struct Help {
    mode: Rc<RefCell<Mode>>,
}

impl Help {
    pub fn new(mode: Rc<RefCell<Mode>>) -> Self {
        Self { mode }
    }
}

impl Widget for Help {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let line = match *self.mode.borrow() {
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

use crate::client::Mode;
use ratatui::{
    crossterm::event::KeyEvent,
    style::Stylize,
    text::Line,
    widgets::{Block, BorderType, Widget},
};
use std::{cell::RefCell, rc::Rc};
use tui_textarea::TextArea;

pub struct Input<'a> {
    mode: Rc<RefCell<Mode>>,
    handler: TextArea<'a>,
}

impl<'a> Input<'a> {
    pub fn new(mode: Rc<RefCell<Mode>>) -> Self {
        Self {
            mode,
            handler: TextArea::default(),
        }
    }

    pub fn register_key(&mut self, key: KeyEvent) {
        self.handler.input(key);
    }

    pub fn get_message(&self) -> Option<String> {
        let msg = self.handler.lines().join("");
        if msg.is_empty() {
            return None;
        }

        Some(msg)
    }

    pub fn clear_input(&mut self) {
        self.handler.delete_line_by_head();
    }
}

impl<'a> Widget for &mut Input<'a> {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let mode = match *self.mode.borrow() {
            Mode::InsertMode => Line::from(" INSERT ").light_green(),
            Mode::NormalMode => Line::from(" NORMAL ").light_blue(),
        };

        let input_block = Block::bordered()
            .title_top(" Input ")
            .title_bottom(mode)
            .border_type(BorderType::Rounded);

        self.handler.set_block(input_block);
        self.handler.render(area, buf);
    }
}

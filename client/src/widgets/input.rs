use crate::client::Mode;
use ratatui::{
    crossterm::event::KeyEvent,
    style::Stylize,
    text::Line,
    widgets::{Block, BorderType, Widget},
};
use tui_textarea::TextArea;

pub struct InputWidget<'a> {
    mode: Mode,
    handler: TextArea<'a>,
}

impl<'a> InputWidget<'a> {
    pub fn new(mode: Mode) -> Self {
        Self {
            mode,
            handler: TextArea::default(),
        }
    }

    pub fn register_key(&mut self, key: KeyEvent) {
        self.handler.input(key);
    }

    pub fn set_mode(&mut self, mode: Mode) {
        self.mode = mode;
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

impl<'a> Widget for &mut InputWidget<'a> {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let mode = match self.mode {
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

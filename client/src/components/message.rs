use ratatui::{
    text::Line,
    widgets::{Block, BorderType, Paragraph},
};

pub struct Message {
    value: String,
}

impl Message {
    pub fn new(value: String) -> Self {
        Self { value }
    }

    pub fn as_paragraph(&self) -> Paragraph {
        let line = Line::from(self.value.clone());

        Paragraph::new(line).block(Block::bordered().border_type(BorderType::Rounded))
    }
}

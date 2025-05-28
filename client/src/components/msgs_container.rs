use super::message::Message;
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::widgets::{Block, BorderType, Paragraph, Widget};
use std::sync::{Arc, Mutex};

pub struct MsgContainer {
    messages: Arc<Mutex<Vec<Message>>>,
}

impl MsgContainer {
    pub fn new(messages: Arc<Mutex<Vec<Message>>>) -> Self {
        Self { messages }
    }
}

impl Widget for MsgContainer {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        if let Ok(lock) = self.messages.lock() {
            let block = Block::bordered().border_type(BorderType::Rounded);

            let paragraphs: Vec<Paragraph> = lock
                .iter()
                .map(|msg| msg.as_line())
                .map(|line| {
                    Paragraph::new(line).block(Block::bordered().border_type(BorderType::Rounded))
                })
                .collect();
            let constraints: Vec<Constraint> = (0..paragraphs.len())
                .map(|_| Constraint::Percentage(7))
                .collect();
            let layout = Layout::new(Direction::Vertical, constraints).split(block.inner(area));

            for (idx, msg) in paragraphs.iter().enumerate() {
                msg.render(layout[idx], buf);
            }

            block.render(area, buf);
        }
    }
}

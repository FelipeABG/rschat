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

            let msgs: Vec<Paragraph> = lock.iter().map(|msg| msg.as_paragraph()).collect();

            let outer_layout = Layout::new(
                Direction::Horizontal,
                [Constraint::Percentage(50), Constraint::Percentage(50)],
            )
            .split(block.inner(area));

            let constraints: Vec<Constraint> =
                (0..msgs.len()).map(|_| Constraint::Length(3)).collect();
            let inner_layout = Layout::new(Direction::Vertical, constraints).split(outer_layout[0]);

            for (idx, msg) in msgs.iter().enumerate() {
                msg.render(inner_layout[idx], buf);
            }

            block.render(area, buf);
        }
    }
}

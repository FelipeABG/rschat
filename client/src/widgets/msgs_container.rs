use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::widgets::{Block, BorderType, Widget};
use server::event::Message;
use std::cell::RefCell;
use std::rc::Rc;

use super::message::MessageWidget;

pub struct MsgContainer<'a> {
    messages: Rc<RefCell<Vec<Message>>>,
    user: &'a str,
}

impl<'a> MsgContainer<'a> {
    pub fn new(messages: Rc<RefCell<Vec<Message>>>, user: &'a str) -> Self {
        Self { messages, user }
    }
}

impl<'a> Widget for MsgContainer<'a> {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let msgs = self.messages.borrow();
        let block = Block::bordered().border_type(BorderType::Rounded);
        let inner_area = block.inner(area);

        let inner_layout = Layout::new(
            Direction::Horizontal,
            [Constraint::Percentage(50), Constraint::Percentage(50)],
        )
        .split(inner_area);

        let constraints = vec![Constraint::Length(6); 5];
        let left_layout = Layout::new(Direction::Vertical, &constraints).split(inner_layout[0]);
        let right_layout = Layout::new(Direction::Vertical, &constraints).split(inner_layout[1]);

        for (idx, msg) in msgs.iter().rev().take(5).rev().enumerate() {
            let widget = MessageWidget(msg, &msg.author);
            let area = if msg.author == self.user {
                right_layout[idx]
            } else {
                left_layout[idx]
            };
            widget.render(area, buf);
        }

        block.render(area, buf);
    }
}

use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::widgets::{Block, BorderType, Widget};
use server::event::Message;
use std::cell::RefCell;
use std::rc::Rc;

use super::message::MessageWidget;

pub struct MsgContainer {
    messages: Rc<RefCell<Vec<Message>>>,
}

impl MsgContainer {
    pub fn new(messages: Rc<RefCell<Vec<Message>>>) -> Self {
        Self { messages }
    }
}

impl Widget for MsgContainer {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let msgs = Rc::clone(&self.messages);
        let block = Block::bordered().border_type(BorderType::Rounded);

        let outer_layout = Layout::new(
            Direction::Horizontal,
            [Constraint::Percentage(50), Constraint::Percentage(50)],
        )
        .split(block.inner(area));

        let constraints: Vec<Constraint> = (0..msgs.borrow().len())
            .map(|_| Constraint::Length(6))
            .collect();
        let inner_layout = Layout::new(Direction::Vertical, constraints).split(outer_layout[0]);

        for (idx, msg) in msgs
            .borrow()
            .iter()
            .map(|msg| MessageWidget(msg))
            .enumerate()
        {
            msg.render(inner_layout[idx], buf);
        }

        block.render(area, buf);
    }
}

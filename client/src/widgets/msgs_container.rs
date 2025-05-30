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

        let inner_layout_left =
            Layout::new(Direction::Vertical, &constraints).split(outer_layout[0]);

        let inner_layout_right =
            Layout::new(Direction::Vertical, &constraints).split(outer_layout[1]);

        for (idx, msg) in msgs
            .borrow()
            .iter()
            .map(|msg| MessageWidget(msg, &msg.author))
            .enumerate()
        {
            if msg.1 == self.user {
                msg.render(inner_layout_right[idx], buf);
            } else {
                msg.render(inner_layout_left[idx], buf);
            }
        }

        block.render(area, buf);
    }
}

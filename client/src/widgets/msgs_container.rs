use super::message::MessageWidget;
use crate::session::Session;
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::widgets::{Block, BorderType, Widget};

pub struct MsgContainer<'a> {
    session: &'a Session,
}

impl<'a> MsgContainer<'a> {
    pub fn new(session: &'a Session) -> Self {
        Self { session }
    }
}

impl<'a> Widget for MsgContainer<'a> {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
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

        for (idx, msg) in self.session.last_n_msgs(5) {
            let widget =
                MessageWidget::from_msg_with_color(msg, self.session.get_user_color(&msg.author));
            let area = if msg.author == *self.session.user() {
                right_layout[idx]
            } else {
                left_layout[idx]
            };
            widget.render(area, buf);
        }

        block.render(area, buf);
    }
}

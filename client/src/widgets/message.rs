use std::time::SystemTime;

use chrono::{DateTime, Local};
use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::Line,
    widgets::{Block, BorderType, Paragraph, Widget},
};
use server::event::Message;

pub struct MessageWidget<'a> {
    author: &'a String,
    date: SystemTime,
    content: &'a String,
    color: Color,
}

impl<'a> MessageWidget<'a> {
    pub fn from_msg_with_color(msg: &'a Message, color: Color) -> Self {
        Self {
            author: &msg.author,
            date: msg.date,
            content: &msg.body,
            color,
        }
    }
}

impl<'a> Widget for MessageWidget<'a> {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let datetime: DateTime<Local> = DateTime::from(self.date);
        let fmt_date = datetime.format("%d/%m %H:%M").to_string();

        let block = Block::bordered().border_type(BorderType::Rounded);

        let outer_layout = Layout::new(
            Direction::Vertical,
            [Constraint::Percentage(60), Constraint::Percentage(30)],
        )
        .split(area);

        let inner_area = block.inner(outer_layout[0]);
        let inner_layout = Layout::new(
            Direction::Vertical,
            [Constraint::Percentage(30), Constraint::Percentage(60)],
        )
        .split(inner_area);

        block.render(outer_layout[0], buf);
        Paragraph::new(
            Line::from(self.author.clone()).style(
                Style::new()
                    .bg(self.color)
                    .fg(self.color)
                    .add_modifier(Modifier::BOLD),
            ),
        )
        .render(inner_layout[0], buf);
        Paragraph::new(Line::from(self.content.clone())).render(inner_layout[1], buf);
        Paragraph::new(
            Line::from(fmt_date).style(
                Style::new()
                    .bg(self.color)
                    .fg(self.color)
                    .add_modifier(Modifier::BOLD),
            ),
        )
        .right_aligned()
        .render(outer_layout[1], buf);
    }
}

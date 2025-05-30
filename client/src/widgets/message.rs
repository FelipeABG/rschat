use chrono::{DateTime, Local};
use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::Stylize,
    text::Line,
    widgets::{Block, BorderType, Paragraph, Widget},
};
use server::event::Message;

pub struct MessageWidget<'a>(pub &'a Message, pub &'a str);

impl<'a> Widget for MessageWidget<'a> {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let datetime: DateTime<Local> = DateTime::from(self.0.date);
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
        Paragraph::new(Line::from(self.0.author.clone()).cyan().bold().underlined())
            .render(inner_layout[0], buf);
        Paragraph::new(Line::from(self.0.body.clone())).render(inner_layout[1], buf);
        Paragraph::new(Line::from(fmt_date.cyan().bold()))
            .right_aligned()
            .render(outer_layout[1], buf);
    }
}

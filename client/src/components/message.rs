use chrono::{DateTime, Local};
use ratatui::{
    text::Line,
    widgets::{Block, BorderType, Paragraph, Widget},
};
use server::event::Message;

pub struct MessageWidget<'a>(pub &'a Message);

impl<'a> Widget for MessageWidget<'a> {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let datetime: DateTime<Local> = DateTime::from(self.0.date);
        let fmt_date = datetime.format("%d/%m %H:%M").to_string();
        let content = format!("[{} - {}] {} ", self.0.author, fmt_date, self.0.body);
        let line = Line::from(content);

        Paragraph::new(line)
            .block(Block::bordered().border_type(BorderType::Rounded))
            .render(area, buf);
    }
}

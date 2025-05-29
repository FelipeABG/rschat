use ratatui::layout::Alignment;
use ratatui::prelude::Stylize;
use ratatui::{
    layout::Margin,
    text::Line,
    widgets::{Block, Paragraph, Widget},
};

pub struct Connecting {
    dot_state: usize,
    message: String,
}

impl Connecting {
    pub fn new() -> Self {
        Self {
            dot_state: 0,
            message: String::from("Connecting to server"),
        }
    }
}

impl Widget for &mut Connecting {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        match self.dot_state {
            0 | 1 | 2 => {
                self.message.push('.');
                self.dot_state += 1
            }
            _ => {
                self.message = self.message.chars().filter(|c| *c != '.').collect();
                self.dot_state = 0
            }
        }

        let margin_area = area.inner(Margin::new(70, 17));
        let btitle = Line::from(vec![" <Ctrl>".red(), "c".red(), " To quit ".into()]);
        Paragraph::new(Line::from(self.message.clone()))
            .block(
                Block::new()
                    .title(" Rschat Connection ".cyan())
                    .title_bottom(btitle)
                    .title_alignment(Alignment::Center),
            )
            .centered()
            .render(margin_area, buf);
    }
}

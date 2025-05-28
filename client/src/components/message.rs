use ratatui::text::Line;

pub struct Message {
    value: String,
}

impl Message {
    pub fn new(value: String) -> Self {
        Self { value }
    }

    pub fn as_line(&self) -> Line {
        Line::from(self.value.clone())
    }
}

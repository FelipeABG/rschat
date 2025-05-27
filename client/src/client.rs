use ratatui::prelude::Stylize;
use ratatui::{
    Frame,
    crossterm::event::{self, Event, KeyCode},
    layout::{Constraint, Layout},
    text::Line,
    widgets::{Block, BorderType, Padding},
};
use tui_textarea::TextArea;

pub type Result<T> = std::io::Result<T>;

pub struct Client<'a> {
    input: TextArea<'a>,
    mode: Mode,
}

#[derive(PartialEq)]
enum Mode {
    InsertMode,
    NormalMode,
}

impl<'a> Client<'a> {
    pub fn new() -> Self {
        Self {
            input: TextArea::default(),
            mode: Mode::NormalMode,
        }
    }

    pub fn run(&mut self, term: &mut ratatui::DefaultTerminal) -> Result<()> {
        loop {
            term.draw(|frame| self.draw(frame))?;
            if self.handle_events()? {
                break Ok(());
            }
        }
    }

    fn draw(&mut self, frame: &mut Frame) {
        let title = Line::from(" Rschat Client ").cyan().centered();
        let info = Line::from(vec![
            "<Q>".red().bold(),
            "Quit ".into(),
            "<ESC>".red().bold(),
            "Normal mode ".into(),
            "<A>".red().bold(),
            "Insert mode ".into(),
        ])
        .centered();

        let mode = match self.mode {
            Mode::InsertMode => Line::from(" INSERT ").light_green(),
            Mode::NormalMode => Line::from(" NORMAL ").light_blue(),
        };

        let main_block = Block::bordered()
            .title_top(title)
            .title_bottom(info)
            .padding(Padding::uniform(1))
            .border_type(BorderType::Rounded);

        let input_block = Block::bordered()
            .title_top(" Input ")
            .title_bottom(mode)
            .border_type(BorderType::Rounded);

        let message_block = Block::new();

        let layout = Layout::new(
            ratatui::layout::Direction::Vertical,
            [Constraint::Length(97), Constraint::Length(3)],
        )
        .split(main_block.inner(frame.area()));

        self.input.set_block(input_block);

        frame.render_widget(main_block, frame.area());
        frame.render_widget(message_block, layout[0]);
        frame.render_widget(&self.input, layout[1]);
    }

    fn handle_events(&mut self) -> Result<bool> {
        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Esc => self.mode = Mode::NormalMode,
                KeyCode::Char('a') if self.mode != Mode::InsertMode => {
                    self.mode = Mode::InsertMode;
                }
                KeyCode::Char('q') if self.mode == Mode::NormalMode => return Ok(true),
                _ => {
                    if let Mode::InsertMode = self.mode {
                        self.input.input(key);
                    }
                }
            }
        }
        Ok(false)
    }
}

use std::cell::RefCell;
use std::rc::Rc;

use crate::components::help::Help;
use crate::components::input::Input;
use ratatui::prelude::Stylize;
use ratatui::{
    Frame,
    crossterm::event::{self, Event, KeyCode},
    layout::{Constraint, Layout},
    text::Line,
    widgets::{Block, BorderType, Padding},
};

pub type Result<T> = std::io::Result<T>;

pub struct Client<'a> {
    // Input handler
    input: Input<'a>,
    // Input mode
    mode: Rc<RefCell<Mode>>,
}

#[derive(PartialEq)]
pub enum Mode {
    InsertMode,
    NormalMode,
}

impl<'a> Client<'a> {
    pub fn new() -> Self {
        let mode = Rc::new(RefCell::new(Mode::InsertMode));
        Self {
            mode: Rc::clone(&mode),
            input: Input::new(Rc::clone(&mode)),
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

        let main_block = Block::bordered()
            .title_top(title)
            .padding(Padding::uniform(1))
            .border_type(BorderType::Rounded);

        let message_block = Block::new();

        let layout = Layout::new(
            ratatui::layout::Direction::Vertical,
            [
                Constraint::Length(97),
                Constraint::Length(1),
                Constraint::Length(3),
            ],
        )
        .split(main_block.inner(frame.area()));

        frame.render_widget(main_block, frame.area());
        frame.render_widget(message_block, layout[0]);
        frame.render_widget(Help::new(Rc::clone(&self.mode)), layout[1]);
        frame.render_widget(&mut self.input, layout[2]);
    }

    fn handle_events(&mut self) -> Result<bool> {
        let mut mode = self.mode.borrow_mut();
        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Esc => *mode = Mode::NormalMode,
                KeyCode::Char('a') if *mode != Mode::InsertMode => {
                    *mode = Mode::InsertMode;
                }
                KeyCode::Char('q') if *mode == Mode::NormalMode => return Ok(true),
                _ => {
                    if let Mode::InsertMode = *mode {
                        self.input.register_key(key);
                    }
                }
            }
        }
        Ok(false)
    }
}

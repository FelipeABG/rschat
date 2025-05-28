use crate::components::help::Help;
use crate::components::input::Input;
use crate::components::message::Message;
use crate::components::msgs_container::MsgContainer;
use ratatui::prelude::Stylize;
use ratatui::{
    Frame,
    crossterm::event::{self, Event, KeyCode},
    layout::{Constraint, Layout},
    text::Line,
};
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::{Arc, Mutex};

pub type Result<T> = std::io::Result<T>;

pub struct Client<'a> {
    // Input handler
    input: Input<'a>,
    // Input mode
    mode: Rc<RefCell<Mode>>,
    // Users messages
    messages: Arc<Mutex<Vec<Message>>>,
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
            messages: Arc::new(Mutex::new(Vec::new())),
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

        let layout = Layout::new(
            ratatui::layout::Direction::Vertical,
            [
                Constraint::Length(1),  // Title
                Constraint::Length(95), // Messages container
                Constraint::Length(1),  // Info
                Constraint::Length(3),  // Input
            ],
        )
        .split(frame.area());

        frame.render_widget(title, layout[0]);
        frame.render_widget(MsgContainer::new(Arc::clone(&self.messages)), layout[1]);
        frame.render_widget(Help::new(Rc::clone(&self.mode)), layout[2]);
        frame.render_widget(&mut self.input, layout[3]);
    }

    fn handle_events(&mut self) -> Result<bool> {
        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Esc => self.switch_mode(),
                KeyCode::Enter => self.update_messages(),
                KeyCode::Char('a') if *self.mode.borrow() == Mode::NormalMode => self.switch_mode(),
                KeyCode::Char('q') if *self.mode.borrow() == Mode::NormalMode => return Ok(true),
                _ => self.input.register_key(key),
            }
        }
        Ok(false)
    }

    fn switch_mode(&self) {
        let mut mode = self.mode.borrow_mut();
        match *mode {
            Mode::InsertMode => *mode = Mode::NormalMode,
            Mode::NormalMode => *mode = Mode::InsertMode,
        }
    }

    fn update_messages(&mut self) {
        let msg_string = self.input.get_message();
        if let Some(msg_str) = msg_string {
            let msg = Message::new(msg_str);
            if let Ok(mut lock) = self.messages.lock() {
                lock.push(msg);
            }
            self.input.clear_input();
        }
    }
}

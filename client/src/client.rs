use crate::components::connecting::Connecting;
use crate::components::help::Help;
use crate::components::input::Input;
use crate::components::message::Message;
use crate::components::msgs_container::MsgContainer;
use ratatui::crossterm::event::KeyModifiers;
use ratatui::layout::Margin;
use ratatui::prelude::Stylize;
use ratatui::{
    Frame,
    crossterm::event::{self, Event, KeyCode},
    layout::{Constraint, Layout},
    text::Line,
};
use std::cell::RefCell;
use std::io::Read;
use std::net::TcpStream;
use std::rc::Rc;
use std::str::from_utf8;
use std::sync::{Arc, Mutex};
use std::time::Duration;

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
        // server connection loop
        let mut stream = TcpStream::connect("127.0.0.1:8080");
        let mut conn_component = Connecting::new();
        while let Err(_) = stream {
            term.draw(|frame| frame.render_widget(&mut conn_component, frame.area()))?;
            if self.handle_connecting_events()? {
                return Ok(());
            }
            stream = TcpStream::connect("127.0.0.1:8080");
            std::thread::sleep(Duration::from_millis(500));
        }
        drop(conn_component);

        // thread responsible to handle incoming messages
        let msgs = Arc::clone(&self.messages);
        std::thread::spawn(move || Self::handle_messages(msgs, stream.unwrap()));

        // main client loop
        loop {
            term.draw(|frame| self.draw(frame))?;
            if self.handle_events()? {
                break Ok(());
            }
            std::thread::sleep(Duration::from_millis(16));
        }
    }

    fn handle_messages(msgs: Arc<Mutex<Vec<Message>>>, mut stream: TcpStream) {
        let mut buffer = [0; 1024];
        // read operations are blocking by default,
        // which causes the mutex to be blocked in this thread.
        stream.set_nonblocking(true).unwrap();
        loop {
            if let Ok(mut lock) = msgs.lock() {
                if let Ok(n) = stream.read(&mut buffer) {
                    if let Ok(msg_str) = from_utf8(&buffer[0..n]) {
                        lock.push(Message::new(msg_str.to_string()));
                    }
                }
            }
            std::thread::sleep(Duration::from_millis(200));
        }
    }

    fn draw(&mut self, frame: &mut Frame) {
        let title = Line::from(" Rschat Client ").cyan().centered();

        let margin_frame = frame.area().inner(Margin::new(20, 3));
        let layout = Layout::new(
            ratatui::layout::Direction::Vertical,
            [
                Constraint::Length(1),  // Title
                Constraint::Length(95), // Messages container
                Constraint::Length(1),  // Info
                Constraint::Length(3),  // Input
            ],
        )
        .split(margin_frame);

        frame.render_widget(title, layout[0]);
        frame.render_widget(MsgContainer::new(Arc::clone(&self.messages)), layout[1]);
        frame.render_widget(Help::new(Rc::clone(&self.mode)), layout[2]);
        frame.render_widget(&mut self.input, layout[3]);
    }

    fn handle_events(&mut self) -> Result<bool> {
        if event::poll(Duration::ZERO)? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Esc => self.switch_mode(),
                    KeyCode::Enter => self.send_msg(),
                    KeyCode::Char('a') if *self.mode.borrow() == Mode::NormalMode => {
                        self.switch_mode()
                    }
                    KeyCode::Char('q') if *self.mode.borrow() == Mode::NormalMode => {
                        return Ok(true);
                    }
                    _ => self.input.register_key(key),
                }
            }
        }
        Ok(false)
    }

    fn handle_connecting_events(&mut self) -> Result<bool> {
        if event::poll(Duration::ZERO)? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                        return Ok(true);
                    }
                    _ => {}
                }
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

    fn send_msg(&mut self) {
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

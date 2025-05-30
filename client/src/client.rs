use crate::widgets::help::Help;
use crate::widgets::input::Input;
use crate::widgets::msgs_container::MsgContainer;
use ratatui::layout::Margin;
use ratatui::prelude::Stylize;
use ratatui::{
    Frame,
    crossterm::event::{self, Event, KeyCode},
    layout::{Constraint, Layout},
    text::Line,
};
use server::error;
use server::event::Message;
use server::server::Result;
use std::cell::RefCell;
use std::fmt::Display;
use std::io::{ErrorKind, Read, Write};
use std::net::{TcpStream, ToSocketAddrs};
use std::rc::Rc;
use std::str::from_utf8;
use std::sync::mpsc::{Sender, TryRecvError, channel};
use std::time::{Duration, SystemTime};

pub struct Client<'a> {
    // Input handler
    input: Input<'a>,
    // Input mode
    mode: Rc<RefCell<Mode>>,
    // User name
    user_name: String,
    // Users messages
    messages: Rc<RefCell<Vec<Message>>>,
    // server socket
    stream: TcpStream,
}

#[derive(PartialEq)]
pub enum Mode {
    InsertMode,
    NormalMode,
}

impl<'a> Client<'a> {
    pub fn build<A: ToSocketAddrs + Display>(addr: A, user_name: String) -> Result<Self> {
        let mode = Rc::new(RefCell::new(Mode::InsertMode));
        let stream = TcpStream::connect(&addr)
            .map_err(|err| error!("Failed to connect to server {addr}: {err}"))?;
        Ok(Self {
            stream,
            user_name,
            mode: Rc::clone(&mode),
            input: Input::new(Rc::clone(&mode)),
            messages: Rc::new(RefCell::new(Vec::new())),
        })
    }

    pub fn run(&mut self, term: &mut ratatui::DefaultTerminal) -> Result<()> {
        // thread responsible to handle incoming messages
        let (sender, receiver) = channel();
        let stream = self.stream.try_clone().unwrap();
        std::thread::spawn(move || Self::incoming_messages(sender, stream));

        // main client loop
        loop {
            term.draw(|frame| self.draw(frame))
                .map_err(|err| error!("Failed to draw frame to terminal: {err}"))?;
            if self.handle_events()? {
                break Ok(());
            }
            match receiver.try_recv() {
                Ok(msg) => self.messages.borrow_mut().push(msg),
                Err(e) => {
                    if let TryRecvError::Disconnected = e {
                        break Ok(());
                    }
                }
            }
        }
    }

    fn incoming_messages(msgs: Sender<Message>, mut stream: TcpStream) {
        let mut buffer = [0; 1024];
        stream.set_nonblocking(true).unwrap();
        loop {
            match stream.read(&mut buffer) {
                Ok(n) => {
                    if n == 0 {
                        break;
                    }

                    if let Ok(msg_str) = from_utf8(&buffer[0..n]) {
                        if let Ok(msg) = serde_json::from_str(msg_str) {
                            msgs.send(msg).unwrap();
                        }
                    }
                }
                Err(e) if e.kind() == ErrorKind::WouldBlock => {
                    std::thread::sleep(Duration::from_millis(50));
                }
                Err(_) => break,
            }
        }
    }

    fn draw(&mut self, frame: &mut Frame) {
        let title = Line::from(" Rschat Client ").cyan().centered();

        let margin_frame = frame.area().inner(Margin::new(20, 1));
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
        frame.render_widget(
            MsgContainer::new(Rc::clone(&self.messages), &self.user_name),
            layout[1],
        );
        frame.render_widget(Help::new(Rc::clone(&self.mode)), layout[2]);
        frame.render_widget(&mut self.input, layout[3]);
    }

    fn handle_events(&mut self) -> Result<bool> {
        if event::poll(Duration::ZERO).map_err(|err| error!("Failed to poll event: {err}"))? {
            // server connection loop
            if let Event::Key(key) =
                event::read().map_err(|err| error!("Failed to read event from terminal: {err}"))?
            {
                match key.code {
                    KeyCode::Esc => self.switch_mode(),
                    KeyCode::Enter => self.send_msg()?,
                    KeyCode::Char('a') if *self.mode.borrow() == Mode::NormalMode => {
                        self.switch_mode()
                    }
                    KeyCode::Char('q') if *self.mode.borrow() == Mode::NormalMode => {
                        return Ok(true);
                    }
                    _ if *self.mode.borrow() == Mode::InsertMode => self.input.register_key(key),
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

    fn send_msg(&mut self) -> Result<()> {
        if let Some(msg) = self.input.get_message() {
            let msg = Message::new(msg, SystemTime::now(), self.user_name.clone());
            let encoded =
                serde_json::to_string(&msg).map_err(|err| error!("Failed to encode msg: {err}"))?;
            let mut stream = self
                .stream
                .try_clone()
                .map_err(|err| error!("Failed to copy stream: {err}"))?;

            stream
                .write_all(&encoded.as_bytes().to_vec())
                .map_err(|err| error!("Failed to write to socket: {err}"))?;
            self.input.clear_input();
        }
        Ok(())
    }
}

use crate::session::Session;
use crate::widgets::help::HelpWidget;
use crate::widgets::input::InputWidget;
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
use std::fmt::Display;
use std::io::{ErrorKind, Read, Write};
use std::net::{TcpStream, ToSocketAddrs};
use std::str::from_utf8;
use std::sync::mpsc::{Sender, TryRecvError, channel};
use std::time::{Duration, SystemTime};

pub struct Client<'a> {
    // InputWidget handler
    input: InputWidget<'a>,
    // client mode
    mode: Mode,
    // User name
    session: Session,
}

#[derive(PartialEq, Clone)]
pub enum Mode {
    InsertMode,
    NormalMode,
}

impl<'a> Client<'a> {
    pub fn build<A: ToSocketAddrs + Display>(addr: A, user_name: String) -> Result<Self> {
        let stream = TcpStream::connect(&addr)
            .map_err(|err| error!("Failed to connect to server {addr}: {err}"))?;
        Ok(Self {
            mode: Mode::InsertMode,
            input: InputWidget::new(Mode::InsertMode),
            session: Session::new(stream, user_name),
        })
    }

    pub fn run(&mut self, term: &mut ratatui::DefaultTerminal) -> Result<()> {
        // thread responsible to handle incoming messages
        let (sender, receiver) = channel();
        let stream = self.session.clone_stream()?;
        std::thread::spawn(move || Self::incoming_messages(sender, stream));

        // main client loop
        loop {
            term.draw(|frame| self.draw(frame))
                .map_err(|err| error!("Failed to draw frame to terminal: {err}"))?;
            if self.handle_events()? {
                break Ok(());
            }
            match receiver.try_recv() {
                Ok(msg) => {
                    self.session.assign_user_color(msg.author.clone());
                    self.session.new_message(msg);
                }
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
                Constraint::Length(3),  // InputWidget
            ],
        )
        .split(margin_frame);

        frame.render_widget(title, layout[0]);
        frame.render_widget(MsgContainer::new(&self.session), layout[1]);
        frame.render_widget(HelpWidget::new(&self.mode), layout[2]);
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
                    KeyCode::Char('a') if self.mode == Mode::NormalMode => self.switch_mode(),
                    KeyCode::Char('q') if self.mode == Mode::NormalMode => {
                        return Ok(true);
                    }
                    _ if self.mode == Mode::InsertMode => self.input.register_key(key),
                    _ => {}
                }
            }
        }
        Ok(false)
    }

    fn switch_mode(&mut self) {
        match self.mode {
            Mode::InsertMode => {
                self.mode = Mode::NormalMode;
                self.input.set_mode(Mode::NormalMode);
            }
            Mode::NormalMode => {
                self.mode = Mode::InsertMode;
                self.input.set_mode(Mode::InsertMode);
            }
        }
    }

    fn send_msg(&mut self) -> Result<()> {
        if let Some(msg) = self.input.get_message() {
            let msg = Message::new(msg, SystemTime::now(), self.session.user().clone());
            let encoded =
                serde_json::to_string(&msg).map_err(|err| error!("Failed to encode msg: {err}"))?;
            let mut stream = self.session.clone_stream()?;

            stream
                .write_all(&encoded.as_bytes().to_vec())
                .map_err(|err| error!("Failed to write to socket: {err}"))?;
            self.input.clear_input();
        }
        Ok(())
    }
}

use rand::random_range;
use ratatui::style::Color;
use server::server::Result;
use server::{error, event::Message};
use std::{collections::HashMap, net::TcpStream};

pub struct Session {
    // server socket
    stream: TcpStream,
    // color of the user messages
    user_colors: HashMap<String, Color>,
    //user identifier
    user_name: String,
    // Users messages
    messages: Vec<Message>,
}

static COLORS: [Color; 14] = [
    Color::Red,
    Color::Green,
    Color::Yellow,
    Color::Blue,
    Color::Magenta,
    Color::Gray,
    Color::DarkGray,
    Color::LightRed,
    Color::LightGreen,
    Color::LightYellow,
    Color::LightBlue,
    Color::LightMagenta,
    Color::LightCyan,
    Color::White,
];

impl Session {
    pub fn new(stream: TcpStream, user_name: String) -> Self {
        let mut user_colors = HashMap::new();
        // the client user is always cyan
        user_colors.insert(user_name.clone(), Color::Cyan);
        Self {
            stream,
            user_name,
            messages: Vec::new(),
            user_colors,
        }
    }

    pub fn assign_user_color(&mut self, user_name: String) {
        if !self.user_colors.contains_key(&user_name) {
            self.user_colors
                .insert(user_name, COLORS[random_range(0..15)]);
        }
    }

    pub fn new_message(&mut self, msg: Message) {
        self.messages.push(msg);
    }

    pub fn clone_stream(&self) -> Result<TcpStream> {
        self.stream
            .try_clone()
            .map_err(|err| error!("Failed to replicate session stream: {err}"))
    }

    pub fn last_n_msgs(&self, n: usize) -> Vec<(usize, &Message)> {
        self.messages
            .iter()
            .rev()
            .take(n)
            .rev()
            .enumerate()
            .collect()
    }

    pub fn user(&self) -> &String {
        &self.user_name
    }

    pub fn get_user_color(&self, user: &String) -> Color {
        *self.user_colors.get(user).unwrap()
    }
}

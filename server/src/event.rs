use serde::{Deserialize, Serialize};

use crate::server::Connection;
use std::time::SystemTime;

pub enum ServerEvent {
    ClientConnected(Connection),
    ClientDisconnected(Connection),
    NewMessage(Connection, Message),
}

#[derive(Serialize, Deserialize)]
pub struct Message {
    pub body: String,
    pub date: SystemTime,
    pub author: String,
}

impl Message {
    pub fn new(body: String, date: SystemTime, author: String) -> Self {
        Self { body, date, author }
    }
}

use crate::error;
use colored::Colorize;
use std::net::{TcpListener, ToSocketAddrs};

pub type Result<T> = std::result::Result<T, ()>;

pub struct Server {
    listener: TcpListener,
}

impl Server {
    /// Tries to create a new instance of the server.
    ///
    /// # Arguments
    /// - `addr`: A type that can be converted into a socket address, such as a string like
    /// `"127.0.0.1:8080"` or a tuple like `("0.0.0.0", 8000)`
    ///
    /// # Returns
    /// - `Ok(Server)`: If the listener successfuly binds to the address.
    /// - `Err(())`: If the bind is unsuccessful, with a message logged.
    pub fn build<A: ToSocketAddrs>(addr: A) -> Result<Self> {
        TcpListener::bind(addr)
            .map(|listener| Self { listener })
            .map_err(|err| error!("Could not bind to given address: {err}"))
    }

    pub fn run(&mut self) {}

    fn server(&mut self) {
        todo!()
    }

    fn client(&mut self) {
        todo!()
    }
}

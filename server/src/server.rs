use crate::{error, info};
use colored::Colorize;
use std::{
    collections::HashMap,
    fmt::Display,
    io::{Read, Write},
    net::{TcpListener, TcpStream, ToSocketAddrs},
    sync::{
        Arc,
        mpsc::{Receiver, Sender},
    },
};

pub type Result<T> = std::result::Result<T, ()>;
type Connection = Arc<TcpStream>;

pub struct Server {
    listener: TcpListener,
}

enum Message {
    ClientConnected(Connection),
    ClientDisconnected(Connection),
    NewMessage { author: Connection, bytes: Vec<u8> },
}

impl Server {
    /// Tries to create a new instance of the server.
    ///
    /// # Arguments
    /// - `addr`: A type that can be converted into a socket address, such as a string like
    /// `"127.0.0.1:8080"` or a tuple like `("0.0.0.0", 8000)`
    ///
    /// # Returns
    /// - `Ok(Server)`: If the listener successfully binds to the address.
    /// - `Err(())`: If the bind is unsuccessful, with a message logged.
    pub fn build<A: ToSocketAddrs + Display>(addr: A) -> Result<Self> {
        TcpListener::bind(&addr)
            .map(|listener| Self { listener })
            .map_err(|err| error!("Could not bind server to {addr}: {err}"))
    }

    /// Starts the server, accepting incoming client connections and
    /// spawning a thread for each client.
    ///
    /// # Returns
    /// - `Ok(())`: If the server runs without fatal errors.
    /// - `Err(())`: If an error occurs during initialization or runtime.
    pub fn run(&mut self) -> Result<()> {
        let (sender, receiver) = std::sync::mpsc::channel();

        std::thread::spawn(|| Self::server(receiver));

        let port = self
            .listener
            .local_addr()
            .map_err(|err| error!("Failed to get listener address: {err}"))?;

        info!("Listening to connections on port {port}");
        for stream in self.listener.incoming() {
            match stream {
                Ok(stream) => {
                    let sender = sender.clone();
                    let stream = Arc::new(stream);
                    std::thread::spawn(|| Self::client(sender, stream));
                }
                Err(err) => error!("Failed to connect to client: {err}"),
            }
        }

        Ok(())
    }

    /// Main server loop that processes messages from connected clients.
    ///
    /// # Arguments
    /// - `messages`: Receiver end of the channel used to receive messages from clients.
    ///
    /// Handles new connections, disconnections, and broadcasting messages
    /// to all connected clients except the sender.
    fn server(messages: Receiver<Message>) {
        let mut clients = HashMap::new();
        loop {
            match messages.recv() {
                Ok(msg) => match msg {
                    Message::ClientConnected(stream) => {
                        let client_addr = stream.peer_addr().unwrap();
                        info!("New client connected: {client_addr}");
                        clients.insert(client_addr, Arc::clone(&stream));
                    }
                    Message::ClientDisconnected(stream) => {
                        let client_addr = stream.peer_addr().unwrap();
                        info!("Client disconnected: {client_addr}");
                        clients.remove(&client_addr);
                    }
                    Message::NewMessage { author, bytes } => {
                        let author_addr = author.peer_addr().unwrap();
                        info!("Message received from {author_addr}: {} bytes", bytes.len());
                        for (client, stream) in &clients {
                            if *client != author_addr {
                                let _ = stream.as_ref().write_all(&bytes).map_err(|err| {
                                    error!("Failed to send message from {author_addr} to {client}: {err}");
                                });
                            }
                        }
                    }
                },
                Err(err) => eprintln!("Failed to receive message from client: {err}"),
            }
        }
    }

    /// Handles communication with a single client.
    ///
    /// # Arguments
    /// - `messages`: Sender used to communicate with the server loop.
    /// - `stream`: An `Arc`-wrapped TCP stream for the client.
    ///
    /// Reads data from the client, detects disconnection, and forwards
    /// received messages to the server loop.
    ///
    /// # Returns
    /// - `Ok(())`: If the client disconnects normally.
    /// - `Err(())`: If an error occurs while reading or sending messages.
    fn client(messages: Sender<Message>, stream: Connection) -> Result<()> {
        let mut buffer = [0u8; 1024];

        let client_addr = stream
            .peer_addr()
            .map_err(|err| error!("Failed to get client addres: {err}"))?;

        messages
            .send(Message::ClientConnected(Arc::clone(&stream)))
            .map_err(|err| eprintln!("Failed to send message to server thread: {err}"))?;

        loop {
            let n = stream.as_ref().read(&mut buffer).map_err(|err| {
                let _ = messages.send(Message::ClientDisconnected(Arc::clone(&stream)));
                error!("Failed to read from client {client_addr}: {err}")
            })?;

            if n == 0 {
                let _ = messages.send(Message::ClientDisconnected(Arc::clone(&stream)));
                break Ok(());
            }

            messages
                .send(Message::NewMessage {
                    author: Arc::clone(&stream),
                    bytes: buffer[0..n].to_vec(),
                })
                .map_err(|err| {
                    let _ = messages.send(Message::ClientDisconnected(Arc::clone(&stream)));
                    eprintln!("Failed to send message to server thread: {err}")
                })?
        }
    }
}

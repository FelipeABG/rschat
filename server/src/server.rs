use crate::{error, event::ServerEvent, info};
use std::{collections::HashMap, fmt::Display, io::ErrorKind, str::from_utf8, sync::Arc};
use tokio::net::ToSocketAddrs;
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::mpsc::{Receiver, Sender};

pub type Result<T> = std::result::Result<T, ()>;
pub type Connection = Arc<TcpStream>;

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
    /// - `Ok(Server)`: If the listener successfully binds to the address.
    /// - `Err(())`: If the bind is unsuccessful, with a message logged.
    pub async fn build<A: ToSocketAddrs + Display>(addr: A) -> Result<Self> {
        TcpListener::bind(&addr)
            .await
            .map(|listener| Self { listener })
            .map_err(|err| error!("Could not bind server to {addr}: {err}"))
    }

    /// Starts the server, accepting incoming client connections and
    /// spawning a thread for each client.
    ///
    /// # Returns
    /// - `Ok(())`: If the server runs without fatal errors.
    /// - `Err(())`: If an error occurs during initialization or runtime.
    pub async fn run(&mut self) -> Result<()> {
        let (sender, receiver) = tokio::sync::mpsc::channel(100);

        tokio::spawn(Self::server(receiver));

        let port = self
            .listener
            .local_addr()
            .map_err(|err| error!("Failed to get listener address: {err}"))?;

        info!("Listening to connections on port {port}");
        loop {
            match self.listener.accept().await {
                Ok((stream, _)) => {
                    let sender = sender.clone();
                    let stream = Arc::new(stream);
                    tokio::spawn(Self::client(sender, stream));
                }
                Err(err) => error!("Failed to connect to client: {err}"),
            }
        }
    }

    /// Main server loop that processes messages from connected clients.
    ///
    /// # Arguments
    /// - `messages`: Receiver end of the channel used to receive messages from clients.
    ///
    /// Handles new connections, disconnections, and broadcasting messages
    /// to all connected clients except the sender.
    async fn server(mut messages: Receiver<ServerEvent>) {
        let mut clients = HashMap::new();

        loop {
            match messages.recv().await {
                Some(msg) => {
                    match msg {
                        ServerEvent::ClientConnected(stream) => {
                            let client_addr = stream.peer_addr().unwrap();
                            info!("Client connected: {client_addr}");
                            clients.insert(client_addr, Arc::clone(&stream));
                        }
                        ServerEvent::ClientDisconnected(stream) => {
                            let client_addr = stream.peer_addr().unwrap();
                            info!("Client disconnected: {client_addr}");
                            clients.remove(&client_addr);
                        }
                        ServerEvent::NewMessage(conn, msg) => {
                            let author_addr = conn.peer_addr().unwrap();
                            let bytes_msg =
                                serde_json::to_string(&msg).unwrap().as_bytes().to_vec();
                            info!("Client {author_addr} sent: {} bytes", bytes_msg.len());
                            for (client, stream) in &clients {
                                let _ = stream.writable().await.map_err(|err| {
                                    error!("Failed waiting for socket to become available: {err}")
                                });

                                let _ = stream.try_write(&bytes_msg).map_err(|err| {
                                    error!("Failed to send message from {author_addr} to {client}: {err}")});
                            }
                        }
                    }
                }
                None => eprintln!("The server channel has been closed"),
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
    async fn client(messages: Sender<ServerEvent>, stream: Connection) -> Result<()> {
        let mut buffer = [0u8; 1024];

        let client_addr = stream
            .peer_addr()
            .map_err(|err| error!("Failed to get client addres: {err}"))?;

        messages
            .send(ServerEvent::ClientConnected(Arc::clone(&stream)))
            .await
            .map_err(|err| eprintln!("Failed to send message to server thread: {err}"))?;

        loop {
            if let Err(err) = stream.readable().await {
                let _ = messages.send(ServerEvent::ClientDisconnected(Arc::clone(&stream)));
                error!("Failed waiting for socket to become readable {client_addr}: {err}");
                break Ok(());
            }

            match stream.as_ref().try_read(&mut buffer) {
                Ok(0) => {
                    // Connection closed
                    let _ = messages.send(ServerEvent::ClientDisconnected(Arc::clone(&stream)));
                    break Ok(());
                }
                Ok(n) => {
                    let msg_str = match from_utf8(&buffer[0..n]) {
                        Ok(s) => s,
                        Err(err) => {
                            let _ =
                                messages.send(ServerEvent::ClientDisconnected(Arc::clone(&stream)));
                            error!("Failed to convert message to UTF8: {err}");
                            break Ok(());
                        }
                    };

                    let msg = match serde_json::from_str(msg_str) {
                        Ok(m) => m,
                        Err(err) => {
                            let _ =
                                messages.send(ServerEvent::ClientDisconnected(Arc::clone(&stream)));
                            error!("Failed to parse message: {err}");
                            break Ok(());
                        }
                    };

                    if let Err(err) = messages
                        .send(ServerEvent::NewMessage(Arc::clone(&stream), msg))
                        .await
                    {
                        let _ = messages.send(ServerEvent::ClientDisconnected(Arc::clone(&stream)));
                        error!("Failed to send message to server thread: {err}");
                        break Ok(());
                    }
                }
                Err(err) if err.kind() == ErrorKind::WouldBlock => {
                    // Tries to read from socket again
                    continue;
                }
                Err(err) => {
                    let _ = messages.send(ServerEvent::ClientDisconnected(Arc::clone(&stream)));
                    error!("Failed to read from client {client_addr}: {err}");
                    break Ok(());
                }
            }
        }
    }
}

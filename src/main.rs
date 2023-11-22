use std::collections::HashMap;
use std::io::Write;
use std::net::SocketAddr;
use std::{
    io::Read,
    net::TcpStream,
    ops::Deref,
    sync::{
        mpsc::{channel, Receiver, Sender},
        Arc,
    },
};

use log::{debug, error, info};

/// Buffer size of a single message
const MESSAGE_SIZE: usize = 32;

/// Main thread, which listens to the TcpConnection for the server
fn main() -> anyhow::Result<()> {
    // channels for server and clients to communicate
    let (tx, rx) = channel();
    let address = "127.0.0.1:6969";
    env_logger::init();

    // Starting TcpListener
    let listener = std::net::TcpListener::bind(address)?;

    //creates a new server and spawns it in a new thread
    let mut server = Server::new();
    info!("Server started and Listing on {address}");
    std::thread::spawn(move || server.start_server(rx));

    // Stars listing to client messages
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                debug!(
                    "Client Connected from {ip}",
                    ip = stream.peer_addr().unwrap()
                );

                // at this stage the Tcp connection is established between the client and the
                // server
                let connection = Arc::new(stream);
                let addr = connection.as_ref().peer_addr().map_err(|err| {
                    error!("Could not read clients addr {err}");
                    err
                })?;
                // Send the connection to the server with the channel from above
                // the receiver should be the server object which was listing to the events
                // in case of errors log it and move on
                let _ = tx
                    .send(ServerEvent::ClientJoinRequest(connection, tx.clone(), addr))
                    .map_err(|err| error!("Could not send message to server {err}"));
            }
            Err(err) => error!("Error connecting to client {err}"),
        }
    }

    Ok(())
}

struct Channel {
    name: String,
}

/// Represents the Messages sent between the client and the server
#[derive(Debug)]
enum ServerEvent {
    /// A new connection is received by the Main thread, the user wants to join the Server <br />
    /// [Arc]<[TcpStream]> - the connection that is established between the user and the server <br />
    /// [Sender]<_> - The sender for future events from the accepted client <br />
    /// [SocketAddr] - the Ip address of the user
    ClientJoinRequest(Arc<TcpStream>, Sender<ServerEvent>, SocketAddr),

    /// A new message by a connected client <br />
    /// [String] - the clients message <br />
    /// [SocketAddr] - the Ip address of the user
    ClientMessage(String, SocketAddr),

    /// The client wants/needs to disconnect from the server <br />
    /// [SocketAddr] - the Ip address of the user
    ClientDisconnect(SocketAddr),
}

/// Listens to the events from [Client] and [main] thread
struct Server {
    /// All the connected the [Client]
    clients: HashMap<SocketAddr, Client>,
}

impl Server {
    /// Gets a new instance of the [Server]
    fn new() -> Self {
        Self {
            clients: HashMap::new(),
        }
    }

    /// Starts listing to events, idealy done in a seperate thread.
    fn start_server(&mut self, rx: Receiver<ServerEvent>) {
        debug!("Server listing to events");
        for r in rx {
            match r {
                ServerEvent::ClientJoinRequest(connection, tx, addr) => {
                    let client = Client::new(connection, tx, addr);

                    self.clients.insert(addr, client);
                }
                ServerEvent::ClientMessage(msg, addr) => {
                    debug!("{addr} send the message {msg}");

                    // Execute the commands here
                    let command = Commands::from(msg.clone());
                    match command {
                        Commands::CloseConnection => {
                            self.disconnect_client(addr);
                            continue;
                        },
                        _ => (),
                    };

                    // Send message to everyone but the author
                    for (key, value) in self.clients.iter_mut() {
                        if key == &addr {
                            continue;
                        }
                        let msg = &msg;
                        let _ = writeln!(value.connection.deref(), "{msg}").map_err(|err| {
                            error!("Could not write to client: {err}");
                        });
                    }
                }
                ServerEvent::ClientDisconnect(ip) => self.disconnect_client(ip),
            }
        }
    }

    fn disconnect_client(&mut self, addr: SocketAddr) {
        let client = self.clients.remove(&addr);
        client.map(|c| {
            c.connection
                .shutdown(std::net::Shutdown::Both)
                .map_err(|err| {
                    error!("Could not disconnect the client {addr} from the server {err}")
                })
        });
    }
}

/// The client that connects to the [Server]
#[derive(Debug)]
struct Client {
    connection: Arc<TcpStream>,
}

impl Client {
    /// get a new instance of the client and also starts a new thread which relays the messages to
    /// [Server]
    fn new(connection: Arc<TcpStream>, tx: Sender<ServerEvent>, addr: SocketAddr) -> Self {
        let con = connection.clone();
        std::thread::spawn(move || {
            let mut buff = [0; MESSAGE_SIZE];
            let mut active = true;
            while active {
                match con.as_ref().read(&mut buff) {
                    Ok(0) => {
                        Client::request_disconnect(&tx, addr);
                        active = false;
                    }
                    Ok(n) => {
                        let buff = std::str::from_utf8(&buff[0..n]);

                        let _ = buff
                            .map(|buff| {
                                let buff = buff.to_string().trim_nulls();
                                let _ = tx.send(ServerEvent::ClientMessage(buff, addr)).map_err(
                                    |err| {
                                        error!("Could not send message to server {err}");
                                    },
                                );
                            })
                            .map_err(|err| {
                                error!(
                                    "User sent NON-UTF8 string. Disconnecting the client: {err}"
                                );
                                Client::request_disconnect(&tx, addr);
                                active = false;
                            });
                    }
                    Err(err) => error!("Something went wrong with the client {err}"),
                }
            }
            debug!("Client Disconnected {addr}");
        });

        Self { connection }
    }

    fn request_disconnect(tx: &Sender<ServerEvent>, addr: SocketAddr) {
        let _ =
        tx.send(ServerEvent::ClientDisconnect(addr)).map_err(|err| {
            error!("Could not send message to server {err}");
        });
    }
}

#[derive(Debug)]
enum Commands {
    CloseConnection,
    NotCommand,
}

impl From<String> for Commands {
    fn from(value: String) -> Self {
        return match value.as_str() {
            ":ext" => Self::CloseConnection,
            _ => Self::NotCommand,
        };
    }
}

trait TcpStreamString {
    fn trim_nulls(&self) -> Self;
}

impl TcpStreamString for String {
    fn trim_nulls(&self) -> Self {
        self.trim_matches(char::from(0)).trim().to_string()
    }
}

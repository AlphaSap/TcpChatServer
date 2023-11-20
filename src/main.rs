use std::{sync::{mpsc::{channel, Receiver, Sender}, Arc}, net::TcpStream, io::Read, ops::Deref, isize};
use std::io::Write;
use std::net::{SocketAddr};


fn main() -> std::io::Result<()> {

    let (tx, rx) = channel();
    let address = "127.0.0.1:6969";

    let listener = std::net::TcpListener::bind(address).map_err(|err| {
        eprintln!("[ERROR]: cannot bind {err}");
    }).unwrap();

    let mut server = Server::new();

    std::thread::spawn(move || server.start_server(rx));

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let connection =  Arc::new(stream);
                tx.send(ServerEvent::ClientJoinRequest(connection, tx.clone())).expect("Failed");
            },
            Err(_) => eprintln!("Error listing"),
        }
    }

    Ok(())
}

#[derive(Debug)]
enum ServerEvent {
    ClientJoinRequest(Arc<TcpStream>, Sender<ServerEvent>),
    ClientMessage(String, SocketAddr),
    ClientDisconnect(SocketAddr),
}

struct Server  {
    clients: Vec<Client>,
}

impl Server {
    fn new() -> Self {
        Self { clients: vec![] }
    }
    fn start_server(&mut self, rx: Receiver<ServerEvent>) {
        for r in rx {
            match r {
                ServerEvent::ClientJoinRequest(connection, tx) => {
                    let client = Client::new(connection, tx);
                    self.clients.push(client);
                },
                ServerEvent::ClientMessage(msg, addr) => {
                    for ele in self.clients.iter_mut() {
                        if ele.connection.deref().peer_addr().unwrap() == addr {
                            continue;
                        }
                        writeln!(ele.connection.deref(),  "{msg}").unwrap();
                    }
                },
                ServerEvent::ClientDisconnect(ip) => {
                    let mut i: isize = -1;
                    for (idx, val) in self.clients.iter().enumerate() {
                        if val.connection.peer_addr().expect("LOL") == ip {
                            i = idx as isize;
                        }
                    }
                    self.clients.remove(i as usize);
                }
            }
        }
    }
}

#[derive(Debug)]
struct Client {
    connection: Arc<TcpStream>,
}

impl Client {

    fn new(connection: Arc<TcpStream>, tx: Sender<ServerEvent>) -> Self {
        let con = connection.clone();
        std::thread::spawn(move || {
            let mut buff = [0; 64];
            loop {
                match con.as_ref().read(&mut buff) {
                    Ok(0) => {
                        tx.send(ServerEvent::ClientDisconnect(con.as_ref().peer_addr().unwrap())).unwrap();
                        break;
                    },
                    Ok(n) => {
                        let buff = String::from_utf8(buff[0..n].to_vec()).unwrap().trim_nulls();
                        tx.send(ServerEvent::ClientMessage(buff, con.as_ref().peer_addr().unwrap())).unwrap();
                    },
                    Err(err) => eprintln!("Somethign went wrong with the client {err}"),
                }
            }
        });

        Self {
            connection
        }
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

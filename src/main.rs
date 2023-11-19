use std::{net::{TcpListener, TcpStream, SocketAddr}, sync::{mpsc::{channel, Receiver, Sender}, Arc, Mutex}, collections::HashMap,io::{Read, Write}, borrow::BorrowMut};

fn main() -> std::io::Result<()>{
    let listener = TcpListener::bind("127.0.0.1:6969")?;

    let (tx, rx) = channel();

    let server = Arc::new(Mutex::new(Server::new()));
    let server_t = server.clone();
    std::thread::spawn(move || server_t.lock().unwrap().start(rx));

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("Connection established -> {}", stream.peer_addr()?);
                // Mantain the connection in a different thread

                let client = Client::new(Arc::new(stream));
                let tx_clone = tx.clone();

                std::thread::spawn(move || client.listen(tx_clone));
            },
            Err(_) => eprintln!("Error Connecting"),
        }
    }
    Ok(())
}

#[derive(Debug)]
struct Client {
    connection: Arc<TcpStream>,
}

impl Client {
    fn new(connection: Arc<TcpStream>) -> Self {
        Client { connection  }
    }

    fn listen(&self, tx: Sender<Event>) {
        let _ = tx.send(Event::ConnectionEstablished(self.connection.clone()));
        loop {
            let mut buff: [u8; MESSAGE_SIZE] = [0; MESSAGE_SIZE];
            let msg = self.connection.as_ref().read(&mut buff).unwrap();
            println!("{}", msg);
            let _ = tx.send(Event::MessageToSever("WHAT".to_string()));
        }
    }

    fn send_message(&mut self, message: &str) {
        self.connection.borrow_mut().write(&message.as_bytes());
    }
}

struct Server {
    clients: HashMap<SocketAddr, Arc<Client>>,
}

impl Server {
    fn add_client(&mut self, client: Arc<Client>) -> std::io::Result<()>{
        let add = client.connection.clone().peer_addr().map_err(|_| {
            eprintln!("CANNOT ADD CLIENT TO MAP")
        });
        self.clients.insert(add.unwrap(), client);
        println!("Added Client ");
        Ok(())
    }

    fn send_message_to_all(&mut self, message: String) {
        self.clients.iter_mut().for_each(|c| c.1.send_message(&message) );
    }

    fn new() -> Self {
        Server {
            clients: HashMap::new(),
        }
    }

    fn start(&mut self, rx: Receiver<Event>) {
        for r in rx {
            match r {
                Event::MessageToSever(msg) => self.send_message_to_all(msg),
                Event::MessageToAll(_) => todo!(),
                Event::ConnectionEstablished(connection) => {
                    let c = Client {
                        connection
                    };
                    let _ = self.add_client(c.into());
                }
            }
        }
    }
}

#[derive(Debug)]
enum Event {
    MessageToSever(String),
    MessageToAll(String),
    ConnectionEstablished(Arc<TcpStream>),

}

// 100 MB of message
const MESSAGE_SIZE: usize = 64;

use std::{ sync::{ Arc }};

use tokio::{sync::{Mutex, mpsc::{channel, Receiver, Sender}}, net::{TcpStream, TcpListener}, io::AsyncReadExt};

mod chat_sync;

#[tokio::main]
async fn main() {
    // Use tokio here for better async 

    let (tx, mut rx) = channel(100);
    println!("Listing");

    let server = Arc::new(Mutex::new(Server::new()));

    let listener = TcpListener::bind("127.0.0.1:6969").await.unwrap();

    tokio::spawn(async move  {
        server.clone().lock().await.start_server(rx).await
    });

    loop {
        match listener.accept().await {
            Ok((stream, addr)) => {
                println!("Client connection Established");
                let client = Client {
                    connection: stream,
                };
                let _ = tx.send(ServerEvent::ClientJoinRequest(client, tx.clone())).await;

            }
            Err(_) => eprintln!("Error"),
        }
    }

}

enum ServerEvent {
    ClientJoinRequest(Client, Sender<ServerEvent>),
    ClientMessage(String),
}

struct Server {
    clients: Vec<Arc<Mutex<Client>>>,
}

impl Server {
    fn new() -> Self {
        Server { clients: vec![] }
    }
    async fn add_client(&mut self, client: Arc<Mutex<Client>>) {
        self.clients.push(client);
    }

    async fn start_server(&mut self, mut rx: Receiver<ServerEvent>) {
        println!("Starting server");
        while let Some(r) = rx.recv().await {
            match r {
                ServerEvent::ClientJoinRequest(client, tx) => {
                    let a_client = Arc::new(Mutex::new(client));
                    let b_client = a_client.clone();
                    self.add_client(a_client).await;
                    tokio::spawn(async move {
                        b_client.lock().await.keep_connection_alive(tx).await; 
                    });
                },
                ServerEvent::ClientMessage(message) => {
                    println!("GOT MESSAGE -> {message}");
                }
            }
        }
    }
}

struct Client {
    connection: TcpStream,
}

impl Client {
    async fn keep_connection_alive(&mut self, tx: Sender<ServerEvent>) {
        println!("Listing to {}", self.connection.peer_addr().unwrap());
        loop {
            let mut buff: [u8; MESSAGE_SIZE] = [0; MESSAGE_SIZE];
            let _ = self.connection.read(&mut buff).await.unwrap();
            let msg = String::from_utf8(buff.to_vec()).unwrap();
            println!("{}", &msg);
            tx.send(ServerEvent::ClientMessage(msg)).await;
        }
    }
}

const MESSAGE_SIZE: usize = 64;

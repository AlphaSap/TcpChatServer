use std::{ sync::{ Arc }};

use tokio::{sync::{Mutex, mpsc::{channel, Receiver, Sender}}, net::{TcpStream, TcpListener}, io::{AsyncReadExt, AsyncWriteExt}};

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
    ClientDisconnected,
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
                    println!(" Sending message to allclients -> {}", &message);
                    for ele in self.clients.iter_mut() {
                        ele.lock().await.send_message(&message).await;
                    }
                },
                ServerEvent::ClientDisconnected => {
                    println!("Client Disconnected");
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
        let _ = self.connection.writable().await.map_err(|err| {
            eprintln!("Could not make the connection writeable {err}");
        });
        loop {
            let mut buff: [u8; MESSAGE_SIZE] = [0; MESSAGE_SIZE];
            let buff_ref = self.connection.read(&mut buff).await.unwrap();
            let msg = String::from_utf8(buff.to_vec()).unwrap().trim_nulls();

            // Break the connection it has been terminated by the client
            if buff_ref == 0 || msg == ":ext" {
                let _ = tx.send(ServerEvent::ClientDisconnected).await;
                let _ = self.connection.shutdown().await;
                break;
            }
            println!("{}", msg);
            let _ = tx.send(ServerEvent::ClientMessage(msg)).await;
        }
    }

    async fn send_message(&mut self, msg: &str) {
        let (_, wh) = self.connection.split();
        let able = wh.writable().await;
        // match able {
        //     Ok(_) => {
        //          let _ = wh.try_write(msg.as_bytes()).map_err(|err| {
        //             eprintln!("Could not write to client {err}");
        //         }).unwrap();
        //     },
        //     Err(_) => {},
        // }
        //

        while let Err(_) = able {
            eprintln!("not able to println");

        }
        let _ = wh.try_write(msg.as_bytes()).map_err(|err| {
            eprintln!("Could not write to client {err}");
        }).unwrap();

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

const MESSAGE_SIZE: usize = 64;

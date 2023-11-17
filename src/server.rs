use std::{net::TcpListener, sync::mpsc::{Sender, Receiver, self}};

use crate::{client::Client, MESSAGE_SIZE};
use std::sync::{ Arc, Mutex };

type AClient = Arc<Mutex<Client>>;

/// Represents the host machine
#[derive(Debug)]
pub struct Server {
    // TODO: change this to tuple of Client and tx channel, and put the rx channel in the client
    clients: Vec<(AClient, Sender<[u8; MESSAGE_SIZE]>)>,
    listener: TcpListener,
}

impl Server {

    /// Start the server
    pub fn new() -> std::io::Result<Server> {

        // TODO: change this later to Socket address version
        let address: String = String::from("127.0.0.1:6969");
        let lis = TcpListener::bind(address)?;
        Ok(Server {
            clients: vec![],
            listener: lis
        })
    }

    pub fn start(&mut self) {
        for stream in self.listener.incoming() {
            match stream {
                Ok(stream) => {
                    let (tx, rx): (Sender<[u8; MESSAGE_SIZE]>, Receiver<[u8; MESSAGE_SIZE]>) = mpsc::channel();
                    let client = Arc::new(Mutex::new(Client {
                        address: stream.peer_addr().unwrap(),
                        stream,
                        receiver: rx,
                    }));
                    println!("Connection Established -> {}", &client.lock().unwrap().address);
                    let client_clone = Arc::clone(&client);
                    std::thread::spawn(move || {
                        let _ = client_clone.lock().unwrap().handle_connection();
                    });
                    self.clients.push((client, tx));
                },
                Err(_) => panic!("Something is wrong")

            }
        }
    }
}

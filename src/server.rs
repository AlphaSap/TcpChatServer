use std::{net::TcpListener, sync::mpsc::{Sender, Receiver, self}};

use crate::{client::Client, MESSAGE_SIZE};
use std::sync::{ Arc, Mutex };

type AClient = Arc<Mutex<Client>>;
type MessageBuffer = [u8; MESSAGE_SIZE];

/// Represents the host machine
#[derive(Debug)]
pub struct Server {
    // TODO: change this to tuple of Client and tx channel, and put the rx channel in the client
    clients: Vec<(AClient, Sender<MessageBuffer>)>,
    listener: TcpListener,
}

impl Server {

    /// Returns a new server
    pub fn new() {

        // TODO: change this later to Socket address version
        let address: String = String::from("127.0.0.1:6969");
        let lis = TcpListener::bind(address).map_err(|err| {
            eprintln!("Cannot Bind {}", err);
        }).unwrap();
        let (tx, rx) : (Sender<MessageBuffer>, Receiver<MessageBuffer>)= mpsc::channel();
        let x = Arc::new(Mutex::new(Server {
            clients: vec![],
            listener: lis
        }));
        let x_clone = Arc::clone(&x);
        std::thread::spawn(move || {
            for r in rx {
                println!("THE FLIPING MESSSGAE");
                x_clone.lock().unwrap().send_message(r);
            }
        });
        x.lock().unwrap().start(tx);
    }

    fn send_message(&self, message: MessageBuffer) {
        println!("GOT THE FLIPING MESSSGAE");
    }

    pub fn start(&mut self, tx: Sender<MessageBuffer>) {
        for stream in self.listener.incoming() {
            match stream {
                Ok(stream) => {
                    let client = Arc::new(Mutex::new(Client {
                        address: stream.peer_addr().unwrap(),
                        stream,
                        sender: tx.clone(),
                    }));
                    println!("Connection Established -> {}", &client.lock().unwrap().address);
                    let client_clone = Arc::clone(&client);
                    std::thread::spawn(move || {
                        let _ = client_clone.lock().unwrap().handle_connection();
                    });
                    self.clients.push((client, tx.clone()));
                },
                Err(_) => panic!("Something is wrong")

            }
        }
    }

}

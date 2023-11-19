use std::{net::{SocketAddr, TcpStream, Shutdown}, sync::mpsc::{Receiver, Sender}};
use std::io::Read;

use crate::MESSAGE_SIZE;

#[derive(Debug )]
pub struct Client {
    pub address: SocketAddr,
    pub stream: TcpStream,
    pub sender: Sender<[u8; MESSAGE_SIZE]>,
}

impl Client {

    pub fn handle_connection(&mut self) -> std::io::Result<()>{
        // Start a continouse connection with the client
        loop {
            let mut buffer = [0; MESSAGE_SIZE];
            self.stream.read(&mut buffer)?;
            let buff = String::from_utf8(buffer.to_vec());

            let buff = match buff {
                Ok(buff) => buff,
                Err(_) => continue,
            };

            let buff = buff .trim_matches(char::from(0)).trim();

            if buff == ":ext" {
                break;
            }
            // Sending message here...
            let _ = self.sender.send(buffer).unwrap();
            println!("{}", buff);
        }
        println!("Connection Closed -> {:?}", self.stream.peer_addr());
        // Close the connection
        let _ = self.stream.shutdown(Shutdown::Both);
        Ok(())
    }
}

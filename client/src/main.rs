use std::net::TcpStream;
use std::io::Write;

use chat_messages::Message;

fn main() {

    let address = "127.0.0.1:6969";
    let mut stream = TcpStream::connect(address).unwrap();
    let msg = String::from("Hello from client");
    let msg = Message::new(msg);
    let msg = serde_json::to_string(&msg).unwrap();
    let msg = msg.trim();

    println!("{msg}");
    
    writeln!(stream, "{msg}");

}

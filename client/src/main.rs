use std::io::Read;
use std::io::Write;
use std::net::TcpStream;
use std::sync::Arc;

use chat_messages::Message;
use termion::clear;
use termion::color;

fn main() {
    println!("{}", clear::All);

    let address = "127.0.0.1:6969";
    let mut stream = Arc::new(TcpStream::connect(address).unwrap());

    loop {
        let mut buff = [0; 10000];
        let x = stream.as_ref().read(&mut buff).unwrap();
        let vec = buff[0..x].to_vec();
        let from_utf8 = String::from_utf8(vec).unwrap();
        println!("{from_utf8}");
        let x = serde_json::from_str::<Message>(&from_utf8).unwrap();
        // let msg = if x.name().is_some() {
        let msg = format!(
            "{}{}{}: {}",
            color::Red.fg_str(),
            &x.name().clone(),
            color::Reset.fg_str(),
            x.message()
        );
        // } else {
        //     format!("{}{}{}: {}", color::Red.fg_str(), stream.local_addr().unwrap().to_string(), color::Reset.fg_str(), x.message())
        // };
        println!("{msg}");
    }
}

use server::Server;
mod client;
mod server;

fn main() -> std::io::Result<()>{
    let mut server = Server::new()?;
    server.start();
    Ok(())
}

// 100 MB of message
const MESSAGE_SIZE: usize = 1024 * 100;

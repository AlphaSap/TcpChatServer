use std::sync::{mpsc::channel, Arc};
mod server_data;
use server_data::Server;
use server_data::ServerEvent;

use log::{debug, error, info};

/// Main thread, which listens to the TcpConnection for the server
fn main() -> anyhow::Result<()> {
    // channels for server and clients to communicate
    let (tx, rx) = channel();
    let address = "0.0.0.0:6969";
    env_logger::init();

    // Starting TcpListener
    let listener = std::net::TcpListener::bind(address)?;

    //creates a new server and spawns it in a new thread
    let mut server = Server::new();
    info!("Server started and Listing on {address}");
    std::thread::spawn(move || server.start_server(rx));

    // Stars listing to client messages
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                debug!(
                    "Client Connected from {ip}",
                    ip = stream.peer_addr().unwrap()
                );

                // at this stage the Tcp connection is established between the client and the
                // server
                let connection = Arc::new(stream);
                let addr = connection.as_ref().peer_addr().map_err(|err| {
                    error!("Could not read clients addr {err}");
                    err
                })?;
                // Send the connection to the server with the channel from above
                // the receiver should be the server object which was listing to the events
                // in case of errors log it and move on
                let _ = tx
                    .send(ServerEvent::ClientJoinRequest(connection, tx.clone(), addr))
                    .map_err(|err| error!("Could not send message to server {err}"));
            }
            Err(err) => error!("Error connecting to client {err}"),
        }
    }

    Ok(())
}


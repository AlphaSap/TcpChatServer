use std::{net::SocketAddr, fmt::Display};

use serde::Serialize;

#[derive(serde::Deserialize, Debug, Serialize)]
pub struct Message {
    message: String,
    addr: SocketAddr,
}

impl Message {
   pub fn new(message: String, addr: SocketAddr) -> Self {
        Self { message, addr }
    }

    pub fn message(&self) -> &String {
        &self.message
    }

    pub fn addr(&self) -> &SocketAddr {
        &self.addr
    }
}


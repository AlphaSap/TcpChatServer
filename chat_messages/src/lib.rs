use std::{net::SocketAddr, fmt::Display};

use serde::Serialize;

#[derive(serde::Deserialize, Debug, Serialize)]
pub struct Message {
    message: String,
}

impl Message {
   pub fn new(message: String) -> Self {
        Self { message}

    }

    pub fn message(&self) -> &String {
        &self.message
    }

}


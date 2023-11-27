use serde::Serialize;

#[derive(serde::Deserialize, Debug, Serialize)]
pub struct Message {
    message: String,
    name: String,
}

impl Message {
    pub fn new(message: String, name: String) -> Self {
        Self { message, name }
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn message(&self) -> &String {
        &self.message
    }
}

use serde::{Deserialize, Serialize};
use crate::provider::Message;

#[derive(Serialize,Deserialize, Debug, Clone)]
pub struct Session{
    session_id: String,
    messages: Vec<Message>,
    time_stamp: String
}

impl Session {

    pub fn new(session_id: String) -> Self {
        Self{
            session_id,
            messages: Vec::new(),
            time_stamp:  "2025-11-19T00:00:00Z".to_string(),
        }
    }
    pub fn add_message(&mut self, message: Message) {
        self.messages.push(message);
    }

    pub fn get_messages(&self) -> &Vec<Message> {
        &self.messages
    }


}

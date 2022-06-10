use serde::{Deserialize, Deserializer, Serialize, Serializer};
pub enum JasmineMessage {
    PlainText(String),
    JNode(JNode),
}

pub struct JNode {
    key: String,
}

#[derive(Serialize, Deserialize)]
pub struct JasmineLog {
    pub jid: u64,
    pub topic: String,
    pub message: String,
}

impl Clone for JasmineLog {
    fn clone(&self) -> Self {
        Self {
            jid: self.jid.clone(),
            topic: self.topic.clone(),
            message: self.message.clone(),
        }
    }
}

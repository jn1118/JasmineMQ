pub enum JasmineMessage {
    PlainText(String),
    JNode(JNode),
}

pub struct JNode {
    key: String,
}

pub struct JasmineLog {
    jid: u64,
    content: JasmineMessage,   
}

impl JasmineLog {
    pub fn new(jid: u64, content: JasmineMessage) -> Self {
        return JasmineLog{ jid, content };
    }
}
pub enum JasmineMessage {
    PlainText(String),
    JNode(JNode),
}

pub struct JNode {
    key: String,
}

pub struct JasmineLog {
    pub jid: u64,
    // content: JasmineMessage,   
    pub content: String,
    pub is_ready: bool,
}

impl JasmineLog {
    pub fn new(jid: u64, content: String, is_ready: bool) -> Self {
        return JasmineLog{ jid, content, is_ready };
    }
}
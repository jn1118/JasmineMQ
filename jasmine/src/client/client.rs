use util::{result::{JasmineError, JasmineResult}, transaction::JasmineMessage};

use super::{publisher::JasminePublisher, subscriber::JasmineSubscriber};

pub trait JasmineClient: JasminePublisher + JasmineSubscriber {
    fn new() -> JasmineResult<Box<Self>>;
    fn connect() -> JasmineResult<()>;
    fn disconnect() -> JasmineResult<()>;
    fn on_message() -> JasmineMessage;
}

/// This struct includes features and functionalities of a frontend mqtt like client
pub struct Client {

    /// Unique client id
    client_id: u64,
}

impl JasmineClient for Client {
    fn new() -> JasmineResult<Box<Self>> {
        todo!()
    }

    fn connect() -> JasmineResult<()> {
        todo!()
    }

    fn disconnect() -> JasmineResult<()> {
        todo!()
    }

    fn on_message() -> JasmineMessage {
        todo!()
    }
}

impl JasminePublisher for Client {
    fn publish(&self, topic: String, message: JasmineMessage) -> JasmineResult<()> {
        self.client_id;
        todo!()
    }
}

impl JasmineSubscriber for Client {
    fn subscribe(topic: String) -> JasmineResult<()> {
        todo!()
    }

    fn unsubscribe(topic: String) -> JasmineResult<()> {
        todo!()
    }
}
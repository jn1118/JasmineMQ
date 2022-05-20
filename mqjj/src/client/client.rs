use util::{result::{MQJJError, MQJJResult}, message::MQJJMessage};

use super::{publisher::MQJJPublisher, subscriber::MQJJSubscriber};

pub trait MQJJClient: MQJJPublisher + MQJJSubscriber {
    fn new() -> MQJJResult<Box<Self>>;
    fn connect() -> MQJJResult<()>;
    fn disconnect() -> MQJJResult<()>;
    fn on_message() -> MQJJMessage;
}

pub struct Client {
    client_id: u64,
}

impl MQJJClient for Client {
    fn new() -> MQJJResult<Box<Self>> {
        todo!()
    }

    fn connect() -> MQJJResult<()> {
        todo!()
    }

    fn disconnect() -> MQJJResult<()> {
        todo!()
    }

    fn on_message() -> MQJJMessage {
        todo!()
    }
}

impl MQJJPublisher for Client {
    fn publish(topic: String, message: MQJJMessage) -> MQJJResult<()> {
        todo!()
    }
}

impl MQJJSubscriber for Client {
    fn subscribe(topic: String) -> MQJJResult<()> {
        todo!()
    }

    fn unsubscribe(topic: String) -> MQJJResult<()> {
        todo!()
    }
}
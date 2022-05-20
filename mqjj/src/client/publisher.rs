use util::{result::{MQJJError, MQJJResult}, message::MQJJMessage};

pub trait MQJJPublisher {
    fn publish(topic: String, message: MQJJMessage) -> MQJJResult<()>;
}






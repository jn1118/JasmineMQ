use util::{result::{MQJJError, MQJJResult}, message::MQJJMessage};

pub trait MQJJSubscriber {
    fn subscribe(topic: String) -> MQJJResult<()>;
    fn unsubscribe(topic: String) -> MQJJResult<()>; 
}
use async_trait::async_trait;
use std::collections::{HashMap, HashSet};
use util::{
    result::{JasmineError, JasmineResult},
    transaction::JasmineMessage,
};

#[async_trait]
///A trait representing a JasmineBroker interface.
pub trait JasmineBroker: Send + Sync {
    ///An async function that takes in the topic and message and then store the messsage and publish the message to the correpsonding subcriber.
    ///It return the result with the size of subscriber.
    async fn on_pub_message(&self, topic: &String, message: &JasmineMessage) -> JasmineResult<u64>;
    // async fn on_connect()
}

/// This struct includes features and functionalities of a broker server
pub struct BrokerServer {
    pub subscriber_map: HashMap<String, HashSet<u64>>,
}

#[async_trait]
impl JasmineBroker for BrokerServer {
    async fn on_pub_message(&self, topic: &String, message: &JasmineMessage) -> JasmineResult<u64> {
        let subscriber_set = match self.subscriber_map.get(topic) {
            Some(set) => set,
            None => {
                return Ok(0);
            }
        };

        for subscriber in subscriber_set.iter() {
            todo!()
        }

        match subscriber_set.len().try_into() {
            Ok(size) => {
                return Ok(size);
            }
            Err(error) => {
                return Err(Box::new(error));
            }
        };
    }
}

impl BrokerServer {
    pub fn send_to_subscriber() {
        todo!()
    }
}

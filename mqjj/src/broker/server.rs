use util::{message::MQJJMessage, result::{MQJJError, MQJJResult}};
use std::collections::{HashMap, HashSet};
use async_trait::async_trait;

#[async_trait]
pub trait MQJJBroker {
    fn on_pub_message(&self, topic: String, message: MQJJMessage) -> MQJJResult<u64>;
}

pub struct BrokerServer {
    pub subscriber_map: HashMap<String, HashSet<u64>>,
}

#[async_trait]
impl MQJJBroker for BrokerServer {
    async fn on_pub_message(&self, topic: String, message: MQJJMessage) -> MQJJResult<u64> {
        let subscriber_set = match self.subscriber_map.get(&topic) {
            Some(set) => set,
            None => {
                return Ok(0);
            },
        };

        for subscriber in subscriber_set.iter() {
            todo!()
        }

        match subscriber_set.len().try_into() {
            Ok(size) => {
                return Ok(size);
            },
            Err(error) => {
                return Err(Box::new(error));
            },
        };
    }

  
}

impl BrokerServer {
    pub fn send_to_subscriber() {
        todo!()
    }
}
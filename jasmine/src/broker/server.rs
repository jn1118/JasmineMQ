use util::{transaction::JasmineMessage, result::{JasmineError, JasmineResult}};
use std::collections::{HashMap, HashSet};
use async_trait::async_trait;

#[async_trait]
pub trait JasmineBroker {
    async fn on_pub_message(&self, topic: String, message: JasmineMessage) -> JasmineResult<u64>;
    // async fn on_connect()
}

pub struct BrokerServer {
    pub subscriber_map: HashMap<String, HashSet<u64>>,
}

#[async_trait]
impl JasmineBroker for BrokerServer {
    async fn on_pub_message(&self, topic: String, message: JasmineMessage) -> JasmineResult<u64> {
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
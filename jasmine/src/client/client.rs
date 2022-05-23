use super::{publisher::JasminePublisher, subscriber::JasmineSubscriber};
use crate::broker::server::JasmineBroker;
use async_trait::async_trait;
use std::collections::{HashMap, HashSet};
// use tonic::Status;
use util::{
    result::{JasmineError, JasmineResult},
    transaction::JasmineMessage,
};

///A trait representing a JasmineClient interface. The trait bounds for JasminePublisher and JasmineSubscriber respectively.
pub trait JasmineClient: JasminePublisher + JasmineSubscriber + Send + Sync {
    ///A function creates and returns the object
    // fn new(&self, client_id: u64, broker: Box<dyn JasmineBroker>) -> JasmineResult<Box<Self>>;
    ///A function connects the client
    fn connect(&mut self, user: &str) -> JasmineResult<()>;
    ///A function disconnets the client
    fn disconnect(&mut self, user: &str) -> JasmineResult<()>;
    fn on_message(&self) -> JasmineMessage;
}

/// This struct includes features and functionalities of a frontend mqtt like client
pub struct Client {
    /// Unique client id
    pub client_id: u64,
    pub client_map: HashMap<String, u64>,
    pub broker: Box<dyn JasmineBroker>,
}

impl JasmineClient for Client {
    // fn new(&self, clientid: u64, broker: Box<dyn JasmineBroker>) -> JasmineResult<Box<Self>> {
    //     return Ok(Box::new(Client {
    //         client_id: clientid,
    //         broker: broker,
    //     }));
    // }

    fn connect(&mut self, user: &str) -> JasmineResult<()> {
        let client_exist = self.client_map.get(user);
        let mut id = 0;
        match client_exist {
            Some(value) => {
                id = *value;
            }
            None => {
                self.client_map.insert(user.to_string(), self.client_id);
                id = self.client_id;
                self.client_id = self.client_id + 1;
            }
        }
        // do something using id
        return Ok(());
    }

    fn disconnect(&mut self, user: &str) -> JasmineResult<()> {
        todo!()
    }

    fn on_message(&self) -> JasmineMessage {
        todo!()
    }
}
#[async_trait]
impl JasminePublisher for Client {
    async fn publish(&self, topic: String, message: JasmineMessage) -> JasmineResult<()> {
        let a = self.broker.on_pub_message(&topic, &message).await;
        match a {
            Ok(_) => {
                return Ok(());
            }
            Err(e) => {
                return Err(e);
            }
        }
    }
}

impl JasmineSubscriber for Client {
    fn subscribe(&self, topic: String) -> JasmineResult<()> {
        // call method in broker
        todo!()
    }

    fn unsubscribe(&self, topic: String) -> JasmineResult<()> {
        // call method in broker
        todo!()
    }
}

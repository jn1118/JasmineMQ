use super::{publisher::JasminePublisher, subscriber::JasmineSubscriber};
use async_trait::async_trait;
use std::collections::{HashMap, HashSet};
use util::rpc::broker::jasmine_broker_server::JasmineBrokerServer;
use util::rpc::client::jasmine_client_client::JasmineClientClient;
// use tonic::Status;
use util::{
    result::{JasmineError, JasmineResult},
    // rpc::broker::jasmine_broker_server::JasmineBroker,
    transaction::JasmineMessage,
};

///A trait representing a JasmineClient interface. The trait bounds for JasminePublisher and JasmineSubscriber respectively.
pub trait JasmineClient: JasminePublisher + JasmineSubscriber + Send + Sync {
    ///A function creates and returns the object
    fn new(&self, broker: Vec<String>) -> JasmineResult<Box<Self>>;
    ///A function connects the client
    fn connect(&self) -> JasmineResult<()>;
    ///A function disconnets the client
    fn disconnect(&self) -> JasmineResult<()>;
    fn on_message(&self) -> JasmineMessage;
}

/// This struct includes features and functionalities of a frontend mqtt like client
pub struct Client {
    /// Unique client id
    // pub client_id: u64,
    // pub client_map: HashMap<String, u64>,
    pub broker_addr: Vec<String>,
}

impl JasmineClient for Client {
    fn new(&self, broker: Vec<String>) -> JasmineResult<Box<Self>> {
        return Ok(Box::new(Client {
            broker_addr: broker,
        }));
    }

    fn connect(&self) -> JasmineResult<()> {
        let mut broker =
            JasmineBrokerServer::connect(format!("http://{}", &self.broker_addr[0])).await;

        // let broker = connect(self.broker_addr[0]);
        return Ok(());
    }
    // fn connect(&mut self, user: &str) -> JasmineResult<()> {
    //     let client_exist = self.client_map.get(user);
    //     let mut id = 0;
    //     match client_exist {
    //         Some(value) => {
    //             id = *value;
    //         }
    //         None => {
    //             self.client_map.insert(user.to_string(), self.client_id);
    //             id = self.client_id;
    //             self.client_id = self.client_id + 1;
    //         }
    //     }
    //     // do something using id
    //     return Ok(());
    // }

    fn disconnect(&self) -> JasmineResult<()> {
        todo!()
    }

    fn on_message(&self) -> JasmineMessage {
        todo!()
    }
}
#[async_trait]
impl JasminePublisher for Client {
    async fn publish(&self, topic: String, message: JasmineMessage) -> JasmineResult<()> {
        return Ok(());
        // self.connect(user)
        // let a = self.publish(&topic, &message).await;
        // match a {
        //     Ok(_) => {
        //         return Ok(());
        //     }
        //     Err(e) => {
        //         return Err(e);
        //     }
        // }
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

use super::{publisher::JasminePublisher, subscriber::JasmineSubscriber};
use async_trait::async_trait;
use std::collections::{HashMap, HashSet};
use util::leader_util::find_leader;
use util::rpc::broker::jasmine_broker_client::JasmineBrokerClient;
use util::rpc::broker::jasmine_broker_server::JasmineBrokerServer;
use util::rpc::broker::{ConnectRequest, PublishRequest, SubscribeRequest};
use util::rpc::client::jasmine_client_client::JasmineClientClient;
// use tonic::Status;
use util::{
    result::{JasmineError, JasmineResult},
    // rpc::broker::jasmine_broker_server::JasmineBroker,
    transaction::JasmineMessage,
};

///A trait representing a JasmineClient interface. The trait bounds for JasminePublisher and JasmineSubscriber respectively.
#[async_trait]
pub trait JasmineClient: JasminePublisher + JasmineSubscriber + Send + Sync {
    ///A function creates and returns the object
    // fn new(broker: Vec<String>) -> Self;
    ///A function connects the client
    async fn connect(&self) -> JasmineResult<()>;
    ///A function disconnets the client
    async fn disconnect(&self) -> JasmineResult<()>;
    fn on_message(&self) -> JasmineMessage;
}

/// This struct includes features and functionalities of a frontend mqtt like client
#[derive(Debug)]
pub struct Client {
    pub broker_addr: Vec<String>,
    pub client_addr: String,
}

// impl Client {
//     pub fn new(broker_addr: Vec<String>) -> Self {
//         return Client {
//             broker_addr: broker_addr,
//         };
//     }
// }

#[async_trait]
impl JasmineClient for Client {
    async fn connect(&self) -> JasmineResult<()> {
        todo!();
        // let broker_addr = find_leader(&topic.clone().to_string());
        // let broker = JasmineBrokerClient::connect(format!("http://{}", broker_addr)).await;
        // match broker {
        //     Ok(mut connection) => {
        //         let result = connection
        //             .hook(ConnectRequest {
        //                 address: self.client_addr.to_string(),
        //             })
        //             .await;
        //         match result {
        //             Ok(_) => {
        //                 return Ok(());
        //             }
        //             Err(e) => {
        //                 return Err(Box::new(e));
        //             }
        //         }
        //     }
        //     Err(e) => {
        //         return Err(Box::new(e));
        //     }
        // }
    }
    async fn disconnect(&self) -> JasmineResult<()> {
        todo!();
        // let broker_addr = find_leader(&topic.clone().to_string());
        // let broker = JasmineBrokerClient::connect(format!("http://{}", broker_addr)).await;
        // match broker {
        //     Ok(mut connection) => {
        //         let result = connection
        //             .unhook(ConnectRequest {
        //                 address: self.client_addr.to_string(),
        //             })
        //             .await;
        //         match result {
        //             Ok(_) => {
        //                 return Ok(());
        //             }
        //             Err(e) => {
        //                 return Err(Box::new(e));
        //             }
        //         }
        //     }
        //     Err(e) => {
        //         return Err(Box::new(e));
        //     }
        // }
    }

    fn on_message(&self) -> JasmineMessage {
        todo!()
    }
}
#[async_trait]
impl JasminePublisher for Client {
    async fn publish(&self, topic: String, message: String) -> JasmineResult<()> {
        let broker_addr = find_leader(&topic.clone().to_string());
        let broker = JasmineBrokerClient::connect(format!("http://{}", broker_addr)).await;
        match broker {
            Ok(mut connection) => {
                let result = connection
                    .publish(PublishRequest {
                        topic: topic,
                        message: message,
                    })
                    .await;
                match result {
                    Ok(_) => {
                        return Ok(());
                    }
                    Err(e) => {
                        return Err(Box::new(e));
                    }
                }
            }
            Err(e) => {
                return Err(Box::new(e));
            }
        }
    }
}
#[async_trait]
impl JasmineSubscriber for Client {
    async fn subscribe(&self, topic: String) -> JasmineResult<()> {
        // call method in broker
        let broker_addr = find_leader(&topic.clone().to_string());
        eprintln!("hahahha");
        dbg!(broker_addr.clone());
        let broker = JasmineBrokerClient::connect(format!("http://{}", broker_addr)).await;
        match broker {
            Ok(mut connection) => {
                dbg!("before hook");
                let result1 = connection
                    .hook(ConnectRequest {
                        address: self.client_addr.to_string(),
                    })
                    .await?;
                dbg!("after hook");
                let result = connection
                    .subscribe(SubscribeRequest {
                        address: self.client_addr.to_string(),
                        topic: topic,
                    })
                    .await;
                match result {
                    Ok(_) => {
                        return Ok(());
                    }
                    Err(e) => {
                        return Err(Box::new(e));
                    }
                }
            }
            Err(e) => {
                return Err(Box::new(e));
            }
        }
    }

    async fn unsubscribe(&self, topic: String) -> JasmineResult<()> {
        // call method in broker
        let broker_addr = find_leader(&topic.clone().to_string());
        let broker = JasmineBrokerClient::connect(format!("http://{}", broker_addr)).await;
        match broker {
            Ok(mut connection) => {
                let result = connection
                    .unsubscribe(SubscribeRequest {
                        address: self.client_addr.to_string(),
                        topic: topic,
                    })
                    .await;
                match result {
                    Ok(_) => {
                        return Ok(());
                    }
                    Err(e) => {
                        return Err(Box::new(e));
                    }
                }
            }
            Err(e) => {
                return Err(Box::new(e));
            }
        }
    }
}

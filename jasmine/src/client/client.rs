use super::{publisher::JasminePublisher, subscriber::JasmineSubscriber};
use async_trait::async_trait;
use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use tokio::sync::Mutex;
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

/// This struct includes features and functionalities of a frontend mqtt like client
#[derive(Debug)]
pub struct Client<T: Fn(String, String, bool) -> ()> {
    pub broker_addr: Vec<String>,
    pub client_addr: String,
    pub message_callback: Option<T>,
    pub message_buffer: Arc<Mutex<Vec<(String, String, bool)>>>,
}

impl<T: Fn(String, String, bool) -> ()> Client<T> {
    pub fn new(
        broker_addr: Vec<String>,
        addr: String,
        message_buffer: Arc<Mutex<Vec<(String, String, bool)>>>,
    ) -> Self {
        return (Client {
            client_addr: addr,
            broker_addr: broker_addr,
            message_callback: None,
            message_buffer: message_buffer.clone(),
        });
    }

    fn on_message(&mut self, func: T) {
        self.message_callback = Some(func);
    }

    async fn publish(
        &self,
        topic: String,
        message: String,
        is_consistent: bool,
    ) -> JasmineResult<()> {
        let broker_addr = find_leader(&topic.clone().to_string());
        let broker = JasmineBrokerClient::connect(format!("http://{}", broker_addr)).await;
        match broker {
            Ok(mut connection) => {
                let result = connection
                    .publish(PublishRequest {
                        topic: topic,
                        message: message,
                        is_consistent: is_consistent,
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

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
pub struct Client {
    pub broker_addr: Vec<String>,
    pub client_addr: String,
    pub message_map: Arc<Mutex<HashMap<(String, bool), Vec<String>>>>,
}

impl Client {
    pub fn new(
        broker_addr: Vec<String>,
        addr: String,
        message_map: Arc<Mutex<HashMap<(String, bool), Vec<String>>>>,
    ) -> Self {
        return (Client {
            client_addr: addr,
            broker_addr: broker_addr,
            message_map: message_map.clone(),
        });
    }

    pub async fn on_message(&self, topic: String, is_consistent: bool) -> Vec<String> {
        let mut temp_message_map = self.message_map.lock().await;
        let value = temp_message_map.get(&(topic, is_consistent));
        let mut result = Vec::new();
        dbg!("inside on_message");
        // dbg!(temp_message_map.clone());
        match value {
            Some(array) => result = array.to_vec(),
            None => result = Vec::new(),
        }
        drop(temp_message_map);
        return result;
    }

    pub async fn publish(
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
                        dbg!(&e);
                        return Err(Box::new(e));
                    }
                }
            }
            Err(e) => {
                return Err(Box::new(e));
            }
        }
    }
    pub async fn subscribe(&self, topic: String) -> JasmineResult<()> {
        // call method in broker
        let broker_addr = find_leader(&topic.clone().to_string());
        dbg!("hahahha");
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

    pub async fn unsubscribe(&self, topic: String) -> JasmineResult<()> {
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

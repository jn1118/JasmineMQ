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
pub struct Client<T: Fn(String, String) -> ()> {
    pub broker_addr: Vec<String>,
    pub client_addr: String,
    pub message_map: Arc<Mutex<HashMap<(String, bool), Vec<String>>>>,
    pub message_callback: Option<T>,
    pub message_buffer: Arc<Mutex<Vec<(String, String)>>>,
}

impl<T: Fn(String, String) -> ()> Client<T> {
    pub fn new(
        broker_addr: Vec<String>,
        addr: String,
        message_map: Arc<Mutex<HashMap<(String, bool), Vec<String>>>>,
        message_buffer: Arc<Mutex<Vec<(String, String)>>>,
    ) -> Self {
        return Client {
            client_addr: addr,
            broker_addr: broker_addr,
            message_map: message_map,
            message_callback: None,
            message_buffer: message_buffer.clone(),
        };
    }

    async fn process_message_buffer(&self) -> () {
        let mut temp_message_buffer = self.message_buffer.lock().await;
        if (*temp_message_buffer).len() == 0 {
            drop(temp_message_buffer);
            return;
        } else {
            let (topic, message) = (*temp_message_buffer).pop().unwrap();
            match &self.message_callback {
                Some(func) => func(topic, message),
                None => {
                    println!("topic: {:?} message: {:?}", topic, message);
                }
            }

            drop(temp_message_buffer);
        }
    }

    fn on_message(&self, func: T) -> () {
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

use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
    sync::Arc,
};

use tokio::sync::Mutex;
use tonic::{transport::Channel, Request};
use util::{
    rpc::{
        broker::jasmine_broker_client::JasmineBrokerClient,
        client::{jasmine_client_client::JasmineClientClient, Message},
    },
    transaction::JasmineLog,
};

pub struct Manager {
    pub subscriber_map: Arc<Mutex<HashMap<String, HashSet<String>>>>,
    pub client_map: Arc<Mutex<HashMap<String, JasmineClientClient<Channel>>>>,
    pub message_queue: Arc<Mutex<Vec<(String, String)>>>,
    pub addr: String,
    // pub logs: Arc<Mutex<HashMap<String, JasmineLog>>>,
    // pub storage_addrs: Vec<String>,
    // pub storage_clients: Vec<>,
}

impl Manager {
    pub fn new(
        subscriber_map: Arc<Mutex<HashMap<String, HashSet<String>>>>,
        client_map: Arc<Mutex<HashMap<String, JasmineClientClient<Channel>>>>,
        message_queue: Arc<Mutex<Vec<(String, String)>>>,
        addr: String,
    ) -> Self {
        return Manager {
            subscriber_map: subscriber_map,
            client_map: client_map,
            message_queue: message_queue,
            addr: addr,
        };
    }

    pub async fn process_message_queue(&self) -> () {
        let mut temp_message_queue = self.message_queue.lock().await;
        let (topic, message) = match (*temp_message_queue).pop() {
            Some(message) => message,
            None => {
                return;
            }
        };

        drop(temp_message_queue);

        let mut temp_subscriber_map = self.subscriber_map.lock().await;
        let mut temp_client_map = self.client_map.lock().await;
        let subscriber_set = match (*temp_subscriber_map).get(&topic) {
            Some(set) => set,
            None => {
                return;
            }
        };

        for ip in subscriber_set.iter() {
            match (*temp_client_map).get_mut(ip) {
                Some(client) => {
                    match client
                        .send_message(Message {
                            topic: topic.clone(),
                            message: message.clone(),
                        })
                        .await
                    {
                        Ok(_) => {}
                        Err(_) => {}
                    }
                }
                None => {
                    continue;
                }
            };
        }

        drop(temp_subscriber_map);
        drop(temp_client_map);

        return;
    }

    pub async fn client_garbage_collect(&self) -> () {
        todo!()
    }

    pub async fn subscriber_garbage_collect(&self) -> () {
        todo!()
    }

    pub async fn append_log(&self, jasmine_log: JasmineLog) -> () {
        // let log = JasmineLog {
        //     jid: 1,
        //     content: todo!(),
        // };
        todo!()
    }

    pub async fn log_garbage_collect(&self) -> () {
        todo!()
    }

    pub async fn check_size(&self, message: String) -> bool {
        todo!()
    }

    pub async fn write_to_storage(&self, message: String) -> () {
        todo!()
    }
}

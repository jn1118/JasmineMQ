use std::{
    collections::{HashMap, HashSet},
    sync::Arc,
};

use tokio::sync::Mutex;
use tonic::{transport::Channel};
use util::{
    leader_util::find_leader,
    result::JasmineResult,
    rpc::{
        broker::{jasmine_broker_client::JasmineBrokerClient, PublishRequest},
        client::{jasmine_client_client::JasmineClientClient, Message},
    },
    transaction::JasmineLog,
};

pub struct Manager {
    pub subscriber_map: Arc<Mutex<HashMap<String, HashSet<String>>>>,
    pub client_map: Arc<Mutex<HashMap<String, JasmineClientClient<Channel>>>>,
    pub message_queue: Arc<Mutex<Vec<(String, String)>>>,
    pub addrs: Vec<String>,
    pub node_id: usize,
    pub logs: Arc<Mutex<HashMap<String, Vec<JasmineLog>>>>,
    // pub storage_addrs: Vec<String>,
    // pub storage_clients: Vec<>,
}

impl Manager {
    pub fn new(
        subscriber_map: Arc<Mutex<HashMap<String, HashSet<String>>>>,
        client_map: Arc<Mutex<HashMap<String, JasmineClientClient<Channel>>>>,
        message_queue: Arc<Mutex<Vec<(String, String)>>>,
        addrs: Vec<String>,
        node_id: usize,
        logs: Arc<Mutex<HashMap<String, Vec<JasmineLog>>>>,
    ) -> Self {
        return Manager {
            subscriber_map: subscriber_map,
            client_map: client_map,
            message_queue: message_queue,
            addrs: addrs,
            node_id: node_id,
            logs: logs,
        };
    }

    pub async fn process_message_queue(&mut self) -> JasmineResult<()> {
        let mut temp_message_queue = self.message_queue.lock().await;
        let (topic, message) = match (*temp_message_queue).pop() {
            Some(message) => message,
            None => {
                return Ok(());
            }
        };

        drop(temp_message_queue);

        let mut temp_all_logs = self.logs.lock().await;
        let logs = match (*temp_all_logs).get_mut(&topic) {
            Some(logs) => logs,
            None => {
                return Ok(());
            }
        };
        let jid = logs.len() as u64;
        logs.push(JasmineLog {
            jid: jid,
            content: message.clone(),
            is_ready: false,
        });

        // If this node is the leader:
        if find_leader(&topic) == self.addrs[self.node_id] {
            // Copy this log to back-up nodes.
            for i in 0..self.addrs.len() {
                if i != self.node_id {
                    let mut backup =
                        match JasmineBrokerClient::connect(format!("http://{}", &self.addrs[i]))
                            .await
                        {
                            Ok(backup) => backup,
                            Err(_) => continue,
                        };
                    let result = backup
                        .publish(PublishRequest {
                            topic: topic.clone(),
                            message: message.clone(),
                        })
                        .await;
                    match result {
                        Ok(_) => continue,
                        Err(e) => {
                            continue;
                        }
                    }
                }
            }
            
            // Send the message to subscribers, note that only the leader will send the message.
            let temp_subscriber_map = self.subscriber_map.lock().await;
            let mut temp_client_map = self.client_map.lock().await;
            let subscriber_set = match (*temp_subscriber_map).get(&topic) {
                Some(set) => set,
                None => {
                    return Ok(());
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
        }

        drop(temp_all_logs);

        return Ok(());
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

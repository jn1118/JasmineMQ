use std::{
    collections::{HashMap, HashSet},
    sync::Arc,
};

use tokio::sync::Mutex;
use tonic::transport::Channel;
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
    pub back_ups: Arc<Mutex<HashMap<String, JasmineBrokerClient<Channel>>>>,
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
        back_ups: Arc<Mutex<HashMap<String, JasmineBrokerClient<Channel>>>>,
        addrs: Vec<String>,
        node_id: usize,
        logs: Arc<Mutex<HashMap<String, Vec<JasmineLog>>>>,
    ) -> Self {
        return Manager {
            subscriber_map: subscriber_map,
            client_map: client_map,
            message_queue: message_queue,
            back_ups: back_ups,
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
        let logs = (*temp_all_logs).entry(topic.clone()).or_insert(Vec::new());
        let jid = logs.len() as u64;
        logs.push(JasmineLog {
            jid: jid,
            content: message.clone(),
            is_ready: false,
        });

        // If this node is the leader:

        if find_leader(&topic) == self.addrs[self.node_id] {
            dbg!(self.addrs.clone());
            // Copy this log to back-up nodes.

            for i in 0..self.addrs.len() {
                if i != self.node_id {
                    let backup_addr = self.addrs[i].clone();
                    let mut temp_backups = self.back_ups.lock().await;

                    let backup = match (*temp_backups).get_mut(&backup_addr) {
                        Some(backup) => backup,
                        None => {
                            let backup_client = match JasmineBrokerClient::connect(format!(
                                "http://{}",
                                &backup_addr
                            ))
                            .await
                            {
                                Ok(backup) => backup,
                                Err(_) => continue,
                            };
                            (*temp_backups).insert(backup_addr.clone(), backup_client);
                            (*temp_backups).get_mut(&backup_addr).unwrap()
                        }
                    };

                    let _ = backup
                        .publish(PublishRequest {
                            topic: topic.clone(),
                            message: message.clone(),
                        })
                        .await;
                    drop(temp_backups);
                }
                dbg!(i);
            }
            dbg!("after for loop");

            // Send the message to subscribers, note that only the leader will send the message.
            let temp_subscriber_map = self.subscriber_map.lock().await;
            let mut temp_client_map = self.client_map.lock().await;
            let subscriber_set = match (*temp_subscriber_map).get(&topic) {
                Some(set) => set,
                None => {
                    return Ok(());
                }
            };
            dbg!("before send");
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

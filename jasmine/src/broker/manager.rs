use std::{
    collections::{HashMap, HashSet, VecDeque},
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
use zookeeper::ZooKeeper;

pub struct Manager {
    pub subscriber_map: Arc<Mutex<HashMap<String, HashSet<String>>>>,
    pub client_map: Arc<Mutex<HashMap<String, JasmineClientClient<Channel>>>>,
    pub message_queue: Arc<Mutex<VecDeque<(u64, String, String, bool)>>>,
    pub back_ups: Arc<Mutex<HashMap<String, JasmineBrokerClient<Channel>>>>,
    pub addrs: Vec<String>,
    pub node_id: usize,
    pub logs: Arc<Mutex<VecDeque<JasmineLog>>>,
    pub keeper_client: Arc<Mutex<ZooKeeper>>,
}

impl Manager {
    pub async fn process_message_queue(&mut self) -> JasmineResult<()> {
        // dbg!("inside process message queue");
        let mut temp_message_queue = self.message_queue.lock().await;

        let (jid, topic, message, is_consistent) = match (*temp_message_queue).pop_front() {
            Some(message) => {
                dbg!("ddffff");
                dbg!(&message);
                message
            }
            None => {
                return Ok(());
            }
        };
        dbg!(&message);
        drop(temp_message_queue);
        match is_consistent {
            true => {
                self.append_log(topic, message).await;
            }
            false => {
                dbg!("inside false");
                self.pub_message_to_subscriber(topic, message, is_consistent)
                    .await;
            }
        }
        return Ok(());
    }

    pub async fn pub_message_to_subscriber(
        &self,
        topic: String,
        message: String,
        is_consistent: bool,
    ) -> JasmineResult<()> {
        dbg!("inside pub_message_to_subscriber");
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
                            is_consistent: is_consistent,
                        })
                        .await
                    {
                        Ok(_) => {
                            dbg!("SEND OK");
                        }
                        Err(e) => {
                            dbg!(e);
                        }
                    }
                }
                None => match JasmineClientClient::connect(format!("http://{}", &ip)).await {
                    Ok(mut client) => {
                        (*temp_client_map).insert(ip.clone(), client.clone());
                        match client
                            .send_message(Message {
                                topic: topic.clone(),
                                message: message.clone(),
                                is_consistent: is_consistent,
                            })
                            .await
                        {
                            Ok(_) => {
                                dbg!("SEND OK");
                            }
                            Err(e) => {
                                dbg!(e);
                            }
                        }
                    }
                    Err(error) => {
                        dbg!(error);
                    }
                },
            };
        }
        drop(temp_subscriber_map);
        drop(temp_client_map);
        return Ok(());
    }

    pub async fn append_log(&mut self, topic: String, message: String) {
        let mut temp_logs = self.logs.lock().await;

        let log = JasmineLog {
            jid: 0,
            topic: topic,
            message: message,
        };
        (*temp_logs).push_back(log);
        drop(temp_logs);
    }

    pub async fn back_up_log(&mut self) {
        let mut temp_logs = self.logs.lock().await;
        let logs = (*temp_logs).clone();
        drop(temp_logs);

        let mut log_string = String::new();

        for log in logs {
            log_string = format!("{}|{}", log_string, serde_json::to_string(&log).unwrap());
        }

        let mut temp_keeper_client = self.keeper_client.lock().await;
        let path = format!("/logs/{}", self.node_id);
        (*temp_keeper_client).set_data(&path, log_string.as_bytes().to_vec(), None);
        drop(temp_keeper_client);
    }

    pub async fn process_log(&mut self) {
        let mut temp_logs = self.logs.lock().await;

        match (*temp_logs).get(0) {
            Some(log) => {
                match self
                    .pub_message_to_subscriber(log.topic.clone(), log.message.clone(), true)
                    .await
                {
                    Ok(_) => {
                        (*temp_logs).pop_front();
                    }
                    Err(e) => {}
                }
            }
            None => {}
        }

        drop(temp_logs)
    }
}

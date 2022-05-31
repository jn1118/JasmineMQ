use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use tokio::sync::Mutex;
use tonic::async_trait;
use tonic::codegen::http::Request;
use tonic::transport::Channel;
use tonic::{Response, Status};
use util::leader_util::find_leader;
use util::result::{JasmineError, JasmineResult};
use util::rpc::broker::jasmine_broker_client::JasmineBrokerClient;
use util::rpc::broker::jasmine_broker_server::{JasmineBroker, JasmineBrokerServer};
use util::rpc::broker::{
    ConnectRequest, Empty, PublishRequest, PublishResponse, SubscribeRequest, SubscribeResponse,
};

use util::rpc::client::jasmine_client_client::JasmineClientClient;

pub struct RpcProcessor {
    pub subscriber_map: Arc<Mutex<HashMap<String, HashSet<String>>>>,
    pub client_map: Arc<Mutex<HashMap<String, JasmineClientClient<Channel>>>>,
    pub message_queue: Arc<Mutex<Vec<(String, String)>>>,
    pub back_ups: Arc<Mutex<HashMap<String, JasmineBrokerClient<Channel>>>>,
    pub addrs: Vec<String>,
    pub node_id: usize,
}

impl RpcProcessor {
    pub fn new(addrs: Vec<String>, node_id: usize) -> Self {
        return RpcProcessor {
            subscriber_map: Arc::new(Mutex::new(HashMap::new())),
            client_map: Arc::new(Mutex::new(HashMap::new())),
            message_queue: Arc::new(Mutex::new(Vec::new())),
            back_ups: Arc::new(Mutex::new(HashMap::new())),
            addrs: addrs,
            node_id: node_id,
        };
    }
}

#[tonic::async_trait]
impl JasmineBroker for RpcProcessor {
    async fn hook(
        &self,
        request: tonic::Request<ConnectRequest>,
    ) -> Result<Response<Empty>, Status> {
        let mut temp_client_map = self.client_map.lock().await;
        let address = request.into_inner().address;
        match JasmineClientClient::connect(format!("http://{}", &address)).await {
            Ok(client) => {
                (*temp_client_map).insert(address, client);
                drop(temp_client_map);
                return Ok(Response::new(Empty {}));
            }
            Err(error) => {
                drop(temp_client_map);
                return Err(Status::unknown("error"));
            }
        }
    }

    async fn unhook(
        &self,
        request: tonic::Request<ConnectRequest>,
    ) -> Result<Response<Empty>, Status> {
        let mut temp_client_map = self.client_map.lock().await;
        let address = request.into_inner().address;

        (*temp_client_map).remove(&address);
        drop(temp_client_map);
        return Ok(Response::new(Empty {}));
    }

    async fn publish(
        &self,
        request: tonic::Request<PublishRequest>,
    ) -> Result<Response<Empty>, Status> {
        let mut temp_message_queue = self.message_queue.lock().await;
        let temp_request = request.into_inner().clone();
        let topic = temp_request.topic;
        let message = temp_request.message;
        (*temp_message_queue).push((topic, message));
        drop(temp_message_queue);
        return Ok(Response::new(Empty {}));
    }

    /// TODO:
    /// In the replicated architecture, the subscriber maps on different broker nodes should be consistent.
    /// This can be done with Zookeeper.
    async fn subscribe(
        &self,
        request: tonic::Request<SubscribeRequest>,
    ) -> Result<Response<Empty>, Status> {
        let mut temp_subscriber_map = self.subscriber_map.lock().await;
        let temp_request = request.into_inner().clone();
        let address = temp_request.address;
        let topic = temp_request.topic;

        match (*temp_subscriber_map).get_mut(&topic) {
            Some(set) => {
                set.insert(address.clone());
            }
            None => {
                let mut set = HashSet::new();
                set.insert(address.clone());
                (*temp_subscriber_map).insert(topic.clone(), set);
            }
        }

        drop(temp_subscriber_map);
        // If this node is the leader, copy to back-up nodes
        if find_leader(&topic) == self.addrs[self.node_id] {
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

                    let result = backup
                        .subscribe(SubscribeRequest {
                            address: address.clone(),
                            topic: topic.clone(),
                        })
                        .await;
                    match result {
                        Ok(_) => continue,
                        Err(e) => {
                            continue;
                        }
                    }
                    drop(temp_backups);
                }
            }
        }

        return Ok(Response::new(Empty {}));
    }

    /// TODO: Consistency issue, see "subscribe"
    async fn unsubscribe(
        &self,
        request: tonic::Request<SubscribeRequest>,
    ) -> Result<Response<Empty>, Status> {
        let mut temp_subscriber_map = self.subscriber_map.lock().await;
        let temp_request = request.into_inner().clone();
        let address = temp_request.address;
        let topic = temp_request.topic;

        match (*temp_subscriber_map).get_mut(&topic) {
            Some(set) => {
                set.remove(&address);
            }
            None => {}
        }

        drop(temp_subscriber_map);
        if find_leader(&topic) == self.addrs[self.node_id] {
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

                    let result = backup
                        .unsubscribe(SubscribeRequest {
                            address: address.clone(),
                            topic: topic.clone(),
                        })
                        .await;
                    match result {
                        Ok(_) => continue,
                        Err(e) => {
                            continue;
                        }
                    }
                    drop(temp_backups);
                }
            }
        }

        return Ok(Response::new(Empty {}));
    }

    async fn ping(&self, request: tonic::Request<Empty>) -> Result<Response<Empty>, Status> {
        return Ok(Response::new(Empty {}));
    }

    async fn publish_persistent(
        &self,
        request: tonic::Request<util::rpc::broker::PublishRequest>,
    ) -> Result<tonic::Response<util::rpc::broker::Empty>, tonic::Status> {
        todo!()
    }

    async fn subscribe_persistent(
        &self,
        request: tonic::Request<util::rpc::broker::SubscribeRequest>,
    ) -> Result<tonic::Response<util::rpc::broker::Empty>, tonic::Status> {
        todo!()
    }

    async fn unsubscribe_persistent(
        &self,
        request: tonic::Request<util::rpc::broker::SubscribeRequest>,
    ) -> Result<tonic::Response<util::rpc::broker::Empty>, tonic::Status> {
        todo!()
    }

    async fn hook_persistent(
        &self,
        request: tonic::Request<util::rpc::broker::ConnectRequest>,
    ) -> Result<tonic::Response<util::rpc::broker::Empty>, tonic::Status> {
        todo!()
    }

    async fn unhook_persistent(
        &self,
        request: tonic::Request<util::rpc::broker::ConnectRequest>,
    ) -> Result<tonic::Response<util::rpc::broker::Empty>, tonic::Status> {
        todo!()
    }
}

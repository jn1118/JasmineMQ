use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use tokio::sync::Mutex;
use tonic::async_trait;
use tonic::codegen::http::Request;
use tonic::transport::Channel;
use tonic::{Response, Status};
use util::result::{JasmineError, JasmineResult};
use util::rpc::broker::jasmine_broker_server::{JasmineBroker, JasmineBrokerServer};
use util::rpc::broker::{
    ConnectRequest, Empty, PublishRequest, PublishResponse, SubscribeRequest, SubscribeResponse,
};

use util::rpc::client::jasmine_client_client::JasmineClientClient;

pub struct RpcProcessor {
    pub subscriber_map: Arc<Mutex<HashMap<String, HashSet<String>>>>,
    pub client_map: Arc<Mutex<HashMap<String, JasmineClientClient<Channel>>>>,
    pub message_queue: Arc<Mutex<Vec<(String, String)>>>,
    pub addr: String,
}

impl RpcProcessor {
    pub fn new(addr: String) -> Self {
        return RpcProcessor {
            subscriber_map: Arc::new(Mutex::new(HashMap::new())),
            client_map: Arc::new(Mutex::new(HashMap::new())),
            message_queue: Arc::new(Mutex::new(Vec::new())),
            addr: addr,
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
                set.insert(address);
                drop(temp_subscriber_map);
                return Ok(Response::new(Empty {}));
            }
            None => {
                let mut set = HashSet::new();
                set.insert(address);
                (*temp_subscriber_map).insert(topic, set);
                drop(temp_subscriber_map);
                return Ok(Response::new(Empty {}));
            }
        }
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

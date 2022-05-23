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
    Empty, PublishRequest, PublishResponse, SubscribeRequest, SubscribeResponse,
};

use util::rpc::client::jasmine_client_client::JasmineClientClient;

struct Broker {
    pub subscriber_map: Arc<Mutex<HashMap<String, HashMap<String, JasmineClientClient<Channel>>>>>,
    pub message_queue: Arc<Mutex<Vec<(String, String)>>>,
}

#[tonic::async_trait]
impl JasmineBroker for Broker {
    async fn publish(
        &self,
        request: tonic::Request<PublishRequest>,
    ) -> Result<Response<Empty>, Status> {
        let mut temp_message_queue = self.message_queue.lock().await;
        let topic = request.into_inner().topic;
        let message = request.into_inner().message;
        (*temp_message_queue).push((topic, message));
        drop(temp_message_queue);
        return Ok(Response::new(Empty {}));
    }

    async fn subscribe(
        &self,
        request: tonic::Request<SubscribeRequest>,
    ) -> Result<Response<Empty>, Status> {
        let mut temp_subscriber_map = self.subscriber_map.lock().await;
        let temp_request = request.into_inner().clone();
        let address = temp_request.address;
        let topic = temp_request.topic;

        match (*temp_subscriber_map).get(&topic) {
            Some(mut map) => match map.get(&address) {
                // This client has already subscribed to this topic
                Some(_) => {
                    drop(temp_subscriber_map);
                    return Ok(Response::new(Empty {}));
                }

                // This client has not subscribed to this topic
                None => match JasmineClientClient::connect(format!("http://{}", &address)).await {
                    Ok(client) => {
                        map.insert(address, client);
                        drop(temp_subscriber_map);
                        return Ok(Response::new(Empty {}));
                    }
                    Err(error) => {
                        drop(temp_subscriber_map);
                        return Err(Status::unknown("error"));
                    }
                },
            },
            None => match JasmineClientClient::connect(format!("http://{}", &address)).await {
                Ok(client) => {
                    let mut map = HashMap::new();
                    map.insert(address.clone(), client);
                    (*temp_subscriber_map).insert(topic.to_string(), map);
                    drop(temp_subscriber_map);
                    return Ok(Response::new(Empty {}));
                }
                Err(error) => {
                    drop(temp_subscriber_map);
                    return Err(Status::unknown("error"));
                }
            },
        }

        // match (*temp_subscriber_map).get(&topic) {
        //     Some(set) => match JasmineClientClient::connect(format!("http://{}", &address)).await {
        //         Ok(client) => {
        //             set.insert(client);
        //             drop(temp_subscriber_map);
        //             return Ok(Response::new(Empty {}));
        //         }
        //         Err(error) => {
        //             drop(temp_subscriber_map);
        //             return Err(Response::new(Status::unknown(error)));
        //         }
        //     },
        //     None => {
        //         let set = HashSet::new();
        //         set.insert();
        //         (*temp_subscriber_map).insert(topic);
        //     }
        // }
    }

    async fn unsubscribe(
        &self,
        request: tonic::Request<SubscribeRequest>,
    ) -> Result<Response<Empty>, Status> {
        todo!()
    }

    async fn ping(&self, request: tonic::Request<Empty>) -> Result<Response<Empty>, Status> {
        todo!()
    }
}

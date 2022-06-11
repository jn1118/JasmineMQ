use crate::client::client::Client;
// use crate::client::client::JasmineClient as OtherClient;
use std::collections::HashMap;
use std::net::{SocketAddr, ToSocketAddrs};
use std::sync::Arc;
use tokio::net::TcpListener;
use tokio::sync::Mutex;
use tonic::transport::Server;
use tonic::Response;
use util::rpc::client::jasmine_client_server::JasmineClient;
use util::rpc::client::jasmine_client_server::JasmineClientServer;
use util::rpc::client::{Bool, Empty, Message};
pub struct ClientRpcProcessor {
    // address for rpc client server
    pub addr: String,
    pub message_map: Arc<Mutex<HashMap<(String, bool), Vec<String>>>>,
}

// #[tonic::async_trait]
impl ClientRpcProcessor {
    pub fn new(addr: String) -> Self {
        return ClientRpcProcessor {
            addr: addr,
            message_map: Arc::new(Mutex::new(HashMap::new())),
        };
    }
}
#[tonic::async_trait]
impl JasmineClient for ClientRpcProcessor {
    async fn send_message(
        &self,
        request: tonic::Request<Message>,
    ) -> Result<tonic::Response<Bool>, tonic::Status> {
        dbg!("Send message rpc call back to client");
        eprintln!("Send message rpc call back to client");
        let a = request.into_inner();
        let topic = a.topic;
        let message = a.message;
        let is_consistent = a.is_consistent;
        // dbg!("message topic is: {:?}", topic.clone());
        // dbg!(message.clone());

        let mut temp_message_map = self.message_map.lock().await;
        match (*temp_message_map).get_mut(&(topic.clone(), is_consistent)) {
            Some(array) => {
                array.push(message.clone());
                // (*temp_message_map).insert(topic.clone(), array);
            }
            None => {
                let mut vec_array = Vec::new();
                vec_array.push(message.clone());
                (*temp_message_map).insert((topic, is_consistent), vec_array);
            }
        }
        eprintln!("map: {:?}", temp_message_map);
        // dbg!(temp_message_map.clone());
        drop(temp_message_map);
        return Ok(Response::new(Bool { value: true }));
    }
    async fn ping(
        &self,
        request: tonic::Request<Empty>,
    ) -> Result<tonic::Response<Empty>, tonic::Status> {
        dbg!("ping rpc call back to client");
        return Ok(Response::new(Empty {}));
    }
}

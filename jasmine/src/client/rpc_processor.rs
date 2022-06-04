use crate::client::client::Client;
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
    pub message_buffer: Arc<Mutex<Vec<(String, String, bool)>>>,
}

// #[tonic::async_trait]
impl ClientRpcProcessor {
    pub fn new(addr: String, message_buffer: Arc<Mutex<Vec<(String, String, bool)>>>) -> Self {
        return ClientRpcProcessor {
            addr: addr,
            message_buffer: message_buffer.clone(),
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
        let a = request.into_inner();
        let topic = a.topic;
        let message = a.message;
        let is_consistent = a.is_consistent;
        let mut temp_message_buffer = self.message_buffer.lock().await;
        (*temp_message_buffer).push((topic, message, is_consistent));
        drop(temp_message_buffer);
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

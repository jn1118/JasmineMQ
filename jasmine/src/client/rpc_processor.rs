use crate::client::client::Client;
use std::collections::HashMap;
use std::net::{SocketAddr, ToSocketAddrs};
use std::sync::Arc;
use tokio::net::TcpListener;
use tokio::sync::Mutex;
use tonic::codegen::http::request;
use tonic::transport::Server;
use tonic::Response;
use util::rpc::client::jasmine_client_server::JasmineClient;
use util::rpc::client::jasmine_client_server::JasmineClientServer;
use util::rpc::client::{Bool, Empty, Message};
pub struct ClientRpcProcessor {
    // address for rpc client server
    pub addr: String,
    pub message_map: Arc<Mutex<HashMap<(String, bool), Vec<String>>>>,

    pub buffer: Arc<Mutex<Vec<(String, String)>>>,
}

// #[tonic::async_trait]
impl ClientRpcProcessor {
    pub fn new(addr: String, buffer: Arc<Mutex<Vec<(String, String)>>>) -> Self {
        return ClientRpcProcessor {
            addr: addr,
            message_map: Arc::new(Mutex::new(HashMap::new())),
            buffer: buffer.clone(),
        };
    }
}
#[tonic::async_trait]
impl JasmineClient for ClientRpcProcessor {
    async fn send_message(
        &self,
        request: tonic::Request<Message>,
    ) -> Result<tonic::Response<Bool>, tonic::Status> {
        // Jefferson's implementation starts
        let m = request.into_inner();
        let topic = m.topic;
        let message = m.message;

        let mut temp_buffer = self.buffer.lock().await;

        (*temp_buffer).push((topic, message));
        drop(temp_buffer);

        return Ok(Response::new(Bool { value: true }));

        // Jefferson's implementation ends
    }
    async fn ping(
        &self,
        request: tonic::Request<Empty>,
    ) -> Result<tonic::Response<Empty>, tonic::Status> {
        dbg!("ping rpc call back to client");
        return Ok(Response::new(Empty {}));
    }
}

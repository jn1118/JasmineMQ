use crate::client::client::Client;
use crate::client::client::JasmineClient as OtherClient;
use std::net::{SocketAddr, ToSocketAddrs};
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
}

// #[tonic::async_trait]
// impl ClientRpcProcessor {
//     pub fn new(addr: String) -> Self {
//         return ClientRpcProcessor { addr };
//     }
// }
#[tonic::async_trait]
impl JasmineClient for ClientRpcProcessor {
    async fn send_message(
        &self,
        request: tonic::Request<Message>,
    ) -> Result<tonic::Response<Bool>, tonic::Status> {
        println!("Send message rpc call back to client");
        let a = request.into_inner();
        let topic = a.topic;
        let message = a.message;
        println!("message topic is: {:?}", topic);
        println!("message body is: {:?}", message);
        return Ok(Response::new(Bool { value: true }));
    }
    async fn ping(
        &self,
        request: tonic::Request<Empty>,
    ) -> Result<tonic::Response<Empty>, tonic::Status> {
        println!("ping rpc call back to client");
        return Ok(Response::new(Empty {}));
    }
}

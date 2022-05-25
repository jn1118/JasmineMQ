// use util::rpc::client::jasmine_client_server::JasmineClient;
use super::subscriber::JasmineSubscriber;
use crate::client::client::Client;
use crate::client::client::JasmineClient;
use crate::client::publisher::JasminePublisher;
use crate::client::rpc_processor::ClientRpcProcessor;
use async_trait::async_trait;
use std::net::{SocketAddr, ToSocketAddrs};
use tokio::net::TcpListener;
use tokio::sync::Mutex;
use tonic::transport::Server;
use util::result::JasmineResult;
use util::rpc::client::jasmine_client_server::JasmineClient as JasmineRpcClient;
use util::rpc::client::jasmine_client_server::JasmineClientServer;
// use util::rpc::client::jasmine_client_server::Jas;
use util::transaction::JasmineMessage;

pub struct JasmineClientWrapper {
    pub client: Client,
    pub broker_addr: Vec<String>,
    pub jasmine_rpc_client: ClientRpcProcessor,
}

#[async_trait]
impl JasmineClient for JasmineClientWrapper {
    fn new(broker: Vec<String>) -> Self {
        let new_client = Client::new(broker);
        let new_rpc_client = ClientRpcProcessor::new();
        return JasmineClientWrapper {
            client: new_client,
            broker_addr: broker,
            jasmine_rpc_client: new_rpc_client,
        };
    }
    async fn connect(&self) -> JasmineResult<()> {
        let result = self.client.connect().await;
        return result;
    }
    async fn disconnect(&self) -> JasmineResult<()> {
        let result = self.client.disconnect().await;
        return result;
    }
    fn on_message(&self) -> JasmineMessage {
        todo!()
    }
}

#[async_trait]
impl JasmineSubscriber for JasmineClientWrapper {
    async fn subscribe(&self, topic: String) -> JasmineResult<()> {
        let result = self.client.subscribe(topic).await;
        return result;
    }

    async fn unsubscribe(&self, topic: String) -> JasmineResult<()> {
        let result = self.client.unsubscribe(topic).await;
        return result;
    }
}

#[async_trait]
impl JasminePublisher for JasmineClientWrapper {
    async fn publish(&self, topic: String, message: String) -> JasmineResult<()> {
        let result = self.client.publish(topic, message).await;
        return result;
    }
}

// takes in a list of broker addresses and then initialize a wrapper and then start the rpc.
async fn initialization(broker: Vec<String>) -> JasmineResult<()> {
    let client_wrapper = JasmineClientWrapper::new(broker);
    let addr = "127.0.0.1:7799";
    let addr = match addr.to_socket_addrs()?.next() {
        Some(addr) => addr,
        None => SocketAddr::from(([127, 0, 0, 1], 3000)),
    };
    let jasmine_client_service = JasmineClientServer::new(client_wrapper.jasmine_rpc_client);
    Server::builder()
        .add_service(jasmine_client_service)
        .serve(addr)
        .await?;
    return Ok(());
}

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
    // pub jasmine_rpc_client: ClientRpcProcessor,
}

#[async_trait]
impl JasmineClient for JasmineClientWrapper {
    // fn new(broker: Vec<String>) -> Self {
    //     let new_client = Client::new(broker);
    //     let new_rpc_client = ClientRpcProcessor::new();
    //     return JasmineClientWrapper {
    //         client: new_client,
    //         broker_addr: broker,
    //         jasmine_rpc_client: new_rpc_client,
    //     };
    // }
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

pub async fn start_rpc_client_server(rpc_server_addr: String) -> JasmineResult<()> {
    let new_rpc_client = ClientRpcProcessor {
        addr: rpc_server_addr.clone(),
    };
    dbg!("001");
    let temp_addr = match rpc_server_addr.to_socket_addrs() {
        Ok(mut addr) => addr.next(),
        Err(e) => return Err(Box::new(e)),
    };
    dbg!("002");
    let (mut sender, mut receiver) = tokio::sync::mpsc::channel::<()>(1);
    Server::builder()
        .add_service(JasmineClientServer::new(new_rpc_client))
        .serve_with_shutdown(temp_addr.unwrap(), async {
            dbg!("003");
            receiver.recv().await;
        })
        .await?;
    dbg!("004");
    // let new_rpc_client = ClientRpcProcessor {
    //     addr: rpc_server_addr.clone(),
    // };
    // let addr = match rpc_server_addr.to_socket_addrs()?.next() {
    //     Some(addr) => addr,
    //     None => SocketAddr::from(([127, 0, 0, 1], 3000)),
    // };
    // let jasmine_rpc_server = JasmineClientServer::new(new_rpc_client);
    // Server::builder()
    //     .add_service(jasmine_rpc_server)
    //     .serve(addr)
    //     .await?;
    return Ok(());
}

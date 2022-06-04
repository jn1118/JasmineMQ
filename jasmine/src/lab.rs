// use crate::broker::server::JasmineBroker;
use crate::{
    broker::broker::Broker,
    client::{client::Client, rpc_processor::ClientRpcProcessor},
    storage::storage::Storage,
};
use std::{
    collections::{HashMap, HashSet},
    net::ToSocketAddrs,
    sync::Arc,
};
// use crate::client::client::JasmineBroker;
use tokio::sync::Mutex;
use util::{
    result::JasmineResult,
    rpc::{
        broker::jasmine_broker_server::JasmineBroker,
        client::jasmine_client_server::JasmineClientServer,
    },
};

use tonic::transport::Server;
// it will return a client wrapper, which user can call publish, subscribe, unsubscribe, connect directly.
pub fn initialize_front_end(
    broker: Vec<String>,
    client_address: String,
    message_map: Arc<Mutex<HashMap<(String, bool), Vec<String>>>>,
) -> () {
    let new_client = Client::new(broker.clone(), client_address.clone(), message_map);

    return Ok(Box::new(new_client));
}

pub async fn initialize_broker(addresses: Vec<String>, node_id: usize) -> JasmineResult<()> {
    eprintln!("in initialize broker");
    let (sender, receiver) = tokio::sync::mpsc::channel(1);
    Broker::new(addresses, node_id, receiver).await;
    return Ok(());
}

pub async fn start_rpc_client_server(
    rpc_server_addr: String,
    new_rpc_client: ClientRpcProcessor,
) -> JasmineResult<()> {
    let temp_addr = match rpc_server_addr.to_socket_addrs() {
        Ok(mut addr) => addr.next(),
        Err(e) => return Err(Box::new(e)),
    };

    tokio::spawn(async move { loop {} });
    let (mut sender, mut receiver) = tokio::sync::mpsc::channel::<()>(1);
    Server::builder()
        .add_service(JasmineClientServer::new(new_rpc_client))
        .serve_with_shutdown(temp_addr.unwrap(), async {
            receiver.recv().await;
        })
        .await?;
    return Ok(());
}

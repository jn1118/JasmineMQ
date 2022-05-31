// use crate::broker::server::JasmineBroker;
use crate::{
    broker::broker::Broker,
    client::{client::Client, rpc_processor::ClientRpcProcessor},
    storage::storage::Storage,
};
use std::{
    collections::{HashMap, HashSet},
    net::ToSocketAddrs,
};
// use crate::client::client::JasmineBroker;
use crate::client::client::JasmineClient;
use util::{
    result::JasmineResult,
    rpc::{
        broker::jasmine_broker_server::JasmineBroker,
        client::jasmine_client_server::JasmineClientServer,
    },
};

use tonic::transport::Server;
// it will return a client wrapper, which user can call publish, subscribe, unsubscribe, connect directly.
pub async fn initialize_front_end(
    broker: Vec<String>,
    client_address: String,
) -> JasmineResult<(Box<dyn JasmineClient>)> {
    let new_client = Client {
        client_addr: client_address.clone(),
        broker_addr: broker.clone(),
    };
    dbg!("01");
    start_rpc_client_server(client_address).await?;
    dbg!("03");
    return Ok(Box::new(new_client));
}

pub async fn initialize_broker(addresses: Vec<String>) -> JasmineResult<()> {
    Broker::new(addresses, 0).await;
    return Ok(());
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
    return Ok(());
}

// use crate::broker::server::JasmineBroker;
use crate::{
    broker::broker::Broker,
    client::{
        client::Client,
        client_wrapper::{start_rpc_client_server, JasmineClientWrapper},
        rpc_processor::ClientRpcProcessor,
    },
    storage::storage::Storage,
};
use std::collections::{HashMap, HashSet};
// use crate::client::client::JasmineBroker;
use crate::client::client::JasmineClient;
use util::{result::JasmineResult, rpc::broker::jasmine_broker_server::JasmineBroker};

// it will return a client wrapper, which user can call publish, subscribe, unsubscribe, connect directly.
pub async fn initialize_front_end(
    broker: Vec<String>,
    client_address: String,
) -> JasmineResult<(Box<JasmineClientWrapper>)> {
    let new_client = Client {
        client_addr: client_address.clone(),
        broker_addr: broker.clone(),
    };
    dbg!("01");
    start_rpc_client_server(client_address).await?;
    dbg!("02");
    let client_wrapper = JasmineClientWrapper { client: new_client };
    dbg!("03");
    return Ok(Box::new(client_wrapper));
}

pub async fn initialize_broker(addresses: Vec<String>) -> JasmineResult<()> {
    Broker::new(addresses).await;
    return Ok(());
}

// async fn initialize_storage() -> JasmineResult<(Box<Storage>)> {
//     let storage = Storage::new();
//     return Ok(Box::new(storage));
// }

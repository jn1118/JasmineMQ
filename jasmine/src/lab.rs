// use crate::broker::server::JasmineBroker;
use crate::client::{
    client::Client,
    client_wrapper::{start_rpc_client_server, JasmineClientWrapper},
    rpc_processor::ClientRpcProcessor,
};
use std::collections::{HashMap, HashSet};
// use crate::client::client::JasmineBroker;
use crate::client::client::JasmineClient;
use util::result::JasmineResult;

// #[allow(unused_variables)]
// pub async fn new_front_client(
//     broker_addr: Vec<String>,
// ) -> JasmineResult<Box<dyn JasmineClient + Send + Sync>> {
//     Ok(Box::new(Client {
//         // client_id: 0,
//         // client_map: HashMap::new(),
//         broker_addr: broker_addr,
//     }))
// }

async fn initialization(
    broker: Vec<String>,
    rpc_server_address: String,
) -> JasmineResult<(Box<JasmineClientWrapper>)> {
    let new_client = Client {
        broker_addr: broker.clone(),
    };
    start_rpc_client_server(rpc_server_address).await?;
    let client_wrapper = JasmineClientWrapper { client: new_client };
    return Ok(Box::new(client_wrapper));
}

use crate::broker::server::JasmineBroker;
use crate::client::client::Client;
use std::collections::{HashMap, HashSet};
// use crate::client::client::JasmineBroker;
use crate::client::client::JasmineClient;
use util::result::JasmineResult;

#[allow(unused_variables)]
pub async fn new_front_client(
    broker: Box<dyn JasmineBroker>,
) -> JasmineResult<Box<dyn JasmineClient + Send + Sync>> {
    Ok(Box::new(Client {
        client_id: 0,
        client_map: HashMap::new(),
        broker: broker,
    }))
}

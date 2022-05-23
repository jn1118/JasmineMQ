// use async_trait::async_trait;
// use std::{
//     collections::{HashMap, HashSet},
//     sync::Arc,
// };
// use tokio::sync::Mutex;
// use tonic::transport::Channel;
// use util::{
//     result::{JasmineError, JasmineResult},
//     rpc::{
//         broker,
//         client::{self, jasmine_client_client::JasmineClientClient},
//         storage,
//     },
//     transaction::JasmineMessage,
// };

// pub struct Manager {
//     pub subscriber_map: Arc<Mutex<HashMap<String, HashSet<JasmineClientClient<Channel>>>>>,
//     pub message_queue: Arc<Mutex<Vec<(String, String)>>>,
// }

// impl Manager {
//     async fn on_pub_message(&self, topic: String, message: JasmineMessage) -> JasmineResult<u64> {
//         let subscriber_set = match self.subscriber_map.get(&topic) {
//             Some(set) => set,
//             None => {
//                 return Ok(0);
//             }
//         };

//         for subscriber in subscriber_set.iter() {
//             todo!()
//         }

//         match subscriber_set.len().try_into() {
//             Ok(size) => {
//                 return Ok(size);
//             }
//             Err(error) => {
//                 return Err(Box::new(error));
//             }
//         };
//     }
// }

// use std::{
//     process,
//     sync::{
//         mpsc::{self, Sender},
//         Arc,
//     },
//     thread,
//     time::Duration,
// };

// use jasmine::client::client::JasmineClient;
// use jasmine::client::{client::Client, client_wrapper::JasmineClientWrapper};
// use util::result::JasmineResult;

// // const CLIENT: String = "127.0.0.1:30000".to_string();
// // let brokers = Vec::new();
// // brokers.push("127.0.0.1:30001");
// // let mut stack = Vec::new();

// // stack.push(1);
// // const BROKER: Vec<&str> = ["127.0.0.1:30001"];

// async fn setup() -> JasmineResult<(Box<dyn JasmineClient>)> {
//     let client_address = "127.0.0.1:30000".to_string();
//     let mut brokers = Vec::new();
//     brokers.push("127.0.0.1:30001".to_string());
//     dbg!("1");
//     spawn_broker(brokers.clone()).await?;
//     dbg!("0");
//     let client = jasmine::lab::initialize_front_end(brokers.clone(), client_address).await?;

//     dbg!("2");
//     return Ok((client));
// }

// async fn spawn_broker(brokers: Vec<String>) -> JasmineResult<()> {
//     jasmine::lab::initialize_broker(brokers).await?;
//     return Ok(());
// }

// #[tokio::test(flavor = "multi_thread", worker_threads = 1)]
// #[allow(unused_must_use)]
// async fn single_machine_test1() -> JasmineResult<()> {
//     dbg!("hihihi1");
//     let client = setup().await?;
//     dbg!("hihihi2");
//     let a = client.connect().await?;
//     dbg!("yoyoyoyo");
//     assert_eq!((), a);
//     // assert_eq!(None, client.disconnect()).await?);
//     // assert!(a.is_err());
//     Ok(())
//     // return Ok(());
// }

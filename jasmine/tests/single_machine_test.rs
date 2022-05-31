// use std::{
//     process,
//     sync::{
//         mpsc::{self, Sender},
//         Arc,
//     },
//     thread,
//     time::Duration,
// };
use jasmine::client::client::Client;
use jasmine::client::client::JasmineClient;
use tokio::task::JoinHandle;
use tokio::time::Duration;
use util::{
    config::{BROKER_ADDRS, BROKER_COUNT, CLIENT_ADDRS},
    result::JasmineResult,
};

async fn setup() -> JasmineResult<(
    Box<dyn JasmineClient>,
    Vec<JoinHandle<JasmineResult<()>>>,
    JoinHandle<JasmineResult<()>>,
)> {
    // let client_address = "127.0.0.1:30000".to_string();
    let mut brokers = Vec::new();
    for i in BROKER_ADDRS {
        brokers.push(i.to_string())
    }

    // brokers.push("127.0.0.1:30001".to_string());
    let broker_handles = spawn_broker(brokers.clone());
    let client_rpc_handle = spawn_client_rpc_server(CLIENT_ADDRS[0].to_string());
    let client = jasmine::lab::initialize_front_end(brokers, CLIENT_ADDRS[0].to_string()).await?;
    return Ok((client, broker_handles, client_rpc_handle));
}

fn spawn_broker(brokers: Vec<String>) -> Vec<tokio::task::JoinHandle<JasmineResult<()>>> {
    let mut handles = vec![];
    for i in 0..BROKER_COUNT {
        let l = tokio::spawn(jasmine::lab::initialize_broker(brokers.clone(), i));
        handles.push(l);
    }
    return handles;
}

fn spawn_client_rpc_server(rpc_server_addr: String) -> tokio::task::JoinHandle<JasmineResult<()>> {
    tokio::spawn(jasmine::lab::start_rpc_client_server(rpc_server_addr))
}

#[tokio::test(flavor = "multi_thread", worker_threads = 1)]
#[allow(unused_must_use)]
async fn single_client_unit_test() -> JasmineResult<()> {
    // dbg!("hihihi1");
    let (client, broker_handle, rpc_client_handle) = match setup().await {
        Ok(value) => value,
        Err(e) => {
            return Err(e);
        }
    };

    tokio::time::sleep(Duration::from_secs(5)).await;
    // let connect = client.connect().await?;
    let topic = "CSE223".to_string();
    let message = "Final project done.".to_string();
    // dbg!("hihihi3");
    let sub_result = client.subscribe(topic.clone()).await?;
    dbg!("hihihi4");
    let pub_result = client.publish(topic, message).await?;
    // let disconnect = client.disconnect().await?;
    dbg!("yoyoyoyo");

    tokio::time::sleep(Duration::from_secs(20)).await;
    // assert_eq!((), a);
    Ok(())
}

async fn multiple_client_unit_test() -> JasmineResult<()> {
    // dbg!("hihihi1");
    let (client, broker_handle, rpc_client_handle) = match setup().await {
        Ok(value) => value,
        Err(e) => {
            return Err(e);
        }
    };

    tokio::time::sleep(Duration::from_secs(5)).await;
    // let connect = client.connect().await?;
    let topic = "CSE223".to_string();
    let message = "Final project done.".to_string();
    // dbg!("hihihi3");
    let sub_result = client.subscribe(topic.clone()).await?;
    dbg!("hihihi4");
    let pub_result = client.publish(topic, message).await?;
    // let disconnect = client.disconnect().await?;
    dbg!("yoyoyoyo");
    // assert_eq!((), a);
    Ok(())
}

fn generate_client_address(num: usize) -> Vec<String> {
    let mut address = Vec::new();
    for i in 30000..(30000 + num) {
        let addr = "127.0.0.1:".to_string() + &i.to_string();
        address.push(addr)
    }
    return address;
}

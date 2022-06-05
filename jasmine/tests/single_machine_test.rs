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
use jasmine::client::rpc_processor::ClientRpcProcessor;
use tokio::sync::mpsc::Receiver;
use tokio::task::JoinHandle;
use tokio::time::Duration;
use util::result::JasmineError;
use util::{
    config::{BROKER_ADDRS, BROKER_COUNT, CLIENT_ADDRS},
    result::JasmineResult,
};

async fn setup(
    client_num: usize,
) -> JasmineResult<(
    Vec<Box<dyn JasmineClient>>,
    Vec<JoinHandle<JasmineResult<()>>>,
    Vec<JoinHandle<JasmineResult<()>>>,
    Vec<Sender<()>>,
)> {
    let mut brokers = Vec::new();
    for i in BROKER_ADDRS {
        brokers.push(i.to_string())
    }

    let client_addrs = generate_client_address(client_num);
    let (broker_handles, broker_shutdown) = spawn_broker(brokers.clone());
    let mut handles = vec![];
    let mut clients = vec![];
    for c_addr in client_addrs {
        let new_rpc_client = ClientRpcProcessor::new(c_addr.clone());

        let client = jasmine::lab::initialize_front_end(
            brokers.clone(),
            c_addr.clone().to_string(),
            new_rpc_client.message_map.clone(),
        )
        .unwrap();

        let client_rpc_handle = spawn_client_rpc_server(0 new_rpc_client);
        handles.push(client_rpc_handle);
        clients.push(client)
    }

    return Ok((clients, handles, broker_handles, broker_shutdown));
}

fn spawn_broker(
    brokers: Vec<String>,
) -> (
    Vec<tokio::task::JoinHandle<JasmineResult<()>>>,
    Vec<Sender<()>>,
) {
    let mut handles = vec![];
    let mut brokers_shutdown = vec![];

    for i in 0..BROKER_COUNT {
        let (shut_tx, shut_rx) = tokio::sync::mpsc::channel(1);
        let l = tokio::spawn(jasmine::lab::initialize_broker(brokers.clone(), i, shut_rx));
        brokers_shutdown.push(shut_tx);
        handles.push(l);
    }
    return (handles, brokers_shutdown);
}

fn spawn_client_rpc_server(
    rpc_server_addr: String,
    new_rpc_client: ClientRpcProcessor,
) -> tokio::task::JoinHandle<JasmineResult<()>> {
    let handle = tokio::spawn(jasmine::lab::start_rpc_client_server(
        rpc_server_addr,
        new_rpc_client,
    ));
    return handle;
}

#[tokio::test(flavor = "multi_thread", worker_threads = 1)]
#[allow(unused_must_use)]
async fn single_client_no_consistent() -> JasmineResult<()> {
    let (client, rpc_client_handle, broker_handle, broker_shut_down) = match setup(1).await {
        Ok(value) => value,
        Err(e) => {
            return Err(e);
        }
    };
    tokio::time::sleep(Duration::from_secs(5)).await;
    let topic = "CSE223".to_string();
    let message = "Final project done.".to_string();
    let is_consistent = false;
    client[0].subscribe(topic.clone()).await?;
    client[0]
        .publish(topic.clone(), message.clone(), is_consistent)
        .await?;
    tokio::time::sleep(Duration::from_secs(20)).await;
    let result = client[0]
        .on_message(topic.clone().to_string(), is_consistent)
        .await;
    // dbg!("yoyoyoyoy");
    dbg!(result.clone());
    let mut expected_result = Vec::new();
    expected_result.push(message);
    assert_eq!(expected_result, result);
    Ok(())
}

#[tokio::test(flavor = "multi_thread", worker_threads = 1)]
#[allow(unused_must_use)]
async fn single_client_consistent() -> JasmineResult<()> {
    let (client, rpc_client_handle, broker_handle, broker_shut_down) = match setup(1).await {
        Ok(value) => value,
        Err(e) => {
            return Err(e);
        }
    };
    tokio::time::sleep(Duration::from_secs(5)).await;
    let topic = "CSE222".to_string();
    let message = "Final project done.".to_string();
    let is_consistent = true;
    client[0].subscribe(topic.clone()).await?;
    client[0]
        .publish(topic.clone(), message.clone(), is_consistent)
        .await?;
    tokio::time::sleep(Duration::from_secs(20)).await;
    let result = client[0]
        .on_message(topic.clone().to_string(), is_consistent)
        .await;
    // dbg!("yoyoyoyoy");
    dbg!(result.clone());
    let mut expected_result = Vec::new();
    expected_result.push(message);
    assert_eq!(expected_result, result);
    Ok(())
}

#[tokio::test(flavor = "multi_thread", worker_threads = 1)]
#[allow(unused_must_use)]
async fn single_client_unsubscribe() -> JasmineResult<()> {
    // dbg!("hihihi1");
    let (client, rpc_client_handle, broker_handle, broker_shut_down) = match setup(1).await {
        Ok(value) => value,
        Err(e) => {
            return Err(e);
        }
    };

    tokio::time::sleep(Duration::from_secs(5)).await;
    let topic = "CSE223".to_string();
    let message = "Final project done.".to_string();
    let is_consistent = true;
    client[0].subscribe(topic.clone()).await?;
    client[0].unsubscribe(topic.clone()).await?;
    client[0].subscribe(topic.clone()).await?;
    client[0].unsubscribe(topic.clone()).await?;
    client[0].subscribe(topic.clone()).await?;
    client[0].unsubscribe(topic.clone()).await?;
    let pub_result = client[0]
        .publish(topic.clone(), message, is_consistent)
        .await?;
    tokio::time::sleep(Duration::from_secs(20)).await;
    let result = client[0]
        .on_message(topic.clone().to_string(), is_consistent)
        .await;
    let expected_result: Vec<String> = Vec::new();
    assert_eq!(expected_result, result);
    Ok(())
}

#[tokio::test(flavor = "multi_thread", worker_threads = 1)]
#[allow(unused_must_use)]
async fn single_client_both() -> JasmineResult<()> {
    // dbg!("hihihi1");
    let (client, rpc_client_handle, broker_handle, broker_shut_down) = match setup(2).await {
        Ok(value) => value,
        Err(e) => {
            return Err(e);
        }
    };

    tokio::time::sleep(Duration::from_secs(5)).await;
    let topics = ["1"];
    let messages = ["a", "b", "c", "d"];
    let is_consistent = [false, true];

    client[0].subscribe(topics.clone()[0].to_string()).await?;
    for i in 0..20 {
        for m1 in messages {
            client[1]
                .publish(
                    topics.clone()[0].to_string(),
                    m1.to_string(),
                    is_consistent.clone()[0],
                )
                .await?;
            client[1]
                .publish(
                    topics.clone()[0].to_string(),
                    m1.to_string(),
                    is_consistent.clone()[1],
                )
                .await?;
        }
    }

    tokio::time::sleep(Duration::from_secs(20)).await;
    let result1 = client[0]
        .on_message(topics.clone()[0].to_string(), is_consistent.clone()[0])
        .await;
    let result2 = client[0]
        .on_message(topics.clone()[0].to_string(), is_consistent.clone()[1])
        .await;
    assert_eq!(result1, result2);
    Ok(())
}

#[tokio::test(flavor = "multi_thread", worker_threads = 1)]
#[allow(unused_must_use)]
async fn multiple_client_unit_test() -> JasmineResult<()> {
    let (client, rpc_client_handle, broker_handle, broker_shut_down) = match setup(4).await {
        Ok(value) => value,
        Err(e) => {
            return Err(e);
        }
    };
    tokio::time::sleep(Duration::from_secs(5)).await;
    let topics = ["1", "2", "3", "4"];
    let messages1 = ["a", "b", "c", "d"];
    let messages2 = ["e", "f", "g"];

    let is_consistent = [false, true];
    client[0].subscribe(topics.clone()[0].to_string()).await?;
    client[0].subscribe(topics.clone()[1].to_string()).await?;
    client[0].subscribe(topics.clone()[2].to_string()).await?;
    client[1].subscribe(topics.clone()[0].to_string()).await?;
    client[1].subscribe(topics.clone()[1].to_string()).await?;

    for m1 in messages1 {
        client[2]
            .publish(
                topics.clone()[0].to_string(),
                m1.clone().to_string(),
                is_consistent.clone()[0],
            )
            .await?;
        client[2]
            .publish(
                topics.clone()[0].to_string(),
                m1.clone().to_string(),
                is_consistent.clone()[1],
            )
            .await?;
        client[2]
            .publish(
                topics.clone()[2].to_string(),
                m1.clone().to_string(),
                is_consistent.clone()[1],
            )
            .await?;
    }

    for m2 in messages2 {
        client[3]
            .publish(
                topics.clone()[0].to_string(),
                m2.to_string(),
                is_consistent[0],
            )
            .await?;
        client[3]
            .publish(
                topics.clone()[1].to_string(),
                m2.to_string(),
                is_consistent[1],
            )
            .await?;
    }

    tokio::time::sleep(Duration::from_secs(20)).await;
    let a = client[0]
        .on_message(topics.clone()[0].to_string(), is_consistent[0])
        .await;
    let expected_message_0_f = ["a", "b", "c", "d", "e", "f", "g"].to_vec();
    assert_eq!(expected_message_0_f, a);

    let a = client[0]
        .on_message(topics.clone()[0].to_string(), is_consistent[1])
        .await;
    let expected_message_0_t = ["a", "b", "c", "d"].to_vec();
    assert_eq!(expected_message_0_t, a);

    let a = client[0]
        .on_message(topics.clone()[1].to_string(), is_consistent[1])
        .await;
    let expected_message_0_t = ["e", "f", "g"].to_vec();
    assert_eq!(expected_message_0_t, a);

    let a = client[0]
        .on_message(topics.clone()[1].to_string(), is_consistent[0])
        .await;
    let expected_message_0_t: Vec<String> = [].to_vec();
    assert_eq!(expected_message_0_t, a);

    let a = client[0]
        .on_message(topics.clone()[2].to_string(), is_consistent[1])
        .await;
    let expected_message_0_t = ["a", "b", "c", "d"].to_vec();
    assert_eq!(expected_message_0_t, a);

    let a = client[1]
        .on_message(topics.clone()[0].to_string(), is_consistent[0])
        .await;
    let expected_message_0_t = ["a", "b", "c", "d", "e", "f", "g"].to_vec();
    assert_eq!(expected_message_0_t, a);

    let a = client[1]
        .on_message(topics.clone()[1].to_string(), is_consistent[1])
        .await;
    let expected_message_0_t = ["e", "f", "g"].to_vec();
    assert_eq!(expected_message_0_t, a);

    let a = client[0]
        .on_message(topics.clone()[1].to_string(), is_consistent[1])
        .await;
    let expected_message_0_t = ["e", "f", "g"].to_vec();
    assert_eq!(expected_message_0_t, a);

    let a = client[0]
        .on_message(topics.clone()[1].to_string(), is_consistent[0])
        .await;
    let expected_message_0_t: Vec<String> = [].to_vec();
    assert_eq!(expected_message_0_t, a);

    let a = client[1]
        .on_message(topics.clone()[2].to_string(), is_consistent[1])
        .await;
    let expected_message_0_t: Vec<String> = [].to_vec();
    assert_eq!(expected_message_0_t, a);

    Ok(())
}

async fn single_client_no_consistent_shutdown() -> JasmineResult<()> {
    let (client, rpc_client_handle, broker_handle, broker_shut_down) = match setup(1).await {
        Ok(value) => value,
        Err(e) => {
            return Err(e);
        }
    };
    tokio::time::sleep(Duration::from_secs(5)).await;
    backs_shutdowns[3].send(()).await;
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

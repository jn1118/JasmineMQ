use std::{
    collections::{HashMap, HashSet},
    net::ToSocketAddrs,
    sync::Arc,
};

use serde::de::Error;
use tokio::{spawn, sync::Mutex, task::JoinHandle};
use tonic::transport::{Channel, Server};
use util::{
    result::{JasmineError, JasmineResult},
    rpc::{
        broker::jasmine_broker_server::{JasmineBroker, JasmineBrokerServer},
        client::jasmine_client_client::JasmineClientClient,
    },
};

use super::{manager::Manager, rpc_processor::RpcProcessor};

pub struct Broker {
    addrs: Vec<String>,
    managers: Vec<Manager>,
}

fn start_manager(
    subscriber_map: Arc<Mutex<HashMap<String, HashSet<String>>>>,
    client_map: Arc<Mutex<HashMap<String, JasmineClientClient<Channel>>>>,
    message_queue: Arc<Mutex<Vec<(String, String)>>>,
    addr: String,
) -> Manager {
    return Manager::new(
        subscriber_map.clone(),
        client_map.clone(),
        message_queue.clone(),
        addr,
    );
}

fn start_rpc_processor(addr: String) -> RpcProcessor {
    return RpcProcessor::new(addr);
}

impl Broker {
    pub async fn new(addrs: Vec<String>) -> JasmineResult<()> {
        let mut managers: Vec<Manager> = Vec::new();
        let mut processors: Vec<RpcProcessor> = Vec::new();
        let temp_addrs = addrs.clone();
        for addr in temp_addrs {
            let processor = start_rpc_processor(addr.clone());
            let manager = start_manager(
                processor.subscriber_map,
                processor.client_map,
                processor.message_queue,
                addr.clone(),
            );
            managers.push(manager);
        }

        for manager in managers {
            let temp_manager_process_message_queue = Arc::new(Mutex::new(manager));
            let handle = tokio::spawn(async move {
                loop {
                    let manager = temp_manager_process_message_queue.lock().await;
                    manager.process_message_queue().await;
                    drop(manager);
                }
            });
        }
        dbg!("i am inside broker new method");
        dbg!(processors.len());
        for processor in processors {
            let temp_addr = match processor.addr.clone().to_socket_addrs() {
                Ok(mut addr) => addr.next(),
                Err(_) => {
                    dbg!("dddddd");
                    continue;
                }
            };
            dbg!("ppppppp");
            let (mut sender, mut receiver) = tokio::sync::mpsc::channel::<()>(1);
            Server::builder()
                .add_service(JasmineBrokerServer::new(processor))
                .serve_with_shutdown(temp_addr.unwrap(), async {
                    dbg!("0000000");
                    receiver.recv().await;
                })
                .await?;
        }

        return Ok(());
    }
}

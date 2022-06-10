use std::{
    collections::{HashMap, HashSet, VecDeque},
    net::ToSocketAddrs,
    sync::Arc,
    time::Duration,
};
use tokio::sync::mpsc::Receiver;
use tokio::sync::Mutex;
use tonic::transport::{Channel, Server};
use util::{
    leader_util::LoggingWatcher,
    result::JasmineResult,
    rpc::{
        broker::{
            jasmine_broker_client::JasmineBrokerClient, jasmine_broker_server::JasmineBrokerServer,
        },
        client::jasmine_client_client::JasmineClientClient,
    },
    transaction::JasmineLog,
};

use super::{
    manager::{self, Manager},
    rpc_processor::RpcProcessor,
};
use zookeeper::{Acl, CreateMode, ZooKeeper};
pub struct Broker {
    addrs: Vec<String>,
    node_id: usize,
    processor: RpcProcessor,
    manager: Manager,
}

fn start_manager(
    subscriber_map: Arc<Mutex<HashMap<String, HashSet<String>>>>,
    client_map: Arc<Mutex<HashMap<String, JasmineClientClient<Channel>>>>,
    message_queue: Arc<Mutex<Vec<(String, String, bool)>>>,
    back_ups: Arc<Mutex<HashMap<String, JasmineBrokerClient<Channel>>>>,
    addrs: Vec<String>,
    node_id: usize,
    logs: Arc<Mutex<HashMap<String, VecDeque<JasmineLog>>>>,
) -> Manager {
    return Manager::new(
        subscriber_map,
        client_map,
        message_queue,
        back_ups,
        addrs,
        node_id,
        logs,
    );
}

fn start_rpc_processor(addrs: Vec<String>, node_id: usize) -> RpcProcessor {
    return RpcProcessor::new(addrs, node_id);
}

impl Broker {
    pub async fn new(
        addrs: Vec<String>,
        node_id: usize,
        shut_down_signal: Option<Receiver<()>>,
    ) -> JasmineResult<()> {
        let zk_urls = "164.92.70.147:2181".to_string();
        let zk = ZooKeeper::connect(&*zk_urls, Duration::from_secs(15), LoggingWatcher).unwrap();
        let path = format!("{}{}", "/brokers/", node_id);
        zk.create(
            &path,
            Vec::new(),
            Acl::open_unsafe().clone(),
            CreateMode::Ephemeral,
        )?;

        let temp_addrs = addrs.clone();
        let addr = &addrs[node_id];

        let processor = start_rpc_processor(addrs.clone(), node_id);
        let manager = start_manager(
            processor.subscriber_map.clone(),
            processor.client_map.clone(),
            processor.message_queue.clone(),
            processor.back_ups.clone(),
            addrs.clone(),
            node_id,
            processor.logs.clone(),
        );

        let temp_manager = Arc::new(Mutex::new(manager));
        let temp_manager1 = temp_manager.clone();
        let temp_manager2 = temp_manager.clone();

        // let logs = manager.logs.clone();
        // let subscriber_map = manager.subscriber_map.clone();
        // let client_map = manager.client_map.clone();

        let message_handle = tokio::spawn(async move {
            loop {
                let mut manager = temp_manager1.lock().await;
                manager.process_message_queue().await;
                drop(manager);
            }
        });

        let log_handle = tokio::spawn(async move {
            loop {
                let mut manager = temp_manager2.lock().await;
                let mut logs = manager.logs.lock().await;
                let mut keys = Vec::<String>::new();

                for key in (*logs).keys() {
                    keys.push(key.clone());
                }
                drop(logs);
                drop(manager);

                for key in keys {
                    let mut manager = temp_manager2.lock().await;
                    manager.process_log(key).await;
                    drop(manager);
                }
            }
        });

        let temp_addr = match addr.clone().to_socket_addrs() {
            Ok(mut addr) => addr.next(),
            Err(error) => {
                return Err(Box::new(error));
            }
        };

        Server::builder()
            .add_service(JasmineBrokerServer::new(processor))
            .serve_with_shutdown(temp_addr.unwrap(), async {
                match shut_down_signal {
                    Some(mut value) => {
                        value.recv().await;
                    }

                    None => {
                        let (sender, mut receiver) = tokio::sync::mpsc::channel::<()>(1);
                        receiver.recv().await;
                    }
                }
            })
            .await?;

        message_handle.abort();

        return Ok(());
    }
}

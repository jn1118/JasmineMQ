use std::{
    collections::{HashMap, HashSet},
    net::ToSocketAddrs,
    sync::Arc,
    time::Duration,
};

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
};

use super::{manager::Manager, rpc_processor::RpcProcessor};
use zookeeper::{Acl, CreateMode, WatchedEvent, Watcher, ZooKeeper};
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
) -> Manager {
    return Manager::new(
        subscriber_map,
        client_map,
        message_queue,
        back_ups,
        addrs,
        node_id,
        Arc::new(Mutex::new(HashMap::new())),
    );
}

fn start_rpc_processor(addrs: Vec<String>, node_id: usize) -> RpcProcessor {
    return RpcProcessor::new(addrs, node_id);
}

impl Broker {
    pub async fn new(addrs: Vec<String>, node_id: usize) -> JasmineResult<()> {
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
        );

        let temp_manager = Arc::new(Mutex::new(manager));
        let handle = tokio::spawn(async move {
            loop {
                let mut manager = temp_manager.lock().await;
                manager.process_message_queue().await;
                drop(manager);
            }

            return true;
        });

        let temp_addr = match addr.clone().to_socket_addrs() {
            Ok(mut addr) => addr.next(),
            Err(error) => {
                return Err(Box::new(error));
            }
        };

        let (sender, mut receiver) = tokio::sync::mpsc::channel::<()>(1);
        Server::builder()
            .add_service(JasmineBrokerServer::new(processor))
            .serve_with_shutdown(temp_addr.unwrap(), async {
                receiver.recv().await;
            })
            .await?;

        handle.abort();
        return Ok(());
    }
}

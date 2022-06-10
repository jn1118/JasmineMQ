use std::{
    collections::{HashMap, HashSet, VecDeque},
    hash::Hash,
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

use crate::client::subscriber;

use super::{
    manager::{self, Manager},
    rpc_processor::RpcProcessor,
};
use zookeeper::{Acl, CreateMode, ZooKeeper};
pub struct Broker {}

impl Broker {
    pub async fn new(
        addrs: Vec<String>,
        node_id: usize,
        shut_down_signal: Option<Receiver<()>>,
    ) -> JasmineResult<()> {
        let my_addr = &addrs[node_id];

        // Connect to ZooKeeper and create ephemeral node for this broker
        let zk_urls = "164.92.70.147:2181".to_string();
        let zk = ZooKeeper::connect(&*zk_urls, Duration::from_secs(15), LoggingWatcher).unwrap();
        let path = format!("{}{}", "/brokers/", node_id);

        zk.create(
            &path,
            Vec::new(),
            Acl::open_unsafe().clone(),
            CreateMode::Ephemeral,
        )?;

        zk.create(
            "/logs",
            Vec::new(),
            Acl::open_unsafe().clone(),
            CreateMode::Persistent,
        );

        let path = format!("{}{}", "/logs/", node_id);

        zk.create(
            &path,
            Vec::new(),
            Acl::open_unsafe().clone(),
            CreateMode::Persistent,
        );

        // Arc references to zookeepers

        let keeper_client = Arc::new(Mutex::new(zk));

        let keeper_client1 = keeper_client.clone();

        // Create shared data structures

        let subscriber_map = Arc::new(Mutex::new(HashMap::new()));
        let client_map = Arc::new(Mutex::new(HashMap::new()));
        let message_queue = Arc::new(Mutex::new(VecDeque::new()));
        let backs_ups = Arc::new(Mutex::new(HashMap::new()));
        let logs = Arc::new(Mutex::new(VecDeque::new()));

        // End of shared data structures

        // Create manager
        let manager = Arc::new(Mutex::new(Manager {
            subscriber_map: subscriber_map.clone(),
            client_map: client_map.clone(),
            message_queue: message_queue.clone(),
            back_ups: backs_ups.clone(),
            addrs: addrs.clone(),
            node_id: node_id.clone(),
            logs: logs.clone(),
            keeper_client: Arc::new(Mutex::new(
                ZooKeeper::connect(&*zk_urls, Duration::from_secs(15), LoggingWatcher).unwrap(),
            )),
        }));

        // Arc references to manager
        let manager1 = manager.clone();
        let manager2 = manager.clone();
        let manager3 = manager.clone();

        // Drivers

        let message_handler = tokio::spawn(async move {
            loop {
                let mut temp_manager = manager1.lock().await;
                (*temp_manager).process_message_queue();
                drop(temp_manager);
            }
        });

        let log_handler = tokio::spawn(async move {
            loop {
                let mut temp_manager = manager2.lock().await;
                (*temp_manager).process_log();
                drop(temp_manager);
            }
        });

        let recovery_handler = tokio::spawn(async move {
            loop {
                let mut temp_keeper_client = keeper_client1.lock().await;
                temp_keeper_client.get_children(&path, false);
            }
        });

        // End of Drviers

        // Create RPC Processor and start rpc server
        let processor = RpcProcessor {
            subscriber_map: subscriber_map.clone(),
            client_map: client_map.clone(),
            message_queue: message_queue.clone(),
            back_ups: backs_ups.clone(),
            addrs: addrs.clone(),
            node_id: node_id,
            logs: logs.clone(),
            clock: Arc::new(Mutex::new(0)),
        };

        let temp_addr = match my_addr.clone().to_socket_addrs() {
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

        message_handler.abort();
        log_handler.abort();
        recovery_handler.abort();

        return Ok(());
    }
}

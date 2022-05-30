use std::{
    collections::{HashMap, HashSet},
    net::ToSocketAddrs,
    sync::Arc,
};

use tokio::sync::Mutex;
use tonic::transport::{Channel, Server};
use util::{
    result::JasmineResult,
    rpc::{
        broker::jasmine_broker_server::JasmineBrokerServer,
        client::jasmine_client_client::JasmineClientClient,
    },
};

use super::{manager::Manager, rpc_processor::RpcProcessor};

struct Broker {
    addrs: Vec<String>,
    node_id: usize,
    processor: RpcProcessor,
    manager: Manager,
}

fn start_manager(
    subscriber_map: Arc<Mutex<HashMap<String, HashSet<String>>>>,
    client_map: Arc<Mutex<HashMap<String, JasmineClientClient<Channel>>>>,
    message_queue: Arc<Mutex<Vec<(String, String)>>>,
    addrs: Vec<String>,
    node_id: usize,
) -> Manager {
    return Manager::new(
        subscriber_map,
        client_map,
        message_queue,
        addrs,
        node_id,
        Arc::new(Mutex::new(HashMap::new())),
    );
}

fn start_rpc_processor(addr: String) -> RpcProcessor {
    return RpcProcessor::new(addr);
}

impl Broker {
    async fn new(addrs: Vec<String>, node_id: usize) -> JasmineResult<()> {
        let temp_addrs = addrs.clone();
        let addr = &addrs[node_id];

        let processor = start_rpc_processor(addr.clone());
        let manager = start_manager(
            processor.subscriber_map.clone(),
            processor.client_map.clone(),
            processor.message_queue.clone(),
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

        let temp_addr = match processor.addr.clone().to_socket_addrs() {
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

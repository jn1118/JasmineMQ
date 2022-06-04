// use crate::broker::server::JasmineBroker;
use crate::{
    broker::broker::Broker,
    client::{self, client::Client, rpc_processor::ClientRpcProcessor},
    storage::storage::Storage,
};
use std::{
    collections::{HashMap, HashSet},
    net::ToSocketAddrs,
    sync::Arc,
};
// use crate::client::client::JasmineBroker;

use tokio::sync::Mutex;
use util::{
    result::JasmineResult,
    rpc::{
        broker::jasmine_broker_server::JasmineBroker,
        client::jasmine_client_server::JasmineClientServer,
    },
};

struct Library {
    message_buffer: Arc<Mutex<Vec<(String, String, bool)>>>,
}

use tonic::transport::Server;

impl Library {
    pub fn new() -> Self {
        return Library {
            message_buffer: Arc::new(Mutex::new(Vec::new())),
        };
    }
    // it will return a client wrapper, which user can call publish, subscribe, unsubscribe, connect directly.
    pub fn initialize_front_end<T: Fn(String, String, bool) -> ()>(
        &mut self,
        broker: Vec<String>,
        client_address: String,
        message_buffer: Arc<Mutex<Vec<(String, String, bool)>>>,
    ) -> JasmineResult<Client<T>> {
        let new_client = Client::new(
            broker.clone(),
            client_address.clone(),
            self.message_buffer.clone(),
        );

        return Ok(new_client);
    }

    pub async fn initialize_broker(
        &self,
        addresses: Vec<String>,
        node_id: usize,
    ) -> JasmineResult<()> {
        eprintln!("in initialize broker");
        Broker::new(addresses, node_id).await;
        return Ok(());
    }

    pub async fn start_rpc_client_server(&self, rpc_server_addr: String) -> JasmineResult<()> {
        tokio::spawn(async move { loop {} });

        let new_rpc_client =
            ClientRpcProcessor::new(rpc_server_addr.clone(), self.message_buffer.clone());

        let temp_addr = match rpc_server_addr.to_socket_addrs() {
            Ok(mut addr) => addr.next(),
            Err(e) => return Err(Box::new(e)),
        };

        let (mut sender, mut receiver) = tokio::sync::mpsc::channel::<()>(1);
        Server::builder()
            .add_service(JasmineClientServer::new(new_rpc_client))
            .serve_with_shutdown(temp_addr.unwrap(), async {
                receiver.recv().await;
            })
            .await?;
        return Ok(());
    }
}

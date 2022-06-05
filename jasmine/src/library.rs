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

struct Library<T: Fn(String, String, bool) -> ()> {
    message_callback: Arc<Mutex<Option<T>>>,
    message_buffer: Arc<Mutex<Vec<(String, String, bool)>>>,
}

use tonic::transport::Server;

impl<T: Fn(String, String, bool) -> ()> Library<T> {
    pub fn new() -> Self {
        let callback = Arc::new(Mutex::new(None)).clone();
        return Library {
            message_callback: callback,
            message_buffer: Arc::new(Mutex::new(Vec::new())),
        };
    }

    fn on_message(&mut self, func: T) {
        let callback = Arc::new(Mutex::new(Some(func))).clone();
        self.message_callback = callback;
    }

    // it will return a client wrapper, which user can call publish, subscribe, unsubscribe, connect directly.
    pub fn initialize_front_end(
        &mut self,
        broker: Vec<String>,
        client_address: String,
        message_buffer: Arc<Mutex<Vec<(String, String, bool)>>>,
    ) -> JasmineResult<Client> {
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
        let message_buffer2 = self.message_buffer.clone();
        let message_callback2 = self.message_callback.clone();

        let handle = tokio::spawn(async move {
            loop {
                let mut temp_message_buffer = message_buffer2.lock().await;
                let mut temp_message_callback = message_callback2.lock().await;

                // if (*temp_message_buffer).len() > 0 {
                //     drop(temp_message_buffer);
                // } else {
                //     drop(temp_message_buffer);
                // }
            }
        });

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

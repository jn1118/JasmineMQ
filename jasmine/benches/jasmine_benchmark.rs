use std::fmt::Error;

use jasmine::{
    broker::{self, broker::Broker},
    client::{self, client::Client},
};
use tokio::sync::mpsc::Sender;
use util::result::JasmineError;

fn generate_addrs(addr: String, base_port: u64, num: u64) -> Vec<String> {
    let mut addrs = Vec::<String>::new();

    for i in 0..num {
        let mut temp = addr.clone();
        temp.push_str(&((base_port + i).to_string()));
        addrs.push(temp);
    }

    return addrs;
}

async fn spawn_brokers(addrs: Vec<String>, num: u64) -> Result<Vec<Sender<()>>, JasmineError> {
    let mut brokers = Vec::<Sender<()>>::new();

    for i in 0..num {
        let (sender, receiver) = tokio::sync::mpsc::channel(1);
        match Broker::new(addrs.clone(), i.try_into().unwrap(), receiver).await {
            Ok(_) => {}
            Err(error) => return Err(JasmineError::Unknown(error.to_string())),
        }
        brokers.push(sender);
    }
    return Ok(brokers);
}

fn spwan_clients(addrs: Vec<String>, brokers: Vec<String>, num: u64) -> Vec<Client> {
    for i in 0..num {
        Client::new(brokers, addrs[i], message_map)
    }
}

fn main() {}

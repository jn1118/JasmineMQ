mod args;
mod bin_broker;

use std::collections::HashMap;

use crate::args::UserCommand::{self, Client};
use args::{
    CreateClient, JasmineArgs, PublishMessage, ReceiveMessage, StartBroker, SubscribeTopic,
    UnsubscribeTopic,
};
use clap::Parser;
use tokio::sync::mpsc::Sender;
use util::{config::BROKER_ADDRS, result::JasmineResult};

struct CLIData {
    client_num: usize,
    client_hashmap: HashMap<String, String>,
}
#[tokio::main]
async fn main() -> JasmineResult<()> {
    let args = JasmineArgs::parse();
    let a = args.command;
    match a {
        UserCommand::StartBroker(a) => handle_start_broker(a).await,
        UserCommand::Client(a) => todo!(),
        UserCommand::Publish(a) => todo!(),
        UserCommand::Subscribe(a) => todo!(),
        UserCommand::Unubscribe(a) => todo!(),
        UserCommand::Retrieve(a) => todo!(),
    };
    Ok(())
}

fn handle_client(input: CreateClient) {
    dbg!(input.name);
}

fn handle_publish(input: PublishMessage) {
    dbg!(input.name);
}
fn handle_subscribe(input: SubscribeTopic) {
    dbg!(input.topic);
}
fn handle_unsubscribe(input: UnsubscribeTopic) {
    dbg!(input.name);
}
fn handle_retrieve(input: ReceiveMessage) {
    dbg!(input.name);
}

// TODO:
fn generate_client_address(num: usize) -> String {
    // let mut address = Vec::new();
    // let a =
    // for i in 30000..(30000 + 1) {
    let addr = "127.0.0.1:".to_string() + &10000.to_string();
    // address.push(addr)
    // }
    return addr;
}

async fn handle_start_broker(input: StartBroker) -> JasmineResult<()> {
    dbg!("start");
    bin_broker::main();
    Ok(())
    // let mut brokers = Vec::new();
    // for i in BROKER_ADDRS {
    //     brokers.push(i.to_string())
    // }

    // let (broker_handles, broker_shutdown) = spawn_broker(brokers.clone());
}

fn spawn_broker(
    brokers: Vec<String>,
) -> (
    Vec<tokio::task::JoinHandle<JasmineResult<()>>>,
    Vec<Sender<()>>,
) {
    let mut handles = vec![];
    let mut brokers_shutdown = vec![];

    for i in 0..3 {
        let (shut_tx, shut_rx) = tokio::sync::mpsc::channel(1);
        let l = tokio::spawn(jasmine::library::initialize_broker(
            brokers.clone(),
            i,
            shut_rx,
        ));
        brokers_shutdown.push(shut_tx);
        handles.push(l);
    }
    return (handles, brokers_shutdown);
}

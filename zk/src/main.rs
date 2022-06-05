#![deny(unused_mut)]
extern crate zookeeper;

use std::io;
use std::time::Duration;
use zookeeper::{Acl, CreateMode, WatchedEvent, Watcher, ZooKeeper};

struct LoggingWatcher;
impl Watcher for LoggingWatcher {
    fn handle(&self, e: WatchedEvent) {
        println!("{:?}", e)
    }
}

fn zk_example() {
    let zk_urls = "164.92.70.147:2181".to_string();
    println!("connecting to {}", zk_urls);

    let zk = ZooKeeper::connect(&*zk_urls, Duration::from_secs(15), LoggingWatcher).unwrap();

    let path = zk.create(
        "/brokers",
        vec![b'a'],
        Acl::open_unsafe().clone(),
        CreateMode::Persistent,
    );

    println!("Create Result {:?}", path);

    let children = zk.get_children("/brokers", false);

    println!("children of /brokers -> {:?}", children);

    let path = zk.create(
        "/brokers/ab",
        vec![b'b'],
        Acl::open_unsafe().clone(),
        CreateMode::Ephemeral,
    );

    println!("Create Result2 {:?}", path);

    let children = zk.get_children("/brokers", false);

    println!("children of /brokers -> {:?}", children);

    let watch_children = zk.get_children_w("/brokers", |event: WatchedEvent| {
        println!("watched event {:?}", event);
    });

    println!("watch children -> {:?}", watch_children);

    println!("press enter to close client");
    let mut tmp = String::new();
    io::stdin().read_line(&mut tmp).unwrap();
}

fn main() {
    zk_example();
}

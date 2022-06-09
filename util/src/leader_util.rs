extern crate zookeeper;
use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
    time::Duration,
};
use zookeeper::{WatchedEvent, Watcher, ZooKeeper};

use crate::config::BROKER_ADDRS;

pub struct LoggingWatcher;
impl Watcher for LoggingWatcher {
    fn handle(&self, e: WatchedEvent) {
        println!("Watcher triggered: {:?}", e)
    }
}

pub fn find_leader(topic: &str) -> String {
    let zk_urls = "164.92.70.147:2181".to_string();
    let zk = ZooKeeper::connect(&*zk_urls, Duration::from_secs(15), LoggingWatcher).unwrap();

    let children = zk.get_children("/brokers", false).unwrap(); // ["0","2"]
    let live_children_len = children.len();
    let mut s = DefaultHasher::new();
    topic.hash(&mut s);

    let hash = s.finish() as usize;
    let original_hash = hash % 3;
    if children.contains(&original_hash.to_string()) {
        BROKER_ADDRS[original_hash].to_string()
    } else {
        let idx = hash % (live_children_len) as usize;
        let broker_id = children[idx].clone().parse::<usize>().unwrap_or(0);
        BROKER_ADDRS[broker_id].to_string()
    }
}

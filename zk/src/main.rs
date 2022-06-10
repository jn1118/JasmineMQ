#![deny(unused_mut)]
extern crate zookeeper;

use std::time::Duration;
use std::{io, str::from_utf8};
use util;
use util::transaction::JasmineLog;
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

fn zk_example2() {
    let zk_urls = "164.92.70.147:2181".to_string();
    let zk = ZooKeeper::connect(&*zk_urls, Duration::from_secs(15), LoggingWatcher).unwrap();

    let path = zk.create(
        "/testing3",
        vec![b'a'],
        Acl::open_unsafe().clone(),
        CreateMode::Persistent,
    );

    println!("create dir: {:?}", path);

    let path = zk.create(
        "/testing3/t",
        vec![b'a'],
        Acl::open_unsafe().clone(),
        CreateMode::Persistent,
    );

    println!("create 1: {:?}", path);

    let path = zk.create(
        "/testing3/t",
        vec![b'a'],
        Acl::open_unsafe().clone(),
        CreateMode::Ephemeral,
    );

    println!("create 2: {:?}", path);

    let children = zk.get_children("/testing3", false);

    println!("children of /testing2 -> {:?}", children);

    let path = zk.set_data(
        "/testing3/t",
        "jefferson is cool".to_string().as_bytes().to_vec(),
        None,
    );

    println!("write 1: {:?}", path);

    let path = zk.set_data(
        "/testing3/t",
        "adam is cool".to_string().as_bytes().to_vec(),
        None,
    );

    println!("write 2: {:?}", path);

    let path = zk.get_data("/testing3/t", false);

    println!("read 1: {:?}", from_utf8(&path.unwrap().0));

    let t1 = util::transaction::JasmineLog {
        jid: 1,
        topic: "testing".to_string(),
        message: "gg0".to_string(),
    };

    let t2 = util::transaction::JasmineLog {
        jid: 1,
        topic: "testing".to_string(),
        message: "gg1".to_string(),
    };

    let t3 = util::transaction::JasmineLog {
        jid: 1,
        topic: "testing".to_string(),
        message: "gg2".to_string(),
    };

    let s1 = serde_json::to_string(&t1).unwrap();
    let s2 = serde_json::to_string(&t2).unwrap();
    let s3 = serde_json::to_string(&t3).unwrap();

    let mut s4 = String::new();

    s4 = format!("{}|{}", s4, s1);
    s4 = format!("{}|{}", s4, s2);
    s4 = format!("{}|{}", s4, s3);

    println!("list: {}", s4);

    let path = zk.set_data("/testing3/t", s4.as_bytes().to_vec(), None);

    let path = zk.get_data("/testing3/t", false).unwrap();

    let list = path.0;

    let my_string = from_utf8(&list).unwrap();

    let mut transactions: Vec<JasmineLog> = Vec::new();

    for string in my_string.split("|") {
        match serde_json::from_str(string) {
            Ok(v) => {
                transactions.push(v);
            }
            Err(_) => {}
        }

        // println!("haha: {}", string);
    }

    println!("haha: {}", transactions[0].message);
}

fn main() {
    zk_example2();
}

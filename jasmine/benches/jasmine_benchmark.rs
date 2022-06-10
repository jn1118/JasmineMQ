use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use rskafka::{
    client::{
        partition::{Compression, PartitionClient},
        ClientBuilder,
    },
    record::Record,
};
use std::{collections::BTreeMap, str::from_utf8, sync::Arc, thread::JoinHandle};
use time::OffsetDateTime;
use tokio::{
    self, runtime,
    sync::{
        mpsc::{channel, Sender},
        Mutex,
    },
};

use jasmine::{
    client::{self, client::Client, rpc_processor::ClientRpcProcessor},
    library,
};

async fn kafka_establish_connection() -> PartitionClient {
    dbg!("A");
    let connection = "164.92.70.147:9092".to_owned();
    dbg!("B");
    let client = ClientBuilder::new(vec![connection]).build().await.unwrap();
    dbg!("C");
    let topic = "benchmark";
    dbg!("D");
    let controller_client = client.controller_client().await.unwrap();
    dbg!("E");
    controller_client.create_topic(topic, 2, 1, 5_000).await;
    dbg!("F");
    let partition_client = client.partition_client(topic.to_owned(), 0).await.unwrap();
    dbg!("G");
    return partition_client;
}

async fn kafka_clean_record() {
    dbg!("1-1-1");
    let client = kafka_establish_connection().await;
    dbg!("1-1-2");
    client.delete_records(1000, 1000).await;
    dbg!("1-2-3");
}

async fn kafka_single_message() {
    tokio::spawn(async move {
        let publisher_client = kafka_establish_connection().await;
        let record = Record {
            key: None,
            value: Some(b"hello kafka100".to_vec()),
            headers: BTreeMap::from([("foo".to_owned(), b"bar".to_vec())]),
            timestamp: OffsetDateTime::now_utc(),
        };
        publisher_client
            .produce(vec![record], Compression::default())
            .await
            .unwrap();
    });

    tokio::spawn(async move {
        let subscriber_client = kafka_establish_connection().await;
        let (mut records, high_watermark) = subscriber_client
            .fetch_records(0, 1..1_000_000, 1_000)
            .await
            .unwrap();

        let size = records.len();
        if from_utf8(records[size - 1].record.value.as_mut().unwrap()) == Ok("hello kafka100") {}
    });
}

async fn kafka_bulk_message_single_topic() {
    let publisher_client = kafka_establish_connection().await;
    let subscriber_client = kafka_establish_connection().await;

    let record = Record {
        key: None,
        value: Some(b"hello kafka".to_vec()),
        headers: BTreeMap::from([("foo".to_owned(), b"bar".to_vec())]),
        timestamp: OffsetDateTime::now_utc(),
    };

    tokio::spawn(async move {
        for i in 0..100000 {
            publisher_client
                .produce(vec![record.clone()], Compression::default())
                .await
                .unwrap();
        }
    });

    tokio::spawn(async move {
        for i in 0..100000 {
            let (records, high_watermark) = subscriber_client
                .fetch_records(0, 1..1_000_000, 1_000)
                .await
                .unwrap();
        }
    });
}

async fn kafka_bulk_message_single_topic2() {
    let mut handles = Vec::<JoinHandle<bool>>::new();

    for i in 0..100000 {
        tokio::spawn(async move {
            let publisher_client = kafka_establish_connection().await;
            let record = Record {
                key: None,
                value: Some(b"hello kafka".to_vec()),
                headers: BTreeMap::from([("foo".to_owned(), b"bar".to_vec())]),
                timestamp: OffsetDateTime::now_utc(),
            };
            publisher_client
                .produce(vec![record], Compression::default())
                .await
                .unwrap();
        });
    }

    for i in 0..100000 {
        tokio::spawn(async move {
            let subscriber_client = kafka_establish_connection().await;
            let (records, high_watermark) = subscriber_client
                .fetch_records(0, 1..1_000_000, 1_000)
                .await
                .unwrap();
        });
    }

    // TODO: add comparison, await handles.
}

fn bench0(c: &mut Criterion) {
    c.bench_function("kafka connection", |b| {
        b.iter(|| async move {
            kafka_establish_connection().await;
        })
    });
}

fn bench1_1(c: &mut Criterion) {
    c.bench_function("kafka single latency", |b| {
        b.iter(|| async move {
            kafka_single_message().await;
        })
    });
    dbg!("3");
}

fn bench2_1(c: &mut Criterion) {
    c.bench_function("kafka single topic mass message", |b| {
        b.iter(|| async move {
            kafka_bulk_message_single_topic().await;
        })
    });
}

fn bench2_2(c: &mut Criterion) {
    c.bench_function("kafka single topic mass message2", |b| {
        b.iter(|| async move {
            kafka_bulk_message_single_topic2().await;
        })
    });
}

fn bench3(c: &mut Criterion) {
    let rt = tokio::runtime::Builder::new_multi_thread().build().unwrap();
    let res = rt.block_on(async move { kafka_establish_connection().await });
    c.bench_function("jasmine single message", |b| {
        b.iter(|| async move { jamines_single_message() })
    });
}

fn gen_addrs(url: String, base: u64, num: u64) -> Vec<String> {
    let mut addrs = Vec::new();
    for i in 0..num {
        let mut temp_url = url.clone();
        temp_url.push_str(":");
        temp_url.push_str(&(base + i).to_string());
        addrs.push(temp_url);
    }

    return addrs;
}

async fn start_broker(broker_count: u64) -> Vec<String> {
    // Initialize brokers
    let broker_addrs = gen_addrs("127.0.0.1".to_string(), 6969, 3);
    let mut broker_shutdowns = Vec::new();
    for i in 0..broker_count {
        let (sender, receiver) = channel(1);
        broker_shutdowns.push(sender);
        library::initialize_broker(broker_addrs.clone(), i.try_into().unwrap(), receiver).await;
    }

    return broker_addrs;
}

async fn start_client(broker_addrs: Vec<String>, client_count: u64, base: u64) -> Vec<Client> {
    // Initialize clients
    let client_addrs = gen_addrs("127.0.0.1".to_string(), base, client_count);
    let mut clients = Vec::new();
    for addr in client_addrs {
        let rpc_processor = ClientRpcProcessor::new(addr.clone());
        let client = library::initialize_front_end(
            broker_addrs.clone(),
            addr.clone(),
            rpc_processor.message_map.clone(),
        )
        .unwrap();
        clients.push(client);
    }

    // on_message returns corresponding message in (topic, is_consistent) tuple: Vec<String>

    return clients;
}

async fn jamines_single_message() {
    let broker_addrs = start_broker(3).await;
    let sub_client = start_client(broker_addrs.clone(), 1, 8000).await;
    let pub_client = start_client(broker_addrs.clone(), 1, 8001).await;

    tokio::spawn(async move {
        sub_client[0].subscribe("testing".to_string());
    });

    tokio::spawn(async move {
        pub_client[0].publish("testing".to_string(), "testing2".to_string(), false);
    });

    // call spawn rpc process server
}

criterion_group!(benches, bench1_1, bench2_1, bench2_2, bench3);
criterion_main!(benches);

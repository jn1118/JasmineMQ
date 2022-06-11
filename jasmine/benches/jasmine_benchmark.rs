use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};

use rskafka::{
    client::{
        partition::{Compression, PartitionClient},
        ClientBuilder,
    },
    record::Record,
};
use std::{
    collections::BTreeMap,
    str::from_utf8,
    sync::Arc,
    thread::{self, JoinHandle},
    time::Duration,
};
use time::OffsetDateTime;
use tokio::{
    self, runtime,
    sync::{
        mpsc::{channel, Sender},
        Mutex,
    },
};
use util::result::JasmineResult;

use jasmine::{
    client::{self, client::Client, rpc_processor::ClientRpcProcessor},
    library,
};

async fn kafka_establish_connection() -> PartitionClient {
    //dbg!("A");
    let connection = "164.92.70.147:9092".to_owned();
    dbg!("B");
    let client = match ClientBuilder::new(vec![connection]).build().await {
        Ok(c) => {
            dbg!(&c);
            c
        }
        Err(e) => {
            dbg!(&e);
            panic!();
        }
    };
    dbg!("C");
    let topic = "rara";
    dbg!("D");
    let controller_client = client.controller_client().await.unwrap();
    dbg!("E");
    //controller_client.create_topic(topic, 2, 1, 5_000).await;
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
    let subscriber_client = kafka_establish_connection().await;
    let (mut records, high_watermark) = subscriber_client
        .fetch_records(0, 1..1_000_000, 1_000)
        .await
        .unwrap();
    let size = records.len();
    if from_utf8(records[size - 1].record.value.as_mut().unwrap()) == Ok("hello kafka100") {}
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

fn bench0_1(c: &mut Criterion) {
    let rt = tokio::runtime::Runtime::new().unwrap();
    c.bench_function("kafka connection", move |b| {
        b.to_async(&rt).iter(|| async move {
            kafka_establish_connection().await;
        })
    });
}

fn bench1_1(c: &mut Criterion) {
    let rt = tokio::runtime::Runtime::new().unwrap();
    c.bench_function("kafka single latency", move |b| {
        b.to_async(&rt).iter(|| async move {
            kafka_single_message().await;
        })
    });
}

fn bench1_2(c: &mut Criterion) {
    let rt = tokio::runtime::Runtime::new().unwrap();
    c.bench_function("jasmine single latency", move |b| {
        b.to_async(&rt).iter(|| async move {
            jasmine_single_message().await;
        })
    });
}

fn bench2_1(c: &mut Criterion) {
    let rt = tokio::runtime::Runtime::new().unwrap();
    c.bench_function("kafka single topic mass message", move |b| {
        b.to_async(&rt).iter(|| async move {
            kafka_bulk_message_single_topic().await;
        })
    });
}

fn bench2_2(c: &mut Criterion) {
    let rt = tokio::runtime::Runtime::new().unwrap();
    c.bench_function("kafka single topic mass message2", move |b| {
        b.to_async(&rt).iter(|| async move {
            kafka_bulk_message_single_topic2().await;
        })
    });
}

fn bench3(c: &mut Criterion) {

    /*     let rt = tokio::runtime::Builder::new_multi_thread().build().unwrap();
    let res = rt.block_on(async move { kafka_establish_connection().await });
    c.bench_function("jasmine single message", |b| {
        b.iter(|| async move { jamines_single_message() })
    }); */
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
    let broker_addrs = gen_addrs("127.0.0.1".to_string(), 10000, 3);
    let mut broker_shutdowns = Vec::new();
    dbg!("brokers starting");
    for i in 0..broker_count {
        let (sender, receiver) = channel(1);
        broker_shutdowns.push(sender);
        tokio::spawn(library::initialize_broker(
            broker_addrs.clone(),
            i.try_into().unwrap(),
            receiver,
        ));
    }
    dbg!("brokers ok");
    return broker_addrs;
}

async fn start_client(
    broker_addrs: Vec<String>,
    client_count: u64,
    base: u64,
) -> (Vec<Client>, Vec<Sender<()>>) {
    // Initialize clients
    let client_addrs = gen_addrs("127.0.0.1".to_string(), base, client_count);
    let mut clients = Vec::new();
    let mut shutdowns = Vec::new();
    dbg!("starting clients");
    dbg!(&client_addrs);
    for addr in client_addrs {
        let rpc_processor = ClientRpcProcessor::new(addr.clone());
        let client = library::initialize_front_end(
            broker_addrs.clone(),
            addr.clone(),
            rpc_processor.message_map.clone(),
        )
        .unwrap();
        let shutdown =
            jasmine::library::start_rpc_client_server_benchmark(addr, rpc_processor).await;
        clients.push(client);
        shutdowns.push(shutdown.unwrap());
    }

    // on_message returns corresponding message in (topic, is_consistent) tuple: Vec<String>

    return (clients, shutdowns);
}

fn spawn_client_rpc_server(
    rpc_server_addr: String,
    new_rpc_client: ClientRpcProcessor,
) -> tokio::task::JoinHandle<JasmineResult<()>> {
    let handle = tokio::spawn(jasmine::library::start_rpc_client_server(
        rpc_server_addr,
        new_rpc_client,
    ));
    return handle;
}

async fn jasmine_single_message() {
    eprintln!("Start!");
    let broker_addrs = vec![
        "127.0.0.1:10000".to_string(),
        "127.0.0.1:10001".to_string(),
        "127.0.0.1:10002".to_string(),
    ];
    let (sub_client, sub_shutdowns) = start_client(broker_addrs.clone(), 1, 30000).await;
    let (pub_client, pub_shutdowns) = start_client(broker_addrs.clone(), 1, 31000).await;

    match sub_client[0].subscribe("testing".to_string()).await {
        Ok(_) => {
            eprintln!("sub ok");
        }
        Err(e) => {
            eprintln!("sub err {:?}", e);
        }
    };

    match pub_client[0]
        .publish("testing".to_string(), "testing2".to_string(), false)
        .await
    {
        Ok(_) => {
            eprintln!("pub ok");
        }
        Err(e) => {
            eprintln!("pub err {:?}", e);
        }
    };

    let mut result: Vec<String> = Vec::new();

    //result = sub_client[0].on_message("testing".to_string(), false).await;
    /*  while result.len() <= 0 {

    } */

    while result.len() <= 0 {
        tokio::time::sleep(Duration::from_millis(10)).await;
        result = sub_client[0].on_message("testing".to_string(), false).await;
    }

    dbg!(&result);
    dbg!("Jasmine Done!");

    match sub_shutdowns[0].send(()).await {
        Ok(_) => {
            eprintln!("ok");
        }
        Err(_) => {
            eprintln!("error");
        }
    };
    pub_shutdowns[0].send(()).await;

    // call spawn rpc process server
}

criterion_group!(benches, bench1_2);
criterion_main!(benches);

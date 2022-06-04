use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use rskafka::{
    client::{
        partition::{Compression, PartitionClient},
        ClientBuilder,
    },
    record::Record,
};
use std::collections::BTreeMap;
use time::OffsetDateTime;
use tokio;

async fn kafka_establish_connection() -> PartitionClient {
    let connection = "164.92.70.147:9092".to_owned();
    let client = ClientBuilder::new(vec![connection]).build().await.unwrap();
    let topic = "benchmark";
    let controller_client = client.controller_client().await.unwrap();
    controller_client.create_topic(topic, 2, 1, 5_000).await;
    let partition_client = client.partition_client(topic.to_owned(), 0).await.unwrap();
    return partition_client;
}

async fn kafka_single_message() {
    let partition_client = kafka_establish_connection().await;

    let record = Record {
        key: None,
        value: Some(b"hello kafka".to_vec()),
        headers: BTreeMap::from([("foo".to_owned(), b"bar".to_vec())]),
        timestamp: OffsetDateTime::now_utc(),
    };

    partition_client
        .produce(vec![record], Compression::default())
        .await
        .unwrap();

    let (records, high_watermark) = partition_client
        .fetch_records(0, 1..1_000_000, 1_000)
        .await
        .unwrap();
}

fn bench0(c: &mut Criterion) {
    c.bench_function("kafka connection", |b| {
        b.iter(|| async move {
            kafka_establish_connection().await;
        })
    });
}

fn bench1(c: &mut Criterion) {
    c.bench_function("kafka single latency", |b| {
        b.iter(|| async move {
            kafka_single_message().await;
        })
    });
}

criterion_group!(benches, bench0, bench1);
criterion_main!(benches);

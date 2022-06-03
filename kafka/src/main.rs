fn main() {
    println!("Hello, world!");
}

use rskafka::{
    client::{
        ClientBuilder,
        partition::Compression,
    },
    record::Record,
};
use time::OffsetDateTime;
use std::collections::BTreeMap;

// setup client
let connection = "164.92.70.147:9092".to_owned();
let client = ClientBuilder::new(vec![connection]).build().await.unwrap();

// create a topic
let topic = "my_topic";
let controller_client = client.controller_client().unwrap();
controller_client.create_topic(
    topic,
    2,      // partitions
    1,      // replication factor
    5_000,  // timeout (ms)
).await.unwrap();

// get a partition-bound client
let partition_client = client
    .partition_client(
        topic.to_owned(),
        0,  // partition
     )
    .unwrap();

// produce some data
let record = Record {
    key: None,
    value: Some(b"hello kafka".to_vec()),
    headers: BTreeMap::from([
        ("foo".to_owned(), b"bar".to_vec()),
    ]),
    timestamp: OffsetDateTime::now_utc(),
};
partition_client.produce(vec![record], Compression::default()).await.unwrap();

// consume data
let (records, high_watermark) = partition_client
    .fetch_records(
        0,  // offset
        1..1_000_000,  // min..max bytes
        1_000,  // max wait time
    )
   .await
   .unwrap();


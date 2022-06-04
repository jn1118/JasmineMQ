use rskafka::{
    client::{partition::Compression, ClientBuilder},
    record::Record,
};
use std::collections::BTreeMap;
use time::OffsetDateTime;
use tokio;

#[tokio::main]
async fn main() {
    test().await;
}

async fn test() {
    println!("Running");
    // setup client
    let connection = "164.92.70.147:9092".to_owned();
    let client = ClientBuilder::new(vec![connection]).build().await.unwrap();

    // create a topic
    println!("Creating topic");
    let topic = "my_topic";
    let controller_client = client.controller_client().await.unwrap();
    /*     controller_client
    .create_topic(
        topic, 2,     // partitions
        1,     // replication factor
        5_000, // timeout (ms)
    )
    .await
    .unwrap(); */

    // get a partition-bound client
    let partition_client = client
        .partition_client(
            topic.to_owned(),
            0, // partition
        )
        .await
        .unwrap();

    // produce some data
    println!("Producing");
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

    // consume data
    println!("Consuming");
    let (records, high_watermark) = partition_client
        .fetch_records(
            0,            // offset
            1..1_000_000, // min..max bytes
            1_000,        // max wait time
        )
        .await
        .unwrap();
    println!("Records: {:#?}", records);
}

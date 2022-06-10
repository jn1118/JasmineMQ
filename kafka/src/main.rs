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
    controller_client
        .create_topic(
            topic, 2,     // partitions
            1,     // replication factor
            5_000, // timeout (ms)
        )
        .await;
    // .unwrap();

    // get a partition-bound client
    let partition_client = client
        .partition_client(
            topic.to_owned(),
            0, // partition
        )
        .await
        .unwrap();

    let my_string = "hello kafka".to_string();
    // produce some data
    println!("Producing");
    let record = Record {
        key: None,
        value: Some(my_string.as_bytes().to_vec()),
        headers: BTreeMap::from([("content-type".to_owned(), b"string".to_vec())]),
        timestamp: OffsetDateTime::now_utc(),
    };
    partition_client
        .produce(vec![record], Compression::default())
        .await
        .unwrap();

    // consume data
    println!("Consuming");
    let (mut records, high_watermark) = partition_client
        .fetch_records(
            0,            // offset
            1..1_000_000, // min..max bytes
            1_000,        // max wait time
        )
        .await
        .unwrap();
    println!("Records: {:#?}", records);
    println!(
        "testing {:?}",
        std::str::from_utf8((records[0].record.value.as_mut().unwrap())) == Ok("hello kafka")
    );
}

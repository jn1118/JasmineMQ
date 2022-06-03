// extern crate pub_sub;

// use criterion::{black_box, criterion_group, criterion_main, Criterion};
// use jasmine::client::publisher::JasminePublisher;
// use tonic::transport::Channel;
// use util::config;

// use redis;

// fn test_redis_pub_sub() {
//     let client = redis::Client::open("redis::/127.0.0.1:6969").unwrap();
//     let mut connection = client.get_connection().unwrap();
//     let mut channel = connection.as_pubsub();

//     channel.subscribe("testing");
<<<<<<< HEAD
// }

// fn establish_connection() {
//     let mut broker_addr = Vec::new();
//     for addr in config::BROKER_ADDRS {
//         broker_addr.push(addr.to_string());
//     }

//     let client = jasmine::client::client::Client {
//         broker_addr: broker_addr,
//         client_addr: config::CLIENT_ADDRS[0].to_string(),
//     };

//     client.publish("test".to_string(), "testing".to_string());
// }

// fn criterion_benchmark(c: &mut Criterion) {
//     c.bench_function("pub;lish", |b| b.iter(|| establish_connection()));
//     // c.bench_function("fib 20", |b| b.iter(|| fibonacci(black_box(20))));
// }

// criterion_group!(benches, criterion_benchmark);
// criterion_main!(benches);

fn main() {}
=======
// }

// fn establish_connection() {
//     let mut broker_addr = Vec::new();
//     for addr in config::BROKER_ADDRS {
//         broker_addr.push(addr.to_string());
//     }

//     let client = jasmine::client::client::Client {
//         broker_addr: broker_addr,
//         client_addr: config::CLIENT_ADDRS[0].to_string(),
//     };

//     client.publish("test".to_string(), "testing".to_string());
// }

// fn criterion_benchmark(c: &mut Criterion) {
//     c.bench_function("pub;lish", |b| b.iter(|| establish_connection()));
//     // c.bench_function("fib 20", |b| b.iter(|| fibonacci(black_box(20))));
// }

// criterion_group!(benches, criterion_benchmark);
// criterion_main!(benches);
>>>>>>> ef79bddcfa73c606c39e38e0c6e011d6d0c4ca50

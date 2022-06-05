#![feature(test)]
#[allow(soft_unstable)]
// extern crate test;

// #[cfg(test)]
// mod tests {
//     #[test]
//     fn it_works() {
//         let result = 2 + 2;
//         assert_eq!(result, 4);
//     }
// }
// use test::Bencher;
pub mod broker;
pub mod client;
pub mod lab;
pub mod storage;

// #[bench]
// fn simple_connection_latency(b: &mut Bencher) {

// }

use pub_sub;

// #[cfg(test)]
// mod test {

//     #[bench]
//     fn it_works(bencher: &mut bencher::Bencher) {
//         b.iter(|| {
//             println!("Hello, world!");
//         })
//     }
// }

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

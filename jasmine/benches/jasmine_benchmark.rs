// use criterion::{
//     criterion_group, criterion_main, Bencher, BenchmarkGroup, BenchmarkId, Criterion,
//     PlotConfiguration,
// };

// use jasmine::client::{client::JasmineClient, publisher::JasminePublisher};
// use util::config;

// async fn simple_latency_benchmark(c: &mut Criterion) {
//     let mut addr = Vec::new();
//     addr.push(config::BROKER_ADDRS[0].to_string());
//     let client = jasmine::client::client::Client::new(addr);
//     c.bench_function("testing", async move {
//         |b| b.iter(|| client.connect().await);
//     });
// }

// criterion_group!(benches, simple_latency_benchmark.await);
// criterion_main!(benches);

pub fn main() {}

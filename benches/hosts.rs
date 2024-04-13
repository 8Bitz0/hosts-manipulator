use criterion::{black_box, criterion_group, criterion_main, Criterion};

use hosts_manipulator::*;

const HOSTS: &str = include_str!("./assets/hosts");

fn host_conversion(c: &mut Criterion) {
    let mut g = c.benchmark_group("host-conversion");
    g.sample_size(80).warm_up_time(std::time::Duration::from_secs(5));

    g.bench_function("string to hosts", |b| b.iter(|| Hosts::from(black_box(HOSTS))));

    let hosts_str = Hosts::from(black_box(HOSTS));

    g.bench_function("hosts to string", |b| b.iter(|| hosts_str.to_string()));
}

criterion_group!(
    benches,
    host_conversion
);
criterion_main!(benches);

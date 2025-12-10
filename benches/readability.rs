use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use readabilityrs::{is_probably_readerable, Readability};
use std::fs;
use std::path::Path;

fn load_test_case(name: &str) -> Option<String> {
    let path = Path::new("tests/test-pages").join(name).join("source.html");
    fs::read_to_string(&path).ok()
}

fn bench_parse_by_size(c: &mut Criterion) {
    let mut group = c.benchmark_group("parse");

    let test_cases = [
        ("001", "small"),
        ("ars-1", "small"),
        ("medium-1", "medium"),
        ("nytimes-1", "medium"),
        ("guardian-1", "large"),
        ("yahoo-2", "large"),
    ];

    for (name, _size) in test_cases {
        let html = match load_test_case(name) {
            Some(h) => h,
            None => continue,
        };

        group.throughput(Throughput::Bytes(html.len() as u64));
        group.bench_with_input(BenchmarkId::new("doc", name), &html, |b, html| {
            b.iter(|| {
                let readability = Readability::new(std::hint::black_box(html), None, None).unwrap();
                std::hint::black_box(readability.parse())
            });
        });
    }

    group.finish();
}

fn bench_readerable_check(c: &mut Criterion) {
    let mut group = c.benchmark_group("readerable");

    let test_cases = ["001", "medium-1", "guardian-1"];

    for name in test_cases {
        let html = match load_test_case(name) {
            Some(h) => h,
            None => continue,
        };

        group.throughput(Throughput::Bytes(html.len() as u64));
        group.bench_with_input(BenchmarkId::new("check", name), &html, |b, html| {
            b.iter(|| {
                std::hint::black_box(is_probably_readerable(std::hint::black_box(html), None))
            });
        });
    }

    group.finish();
}

fn bench_batch(c: &mut Criterion) {
    let docs: Vec<String> = ["001", "002", "aclu", "ars-1", "bbc-1", "medium-1"]
        .iter()
        .filter_map(|name| load_test_case(name))
        .collect();

    if docs.is_empty() {
        return;
    }

    let total_bytes: usize = docs.iter().map(|d| d.len()).sum();

    let mut group = c.benchmark_group("batch");
    group.throughput(Throughput::Bytes(total_bytes as u64));
    group.bench_function("6_documents", |b| {
        b.iter(|| {
            for html in &docs {
                let readability = Readability::new(std::hint::black_box(html), None, None).unwrap();
                std::hint::black_box(readability.parse());
            }
        });
    });
    group.finish();
}

criterion_group!(
    benches,
    bench_parse_by_size,
    bench_readerable_check,
    bench_batch
);
criterion_main!(benches);

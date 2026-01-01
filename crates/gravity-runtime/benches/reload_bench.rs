use criterion::{black_box, criterion_group, criterion_main, Criterion, Throughput};
use gravity_core::parse;
use std::time::Instant;

/// Simulate hot-reload workflow
fn bench_hot_reload_latency(c: &mut Criterion) {
    let xml1 = r#"<column><text value="Version 1" /></column>"#;
    let xml2 = r#"<column><text value="Version 2" /></column>"#;

    let mut group = c.benchmark_group("hot_reload");
    group.throughput(Throughput::Elements(1));

    group.bench_function("reload_latency", |b| {
        b.iter(|| {
            // Simulate the hot-reload pipeline:
            // 1. Parse new XML
            let doc1 = parse(black_box(xml1)).unwrap();
            black_box(&doc1);

            // 2. Parse updated XML
            let doc2 = parse(black_box(xml2)).unwrap();
            black_box(&doc2);

            // 3. Compare (simulated state preservation)
            let _changed = doc1.root != doc2.root;
        });
    });

    group.finish();
}

/// Benchmark parsing + evaluation overhead
fn bench_parse_and_evaluate(c: &mut Criterion) {
    let xml = r#"<column spacing="{spacing}" padding="{padding}">
        <text value="Count: {count}" />
        <button label="{button_label}" on_click="handle" enabled="{count > 0}" />
    </column>"#;

    let mut group = c.benchmark_group("parse_evaluate");
    group.throughput(Throughput::Elements(1));

    group.bench_function("full_pipeline", |b| {
        b.iter(|| {
            // Parse
            let doc = parse(black_box(xml)).unwrap();
            black_box(&doc);

            // In a real scenario, bindings would be evaluated here
            // For benchmark, we just measure parse time
        });
    });

    group.finish();
}

criterion_group!(benches, bench_hot_reload_latency, bench_parse_and_evaluate);
criterion_main!(benches);

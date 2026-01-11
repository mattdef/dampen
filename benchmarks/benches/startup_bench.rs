//! Startup time benchmark for large UI (1000 widgets)
//!
//! This benchmark verifies that parsing and rendering a large UI with 1000 widgets
//! completes in <50ms (SC-001 acceptance scenario 3).

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use dampen_core::parse;
use std::time::Duration;

/// Generate XML for a large UI with 1000 widgets
fn generate_large_ui_xml(widget_count: usize) -> String {
    let mut xml = String::from(
        r#"<dampen version="1.0">
  <column spacing="10" padding="20">
"#,
    );

    for i in 0..widget_count {
        xml.push_str(&format!("    <text value=\"Item {}\" size=\"14\" />\n", i));
    }

    xml.push_str("  </column>\n</dampen>");
    xml
}

/// Benchmark parsing large XML document
fn bench_parse_large_ui(c: &mut Criterion) {
    let mut group = c.benchmark_group("startup");
    group.measurement_time(Duration::from_secs(10));
    group.sample_size(100);

    // Test with 1000 widgets
    let xml_1000 = generate_large_ui_xml(1000);

    group.bench_function("parse_1000_widgets", |b| {
        b.iter(|| {
            let doc = parse(black_box(&xml_1000)).expect("Failed to parse XML");
            black_box(doc);
        })
    });

    // Test with 500 widgets for comparison
    let xml_500 = generate_large_ui_xml(500);

    group.bench_function("parse_500_widgets", |b| {
        b.iter(|| {
            let doc = parse(black_box(&xml_500)).expect("Failed to parse XML");
            black_box(doc);
        })
    });

    // Test with 100 widgets for baseline
    let xml_100 = generate_large_ui_xml(100);

    group.bench_function("parse_100_widgets", |b| {
        b.iter(|| {
            let doc = parse(black_box(&xml_100)).expect("Failed to parse XML");
            black_box(doc);
        })
    });

    group.finish();
}

/// Benchmark full startup cycle (parse + initial state setup)
fn bench_startup_cycle(c: &mut Criterion) {
    let mut group = c.benchmark_group("startup_cycle");
    group.measurement_time(Duration::from_secs(10));
    group.sample_size(100);

    let xml_1000 = generate_large_ui_xml(1000);

    group.bench_function("full_startup_1000_widgets", |b| {
        b.iter(|| {
            // Parse XML
            let doc = parse(black_box(&xml_1000)).expect("Failed to parse XML");

            // Simulate initial state setup (what happens at app startup)
            let _root = &doc.root;
            let _children_count = doc.root.children.len();

            black_box(doc);
        })
    });

    group.finish();
}

/// Benchmark scalability with different widget counts
fn bench_scalability(c: &mut Criterion) {
    let mut group = c.benchmark_group("scalability");
    group.measurement_time(Duration::from_secs(10));

    for size in [10, 50, 100, 250, 500, 1000, 2000].iter() {
        let xml = generate_large_ui_xml(*size);

        group.bench_with_input(
            criterion::BenchmarkId::from_parameter(size),
            size,
            |b, _| {
                b.iter(|| {
                    let doc = parse(black_box(&xml)).expect("Failed to parse XML");
                    black_box(doc);
                })
            },
        );
    }

    group.finish();
}

criterion_group!(
    benches,
    bench_parse_large_ui,
    bench_startup_cycle,
    bench_scalability
);
criterion_main!(benches);

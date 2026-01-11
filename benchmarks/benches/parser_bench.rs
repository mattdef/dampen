use criterion::{black_box, criterion_group, criterion_main, Criterion, Throughput};
use dampen_core::parse;

/// Generate XML with 1000 widgets
fn generate_large_xml() -> String {
    let mut xml = String::from(r#"<column spacing="10" padding="20">"#);

    for i in 0..1000 {
        xml.push_str(&format!(
            r#"<text value="Widget {}" size="{}" />"#,
            i,
            12 + (i % 20)
        ));
    }

    xml.push_str("</column>");
    xml
}

fn bench_parse_1000_widgets(c: &mut Criterion) {
    let xml = generate_large_xml();

    let mut group = c.benchmark_group("parser");
    group.throughput(Throughput::Elements(1000));

    group.bench_function("parse_1000_widgets", |b| {
        b.iter(|| {
            let result = parse(black_box(&xml));
            black_box(result)
        });
    });

    group.finish();
}

fn bench_parse_small(c: &mut Criterion) {
    let xml =
        r#"<column><text value="Hello" /><button label="Click" on_click="handle" /></column>"#;

    let mut group = c.benchmark_group("parser");
    group.throughput(Throughput::Elements(3));

    group.bench_function("parse_small", |b| {
        b.iter(|| {
            let result = parse(black_box(xml));
            black_box(result)
        });
    });

    group.finish();
}

criterion_group!(benches, bench_parse_1000_widgets, bench_parse_small);
criterion_main!(benches);

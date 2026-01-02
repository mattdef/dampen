use criterion::{black_box, criterion_group, criterion_main, Criterion};
use gravity_core::parse;

fn bench_parse_1000_widgets(c: &mut Criterion) {
    // Create XML with 1000 text widgets
    let mut xml = String::from(r#"<?xml version="1.0" encoding="UTF-8"?><gravity><column>"#);
    for i in 0..1000 {
        xml.push_str(&format!("<text value=\"Widget {}\" />", i));
    }
    xml.push_str("</column></gravity>");
    
    c.bench_function("parse_1000_widgets", |b| {
        b.iter(|| {
            let result = parse(black_box(&xml));
            black_box(result)
        })
    });
}

criterion_group!(benches, bench_parse_1000_widgets);
criterion_main!(benches);

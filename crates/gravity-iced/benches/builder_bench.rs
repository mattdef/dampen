//! Performance benchmarks for GravityWidgetBuilder
//!
//! Run with: cargo bench -p gravity-iced

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use gravity_core::binding::{BindingValue, UiBindable};
use gravity_core::{parse, HandlerRegistry};
use gravity_iced::{GravityWidgetBuilder, HandlerMessage};
use iced::{Element, Renderer, Theme};

/// Test model for benchmarks
#[derive(Clone)]
struct BenchModel {
    count: i32,
    items: Vec<String>,
}

impl UiBindable for BenchModel {
    fn get_field(&self, path: &[&str]) -> Option<BindingValue> {
        match path {
            ["count"] => Some(BindingValue::Integer(self.count as i64)),
            ["items"] => Some(BindingValue::List(
                self.items
                    .iter()
                    .map(|s| BindingValue::String(s.clone()))
                    .collect(),
            )),
            _ => None,
        }
    }

    fn available_fields() -> Vec<String> {
        vec!["count".to_string(), "items".to_string()]
    }
}

fn create_model() -> BenchModel {
    BenchModel {
        count: 42,
        items: (0..100).map(|i| format!("Item {}", i)).collect(),
    }
}

fn create_registry() -> HandlerRegistry {
    HandlerRegistry::new()
}

/// Generate XML with N widgets
fn generate_large_ui(widget_count: usize) -> String {
    let mut xml = String::from("<column spacing=\"5\">\n");

    for i in 0..widget_count {
        if i % 3 == 0 {
            xml.push_str(&format!(
                "    <text value=\"Item {} - Count: {{count}}\" />\n",
                i
            ));
        } else if i % 3 == 1 {
            xml.push_str(&format!(
                "    <button label=\"Button {}\" on_click=\"handler{}\" />\n",
                i, i
            ));
        } else {
            xml.push_str("    <row spacing=\"2\">\n");
            xml.push_str(&format!("        <text value=\"Row {}\" />\n", i));
            xml.push_str(&format!("        <text value=\"{{count}}\" />\n"));
            xml.push_str("    </row>\n");
        }
    }

    xml.push_str("</column>");
    xml
}

// T071: Benchmark 1000 widget rendering
fn bench_1000_widgets(c: &mut Criterion) {
    let xml = generate_large_ui(1000);
    let doc = parse(&xml).expect("Failed to parse XML");
    let model = create_model();
    let registry = create_registry();

    c.bench_function("build_1000_widgets", |b| {
        b.iter(|| {
            let builder =
                GravityWidgetBuilder::new(black_box(&doc.root), black_box(&model), Some(&registry));
            let _element: Element<'_, HandlerMessage, Theme, Renderer> = builder.build();
        });
    });
}

// T072: Verify < 50ms target (smaller benchmark for more iterations)
fn bench_100_widgets(c: &mut Criterion) {
    let xml = generate_large_ui(100);
    let doc = parse(&xml).expect("Failed to parse XML");
    let model = create_model();
    let registry = create_registry();

    c.bench_function("build_100_widgets", |b| {
        b.iter(|| {
            let builder =
                GravityWidgetBuilder::new(black_box(&doc.root), black_box(&model), Some(&registry));
            let _element: Element<'_, HandlerMessage, Theme, Renderer> = builder.build();
        });
    });
}

// T073: Profile binding evaluation overhead
fn bench_binding_evaluation(c: &mut Criterion) {
    let xml = r#"
        <column spacing="10">
            <text value="Static text" />
            <text value="{count}" />
            <text value="Count: {count}, doubled: {count * 2}" />
            <text value="{if count > 10 then 'High' else 'Low'}" />
        </column>
    "#;
    let doc = parse(xml).expect("Failed to parse XML");
    let model = create_model();
    let registry = create_registry();

    c.bench_function("binding_evaluation", |b| {
        b.iter(|| {
            let builder =
                GravityWidgetBuilder::new(black_box(&doc.root), black_box(&model), Some(&registry));
            let _element: Element<'_, HandlerMessage, Theme, Renderer> = builder.build();
        });
    });
}

// T074: Profile event connection overhead
fn bench_event_connection(c: &mut Criterion) {
    let xml = r#"
        <column spacing="10">
            <button label="Button 1" on_click="handler1" />
            <button label="Button 2" on_click="handler2" />
            <button label="Button 3" on_click="handler3" />
            <button label="Button 4" on_click="handler4" />
            <button label="Button 5" on_click="handler5" />
        </column>
    "#;
    let doc = parse(xml).expect("Failed to parse XML");
    let model = create_model();
    let registry = create_registry();

    c.bench_function("event_connection", |b| {
        b.iter(|| {
            let builder =
                GravityWidgetBuilder::new(black_box(&doc.root), black_box(&model), Some(&registry));
            let _element: Element<'_, HandlerMessage, Theme, Renderer> = builder.build();
        });
    });
}

criterion_group!(
    benches,
    bench_1000_widgets,
    bench_100_widgets,
    bench_binding_evaluation,
    bench_event_connection
);
criterion_main!(benches);

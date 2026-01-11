//! Hot-reload performance benchmarks
//!
//! Measures the latency of hot-reload operations with various UI sizes
//! to validate the <300ms target from spec SC-002.

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use dampen_core::binding::UiBindable;
use dampen_core::handler::HandlerRegistry;
use dampen_core::parser;
use dampen_core::state::AppState;
use dampen_dev::reload::{attempt_hot_reload, HotReloadContext};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct BenchModel {
    count: i32,
    name: String,
    items: Vec<String>,
}

impl Default for BenchModel {
    fn default() -> Self {
        Self {
            count: 0,
            name: "Benchmark".to_string(),
            items: vec![],
        }
    }
}

impl UiBindable for BenchModel {
    fn get_field(&self, path: &[&str]) -> Option<dampen_core::binding::BindingValue> {
        match path {
            ["count"] => Some(dampen_core::binding::BindingValue::Integer(
                self.count as i64,
            )),
            ["name"] => Some(dampen_core::binding::BindingValue::String(
                self.name.clone(),
            )),
            _ => None,
        }
    }

    fn available_fields() -> Vec<String> {
        vec!["count".to_string(), "name".to_string(), "items".to_string()]
    }
}

fn create_handlers() -> HandlerRegistry {
    let registry = HandlerRegistry::new();
    registry.register_simple("increment", |model: &mut dyn std::any::Any| {
        if let Some(m) = model.downcast_mut::<BenchModel>() {
            m.count += 1;
        }
    });
    registry.register_simple("decrement", |model: &mut dyn std::any::Any| {
        if let Some(m) = model.downcast_mut::<BenchModel>() {
            m.count -= 1;
        }
    });
    registry
}

/// Generate XML with N widgets
fn generate_xml_with_widgets(widget_count: usize) -> String {
    let mut xml = String::from(r#"<dampen version="1.0"><column spacing="10">"#);

    for i in 0..widget_count {
        xml.push_str(&format!(r#"<text value="Item {}" size="16" />"#, i));
    }

    xml.push_str("</column></dampen>");
    xml
}

fn bench_hot_reload_small(c: &mut Criterion) {
    let xml_v1 = generate_xml_with_widgets(10);
    let xml_v2 = generate_xml_with_widgets(10); // Same size, different content

    let doc = parser::parse(&xml_v1).unwrap();
    let model = BenchModel::default();
    let registry = create_handlers();
    let state = AppState::with_all(doc, model, registry);

    c.bench_function("hot_reload_10_widgets", |b| {
        b.iter(|| {
            let mut context = HotReloadContext::<BenchModel>::new();
            let result = attempt_hot_reload(black_box(&xml_v2), &state, &mut context, || {
                create_handlers()
            });
            black_box(result);
        });
    });
}

fn bench_hot_reload_medium(c: &mut Criterion) {
    let xml_v1 = generate_xml_with_widgets(100);
    let xml_v2 = generate_xml_with_widgets(100);

    let doc = parser::parse(&xml_v1).unwrap();
    let model = BenchModel::default();
    let registry = create_handlers();
    let state = AppState::with_all(doc, model, registry);

    c.bench_function("hot_reload_100_widgets", |b| {
        b.iter(|| {
            let mut context = HotReloadContext::<BenchModel>::new();
            let result = attempt_hot_reload(black_box(&xml_v2), &state, &mut context, || {
                create_handlers()
            });
            black_box(result);
        });
    });
}

fn bench_hot_reload_large(c: &mut Criterion) {
    let xml_v1 = generate_xml_with_widgets(1000);
    let xml_v2 = generate_xml_with_widgets(1000);

    let doc = parser::parse(&xml_v1).unwrap();
    let model = BenchModel::default();
    let registry = create_handlers();
    let state = AppState::with_all(doc, model, registry);

    c.bench_function("hot_reload_1000_widgets", |b| {
        b.iter(|| {
            let mut context = HotReloadContext::<BenchModel>::new();
            let result = attempt_hot_reload(black_box(&xml_v2), &state, &mut context, || {
                create_handlers()
            });
            black_box(result);
        });
    });
}

fn bench_hot_reload_with_cache(c: &mut Criterion) {
    let xml = generate_xml_with_widgets(100);

    let doc = parser::parse(&xml).unwrap();
    let model = BenchModel::default();
    let registry = create_handlers();
    let state = AppState::with_all(doc, model, registry);

    let mut group = c.benchmark_group("hot_reload_cache");

    // First reload (cache miss)
    group.bench_function(BenchmarkId::new("cache_miss", 100), |b| {
        b.iter(|| {
            let mut context = HotReloadContext::<BenchModel>::new();
            let result =
                attempt_hot_reload(black_box(&xml), &state, &mut context, || create_handlers());
            black_box(result);
        });
    });

    // Second reload (cache hit)
    group.bench_function(BenchmarkId::new("cache_hit", 100), |b| {
        let mut context = HotReloadContext::<BenchModel>::new();
        // Prime the cache
        let _ = attempt_hot_reload(&xml, &state, &mut context, || create_handlers());

        b.iter(|| {
            let result =
                attempt_hot_reload(black_box(&xml), &state, &mut context, || create_handlers());
            black_box(result);
        });
    });

    group.finish();
}

fn bench_model_serialization(c: &mut Criterion) {
    let model = BenchModel {
        count: 42,
        name: "Test Model".to_string(),
        items: (0..100).map(|i| format!("Item {}", i)).collect(),
    };

    c.bench_function("model_serialization", |b| {
        b.iter(|| {
            let json = serde_json::to_string(black_box(&model)).unwrap();
            black_box(json);
        });
    });

    c.bench_function("model_deserialization", |b| {
        let json = serde_json::to_string(&model).unwrap();
        b.iter(|| {
            let model: BenchModel = serde_json::from_str(black_box(&json)).unwrap();
            black_box(model);
        });
    });
}

criterion_group!(
    benches,
    bench_hot_reload_small,
    bench_hot_reload_medium,
    bench_hot_reload_large,
    bench_hot_reload_with_cache,
    bench_model_serialization
);

criterion_main!(benches);

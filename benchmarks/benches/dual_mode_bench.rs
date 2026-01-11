//! Dual-mode architecture benchmarks
//!
//! Compares performance between interpreted and codegen modes
//! to validate the production performance targets.

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use dampen_core::binding::UiBindable;
use dampen_core::handler::HandlerRegistry;
use dampen_core::parser;
use dampen_core::state::AppState;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct CounterModel {
    count: i32,
}

impl Default for CounterModel {
    fn default() -> Self {
        Self { count: 0 }
    }
}

impl UiBindable for CounterModel {
    fn get_field(&self, path: &[&str]) -> Option<dampen_core::binding::BindingValue> {
        match path {
            ["count"] => Some(dampen_core::binding::BindingValue::Integer(
                self.count as i64,
            )),
            _ => None,
        }
    }

    fn available_fields() -> Vec<String> {
        vec!["count".to_string()]
    }
}

fn create_handlers() -> HandlerRegistry {
    let registry = HandlerRegistry::new();
    registry.register_simple("increment", |model: &mut dyn std::any::Any| {
        if let Some(m) = model.downcast_mut::<CounterModel>() {
            m.count += 1;
        }
    });
    registry.register_simple("decrement", |model: &mut dyn std::any::Any| {
        if let Some(m) = model.downcast_mut::<CounterModel>() {
            m.count -= 1;
        }
    });
    registry
}

const COUNTER_XML: &str = r#"
<dampen version="1.0">
    <column spacing="20" padding="40">
        <text value="Counter App" size="24" weight="bold" />
        <row spacing="10">
            <button label="Decrement" on_click="decrement" />
            <text value="{count}" size="20" />
            <button label="Increment" on_click="increment" />
        </row>
    </column>
</dampen>
"#;

fn bench_interpreted_mode_parsing(c: &mut Criterion) {
    c.bench_function("interpreted_parse_xml", |b| {
        b.iter(|| {
            let doc = parser::parse(black_box(COUNTER_XML)).unwrap();
            black_box(doc);
        });
    });
}

fn bench_interpreted_mode_state_creation(c: &mut Criterion) {
    let doc = parser::parse(COUNTER_XML).unwrap();

    c.bench_function("interpreted_create_state", |b| {
        b.iter(|| {
            let model = CounterModel::default();
            let registry = create_handlers();
            let state = AppState::with_all(black_box(doc.clone()), model, registry);
            black_box(state);
        });
    });
}

fn bench_interpreted_mode_binding_evaluation(c: &mut Criterion) {
    let model = CounterModel { count: 42 };

    c.bench_function("interpreted_binding_eval", |b| {
        b.iter(|| {
            let value = model.get_field(black_box(&["count"]));
            black_box(value);
        });
    });
}

fn bench_codegen_mode_field_access(c: &mut Criterion) {
    let model = CounterModel { count: 42 };

    c.bench_function("codegen_field_access", |b| {
        b.iter(|| {
            // Simulates generated code: model.count.to_string()
            let value = black_box(&model).count.to_string();
            black_box(value);
        });
    });
}

fn bench_mode_comparison(c: &mut Criterion) {
    let mut group = c.benchmark_group("mode_comparison");

    // Interpreted mode: binding evaluation
    let model = CounterModel { count: 100 };
    group.bench_function(BenchmarkId::new("interpreted", "binding_eval"), |b| {
        b.iter(|| {
            let value = model.get_field(black_box(&["count"]));
            let string = match value {
                Some(dampen_core::binding::BindingValue::Integer(i)) => i.to_string(),
                _ => String::new(),
            };
            black_box(string);
        });
    });

    // Codegen mode: direct field access
    group.bench_function(BenchmarkId::new("codegen", "direct_access"), |b| {
        b.iter(|| {
            let value = black_box(&model).count.to_string();
            black_box(value);
        });
    });

    group.finish();
}

fn bench_handler_dispatch(c: &mut Criterion) {
    let registry = create_handlers();
    let mut model = CounterModel::default();

    c.bench_function("handler_dispatch_interpreted", |b| {
        b.iter(|| {
            registry.dispatch(
                "increment",
                black_box(&mut model as &mut dyn std::any::Any),
                None,
            );
        });
    });

    // Codegen mode: direct function call
    c.bench_function("handler_dispatch_codegen", |b| {
        b.iter(|| {
            // Simulates generated code: direct function call
            black_box(&mut model).count += 1;
        });
    });
}

fn bench_xml_size_scaling(c: &mut Criterion) {
    let mut group = c.benchmark_group("xml_size_scaling");

    for size in [10, 50, 100, 500, 1000].iter() {
        let xml = generate_xml_with_widgets(*size);

        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, _| {
            b.iter(|| {
                let doc = parser::parse(black_box(&xml)).unwrap();
                black_box(doc);
            });
        });
    }

    group.finish();
}

fn generate_xml_with_widgets(count: usize) -> String {
    let mut xml = String::from(r#"<dampen version="1.0"><column>"#);
    for i in 0..count {
        xml.push_str(&format!("<text value=\"Widget {}\" />", i));
    }
    xml.push_str("</column></dampen>");
    xml
}

criterion_group!(
    benches,
    bench_interpreted_mode_parsing,
    bench_interpreted_mode_state_creation,
    bench_interpreted_mode_binding_evaluation,
    bench_codegen_mode_field_access,
    bench_mode_comparison,
    bench_handler_dispatch,
    bench_xml_size_scaling
);

criterion_main!(benches);

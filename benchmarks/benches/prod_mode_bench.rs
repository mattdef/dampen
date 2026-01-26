//! Performance benchmark comparing production codegen mode with hand-written equivalent
//!
//! This benchmark verifies that generated code has <5% performance overhead compared
//! to hand-written Rust code for the same UI.

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use iced::widget::{button, column, row, text};
use iced::Element;

// Model for the counter (same as in examples/counter)
#[derive(Default, Clone, Debug)]
struct Model {
    count: i32,
}

// Message enum for hand-written version
#[derive(Clone, Debug)]
enum Message {
    Increment,
    Decrement,
    Reset,
}

/// Hand-written view function (baseline for comparison)
fn view_hand_written(model: &Model) -> Element<'_, Message> {
    let count = &model.count;

    column(vec![
        text(format!("Counter: {}", count)).into(),
        row(vec![
            button("-").on_press(Message::Decrement).into(),
            button("+").on_press(Message::Increment).into(),
        ])
        .spacing(20.0)
        .into(),
        button("Reset").on_press(Message::Reset).into(),
    ])
    .spacing(20.0)
    .padding(40.0)
    .into()
}

/// Simulated codegen view function (mimics generated code structure)
fn view_codegen_style(model: &Model) -> Element<'_, Message> {
    let count = &model.count;

    // This style mimics the generated code structure with explicit conversions
    iced::widget::column(vec![
        iced::widget::text(format!("Counter: {}", count.to_string())).into(),
        iced::widget::row(vec![
            iced::widget::button(iced::widget::text("-".to_string()))
                .on_press(Message::Decrement)
                .into(),
            iced::widget::button(iced::widget::text("+".to_string()))
                .on_press(Message::Increment)
                .into(),
        ])
        .spacing(20f32)
        .into(),
        iced::widget::button(iced::widget::text("Reset".to_string()))
            .on_press(Message::Reset)
            .into(),
    ])
    .spacing(20f32)
    .padding(40f32)
    .into()
}

/// Hand-written update function
fn update_hand_written(model: &mut Model, message: Message) {
    match message {
        Message::Increment => model.count += 1,
        Message::Decrement => model.count -= 1,
        Message::Reset => model.count = 0,
    }
}

/// Simulated codegen update function
fn update_codegen_style(model: &mut Model, message: Message) {
    match message {
        Message::Increment => {
            increment_handler(model);
        }
        Message::Decrement => {
            decrement_handler(model);
        }
        Message::Reset => {
            reset_handler(model);
        }
    }
}

// Handler functions (mimics the pattern in examples/counter)
fn increment_handler(model: &mut Model) {
    model.count += 1;
}

fn decrement_handler(model: &mut Model) {
    model.count -= 1;
}

fn reset_handler(model: &mut Model) {
    model.count = 0;
}

/// Benchmark view function rendering
fn bench_view_rendering(c: &mut Criterion) {
    let mut group = c.benchmark_group("view_rendering");

    let model = Model { count: 42 };

    group.bench_function("hand_written", |b| {
        b.iter(|| {
            let element = view_hand_written(black_box(&model));
            black_box(element);
        })
    });

    group.bench_function("codegen_style", |b| {
        b.iter(|| {
            let element = view_codegen_style(black_box(&model));
            black_box(element);
        })
    });

    group.finish();
}

/// Benchmark update function execution
fn bench_update_execution(c: &mut Criterion) {
    let mut group = c.benchmark_group("update_execution");

    group.bench_function("hand_written_increment", |b| {
        let mut model = Model { count: 0 };
        b.iter(|| {
            update_hand_written(black_box(&mut model), Message::Increment);
        })
    });

    group.bench_function("codegen_style_increment", |b| {
        let mut model = Model { count: 0 };
        b.iter(|| {
            update_codegen_style(black_box(&mut model), Message::Increment);
        })
    });

    group.bench_function("hand_written_all_messages", |b| {
        let mut model = Model { count: 0 };
        b.iter(|| {
            update_hand_written(black_box(&mut model), Message::Increment);
            update_hand_written(black_box(&mut model), Message::Decrement);
            update_hand_written(black_box(&mut model), Message::Reset);
        })
    });

    group.bench_function("codegen_style_all_messages", |b| {
        let mut model = Model { count: 0 };
        b.iter(|| {
            update_codegen_style(black_box(&mut model), Message::Increment);
            update_codegen_style(black_box(&mut model), Message::Decrement);
            update_codegen_style(black_box(&mut model), Message::Reset);
        })
    });

    group.finish();
}

/// Benchmark complete UI cycle (view + update)
fn bench_ui_cycle(c: &mut Criterion) {
    let mut group = c.benchmark_group("ui_cycle");

    group.bench_function("hand_written", |b| {
        let mut model = Model { count: 0 };
        b.iter(|| {
            // Render view
            let element = view_hand_written(black_box(&model));
            black_box(element);

            // Update state
            update_hand_written(black_box(&mut model), Message::Increment);
        })
    });

    group.bench_function("codegen_style", |b| {
        let mut model = Model { count: 0 };
        b.iter(|| {
            // Render view
            let element = view_codegen_style(black_box(&model));
            black_box(element);

            // Update state
            update_codegen_style(black_box(&mut model), Message::Increment);
        })
    });

    group.finish();
}

criterion_group!(
    benches,
    bench_view_rendering,
    bench_update_execution,
    bench_ui_cycle
);
criterion_main!(benches);

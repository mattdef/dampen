use gravity::prelude::*;
use gravity_iced::IcedBackend;
use gravity_runtime::Runtime;
use serde::{Deserialize, Serialize};
use std::time::{Duration, Instant};

#[derive(Default, UiModel, Serialize, Deserialize)]
struct Model {
    // Basic bindings
    counter: i32,

    // Form inputs
    username: String,
    password: String,

    // Conditional rendering
    is_loading: bool,
    is_error: bool,

    // Lists
    new_item: String,
    items: Vec<String>,

    // Status tracking
    interaction_count: i32,
    last_action: String,
    #[ui_skip]
    start_time: Instant,
}

impl Model {
    fn session_time(&self) -> u64 {
        self.start_time.elapsed().as_secs()
    }

    fn total_length(&self) -> usize {
        self.items.iter().map(|s| s.len()).sum()
    }
}

#[derive(Clone, Debug)]
enum Message {
    // Basic
    Increment,
    Decrement,
    Reset,

    // Form
    UpdateUsername(String),
    UpdatePassword(String),

    // Conditional
    ToggleLoading,
    ToggleError,

    // Lists
    UpdateNewItem(String),
    AddItem,
    ClearItems,
    SortItems,

    // Other
    ShowAbout,
    Quit,
}

#[ui_handler]
fn increment(model: &mut Model) {
    model.counter += 1;
    model.interaction_count += 1;
    model.last_action = "Increment".to_string();
}

#[ui_handler]
fn decrement(model: &mut Model) {
    if model.counter > 0 {
        model.counter -= 1;
    }
    model.interaction_count += 1;
    model.last_action = "Decrement".to_string();
}

#[ui_handler]
fn reset(model: &mut Model) {
    model.counter = 0;
    model.interaction_count += 1;
    model.last_action = "Reset".to_string();
}

#[ui_handler]
fn update_username(model: &mut Model, value: String) {
    model.username = value;
    model.interaction_count += 1;
    model.last_action = "Update Username".to_string();
}

#[ui_handler]
fn update_password(model: &mut Model, value: String) {
    model.password = value;
    model.interaction_count += 1;
    model.last_action = "Update Password".to_string();
}

#[ui_handler]
fn toggle_loading(model: &mut Model) {
    model.is_loading = !model.is_loading;
    model.interaction_count += 1;
    model.last_action = "Toggle Loading".to_string();
}

#[ui_handler]
fn toggle_error(model: &mut Model) {
    model.is_error = !model.is_error;
    model.interaction_count += 1;
    model.last_action = "Toggle Error".to_string();
}

#[ui_handler]
fn update_new_item(model: &mut Model, value: String) {
    model.new_item = value;
    model.interaction_count += 1;
    model.last_action = "Update New Item".to_string();
}

#[ui_handler]
fn add_item(model: &mut Model) {
    if !model.new_item.is_empty() {
        model.items.push(model.new_item.clone());
        model.new_item.clear();
        model.interaction_count += 1;
        model.last_action = "Add Item".to_string();
    }
}

#[ui_handler]
fn clear_items(model: &mut Model) {
    model.items.clear();
    model.interaction_count += 1;
    model.last_action = "Clear Items".to_string();
}

#[ui_handler]
fn sort_items(model: &mut Model) {
    model.items.sort();
    model.interaction_count += 1;
    model.last_action = "Sort Items".to_string();
}

#[ui_handler]
fn show_about(model: &mut Model) {
    model.interaction_count += 1;
    model.last_action = "Show About".to_string();
    println!("Gravity Full Demo - v0.1.0");
    println!("A comprehensive demonstration of all framework features.");
}

#[ui_handler]
fn quit(_model: &mut Model) {
    println!("Goodbye!");
    std::process::exit(0);
}

fn main() {
    // Initialize model with start time
    let mut model = Model::default();
    model.start_time = Instant::now();

    // Run in dev mode with hot-reload
    let runtime = Runtime::new("ui/main.gravity", model);

    // Use Iced backend
    let backend = IcedBackend;

    // This would normally start the Iced application
    // For this example, we're just showing the structure
    println!("Full Demo Application Ready!");
    println!("Features demonstrated:");
    println!("- Basic bindings (counter)");
    println!("- Form inputs (username, password)");
    println!("- Conditional rendering (loading, error states)");
    println!("- Lists and collections");
    println!("- Layout containers");
    println!("- Status tracking");
    println!("\nTo run: cargo run --features dev");
}

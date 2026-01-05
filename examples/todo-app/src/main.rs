use gravity_core::{parse, BindingValue, HandlerRegistry, ToBindingValue};
use gravity_iced::{GravityWidgetBuilder, HandlerMessage};
use gravity_macros::{ui_handler, UiModel};
use iced::widget::canvas;
use iced::{Color, Element, Point, Rectangle, Renderer, Task, Theme};
use serde::{Deserialize, Serialize};
use std::any::Any;
use std::collections::HashMap;

// ============================================================================
// Data Models (T092-T094)
// ============================================================================

/// Priority level for todo items
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Priority {
    Low,
    Medium,
    High,
}

impl Priority {
    pub fn as_str(&self) -> &str {
        match self {
            Priority::Low => "Low",
            Priority::Medium => "Medium",
            Priority::High => "High",
        }
    }

    pub fn icon_path(&self) -> &str {
        match self {
            Priority::Low => "assets/priority-low.svg",
            Priority::Medium => "assets/priority-medium.svg",
            Priority::High => "assets/priority-high.svg",
        }
    }
}

impl Default for Priority {
    fn default() -> Self {
        Priority::Medium
    }
}

impl std::fmt::Display for Priority {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl ToBindingValue for Priority {
    fn to_binding_value(&self) -> BindingValue {
        BindingValue::String(self.to_string())
    }
}

/// Filter for displaying todos
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TodoFilter {
    All,
    Active,
    Completed,
}

impl TodoFilter {
    pub fn as_str(&self) -> &str {
        match self {
            TodoFilter::All => "All",
            TodoFilter::Active => "Active",
            TodoFilter::Completed => "Completed",
        }
    }

    pub fn matches(&self, completed: bool) -> bool {
        match self {
            TodoFilter::All => true,
            TodoFilter::Active => !completed,
            TodoFilter::Completed => completed,
        }
    }
}

impl Default for TodoFilter {
    fn default() -> Self {
        TodoFilter::All
    }
}

impl std::fmt::Display for TodoFilter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// Individual todo item
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TodoItem {
    pub id: usize,
    pub text: String,
    pub category: String,
    pub priority: Priority,
    pub completed: bool,
}

impl TodoItem {
    pub fn new(id: usize, text: String, category: String, priority: Priority) -> Self {
        Self {
            id,
            text,
            category,
            priority,
            completed: false,
        }
    }
}

impl ToBindingValue for TodoItem {
    fn to_binding_value(&self) -> BindingValue {
        let mut map = HashMap::new();
        map.insert("id".to_string(), BindingValue::Integer(self.id as i64));
        map.insert("text".to_string(), BindingValue::String(self.text.clone()));
        map.insert(
            "category".to_string(),
            BindingValue::String(self.category.clone()),
        );
        map.insert("priority".to_string(), self.priority.to_binding_value());
        map.insert("completed".to_string(), BindingValue::Bool(self.completed));
        BindingValue::Object(map)
    }
}

// ============================================================================
// Canvas Program for Statistics Chart (T106-T108)
// ============================================================================

/// Statistics chart showing 7-day completion trend
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct StatisticsChart {
    pub completion_history: Vec<f32>, // Last 7 days percentages
}

impl canvas::Program<Message> for StatisticsChart {
    type State = ();

    fn draw(
        &self,
        _state: &(),
        renderer: &Renderer,
        _theme: &Theme,
        bounds: Rectangle,
        _cursor: iced::mouse::Cursor,
    ) -> Vec<canvas::Geometry> {
        let mut frame = canvas::Frame::new(renderer, bounds.size());

        // Draw axes
        let x_axis = canvas::Path::line(
            Point::new(30.0, bounds.height - 30.0),
            Point::new(bounds.width - 10.0, bounds.height - 30.0),
        );
        frame.stroke(
            &x_axis,
            canvas::Stroke::default()
                .with_width(2.0)
                .with_color(Color::from_rgb(0.5, 0.5, 0.5)),
        );

        let y_axis = canvas::Path::line(
            Point::new(30.0, 10.0),
            Point::new(30.0, bounds.height - 30.0),
        );
        frame.stroke(
            &y_axis,
            canvas::Stroke::default()
                .with_width(2.0)
                .with_color(Color::from_rgb(0.5, 0.5, 0.5)),
        );

        // Draw data points
        if !self.completion_history.is_empty() {
            let data_width = bounds.width - 40.0;
            let data_height = bounds.height - 40.0;
            let point_spacing = data_width / (self.completion_history.len() as f32).max(1.0);

            for (i, &percentage) in self.completion_history.iter().enumerate() {
                let x = 30.0 + (i as f32 * point_spacing);
                let y = (bounds.height - 30.0) - (percentage.min(1.0) * data_height);

                let circle = canvas::Path::circle(Point::new(x, y), 4.0);
                frame.fill(&circle, Color::from_rgb(0.2, 0.6, 1.0));
            }
        }

        vec![frame.into_geometry()]
    }
}

// ============================================================================
// Application Model (T095)
// ============================================================================

/// Main application state
#[derive(UiModel, Debug, Clone, Serialize, Deserialize)]
pub struct TodoAppModel {
    // Todo items - exposed as BindingValue::List for <for> loops
    pub items: Vec<TodoItem>,

    // Filtered items cache (updated when items or filter changes)
    pub filtered_items_cache: Vec<TodoItem>,

    // Current filter (use string representation for bindings)
    #[ui_skip]
    pub current_filter: TodoFilter,

    // UI state
    pub new_item_text: String,
    pub selected_category: String,

    // Use string representation for priority bindings
    #[ui_skip]
    pub selected_priority: Priority,

    pub dark_mode: bool,

    // Computed properties (using i64 for binding compatibility)
    pub completed_count: i64,
    pub pending_count: i64,
    pub completion_percentage: f32,
    pub items_len: i64,

    // String representations for PickList bindings
    pub selected_priority_display: String,
    pub current_filter_display: String,

    // Next ID for new items (using i64 for binding compatibility)
    pub next_id: i64,

    // Canvas data (for statistics chart)
    #[ui_skip]
    pub statistics_chart: StatisticsChart,
}

impl TodoAppModel {
    /// Get string representation of current filter for bindings
    pub fn current_filter_str(&self) -> String {
        self.current_filter.to_string()
    }

    /// Get string representation of selected priority for bindings
    pub fn selected_priority_str(&self) -> String {
        self.selected_priority.to_string()
    }
}

impl Default for TodoAppModel {
    fn default() -> Self {
        Self {
            items: Vec::new(),
            filtered_items_cache: Vec::new(),
            current_filter: TodoFilter::All,
            new_item_text: String::new(),
            selected_category: "Personal".to_string(),
            selected_priority: Priority::Medium,
            dark_mode: false,
            completed_count: 0,
            pending_count: 0,
            completion_percentage: 0.0,
            items_len: 0,
            selected_priority_display: "Medium".to_string(),
            current_filter_display: "All".to_string(),
            next_id: 1,
            statistics_chart: StatisticsChart::default(),
        }
    }
}

impl TodoAppModel {
    /// Update computed counts (T105)
    pub fn update_counts(&mut self) {
        self.items_len = self.items.len() as i64;
        self.completed_count = self.items.iter().filter(|i| i.completed).count() as i64;
        self.pending_count = self.items_len - self.completed_count;

        self.completion_percentage = if self.items.is_empty() {
            0.0
        } else {
            (self.completed_count as f32 / self.items.len() as f32) * 100.0
        };

        self.update_filtered_items();
    }

    /// Get filtered items based on current filter
    pub fn filtered_items(&self) -> Vec<&TodoItem> {
        self.items
            .iter()
            .filter(|item| self.current_filter.matches(item.completed))
            .collect()
    }

    /// Update filtered items property (for binding)
    /// Called after any filter or items change
    fn update_filtered_items(&mut self) {
        self.filtered_items_cache = self
            .items
            .iter()
            .filter(|item| self.current_filter.matches(item.completed))
            .cloned()
            .collect();
    }
}

// ============================================================================
// Messages
// ============================================================================

type Message = HandlerMessage;

// ============================================================================
// Event Handlers (T096-T104)
// ============================================================================

#[ui_handler]
fn add_item(model: &mut TodoAppModel) {
    if !model.new_item_text.is_empty() {
        let item = TodoItem::new(
            model.next_id as usize,
            model.new_item_text.clone(),
            model.selected_category.clone(),
            model.selected_priority,
        );
        model.items.push(item);
        model.next_id += 1;
        model.new_item_text.clear();
        model.update_counts();
        println!(
            "Added item: {} (Category: {}, Priority: {})",
            model.items.last().unwrap().text,
            model.selected_category,
            model.selected_priority
        );
    }
}

#[ui_handler]
fn toggle_item(model: &mut TodoAppModel, id: i64) {
    let id = id as usize;
    if let Some(item) = model.items.iter_mut().find(|i| i.id == id) {
        item.completed = !item.completed;
    }
    model.update_counts();
    println!("Toggled item {}", id);
}

#[ui_handler]
fn delete_item(model: &mut TodoAppModel, id: i64) {
    let id = id as usize;
    model.items.retain(|i| i.id != id);
    model.update_counts();
    println!("Deleted item {}", id);
}

#[ui_handler]
fn clear_all(model: &mut TodoAppModel) {
    let count = model.items.len();
    model.items.clear();
    model.update_counts();
    println!("Cleared {} items", count);
}

#[ui_handler]
fn clear_completed(model: &mut TodoAppModel) {
    let before = model.items.len();
    model.items.retain(|i| !i.completed);
    model.update_counts();
    println!("Cleared {} completed items", before - model.items.len());
}

#[ui_handler]
fn update_category(model: &mut TodoAppModel, value: String) {
    model.selected_category = value;
    println!("Selected category: {}", model.selected_category);
}

#[ui_handler]
fn update_priority(model: &mut TodoAppModel, value: String) {
    model.selected_priority = match value.as_str() {
        "Low" => Priority::Low,
        "Medium" => Priority::Medium,
        "High" => Priority::High,
        _ => Priority::Medium,
    };
    model.selected_priority_display = value;
    println!("Selected priority: {}", model.selected_priority);
}

#[ui_handler]
fn apply_filter(model: &mut TodoAppModel, value: String) {
    model.current_filter = match value.as_str() {
        "All" => TodoFilter::All,
        "Active" => TodoFilter::Active,
        "Completed" => TodoFilter::Completed,
        _ => TodoFilter::All,
    };
    model.current_filter_display = value;
    model.update_filtered_items();
    println!("Applied filter: {}", model.current_filter);
}

#[ui_handler]
fn toggle_dark_mode(model: &mut TodoAppModel) {
    model.dark_mode = !model.dark_mode;
    println!("Dark mode: {}", model.dark_mode);
}

#[ui_handler]
fn update_new_item(model: &mut TodoAppModel, value: String) {
    model.new_item_text = value;
}

// ============================================================================
// Application State (T117-T119)
// ============================================================================

struct AppState {
    model: TodoAppModel,
    document: gravity_core::GravityDocument,
    handler_registry: HandlerRegistry,
}

impl AppState {
    fn new() -> Self {
        let xml = include_str!("../ui/main.gravity");
        let document = parse(xml).expect("Failed to parse XML");

        let handler_registry = HandlerRegistry::new();

        // Register simple handlers
        handler_registry.register_simple("add_item", |model: &mut dyn Any| {
            let model = model.downcast_mut::<TodoAppModel>().unwrap();
            add_item(model);
        });

        handler_registry.register_simple("clear_all", |model: &mut dyn Any| {
            let model = model.downcast_mut::<TodoAppModel>().unwrap();
            clear_all(model);
        });

        handler_registry.register_simple("clear_completed", |model: &mut dyn Any| {
            let model = model.downcast_mut::<TodoAppModel>().unwrap();
            clear_completed(model);
        });

        handler_registry.register_simple("toggle_dark_mode", |model: &mut dyn Any| {
            let model = model.downcast_mut::<TodoAppModel>().unwrap();
            toggle_dark_mode(model);
        });

        // Register handlers with values
        handler_registry.register_with_value(
            "update_new_item",
            |model: &mut dyn Any, value: Box<dyn Any>| {
                let model = model.downcast_mut::<TodoAppModel>().unwrap();
                if let Ok(text) = value.downcast::<String>() {
                    update_new_item(model, *text);
                }
            },
        );

        handler_registry.register_with_value(
            "update_category",
            |model: &mut dyn Any, value: Box<dyn Any>| {
                let model = model.downcast_mut::<TodoAppModel>().unwrap();
                if let Ok(text) = value.downcast::<String>() {
                    update_category(model, *text);
                }
            },
        );

        handler_registry.register_with_value(
            "update_priority",
            |model: &mut dyn Any, value: Box<dyn Any>| {
                let model = model.downcast_mut::<TodoAppModel>().unwrap();
                if let Ok(text) = value.downcast::<String>() {
                    update_priority(model, *text);
                }
            },
        );

        handler_registry.register_with_value(
            "apply_filter",
            |model: &mut dyn Any, value: Box<dyn Any>| {
                let model = model.downcast_mut::<TodoAppModel>().unwrap();
                if let Ok(text) = value.downcast::<String>() {
                    apply_filter(model, *text);
                }
            },
        );

        handler_registry.register_with_value(
            "toggle_item",
            |model: &mut dyn Any, value: Box<dyn Any>| {
                let model = model.downcast_mut::<TodoAppModel>().unwrap();
                // Try String first (most common from UI), then i64
                let id_value = if let Some(text) = value.downcast_ref::<String>() {
                    text.parse::<i64>().ok()
                } else if let Some(id) = value.downcast_ref::<i64>() {
                    Some(*id)
                } else {
                    None
                };

                if let Some(id) = id_value {
                    toggle_item(model, id);
                }
            },
        );

        handler_registry.register_with_value(
            "delete_item",
            |model: &mut dyn Any, value: Box<dyn Any>| {
                let model = model.downcast_mut::<TodoAppModel>().unwrap();
                // Try String first (most common from UI), then i64
                let id_value = if let Some(text) = value.downcast_ref::<String>() {
                    text.parse::<i64>().ok()
                } else if let Some(id) = value.downcast_ref::<i64>() {
                    Some(*id)
                } else {
                    None
                };

                if let Some(id) = id_value {
                    delete_item(model, id);
                }
            },
        );

        Self {
            model: TodoAppModel::default(),
            document,
            handler_registry,
        }
    }
}

/// Update function
fn update(state: &mut AppState, message: Message) -> Task<Message> {
    match message {
        HandlerMessage::Handler(name, value_opt) => {
            if let Some(value) = value_opt {
                // Try handler with value first
                if let Some(gravity_core::HandlerEntry::WithValue(h)) =
                    state.handler_registry.get(&name)
                {
                    h(&mut state.model, Box::new(value));
                } else if let Some(gravity_core::HandlerEntry::Simple(h)) =
                    state.handler_registry.get(&name)
                {
                    // Fallback to simple handler (ignore the value)
                    h(&mut state.model);
                }
            } else {
                // Simple handler
                if let Some(gravity_core::HandlerEntry::Simple(h)) =
                    state.handler_registry.get(&name)
                {
                    h(&mut state.model);
                }
            }
        }
    }
    Task::none()
}

/// View function using GravityWidgetBuilder
fn view(state: &AppState) -> Element<'_, Message> {
    GravityWidgetBuilder::new(
        &state.document.root,
        &state.model,
        Some(&state.handler_registry),
    )
    .build()
}

pub fn main() -> iced::Result {
    iced::application(AppState::new, update, view)
        .theme(|state: &AppState| {
            if state.model.dark_mode {
                Theme::Dark
            } else {
                Theme::Light
            }
        })
        .run()
}

//! Dev command - runs app with hot-reload

#![allow(clippy::print_stderr)]

use gravity_core::ir::layout::Breakpoint;
use gravity_core::{AttributeValue, EventKind, HandlerRegistry, InterpolatedPart, WidgetNode};
use gravity_iced::state::WidgetStateManager;
use gravity_runtime::{resolve_tree_breakpoint_attributes, ErrorOverlay, HotReloadInterpreter};
use iced::time;
use iced::window;
use iced::{Element, Subscription, Task};
use std::fs;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

#[derive(clap::Args)]
pub struct DevArgs {
    /// UI directory containing .gravity files
    #[arg(short, long, default_value = "ui")]
    ui: String,

    /// Main .gravity file (relative to ui directory)
    #[arg(long, default_value = "main.gravity")]
    file: String,

    /// State file for persistence
    #[arg(long, default_value = ".gravity-state.json")]
    state: String,

    /// Verbose output
    #[arg(short, long)]
    verbose: bool,
}

pub fn execute(args: &DevArgs) -> Result<(), String> {
    println!("Starting Gravity Dev Mode...");
    println!("UI Directory: {}", args.ui);
    println!("Main File: {}", args.file);
    println!("State File: {}", args.state);
    println!("\nPress Ctrl+C to stop\n");

    // Build full paths
    let ui_path = PathBuf::from(&args.ui);
    let main_file = ui_path.join(&args.file);
    let state_file = PathBuf::from(&args.state);

    if !main_file.exists() {
        return Err(format!("Main UI file not found: {}", main_file.display()));
    }

    // Read initial UI
    let initial_xml = fs::read_to_string(&main_file)
        .map_err(|e| format!("Failed to read {}: {}", main_file.display(), e))?;

    // Create handler registry (empty for now)
    let registry = HandlerRegistry::new();

    // Create hot-reload interpreter
    let interpreter = HotReloadInterpreter::new(registry).with_state_file(state_file.clone());

    // Create shared state
    let shared_state = Arc::new(Mutex::new(DevState {
        interpreter,
        xml: initial_xml,
        file_path: main_file.clone(),
        error_overlay: None,
        verbose: args.verbose,
        last_modified: None,
        viewport_width: 800.0, // Default window width
        current_breakpoint: Breakpoint::from_viewport_width(800.0),
        previous_breakpoint: None,
        widget_state: WidgetStateManager::new(),
    }));

    // Load initial document
    {
        #[allow(clippy::unwrap_used)]
        let mut state = shared_state.lock().unwrap();
        let xml = state.xml.clone();
        if let Err(e) = state.interpreter.load_document(&xml) {
            eprintln!("Error loading initial document: {}", e);
            return Err(e);
        }
    }

    // Clone state for closures
    let state_clone = shared_state.clone();

    // Run Iced application using the builder
    let result = iced::application(
        move || State {
            state: state_clone.clone(),
        },
        update,
        view,
    )
    .subscription(subscription)
    .window(iced::window::Settings {
        size: iced::Size::new(800.0, 600.0),
        ..Default::default()
    })
    .run();

    result.map_err(|e| format!("Failed to run app: {}", e))
}

/// Shared state for the dev application
struct DevState {
    interpreter: HotReloadInterpreter,
    xml: String,
    file_path: PathBuf,
    error_overlay: Option<ErrorOverlay>,
    verbose: bool,
    last_modified: Option<std::time::SystemTime>,
    /// Current viewport width in pixels
    viewport_width: f32,
    /// Current breakpoint based on viewport width
    current_breakpoint: Breakpoint,
    /// Previous breakpoint for change detection
    previous_breakpoint: Option<Breakpoint>,
    /// Widget state manager for hover/focus/active/disabled
    widget_state: WidgetStateManager,
}

/// Application state
struct State {
    state: Arc<Mutex<DevState>>,
}

/// Messages for the dev app
#[derive(Clone, Debug)]
#[allow(dead_code)] // Some variants used in state tracking
enum Message {
    FileChanged,
    ReloadComplete(Result<(), String>),
    DismissError,
    Tick,
    WindowResized(f32),
    // Widget state messages
    WidgetMouseEnter(String),
    WidgetMouseLeave(String),
    WidgetMousePress(String),
    WidgetMouseRelease(String),
    WidgetFocus(String),
    WidgetBlur(String),
}

fn update(state: &mut State, message: Message) -> Task<Message> {
    match message {
        Message::Tick => {
            // Check for file changes
            #[allow(clippy::unwrap_used)]
            let mut dev_state = state.state.lock().unwrap();

            if let Ok(metadata) = dev_state.file_path.metadata() {
                if let Ok(modified) = metadata.modified() {
                    if let Some(last) = dev_state.last_modified {
                        if modified > last {
                            // File changed
                            if dev_state.verbose {
                                eprintln!("[DEV] File modified detected");
                            }
                            dev_state.last_modified = Some(modified);
                            drop(dev_state);
                            return reload_file(state.state.clone());
                        }
                    } else {
                        // First time - store the modification time
                        dev_state.last_modified = Some(modified);
                        if dev_state.verbose {
                            eprintln!("[DEV] Initialized last_modified");
                        }
                    }
                }
            }
        }
        Message::FileChanged => {
            return reload_file(state.state.clone());
        }
        Message::ReloadComplete(result) => {
            #[allow(clippy::unwrap_used)]
            let mut dev_state = state.state.lock().unwrap();
            match result {
                Ok(_) => {
                    if dev_state.verbose {
                        eprintln!("[DEV] ✓ Reload successful");
                    }
                    dev_state.error_overlay = None;
                    // Clear widget states on successful reload
                    dev_state.widget_state.clear();
                }
                Err(e) => {
                    eprintln!("[DEV] ✗ Reload failed: {}", e);
                    dev_state.error_overlay = Some(ErrorOverlay::new("Reload Failed", &e));
                }
            }
        }
        Message::DismissError => {
            #[allow(clippy::unwrap_used)]
            let mut dev_state = state.state.lock().unwrap();
            dev_state.error_overlay = None;
        }
        Message::WindowResized(width) => {
            #[allow(clippy::unwrap_used)]
            let mut dev_state = state.state.lock().unwrap();

            // Update viewport width
            dev_state.viewport_width = width;

            // Calculate new breakpoint
            let new_breakpoint = Breakpoint::from_viewport_width(width);

            // Store previous breakpoint for change detection
            dev_state.previous_breakpoint = Some(dev_state.current_breakpoint);

            // Update current breakpoint
            dev_state.current_breakpoint = new_breakpoint;

            if dev_state.verbose {
                eprintln!(
                    "[DEV] Window resized to {}px, breakpoint: {:?}",
                    width, new_breakpoint
                );
            }
        }
        // Widget state management
        Message::WidgetMouseEnter(widget_id) => {
            #[allow(clippy::unwrap_used)]
            let mut dev_state = state.state.lock().unwrap();
            dev_state.widget_state.on_mouse_enter(widget_id);
            if dev_state.verbose {
                eprintln!("[DEV] Widget mouse enter");
            }
        }
        Message::WidgetMouseLeave(widget_id) => {
            #[allow(clippy::unwrap_used)]
            let mut dev_state = state.state.lock().unwrap();
            dev_state.widget_state.on_mouse_leave(widget_id);
            if dev_state.verbose {
                eprintln!("[DEV] Widget mouse leave");
            }
        }
        Message::WidgetMousePress(widget_id) => {
            #[allow(clippy::unwrap_used)]
            let mut dev_state = state.state.lock().unwrap();
            dev_state.widget_state.on_mouse_press(widget_id);
            if dev_state.verbose {
                eprintln!("[DEV] Widget mouse press");
            }
        }
        Message::WidgetMouseRelease(widget_id) => {
            #[allow(clippy::unwrap_used)]
            let mut dev_state = state.state.lock().unwrap();
            dev_state.widget_state.on_mouse_release(widget_id);
            if dev_state.verbose {
                eprintln!("[DEV] Widget mouse release");
            }
        }
        Message::WidgetFocus(widget_id) => {
            #[allow(clippy::unwrap_used)]
            let mut dev_state = state.state.lock().unwrap();
            dev_state.widget_state.on_focus(widget_id);
            if dev_state.verbose {
                eprintln!("[DEV] Widget focus");
            }
        }
        Message::WidgetBlur(widget_id) => {
            #[allow(clippy::unwrap_used)]
            let mut dev_state = state.state.lock().unwrap();
            dev_state.widget_state.on_blur(widget_id);
            if dev_state.verbose {
                eprintln!("[DEV] Widget blur");
            }
        }
    }
    Task::none()
}

fn view(state: &State) -> Element<'_, Message> {
    // Clone the data we need while holding the lock
    let (error_overlay, doc, viewport_width, widget_state) = {
        #[allow(clippy::unwrap_used)]
        let dev_state = state.state.lock().unwrap();
        (
            dev_state.error_overlay.clone(),
            dev_state.interpreter.document().cloned(),
            dev_state.viewport_width,
            dev_state.widget_state.clone(),
        )
    };

    // Show error overlay if present
    if let Some(overlay) = error_overlay {
        return iced::widget::container(
            iced::widget::column![
                iced::widget::text(overlay.title).size(24),
                iced::widget::text(overlay.message).size(16),
                if let Some(loc) = overlay.location {
                    iced::widget::text(loc).size(12)
                } else {
                    iced::widget::text("")
                },
                if let Some(sugg) = overlay.suggestion {
                    iced::widget::text(format!("Suggestion: {}", sugg)).size(12)
                } else {
                    iced::widget::text("")
                },
                iced::widget::button(iced::widget::text("Dismiss"))
                    .on_press(Message::DismissError)
                    .padding([8, 16])
            ]
            .spacing(10)
            .padding(20),
        )
        .style(|_theme| iced::widget::container::Style {
            background: Some(iced::Background::Color(iced::Color::from_rgb(
                0.8, 0.2, 0.2,
            ))),
            text_color: Some(iced::Color::WHITE),
            border: iced::Border {
                radius: 8.0.into(),
                width: 2.0,
                color: iced::Color::from_rgb(0.6, 0.1, 0.1),
            },
            ..Default::default()
        })
        .into();
    }

    // Render the UI from the document
    if let Some(doc) = doc {
        // Resolve breakpoint attributes for the entire tree
        let resolved_root = resolve_tree_breakpoint_attributes(&doc.root, viewport_width);

        // Render with resolved attributes and state
        render_widget_tree(&resolved_root, &doc, &widget_state)
    } else {
        iced::widget::container(iced::widget::text("No document loaded").size(20)).into()
    }
}

fn subscription(_state: &State) -> Subscription<Message> {
    // Subscribe to periodic ticks for file checking (every 200ms)
    // Subscribe to window resize events
    Subscription::batch([
        time::every(std::time::Duration::from_millis(200)).map(|_| Message::Tick),
        window::resize_events().map(|(_id, size)| Message::WindowResized(size.width)),
    ])
}

fn reload_file(state: Arc<Mutex<DevState>>) -> Task<Message> {
    #[allow(clippy::unwrap_used)]
    let dev_state = state.lock().unwrap();
    let file_path = dev_state.file_path.clone();
    let verbose = dev_state.verbose;
    drop(dev_state);

    // Read file in a task
    Task::perform(
        async move {
            let new_xml = fs::read_to_string(&file_path)
                .map_err(|e| format!("Failed to read file: {}", e))?;
            Ok(new_xml)
        },
        move |result: Result<String, String>| {
            match result {
                Ok(new_xml) => {
                    #[allow(clippy::unwrap_used)]
                    let mut dev_state = state.lock().unwrap();

                    if new_xml == dev_state.xml {
                        return Message::Tick; // No change
                    }

                    if verbose {
                        eprintln!("[DEV] File changed, reloading...");
                    }

                    // Reload
                    let result = dev_state.interpreter.reload_document(&new_xml);

                    // Update stored XML
                    dev_state.xml = new_xml;

                    Message::ReloadComplete(result.map(|_| ()))
                }
                Err(e) => Message::ReloadComplete(Err(e)),
            }
        },
    )
}

/// Render a widget tree using Iced widgets with state-based styling
fn render_widget_tree(
    node: &WidgetNode,
    doc: &gravity_core::ir::GravityDocument,
    state_manager: &WidgetStateManager,
) -> Element<'static, Message> {
    use gravity_iced::style_mapping::{map_layout_constraints, map_style_properties};
    use iced::widget::container;

    // Get spacing from the node
    let spacing = node.layout.as_ref().and_then(|l| l.spacing);

    // Resolve style for this widget (base + classes + state)
    let resolved_style = resolve_widget_style(node, doc, state_manager);

    // Render base widget based on kind
    // Render base widget
    let widget = match node.kind {
        gravity_core::WidgetKind::Text => {
            let value = node
                .attributes
                .get("value")
                .map(|attr| match attr {
                    AttributeValue::Static(s) => s.clone(),
                    AttributeValue::Binding(_) => "[binding]".to_string(),
                    AttributeValue::Interpolated(parts) => {
                        let mut s = String::new();
                        for part in parts {
                            match part {
                                InterpolatedPart::Literal(l) => s.push_str(l),
                                InterpolatedPart::Binding(_) => s.push_str("[binding]"),
                            }
                        }
                        s
                    }
                })
                .unwrap_or_else(|| "No value".to_string());

            // Apply text color from style if present
            let mut text_widget = iced::widget::text(value);
            if let Some(color) = &resolved_style.color {
                text_widget = text_widget.color(gravity_iced::style_mapping::map_color(color));
            }
            text_widget.into()
        }
        gravity_core::WidgetKind::Button => {
            let label = node
                .attributes
                .get("label")
                .map(|attr| match attr {
                    AttributeValue::Static(s) => s.clone(),
                    _ => "[button]".to_string(),
                })
                .unwrap_or_else(|| "Button".to_string());

            // Check for click handler
            let has_handler = node.events.iter().any(|e| e.event == EventKind::Click);

            // Create button
            let mut btn = iced::widget::button(iced::widget::text(label));

            // Apply click handler
            if has_handler {
                // For now, just use a placeholder message
                // In a full implementation, this would map to actual handlers
                btn = btn.on_press(Message::Tick);
            }

            btn.into()
        }
        gravity_core::WidgetKind::Column => {
            let children: Vec<Element<Message>> = node
                .children
                .iter()
                .map(|child| render_widget_tree(child, doc, state_manager))
                .collect();

            let mut col = iced::widget::column(children);
            if let Some(s) = spacing {
                col = col.spacing(s);
            }
            col.into()
        }
        gravity_core::WidgetKind::Row => {
            let children: Vec<Element<Message>> = node
                .children
                .iter()
                .map(|child| render_widget_tree(child, doc, state_manager))
                .collect();

            let mut row = iced::widget::row(children);
            if let Some(s) = spacing {
                row = row.spacing(s);
            }
            row.into()
        }
        gravity_core::WidgetKind::Container => {
            let children: Vec<Element<Message>> = node
                .children
                .iter()
                .map(|child| render_widget_tree(child, doc, state_manager))
                .collect();

            let mut col = iced::widget::column(children);
            if let Some(s) = spacing {
                col = col.spacing(s);
            }
            col.into()
        }
        _ => {
            // For unsupported widgets, show a placeholder
            iced::widget::text(format!("[{:?}]", node.kind)).into()
        }
    };

    // Apply layout constraints and style via container wrapper
    let has_layout = node.layout.is_some();
    let has_style = resolved_style.background.is_some()
        || resolved_style.color.is_some()
        || resolved_style.border.is_some()
        || resolved_style.shadow.is_some()
        || resolved_style.opacity.is_some()
        || resolved_style.transform.is_some();

    if has_layout || has_style {
        let mut container = container(widget);

        // Apply layout
        if let Some(layout) = &node.layout {
            let iced_layout = map_layout_constraints(layout);
            container = container
                .width(iced_layout.width)
                .height(iced_layout.height)
                .padding(iced_layout.padding);
            if let Some(align) = iced_layout.align_items {
                container = container.align_y(align);
            }
        }

        // Apply style
        if has_style {
            let container_style = map_style_properties(&resolved_style);
            container = container.style(move |_theme| container_style);
        }

        return container.into();
    }

    widget
}

/// Resolve the final style for a widget by merging base style, class styles, and state styles
fn resolve_widget_style(
    node: &WidgetNode,
    doc: &gravity_core::ir::GravityDocument,
    state_manager: &WidgetStateManager,
) -> gravity_core::ir::style::StyleProperties {
    use gravity_runtime::StyleCascade;

    // Get base style from node
    let base_style = node.style.clone().unwrap_or_default();

    // Get class names
    let class_names = &node.classes;

    // Get theme style (if any)
    let theme_style = None; // Would need theme manager for this

    // Create cascade with document
    let mut cascade = StyleCascade::new(doc);

    // Get widget state
    if let Some(id) = &node.id {
        if let Some(state) = state_manager.get_state(id) {
            cascade = cascade.with_state(state);
        }
    }

    // Resolve final style
    cascade.resolve(Some(&base_style), class_names, theme_style)
}

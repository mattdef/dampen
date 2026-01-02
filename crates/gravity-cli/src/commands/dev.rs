//! Dev command - runs app with hot-reload

use gravity_core::ir::layout::Breakpoint;
use gravity_core::{AttributeValue, EventKind, HandlerRegistry, InterpolatedPart, WidgetNode};
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
}

/// Application state
struct State {
    state: Arc<Mutex<DevState>>,
}

/// Messages for the dev app
#[derive(Clone, Debug)]
enum Message {
    FileChanged,
    ReloadComplete(Result<(), String>),
    DismissError,
    Tick,
    WindowResized(f32),
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
    }
    Task::none()
}

fn view(state: &State) -> Element<'_, Message> {
    // Clone the data we need while holding the lock
    let (error_overlay, doc, viewport_width) = {
        #[allow(clippy::unwrap_used)]
        let dev_state = state.state.lock().unwrap();
        (
            dev_state.error_overlay.clone(),
            dev_state.interpreter.document().cloned(),
            dev_state.viewport_width,
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

        // Render with resolved attributes
        render_widget_tree(&resolved_root)
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

/// Render a widget tree using Iced widgets
fn render_widget_tree(node: &WidgetNode) -> Element<'static, Message> {
    // Get layout attributes from the node
    let width = node.layout.as_ref().and_then(|l| l.width.as_ref());
    let height = node.layout.as_ref().and_then(|l| l.height.as_ref());
    let _padding = node.layout.as_ref().and_then(|l| l.padding.as_ref());
    let spacing = node.layout.as_ref().and_then(|l| l.spacing);

    match node.kind {
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

            let text_widget = iced::widget::text(value);

            // Apply width/height if specified
            apply_size_constraints(text_widget.into(), width, height)
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

            let btn = if has_handler {
                iced::widget::button(iced::widget::text(label)).on_press(Message::Tick)
            // Placeholder
            } else {
                iced::widget::button(iced::widget::text(label))
            };

            // Note: Padding handling simplified for dev mode
            // In production, use gravity-iced style_mapping for full support

            apply_size_constraints(btn.into(), width, height)
        }
        gravity_core::WidgetKind::Column => {
            let children: Vec<Element<Message>> = node
                .children
                .iter()
                .map(|child| render_widget_tree(child))
                .collect();

            let mut col = iced::widget::column(children);

            // Apply spacing
            if let Some(s) = spacing {
                col = col.spacing(s);
            }

            // Note: Padding handling simplified
            apply_size_constraints(col.into(), width, height)
        }
        gravity_core::WidgetKind::Row => {
            let children: Vec<Element<Message>> = node
                .children
                .iter()
                .map(|child| render_widget_tree(child))
                .collect();

            let mut row = iced::widget::row(children);

            // Apply spacing
            if let Some(s) = spacing {
                row = row.spacing(s);
            }

            // Note: Padding handling simplified
            apply_size_constraints(row.into(), width, height)
        }
        gravity_core::WidgetKind::Container => {
            let children: Vec<Element<Message>> = node
                .children
                .iter()
                .map(|child| render_widget_tree(child))
                .collect();

            // Container wraps its children in a column for now
            let mut container = iced::widget::column(children);

            // Apply spacing
            if let Some(s) = spacing {
                container = container.spacing(s);
            }

            // Note: Padding handling simplified
            apply_size_constraints(container.into(), width, height)
        }
        _ => {
            // For unsupported widgets, show a placeholder
            let text = iced::widget::text(format!("[{:?}]", node.kind));
            apply_size_constraints(text.into(), width, height)
        }
    }
}

/// Helper to apply size constraints to a widget
fn apply_size_constraints(
    widget: Element<'static, Message>,
    width: Option<&gravity_core::ir::layout::Length>,
    height: Option<&gravity_core::ir::layout::Length>,
) -> Element<'static, Message> {
    use gravity_core::ir::layout::Length;

    // Convert Length to Iced Length
    let iced_width = match width {
        Some(Length::Fixed(pixels)) => iced::Length::Fixed(*pixels),
        Some(Length::Fill) => iced::Length::Fill,
        Some(Length::Shrink) => iced::Length::Shrink,
        Some(Length::FillPortion(n)) => iced::Length::FillPortion(*n as u16),
        Some(Length::Percentage(_p)) => {
            // Percentage needs to be calculated based on parent, which Iced handles
            // For now, treat as Fill
            iced::Length::Fill
        }
        None => iced::Length::Shrink,
    };

    let iced_height = match height {
        Some(Length::Fixed(pixels)) => iced::Length::Fixed(*pixels),
        Some(Length::Fill) => iced::Length::Fill,
        Some(Length::Shrink) => iced::Length::Shrink,
        Some(Length::FillPortion(n)) => iced::Length::FillPortion(*n as u16),
        Some(Length::Percentage(_p)) => iced::Length::Fill,
        None => iced::Length::Shrink,
    };

    // Apply size to widget using container wrapper
    iced::widget::container(widget)
        .width(iced_width)
        .height(iced_height)
        .into()
}

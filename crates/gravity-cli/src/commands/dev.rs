//! Dev command - runs app with hot-reload

use gravity_core::{HandlerRegistry, WidgetNode, AttributeValue, InterpolatedPart, EventKind};
use gravity_runtime::{HotReloadInterpreter, ErrorOverlay};
use iced::{Element, Subscription, Task};
use iced::time;
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
    let interpreter = HotReloadInterpreter::new(registry)
        .with_state_file(state_file.clone());
    
    // Create shared state
    let shared_state = Arc::new(Mutex::new(DevState {
        interpreter,
        xml: initial_xml,
        file_path: main_file.clone(),
        error_overlay: None,
        verbose: args.verbose,
        last_modified: None,
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
        move || State { state: state_clone.clone() },
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
    }
    Task::none()
}

fn view(state: &State) -> Element<Message> {
    #[allow(clippy::unwrap_used)]
    let dev_state = state.state.lock().unwrap();
    
    // Clone the data we need before releasing the lock
    let error_overlay = dev_state.error_overlay.clone();
    let doc = dev_state.interpreter.document().cloned();
    drop(dev_state); // Release lock
    
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
            .padding(20)
        )
        .style(|_theme| iced::widget::container::Style {
            background: Some(iced::Background::Color(iced::Color::from_rgb(0.8, 0.2, 0.2))),
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
        render_widget_tree(&doc.root)
    } else {
        iced::widget::container(
            iced::widget::text("No document loaded").size(20)
        )
        .into()
    }
}

fn subscription(_state: &State) -> Subscription<Message> {
    // Subscribe to periodic ticks for file checking (every 200ms)
    time::every(std::time::Duration::from_millis(200)).map(|_| Message::Tick)
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
    match node.kind {
        gravity_core::WidgetKind::Text => {
            let value = node.attributes.get("value")
                .and_then(|attr| {
                    match attr {
                        AttributeValue::Static(s) => Some(s.clone()),
                        AttributeValue::Binding(_) => Some("[binding]".to_string()),
                        AttributeValue::Interpolated(parts) => {
                            let mut s = String::new();
                            for part in parts {
                                match part {
                                    InterpolatedPart::Literal(l) => s.push_str(l),
                                    InterpolatedPart::Binding(_) => s.push_str("[binding]"),
                                }
                            }
                            Some(s)
                        }
                    }
                })
                .unwrap_or_else(|| "No value".to_string());
            
            iced::widget::text(value).into()
        }
        gravity_core::WidgetKind::Button => {
            let label = node.attributes.get("label")
                .and_then(|attr| {
                    match attr {
                        AttributeValue::Static(s) => Some(s.clone()),
                        _ => Some("[button]".to_string()),
                    }
                })
                .unwrap_or_else(|| "Button".to_string());
            
            // Check for click handler
            let has_handler = node.events.iter().any(|e| e.event == EventKind::Click);
            
            if has_handler {
                iced::widget::button(iced::widget::text(label))
                    .on_press(Message::Tick) // Placeholder
                    .into()
            } else {
                iced::widget::button(iced::widget::text(label)).into()
            }
        }
        gravity_core::WidgetKind::Column => {
            let children: Vec<Element<Message>> = node.children
                .iter()
                .map(|child| render_widget_tree(child))
                .collect();
            
            iced::widget::column(children).into()
        }
        gravity_core::WidgetKind::Row => {
            let children: Vec<Element<Message>> = node.children
                .iter()
                .map(|child| render_widget_tree(child))
                .collect();
            
            iced::widget::row(children).into()
        }
        _ => {
            // For unsupported widgets, show a placeholder
            iced::widget::text(format!("[{:?}]", node.kind)).into()
        }
    }
}


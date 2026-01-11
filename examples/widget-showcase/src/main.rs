//! Widget showcase demonstrating all Dampen UI widgets.
//!
//! This example shows all currently supported widgets in Dampen.

mod ui;

use dampen_core::AppState;
use dampen_iced::{DampenWidgetBuilder, HandlerMessage};
use iced::{Element, Subscription, Task};
use std::path::PathBuf;

#[cfg(debug_assertions)]
use dampen_dev::{ErrorOverlay, FileEvent, watch_files};

/// Application messages
#[derive(Clone, Debug)]
enum Message {
    /// Handler invocation from UI widgets
    Handler(HandlerMessage),
    /// Hot-reload event (development mode only)
    #[cfg(debug_assertions)]
    HotReload(FileEvent),
    /// Dismiss error overlay
    #[cfg(debug_assertions)]
    DismissError,
}

#[derive(Clone, Debug, PartialEq)]
enum CurrentView {
    Window,
    Button,
    Text,
    TextInput,
    Checkbox,
    Slider,
    Toggler,
    Image,
    Svg,
    Scrollable,
    Stack,
    Space,
    Layout,
    ForLoop,
    Combobox,
    Picklist,
    Progressbar,
    Radio,
    Tooltip,
    Grid,
}

#[derive(Clone, Debug)]
struct ShowcaseApp {
    current_view: CurrentView,
    window_state: AppState<ui::window::Model>,
    button_state: AppState<ui::button::Model>,
    text_state: AppState<ui::text::Model>,
    textinput_state: AppState<ui::textinput::Model>,
    checkbox_state: AppState<ui::checkbox::Model>,
    slider_state: AppState<ui::slider::Model>,
    toggler_state: AppState<ui::toggler::Model>,
    image_state: AppState<ui::image::Model>,
    svg_state: AppState<ui::svg::Model>,
    scrollable_state: AppState<ui::scrollable::Model>,
    stack_state: AppState<ui::stack::Model>,
    space_state: AppState<ui::space::Model>,
    layout_state: AppState<ui::layout::Model>,
    for_loop_state: AppState<ui::for_loop::Model>,
    combobox_state: AppState<ui::combobox::Model>,
    picklist_state: AppState<ui::picklist::Model>,
    progressbar_state: AppState<ui::progressbar::Model>,
    radio_state: AppState<ui::radio::Model>,
    tooltip_state: AppState<ui::tooltip::Model>,
    grid_state: AppState<ui::grid::Model>,
    #[cfg(debug_assertions)]
    error_overlay: ErrorOverlay,
}

impl ShowcaseApp {
    fn new() -> (Self, Task<Message>) {
        (
            ShowcaseApp {
                current_view: CurrentView::Window,
                window_state: ui::window::create_app_state(),
                button_state: ui::button::create_app_state(),
                combobox_state: ui::combobox::create_app_state(),
                picklist_state: ui::picklist::create_app_state(),
                progressbar_state: ui::progressbar::create_app_state(),
                radio_state: ui::radio::create_app_state(),
                tooltip_state: ui::tooltip::create_app_state(),
                grid_state: ui::grid::create_app_state(),
                scrollable_state: ui::scrollable::create_app_state(),
                slider_state: ui::slider::create_app_state(),
                textinput_state: ui::textinput::create_app_state(),
                toggler_state: ui::toggler::create_app_state(),
                text_state: ui::text::create_app_state(),
                image_state: ui::image::create_app_state(),
                svg_state: ui::svg::create_app_state(),
                checkbox_state: ui::checkbox::create_app_state(),
                stack_state: ui::stack::create_app_state(),
                space_state: ui::space::create_app_state(),
                layout_state: ui::layout::create_app_state(),
                for_loop_state: ui::for_loop::create_app_state(),
                #[cfg(debug_assertions)]
                error_overlay: ErrorOverlay::new(),
            },
            Task::none(),
        )
    }
}

fn update(app: &mut ShowcaseApp, message: Message) -> Task<Message> {
    match message {
        Message::Handler(HandlerMessage::Handler(handler_name, value)) => {
            match handler_name.as_str() {
                "switch_to_window" => {
                    app.current_view = CurrentView::Window;
                    Task::none()
                }
                "switch_to_text" => {
                    app.current_view = CurrentView::Text;
                    Task::none()
                }
                "switch_to_button" => {
                    app.current_view = CurrentView::Button;
                    Task::none()
                }
                "switch_to_textinput" => {
                    app.current_view = CurrentView::TextInput;
                    Task::none()
                }
                "switch_to_checkbox" => {
                    app.current_view = CurrentView::Checkbox;
                    Task::none()
                }
                "switch_to_slider" => {
                    app.current_view = CurrentView::Slider;
                    Task::none()
                }
                "switch_to_toggler" => {
                    app.current_view = CurrentView::Toggler;
                    Task::none()
                }
                "switch_to_image" => {
                    app.current_view = CurrentView::Image;
                    Task::none()
                }
                "switch_to_svg" => {
                    app.current_view = CurrentView::Svg;
                    Task::none()
                }
                "switch_to_scrollable" => {
                    app.current_view = CurrentView::Scrollable;
                    Task::none()
                }
                "switch_to_stack" => {
                    app.current_view = CurrentView::Stack;
                    Task::none()
                }
                "switch_to_space" => {
                    app.current_view = CurrentView::Space;
                    Task::none()
                }
                "switch_to_layout" => {
                    app.current_view = CurrentView::Layout;
                    Task::none()
                }
                "switch_to_for" => {
                    app.current_view = CurrentView::ForLoop;
                    Task::none()
                }
                "switch_to_combobox" => {
                    app.current_view = CurrentView::Combobox;
                    Task::none()
                }
                "switch_to_picklist" => {
                    app.current_view = CurrentView::Picklist;
                    Task::none()
                }
                "switch_to_progressbar" => {
                    app.current_view = CurrentView::Progressbar;
                    Task::none()
                }
                "switch_to_radio" => {
                    app.current_view = CurrentView::Radio;
                    Task::none()
                }
                "switch_to_tooltip" => {
                    app.current_view = CurrentView::Tooltip;
                    Task::none()
                }
                "switch_to_grid" => {
                    app.current_view = CurrentView::Grid;
                    Task::none()
                }
                _ => {
                    dispatch_handler(app, &handler_name, value);
                    Task::none()
                }
            }
        }
        #[cfg(debug_assertions)]
        Message::HotReload(event) => {
            match event {
                FileEvent::Success { document, path } => {
                    println!("🔄 Hot-reloading {:?}...", path);

                    // Determine which view to reload based on the file path
                    if path.to_string_lossy().contains("window.dampen") {
                        app.window_state.hot_reload(*document);
                    } else if path.to_string_lossy().contains("button.dampen") {
                        app.button_state.hot_reload(*document);
                    } else if path.to_string_lossy().contains("checkbox.dampen") {
                        app.checkbox_state.hot_reload(*document);
                    } else if path.to_string_lossy().contains("combobox.dampen") {
                        app.combobox_state.hot_reload(*document);
                    } else if path.to_string_lossy().contains("for_loop.dampen") {
                        app.for_loop_state.hot_reload(*document);
                    } else if path.to_string_lossy().contains("grid.dampen") {
                        app.grid_state.hot_reload(*document);
                    } else if path.to_string_lossy().contains("image.dampen") {
                        app.image_state.hot_reload(*document);
                    } else if path.to_string_lossy().contains("layout.dampen") {
                        app.layout_state.hot_reload(*document);
                    } else if path.to_string_lossy().contains("picklist.dampen") {
                        app.picklist_state.hot_reload(*document);
                    } else if path.to_string_lossy().contains("progressbar.dampen") {
                        app.progressbar_state.hot_reload(*document);
                    } else if path.to_string_lossy().contains("radio.dampen") {
                        app.radio_state.hot_reload(*document);
                    } else if path.to_string_lossy().contains("scrollable.dampen") {
                        app.scrollable_state.hot_reload(*document);
                    } else if path.to_string_lossy().contains("slider.dampen") {
                        app.slider_state.hot_reload(*document);
                    } else if path.to_string_lossy().contains("space.dampen") {
                        app.space_state.hot_reload(*document);
                    } else if path.to_string_lossy().contains("stack.dampen") {
                        app.stack_state.hot_reload(*document);
                    } else if path.to_string_lossy().contains("svg.dampen") {
                        app.svg_state.hot_reload(*document);
                    } else if path.to_string_lossy().contains("text.dampen") {
                        app.text_state.hot_reload(*document);
                    } else if path.to_string_lossy().contains("textinput.dampen") {
                        app.textinput_state.hot_reload(*document);
                    } else if path.to_string_lossy().contains("toggler.dampen") {
                        app.toggler_state.hot_reload(*document);
                    } else if path.to_string_lossy().contains("tooltip.dampen") {
                        app.tooltip_state.hot_reload(*document);
                    } else {
                        println!("⚠️  Unknown file: {:?}", path);
                    }

                    app.error_overlay.hide();
                    println!("✅ Hot-reload successful!");
                }
                FileEvent::ParseError { error, path, .. } => {
                    println!("❌ Parse error in {:?}: {}", path, error);
                    app.error_overlay.show(error);
                }
                FileEvent::WatcherError { path, error } => {
                    println!("⚠️  File watcher error for {:?}: {}", path, error);
                }
            }
            Task::none()
        }
        #[cfg(debug_assertions)]
        Message::DismissError => {
            app.error_overlay.hide();
            Task::none()
        }
    }
}

fn view(app: &ShowcaseApp) -> Element<'_, Message> {
    #[cfg(debug_assertions)]
    if app.error_overlay.is_visible() {
        // Show error overlay on top of normal UI
        return app.error_overlay.render(Message::DismissError);
    }

    let element = match app.current_view {
        CurrentView::Window => DampenWidgetBuilder::from_app_state(&app.window_state).build(),
        CurrentView::Button => DampenWidgetBuilder::from_app_state(&app.button_state).build(),
        CurrentView::Text => DampenWidgetBuilder::from_app_state(&app.text_state).build(),
        CurrentView::TextInput => DampenWidgetBuilder::from_app_state(&app.textinput_state).build(),
        CurrentView::Checkbox => DampenWidgetBuilder::from_app_state(&app.checkbox_state).build(),
        CurrentView::Slider => DampenWidgetBuilder::from_app_state(&app.slider_state).build(),
        CurrentView::Toggler => DampenWidgetBuilder::from_app_state(&app.toggler_state).build(),
        CurrentView::Image => DampenWidgetBuilder::from_app_state(&app.image_state).build(),
        CurrentView::Svg => DampenWidgetBuilder::from_app_state(&app.svg_state).build(),
        CurrentView::Scrollable => {
            DampenWidgetBuilder::from_app_state(&app.scrollable_state).build()
        }
        CurrentView::Stack => DampenWidgetBuilder::from_app_state(&app.stack_state).build(),
        CurrentView::Space => DampenWidgetBuilder::from_app_state(&app.space_state).build(),
        CurrentView::Layout => DampenWidgetBuilder::from_app_state(&app.layout_state).build(),
        CurrentView::ForLoop => DampenWidgetBuilder::from_app_state(&app.for_loop_state).build(),
        CurrentView::Combobox => DampenWidgetBuilder::from_app_state(&app.combobox_state).build(),
        CurrentView::Picklist => DampenWidgetBuilder::from_app_state(&app.picklist_state).build(),
        CurrentView::Progressbar => {
            DampenWidgetBuilder::from_app_state(&app.progressbar_state).build()
        }
        CurrentView::Radio => DampenWidgetBuilder::from_app_state(&app.radio_state).build(),
        CurrentView::Tooltip => DampenWidgetBuilder::from_app_state(&app.tooltip_state).build(),
        CurrentView::Grid => DampenWidgetBuilder::from_app_state(&app.grid_state).build(),
    };

    // Map HandlerMessage to Message
    element.map(Message::Handler)
}

fn init() -> (ShowcaseApp, Task<Message>) {
    ShowcaseApp::new()
}

pub fn main() -> iced::Result {
    println!("🔥 Hot-reload enabled! Edit src/ui/*.dampen files to see live updates.");

    iced::application(init, update, view)
        .window_size(iced::Size::new(1024.0, 800.0))
        .centered()
        .subscription(subscription)
        .run()
}

fn subscription(_app: &ShowcaseApp) -> Subscription<Message> {
    #[cfg(debug_assertions)]
    {
        // Resolve UI file path relative to the manifest directory
        // This works whether running from workspace root or example directory

        let manifest_dir = env!("CARGO_MANIFEST_DIR");
        let window_file = PathBuf::from(manifest_dir).join("src/ui/window.dampen");
        let button_file = PathBuf::from(manifest_dir).join("src/ui/button.dampen");
        let checkbox_file = PathBuf::from(manifest_dir).join("src/ui/checkbox.dampen");
        let combobox_file = PathBuf::from(manifest_dir).join("src/ui/combobox.dampen");
        let for_loop_file = PathBuf::from(manifest_dir).join("src/ui/for_loop.dampen");
        let grid_file = PathBuf::from(manifest_dir).join("src/ui/grid.dampen");
        let image_file = PathBuf::from(manifest_dir).join("src/ui/image.dampen");
        let progressbar_file = PathBuf::from(manifest_dir).join("src/ui/progressbar.dampen");
        let radio_file = PathBuf::from(manifest_dir).join("src/ui/radio.dampen");
        let scrollable_file = PathBuf::from(manifest_dir).join("src/ui/scrollable.dampen");
        let slider_file = PathBuf::from(manifest_dir).join("src/ui/slider.dampen");
        let text_file = PathBuf::from(manifest_dir).join("src/ui/text.dampen");
        let textinput_file = PathBuf::from(manifest_dir).join("src/ui/textinput.dampen");
        let layout_file = PathBuf::from(manifest_dir).join("src/ui/layout.dampen");
        let picklist_file = PathBuf::from(manifest_dir).join("src/ui/picklist.dampen");
        let space_file = PathBuf::from(manifest_dir).join("src/ui/space.dampen");
        let stack_file = PathBuf::from(manifest_dir).join("src/ui/stack.dampen");
        let svg_file = PathBuf::from(manifest_dir).join("src/ui/svg.dampen");
        let toggler_file = PathBuf::from(manifest_dir).join("src/ui/toggler.dampen");
        let tooltip_file = PathBuf::from(manifest_dir).join("src/ui/tooltip.dampen");

        println!("👀 Watching for changes:");
        println!("   - {}", window_file.display());
        println!("   - {}", button_file.display());
        println!("   - {}", checkbox_file.display());
        println!("   - {}", combobox_file.display());
        println!("   - {}", for_loop_file.display());
        println!("   - {}", grid_file.display());
        println!("   - {}", image_file.display());
        println!("   - {}", progressbar_file.display());
        println!("   - {}", radio_file.display());
        println!("   - {}", scrollable_file.display());
        println!("   - {}", slider_file.display());
        println!("   - {}", text_file.display());
        println!("   - {}", textinput_file.display());
        println!("   - {}", layout_file.display());
        println!("   - {}", picklist_file.display());
        println!("   - {}", space_file.display());
        println!("   - {}", stack_file.display());
        println!("   - {}", svg_file.display());
        println!("   - {}", toggler_file.display());
        println!("   - {}", tooltip_file.display());

        // Watch the UI files for changes in development mode (100ms debounce)
        watch_files(
            vec![
                window_file,
                button_file,
                checkbox_file,
                combobox_file,
                for_loop_file,
                grid_file,
                image_file,
                progressbar_file,
                radio_file,
                scrollable_file,
                slider_file,
                text_file,
                textinput_file,
                layout_file,
                picklist_file,
                space_file,
                stack_file,
                svg_file,
                toggler_file,
                tooltip_file,
            ],
            100,
        )
        .map(Message::HotReload)
    }

    #[cfg(not(debug_assertions))]
    {
        Subscription::none()
    }
}

fn dispatch_handler(app: &mut ShowcaseApp, handler_name: &str, value: Option<String>) {
    let (model, registry) = match app.current_view {
        CurrentView::Button => (
            &mut app.button_state.model as &mut dyn std::any::Any,
            &app.button_state.handler_registry,
        ),
        CurrentView::Text => (
            &mut app.text_state.model as &mut dyn std::any::Any,
            &app.text_state.handler_registry,
        ),
        CurrentView::TextInput => (
            &mut app.textinput_state.model as &mut dyn std::any::Any,
            &app.textinput_state.handler_registry,
        ),
        CurrentView::Checkbox => (
            &mut app.checkbox_state.model as &mut dyn std::any::Any,
            &app.checkbox_state.handler_registry,
        ),
        CurrentView::Slider => (
            &mut app.slider_state.model as &mut dyn std::any::Any,
            &app.slider_state.handler_registry,
        ),
        CurrentView::Toggler => (
            &mut app.toggler_state.model as &mut dyn std::any::Any,
            &app.toggler_state.handler_registry,
        ),
        CurrentView::Image => (
            &mut app.image_state.model as &mut dyn std::any::Any,
            &app.image_state.handler_registry,
        ),
        CurrentView::Svg => (
            &mut app.svg_state.model as &mut dyn std::any::Any,
            &app.svg_state.handler_registry,
        ),
        CurrentView::Stack => (
            &mut app.stack_state.model as &mut dyn std::any::Any,
            &app.stack_state.handler_registry,
        ),
        CurrentView::Space => (
            &mut app.space_state.model as &mut dyn std::any::Any,
            &app.space_state.handler_registry,
        ),
        CurrentView::Layout => (
            &mut app.layout_state.model as &mut dyn std::any::Any,
            &app.layout_state.handler_registry,
        ),
        CurrentView::ForLoop => (
            &mut app.for_loop_state.model as &mut dyn std::any::Any,
            &app.for_loop_state.handler_registry,
        ),
        CurrentView::Scrollable => (
            &mut app.scrollable_state.model as &mut dyn std::any::Any,
            &app.scrollable_state.handler_registry,
        ),
        CurrentView::Combobox => (
            &mut app.combobox_state.model as &mut dyn std::any::Any,
            &app.combobox_state.handler_registry,
        ),
        CurrentView::Picklist => (
            &mut app.picklist_state.model as &mut dyn std::any::Any,
            &app.picklist_state.handler_registry,
        ),
        CurrentView::Progressbar => (
            &mut app.progressbar_state.model as &mut dyn std::any::Any,
            &app.progressbar_state.handler_registry,
        ),
        CurrentView::Radio => (
            &mut app.radio_state.model as &mut dyn std::any::Any,
            &app.radio_state.handler_registry,
        ),
        CurrentView::Tooltip => (
            &mut app.tooltip_state.model as &mut dyn std::any::Any,
            &app.tooltip_state.handler_registry,
        ),
        CurrentView::Grid => (
            &mut app.grid_state.model as &mut dyn std::any::Any,
            &app.grid_state.handler_registry,
        ),
        CurrentView::Window => (
            &mut app.window_state.model as &mut dyn std::any::Any,
            &app.window_state.handler_registry,
        ),
    };
    registry.dispatch(handler_name, model, value);
}

//! Widget showcase demonstrating all Gravity UI widgets.
//!
//! This example shows all currently supported widgets in Gravity.

mod ui;

use gravity_core::{AppState, HandlerRegistry};
use gravity_iced::{GravityWidgetBuilder, HandlerMessage};
use iced::{Element, Task};

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
    Tooltip,
    Grid,
}

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
    tooltip_state: AppState<ui::tooltip::Model>,
    grid_state: AppState<ui::grid::Model>,
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

fn update(app: &mut ShowcaseApp, message: HandlerMessage) -> Task<HandlerMessage> {
    match message {
        HandlerMessage::Handler(handler_name, value) => match handler_name.as_str() {
            "switch_to_button" => app.current_view = CurrentView::Button,
            "switch_to_text" => app.current_view = CurrentView::Text,
            "switch_to_textinput" => app.current_view = CurrentView::TextInput,
            "switch_to_checkbox" => app.current_view = CurrentView::Checkbox,
            "switch_to_slider" => app.current_view = CurrentView::Slider,
            "switch_to_toggler" => app.current_view = CurrentView::Toggler,
            "switch_to_image" => app.current_view = CurrentView::Image,
            "switch_to_svg" => app.current_view = CurrentView::Svg,
            "switch_to_scrollable" => app.current_view = CurrentView::Scrollable,
            "switch_to_stack" => app.current_view = CurrentView::Stack,
            "switch_to_space" => app.current_view = CurrentView::Space,
            "switch_to_layout" => app.current_view = CurrentView::Layout,
            "switch_to_for" => app.current_view = CurrentView::ForLoop,
            "switch_to_combobox" => app.current_view = CurrentView::Combobox,
            "switch_to_picklist" => app.current_view = CurrentView::Picklist,
            "switch_to_progressbar" => app.current_view = CurrentView::Progressbar,
            "switch_to_tooltip" => app.current_view = CurrentView::Tooltip,
            "switch_to_grid" => app.current_view = CurrentView::Grid,
            "switch_to_window" => app.current_view = CurrentView::Window,
            _ => dispatch_handler(app, &handler_name, value),
        },
    }
    Task::none()
}

fn view(app: &ShowcaseApp) -> Element<'_, HandlerMessage> {
    match app.current_view {
        CurrentView::Window => GravityWidgetBuilder::from_app_state(&app.window_state),
        CurrentView::Button => GravityWidgetBuilder::from_app_state(&app.button_state),
        CurrentView::Text => GravityWidgetBuilder::from_app_state(&app.text_state),
        CurrentView::TextInput => GravityWidgetBuilder::from_app_state(&app.textinput_state),
        CurrentView::Checkbox => GravityWidgetBuilder::from_app_state(&app.checkbox_state),
        CurrentView::Slider => GravityWidgetBuilder::from_app_state(&app.slider_state),
        CurrentView::Toggler => GravityWidgetBuilder::from_app_state(&app.toggler_state),
        CurrentView::Image => GravityWidgetBuilder::from_app_state(&app.image_state),
        CurrentView::Svg => GravityWidgetBuilder::from_app_state(&app.svg_state),
        CurrentView::Scrollable => GravityWidgetBuilder::from_app_state(&app.scrollable_state),
        CurrentView::Stack => GravityWidgetBuilder::from_app_state(&app.stack_state),
        CurrentView::Space => GravityWidgetBuilder::from_app_state(&app.space_state),
        CurrentView::Layout => GravityWidgetBuilder::from_app_state(&app.layout_state),
        CurrentView::ForLoop => GravityWidgetBuilder::from_app_state(&app.for_loop_state),
        CurrentView::Combobox => GravityWidgetBuilder::from_app_state(&app.combobox_state),
        CurrentView::Picklist => GravityWidgetBuilder::from_app_state(&app.picklist_state),
        CurrentView::Progressbar => GravityWidgetBuilder::from_app_state(&app.progressbar_state),
        CurrentView::Tooltip => GravityWidgetBuilder::from_app_state(&app.tooltip_state),
        CurrentView::Grid => GravityWidgetBuilder::from_app_state(&app.grid_state),
    }
    .build()
}

fn init() -> (ShowcaseApp, Task<HandlerMessage>) {
    (
        ShowcaseApp {
            current_view: CurrentView::Window,
            window_state: ui::window::create_app_state(),
            button_state: ui::button::create_app_state(),
            text_state: ui::text::create_app_state(),
            textinput_state: ui::textinput::create_app_state(),
            checkbox_state: ui::checkbox::create_app_state(),
            slider_state: ui::slider::create_app_state(),
            toggler_state: ui::toggler::create_app_state(),
            image_state: ui::image::create_app_state(),
            svg_state: ui::svg::create_app_state(),
            scrollable_state: ui::scrollable::create_app_state(),
            stack_state: ui::stack::create_app_state(),
            space_state: ui::space::create_app_state(),
            layout_state: ui::layout::create_app_state(),
            for_loop_state: ui::for_loop::create_app_state(),
            combobox_state: ui::combobox::create_app_state(),
            picklist_state: ui::picklist::create_app_state(),
            progressbar_state: ui::progressbar::create_app_state(),
            tooltip_state: ui::tooltip::create_app_state(),
            grid_state: ui::grid::create_app_state(),
        },
        Task::none(),
    )
}

pub fn main() -> iced::Result {
    iced::application(init, update, view).run()
}

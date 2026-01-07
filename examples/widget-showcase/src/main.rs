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
    combobox_state: AppState<()>,
    picklist_state: AppState<ui::picklist::Model>,
    progressbar_state: AppState<ui::progressbar::Model>,
    tooltip_state: AppState<()>,
    grid_state: AppState<()>,
}

type AppMessage = HandlerMessage;

fn update(app: &mut ShowcaseApp, message: AppMessage) -> Task<AppMessage> {
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
            _ => {
                let (model, registry): (&mut dyn std::any::Any, &HandlerRegistry) =
                    match app.current_view {
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
                        CurrentView::Picklist => (
                            &mut app.picklist_state.model as &mut dyn std::any::Any,
                            &app.picklist_state.handler_registry,
                        ),
                        CurrentView::Progressbar => (
                            &mut app.progressbar_state.model as &mut dyn std::any::Any,
                            &app.progressbar_state.handler_registry,
                        ),
                        _ => return Task::none(),
                    };
                match registry.get(&handler_name) {
                    Some(gravity_core::HandlerEntry::Simple(h)) => {
                        h(model);
                    }
                    Some(gravity_core::HandlerEntry::WithValue(h)) => {
                        let val = value.unwrap_or_default();
                        h(model, Box::new(val));
                    }
                    Some(gravity_core::HandlerEntry::WithCommand(_)) => {}
                    None => {}
                }
            }
        },
    }
    Task::none()
}

fn view(app: &ShowcaseApp) -> Element<'_, AppMessage> {
    match app.current_view {
        CurrentView::Window => GravityWidgetBuilder::new(
            &app.window_state.document,
            &app.window_state.model,
            Some(&app.window_state.handler_registry),
        )
        .build(),
        CurrentView::Button => GravityWidgetBuilder::new(
            &app.button_state.document,
            &app.button_state.model,
            Some(&app.button_state.handler_registry),
        )
        .build(),
        CurrentView::Text => GravityWidgetBuilder::new(
            &app.text_state.document,
            &app.text_state.model,
            Some(&app.text_state.handler_registry),
        )
        .build(),
        CurrentView::TextInput => GravityWidgetBuilder::new(
            &app.textinput_state.document,
            &app.textinput_state.model,
            Some(&app.textinput_state.handler_registry),
        )
        .build(),
        CurrentView::Checkbox => GravityWidgetBuilder::new(
            &app.checkbox_state.document,
            &app.checkbox_state.model,
            Some(&app.checkbox_state.handler_registry),
        )
        .build(),
        CurrentView::Slider => GravityWidgetBuilder::new(
            &app.slider_state.document,
            &app.slider_state.model,
            Some(&app.slider_state.handler_registry),
        )
        .build(),
        CurrentView::Toggler => GravityWidgetBuilder::new(
            &app.toggler_state.document,
            &app.toggler_state.model,
            Some(&app.toggler_state.handler_registry),
        )
        .build(),
        CurrentView::Image => GravityWidgetBuilder::new(
            &app.image_state.document,
            &app.image_state.model,
            Some(&app.image_state.handler_registry),
        )
        .build(),
        CurrentView::Svg => GravityWidgetBuilder::new(
            &app.svg_state.document,
            &app.svg_state.model,
            Some(&app.svg_state.handler_registry),
        )
        .build(),
        CurrentView::Scrollable => GravityWidgetBuilder::new(
            &app.scrollable_state.document,
            &app.scrollable_state.model,
            Some(&app.scrollable_state.handler_registry),
        )
        .build(),
        CurrentView::Stack => GravityWidgetBuilder::new(
            &app.stack_state.document,
            &app.stack_state.model,
            Some(&app.stack_state.handler_registry),
        )
        .build(),
        CurrentView::Space => GravityWidgetBuilder::new(
            &app.space_state.document,
            &app.space_state.model,
            Some(&app.space_state.handler_registry),
        )
        .build(),
        CurrentView::Layout => GravityWidgetBuilder::new(
            &app.layout_state.document,
            &app.layout_state.model,
            Some(&app.layout_state.handler_registry),
        )
        .build(),
        CurrentView::ForLoop => GravityWidgetBuilder::new(
            &app.for_loop_state.document,
            &app.for_loop_state.model,
            Some(&app.for_loop_state.handler_registry),
        )
        .build(),
        CurrentView::Combobox => GravityWidgetBuilder::new(
            &app.combobox_state.document,
            &app.combobox_state.model,
            None,
        )
        .build(),
        CurrentView::Picklist => GravityWidgetBuilder::new(
            &app.picklist_state.document,
            &app.picklist_state.model,
            Some(&app.picklist_state.handler_registry),
        )
        .build(),
        CurrentView::Progressbar => GravityWidgetBuilder::new(
            &app.progressbar_state.document,
            &app.progressbar_state.model,
            Some(&app.progressbar_state.handler_registry),
        )
        .build(),
        CurrentView::Tooltip => {
            GravityWidgetBuilder::new(&app.tooltip_state.document, &app.tooltip_state.model, None)
                .build()
        }
        CurrentView::Grid => {
            GravityWidgetBuilder::new(&app.grid_state.document, &app.grid_state.model, None).build()
        }
    }
}

fn init() -> (ShowcaseApp, Task<AppMessage>) {
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

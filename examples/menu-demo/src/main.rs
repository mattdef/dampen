use dampen_iced::prelude::*;
use dampen_macros::{UiModel, dampen_ui};
use serde::{Deserialize, Serialize};

#[dampen_ui("ui/main.dampen")]
mod ui {}

#[derive(UiModel, Serialize, Deserialize, Default, Clone)]
struct Model {
    status: String,
}

fn main() -> iced::Result {
    let mut registry = HandlerRegistry::new();

    registry.register_simple("new_file", |m| {
        let model = m.downcast_mut::<Model>().unwrap();
        model.status = "New File clicked".to_string();
    });

    registry.register_simple("exit_app", |_| {
        std::process::exit(0);
    });

    dampen_iced::run::<Model>("Menu Demo", registry)
}

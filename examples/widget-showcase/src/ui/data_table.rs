// Data Table widget showcase UI module.
//
// This file auto-loads the corresponding data_table.dampen XML file.

use crate::{CurrentView, Message};
use dampen_core::{AppState, BindingValue, HandlerRegistry, ToBindingValue};
use dampen_macros::{UiModel, dampen_ui};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Clone, Debug, Default, UiModel)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
}

impl ToBindingValue for User {
    fn to_binding_value(&self) -> BindingValue {
        let mut map = HashMap::new();
        map.insert("id".to_string(), self.id.to_binding_value());
        map.insert("name".to_string(), self.name.to_binding_value());
        map.insert("email".to_string(), self.email.to_binding_value());
        BindingValue::Object(map)
    }
}

#[derive(Default, UiModel, Serialize, Deserialize, Clone, Debug)]
pub struct Model {
    pub users: Vec<User>,
}

#[dampen_ui("data_table.dampen")]
mod _app {}

pub fn create_app_state() -> AppState<Model> {
    let mut model = Model::default();
    model.users = vec![
        User {
            id: 1,
            name: "Alice".to_string(),
            email: "alice@example.com".to_string(),
        },
        User {
            id: 2,
            name: "Bob".to_string(),
            email: "bob@example.com".to_string(),
        },
        User {
            id: 3,
            name: "Charlie".to_string(),
            email: "charlie@example.com".to_string(),
        },
        User {
            id: 4,
            name: "David".to_string(),
            email: "david@example.com".to_string(),
        },
        User {
            id: 5,
            name: "Eve".to_string(),
            email: "eve@example.com".to_string(),
        },
    ];
    let document = _app::document();
    let handler_registry = create_handler_registry();
    AppState::with_all(document, model, handler_registry)
}

pub fn create_handler_registry() -> HandlerRegistry {
    let registry = HandlerRegistry::new();

    registry.register_with_command("switch_to_window", |_model: &mut dyn std::any::Any| {
        Box::new(iced::Task::done(Message::SwitchToView(CurrentView::Window)))
    });

    registry
}

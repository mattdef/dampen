// ComboBox widget showcase UI module.
//
// This file auto-loads the corresponding combobox.gravity XML file.

use gravity_core::{AppState, GravityDocument};
use gravity_macros::gravity_ui;

#[gravity_ui("combobox.gravity")]
mod _settings {}

pub fn create_document() -> GravityDocument {
    _settings::document()
}

pub fn create_app_state() -> AppState<()> {
    AppState::new(_settings::document())
}

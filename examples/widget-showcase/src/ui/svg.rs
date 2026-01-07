// Svg widget showcase UI module.
//
// This file auto-loads the corresponding svg.gravity XML file.

use gravity_core::{AppState, GravityDocument};
use gravity_macros::gravity_ui;

#[gravity_ui("svg.gravity")]
mod _app {}

pub fn create_document() -> GravityDocument {
    _app::document()
}

pub fn create_app_state() -> AppState<()> {
    AppState::new(_app::document())
}

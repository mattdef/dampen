// Space and Rule widgets showcase UI module.
//
// This file auto-loads the corresponding space.gravity XML file.

use gravity_core::{AppState, GravityDocument};
use gravity_macros::gravity_ui;

#[gravity_ui("space.gravity")]
mod _app {}

pub fn create_document() -> GravityDocument {
    _app::document()
}

pub fn create_app_state() -> AppState<()> {
    AppState::new(_app::document())
}

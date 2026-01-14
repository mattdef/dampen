//! Test fixture for shared_model tests across workspace
use dampen_core::UiBindable;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, Serialize, Deserialize, UiBindable)]
pub struct SharedState {
    pub theme: String,
}

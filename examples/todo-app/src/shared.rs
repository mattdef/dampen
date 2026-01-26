//! Shared state for future extensions
//!
//! Currently unused in the simplified single-view architecture, but kept
//! to demonstrate how shared state can be integrated with #[dampen_app].

use dampen_macros::UiModel;
use serde::{Deserialize, Serialize};

/// Shared state accessible across multiple windows
#[derive(Default, Clone, Debug, Serialize, Deserialize, UiModel)]
pub struct SharedState {
    // Placeholder for shared data
}

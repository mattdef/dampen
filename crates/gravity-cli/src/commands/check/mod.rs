/// Check command validation modules.
pub mod attributes;
pub mod custom_widgets;
pub mod errors;
mod main_command;
pub mod suggestions;

// Re-exports for convenience
pub use attributes::WidgetAttributeSchema;
pub use custom_widgets::{CustomWidgetConfig, CustomWidgetRegistry};
pub use errors::CheckError as EnhancedCheckError;
pub use main_command::{execute, CheckArgs, CheckError};
pub use suggestions::{find_closest_match, levenshtein_distance, suggest};

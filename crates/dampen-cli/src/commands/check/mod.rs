/// Check command validation modules.
pub mod attributes;
pub mod cross_widget;
pub mod custom_widgets;
pub mod errors;
pub mod handlers;
mod main_command;
pub mod model;
pub mod suggestions;
pub mod themes;

// Re-exports for convenience
pub use attributes::WidgetAttributeSchema;
pub use cross_widget::{RadioButton, RadioGroup, RadioGroupValidator};
pub use custom_widgets::{CustomWidgetConfig, CustomWidgetRegistry};
pub use errors::CheckError as EnhancedCheckError;
pub use handlers::{HandlerDefinition, HandlerRegistry};
pub use main_command::{CheckArgs, CheckError, execute};
pub use model::{ModelField, ModelInfo};
pub use suggestions::{find_closest_match, levenshtein_distance, suggest};
pub use themes::ThemeValidator;

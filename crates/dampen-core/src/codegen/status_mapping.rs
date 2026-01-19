//! Status mapping code generation for widget state styling
//!
//! This module generates Rust code that maps Iced widget-specific status enums
//! to Dampen's unified WidgetState enum. This enables state-aware styling in
//! codegen mode (matching the behavior of interpreted mode).
//!
//! # Architecture
//!
//! The generated code mirrors the runtime status mapping logic from
//! `dampen-iced::style_mapping`, but produces compile-time Rust code instead
//! of runtime function calls.
//!
//! # Example Generated Code
//!
//! For a button with hover styling:
//! ```text
//! let style = move |_theme: &Theme, status: iced::widget::button::Status| {
//!     use iced::widget::button::Status;
//!     let widget_state = match status {
//!         Status::Active => None,
//!         Status::Hovered => Some(dampen_core::ir::WidgetState::Hover),
//!         Status::Pressed => Some(dampen_core::ir::WidgetState::Active),
//!         Status::Disabled => Some(dampen_core::ir::WidgetState::Disabled),
//!     };
//!
//!     if let Some(state) = widget_state {
//!         // Apply state-specific styling
//!     } else {
//!         // Apply base styling
//!     }
//! };
//! button.style(style)
//! ```

use proc_macro2::TokenStream;
use quote::quote;

use crate::WidgetKind;

/// Generate status mapping code for a specific widget kind
///
/// Returns a TokenStream that generates a `match` expression mapping
/// the widget's Iced status enum to `Option<WidgetState>`.
///
/// Returns `None` for the base/default state, and `Some(WidgetState::X)`
/// for hover/focus/active/disabled states.
///
/// # Arguments
/// * `widget_kind` - The type of widget to generate mapping for
/// * `status_ident` - The identifier for the status variable (usually `status`)
///
/// # Returns
/// * `Some(TokenStream)` - Generated match expression for widgets with state support
/// * `None` - For widgets that don't support state styling (e.g., Text, Space)
///
/// # Example
/// ```
/// use dampen_core::codegen::status_mapping::generate_status_mapping;
/// use dampen_core::WidgetKind;
/// use quote::format_ident;
///
/// let status_ident = format_ident!("status");
/// let mapping = generate_status_mapping(&WidgetKind::Button, &status_ident);
/// assert!(mapping.is_some());
/// ```
pub fn generate_status_mapping(
    widget_kind: &WidgetKind,
    status_ident: &syn::Ident,
) -> Option<TokenStream> {
    match widget_kind {
        WidgetKind::Button => Some(generate_button_status_mapping(status_ident)),
        WidgetKind::TextInput => Some(generate_text_input_status_mapping(status_ident)),
        WidgetKind::Checkbox => Some(generate_checkbox_status_mapping(status_ident)),
        WidgetKind::Radio => Some(generate_radio_status_mapping(status_ident)),
        WidgetKind::Toggler => Some(generate_toggler_status_mapping(status_ident)),
        WidgetKind::Slider => Some(generate_slider_status_mapping(status_ident)),
        WidgetKind::PickList => Some(generate_picklist_status_mapping(status_ident)),
        WidgetKind::ComboBox => Some(generate_combo_box_status_mapping(status_ident)),
        _ => None, // Widgets without state support
    }
}

/// Generate button status mapping code
///
/// Maps `iced::widget::button::Status` to `Option<WidgetState>`:
/// - `Active` → `None` (base state)
/// - `Hovered` → `Some(Hover)`
/// - `Pressed` → `Some(Active)`
/// - `Disabled` → `Some(Disabled)`
fn generate_button_status_mapping(status_ident: &syn::Ident) -> TokenStream {
    quote! {
        {
            use iced::widget::button::Status;
            match #status_ident {
                Status::Active => None,
                Status::Hovered => Some(dampen_core::ir::WidgetState::Hover),
                Status::Pressed => Some(dampen_core::ir::WidgetState::Active),
                Status::Disabled => Some(dampen_core::ir::WidgetState::Disabled),
            }
        }
    }
}

/// Generate text input status mapping code
///
/// Maps `iced::widget::text_input::Status` to `Option<WidgetState>`:
/// - `Active` → `None` (base state)
/// - `Hovered` → `Some(Hover)`
/// - `Focused { .. }` → `Some(Focus)`
/// - `Disabled` → `Some(Disabled)`
fn generate_text_input_status_mapping(status_ident: &syn::Ident) -> TokenStream {
    quote! {
        {
            use iced::widget::text_input::Status;
            match #status_ident {
                Status::Active => None,
                Status::Hovered => Some(dampen_core::ir::WidgetState::Hover),
                Status::Focused { .. } => Some(dampen_core::ir::WidgetState::Focus),
                Status::Disabled => Some(dampen_core::ir::WidgetState::Disabled),
            }
        }
    }
}

/// Generate checkbox status mapping code
///
/// Maps `iced::widget::checkbox::Status` to `Option<WidgetState>`:
/// - `Active { .. }` → `None` (base state)
/// - `Hovered { .. }` → `Some(Hover)`
/// - `Disabled { .. }` → `Some(Disabled)`
///
/// Note: The `is_checked` field in each variant is ignored for state mapping.
fn generate_checkbox_status_mapping(status_ident: &syn::Ident) -> TokenStream {
    quote! {
        {
            use iced::widget::checkbox::Status;
            match #status_ident {
                Status::Active { .. } => None,
                Status::Hovered { .. } => Some(dampen_core::ir::WidgetState::Hover),
                Status::Disabled { .. } => Some(dampen_core::ir::WidgetState::Disabled),
            }
        }
    }
}

/// Generate radio button status mapping code
///
/// Maps `iced::widget::radio::Status` to `Option<WidgetState>`:
/// - `Active { .. }` → `None` (base state)
/// - `Hovered { .. }` → `Some(Hover)`
///
/// Note: Radio buttons don't have a Disabled status in Iced 0.14.
/// The `is_selected` field is ignored for state mapping.
fn generate_radio_status_mapping(status_ident: &syn::Ident) -> TokenStream {
    quote! {
        {
            use iced::widget::radio::Status;
            match #status_ident {
                Status::Active { .. } => None,
                Status::Hovered { .. } => Some(dampen_core::ir::WidgetState::Hover),
            }
        }
    }
}

/// Generate toggler status mapping code
///
/// Maps `iced::widget::toggler::Status` to `Option<WidgetState>`:
/// - `Active { .. }` → `None` (base state)
/// - `Hovered { .. }` → `Some(Hover)`
/// - `Disabled { .. }` → `Some(Disabled)`
///
/// Note: The `is_toggled` field is ignored for state mapping.
fn generate_toggler_status_mapping(status_ident: &syn::Ident) -> TokenStream {
    quote! {
        {
            use iced::widget::toggler::Status;
            match #status_ident {
                Status::Active { .. } => None,
                Status::Hovered { .. } => Some(dampen_core::ir::WidgetState::Hover),
                Status::Disabled { .. } => Some(dampen_core::ir::WidgetState::Disabled),
            }
        }
    }
}

/// Generate slider status mapping code
///
/// Maps `iced::widget::slider::Status` to `Option<WidgetState>`:
/// - `Active` → `None` (base state)
/// - `Hovered` → `Some(Hover)`
/// - `Dragged` → `Some(Active)`
///
/// Note: Iced 0.14's slider::Status does NOT have a Disabled variant.
/// Disabled state must be checked separately via the `disabled` attribute.
fn generate_slider_status_mapping(status_ident: &syn::Ident) -> TokenStream {
    quote! {
        {
            use iced::widget::slider::Status;
            match #status_ident {
                Status::Active => None,
                Status::Hovered => Some(dampen_core::ir::WidgetState::Hover),
                Status::Dragged => Some(dampen_core::ir::WidgetState::Active),
            }
        }
    }
}

/// Generate pick list status mapping code
///
/// Maps `iced::widget::pick_list::Status` to `Option<WidgetState>`:
/// - `Active` → `None` (base state)
/// - `Hovered` → `Some(Hover)`
/// - `Opened { .. }` → `Some(Focus)` (dropdown menu is open)
fn generate_picklist_status_mapping(status_ident: &syn::Ident) -> TokenStream {
    quote! {
        {
            use iced::widget::pick_list::Status;
            match #status_ident {
                Status::Active => None,
                Status::Hovered => Some(dampen_core::ir::WidgetState::Hover),
                Status::Opened { .. } => Some(dampen_core::ir::WidgetState::Focus),
            }
        }
    }
}

/// Generate combo box status mapping code
///
/// ComboBox uses `text_input::Status`, so we reuse the text input mapping.
fn generate_combo_box_status_mapping(status_ident: &syn::Ident) -> TokenStream {
    generate_text_input_status_mapping(status_ident)
}

#[cfg(test)]
mod tests {
    use super::*;
    use quote::format_ident;

    #[test]
    fn test_button_status_mapping_generated() {
        let status = format_ident!("status");
        let mapping = generate_status_mapping(&WidgetKind::Button, &status);
        assert!(mapping.is_some());

        let code = mapping.unwrap().to_string();
        // Check for key status variants (format may vary due to quote!)
        assert!(code.contains("Active"));
        assert!(code.contains("Hovered"));
        assert!(code.contains("Pressed"));
        assert!(code.contains("Disabled"));
        assert!(code.contains("WidgetState"));
    }

    #[test]
    fn test_text_input_status_mapping_generated() {
        let status = format_ident!("status");
        let mapping = generate_status_mapping(&WidgetKind::TextInput, &status);
        assert!(mapping.is_some());

        let code = mapping.unwrap().to_string();
        assert!(code.contains("Focused"));
        assert!(code.contains("WidgetState"));
    }

    #[test]
    fn test_checkbox_status_mapping_generated() {
        let status = format_ident!("status");
        let mapping = generate_status_mapping(&WidgetKind::Checkbox, &status);
        assert!(mapping.is_some());

        let code = mapping.unwrap().to_string();
        assert!(code.contains("Active"));
        assert!(code.contains("Hovered"));
        assert!(code.contains("WidgetState"));
    }

    #[test]
    fn test_unsupported_widget_returns_none() {
        let status = format_ident!("status");
        let mapping = generate_status_mapping(&WidgetKind::Text, &status);
        assert!(mapping.is_none());
    }

    #[test]
    fn test_slider_status_mapping_generated() {
        let status = format_ident!("status");
        let mapping = generate_status_mapping(&WidgetKind::Slider, &status);
        assert!(mapping.is_some());

        let code = mapping.unwrap().to_string();
        assert!(code.contains("Dragged"));
        assert!(code.contains("WidgetState"));
    }

    #[test]
    fn test_combo_box_uses_text_input_mapping() {
        let status = format_ident!("status");
        let combo_mapping = generate_status_mapping(&WidgetKind::ComboBox, &status);
        let text_mapping = generate_status_mapping(&WidgetKind::TextInput, &status);

        assert!(combo_mapping.is_some());
        assert!(text_mapping.is_some());

        // ComboBox should generate the same code as TextInput
        let combo_code = combo_mapping.unwrap().to_string();
        let text_code = text_mapping.unwrap().to_string();
        assert_eq!(combo_code, text_code);
    }
}

//! View function generation
//!
//! This module generates static Rust code for widget trees with inlined bindings.

#![allow(dead_code)]

use crate::DampenDocument;
use crate::codegen::bindings::generate_expr;
use crate::ir::layout::{LayoutConstraints, Length as LayoutLength};
use crate::ir::node::{AttributeValue, InterpolatedPart, WidgetKind};
use crate::ir::style::{
    Background, Border, BorderRadius, Color, Gradient, Shadow, StyleProperties,
};
use crate::ir::theme::StyleClass;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use std::collections::HashMap;

/// Generate the view function body from a Dampen document
pub fn generate_view(
    document: &DampenDocument,
    _model_name: &str,
    message_name: &str,
) -> Result<TokenStream, super::CodegenError> {
    let message_ident = syn::Ident::new(message_name, proc_macro2::Span::call_site());
    let model_ident = syn::Ident::new("model", proc_macro2::Span::call_site());

    let root_widget = generate_widget(
        &document.root,
        &model_ident,
        &message_ident,
        &document.style_classes,
    )?;

    Ok(quote! {
        #root_widget
    })
}

/// Get merged layout constraints from node.layout and style classes
fn get_merged_layout<'a>(
    node: &'a crate::WidgetNode,
    style_classes: &'a HashMap<String, StyleClass>,
) -> Option<MergedLayout<'a>> {
    // Priority: node.layout > style_class.layout
    let node_layout = node.layout.as_ref();
    let class_layout = node
        .classes
        .first()
        .and_then(|class_name| style_classes.get(class_name))
        .and_then(|class| class.layout.as_ref());

    if node_layout.is_some() || class_layout.is_some() {
        Some(MergedLayout {
            node_layout,
            class_layout,
        })
    } else {
        None
    }
}

/// Helper struct to hold merged layout info from node and style class
struct MergedLayout<'a> {
    node_layout: Option<&'a LayoutConstraints>,
    class_layout: Option<&'a LayoutConstraints>,
}

impl<'a> MergedLayout<'a> {
    fn padding(&self) -> Option<f32> {
        self.node_layout
            .and_then(|l| l.padding.as_ref())
            .map(|p| p.top)
            .or_else(|| {
                self.class_layout
                    .and_then(|l| l.padding.as_ref())
                    .map(|p| p.top)
            })
    }

    fn spacing(&self) -> Option<f32> {
        self.node_layout
            .and_then(|l| l.spacing)
            .or_else(|| self.class_layout.and_then(|l| l.spacing))
    }

    fn width(&self) -> Option<&'a LayoutLength> {
        self.node_layout
            .and_then(|l| l.width.as_ref())
            .or_else(|| self.class_layout.and_then(|l| l.width.as_ref()))
    }

    fn height(&self) -> Option<&'a LayoutLength> {
        self.node_layout
            .and_then(|l| l.height.as_ref())
            .or_else(|| self.class_layout.and_then(|l| l.height.as_ref()))
    }
}

/// Generate code for a widget node
fn generate_widget(
    node: &crate::WidgetNode,
    model_ident: &syn::Ident,
    message_ident: &syn::Ident,
    style_classes: &HashMap<String, StyleClass>,
) -> Result<TokenStream, super::CodegenError> {
    generate_widget_with_locals(
        node,
        model_ident,
        message_ident,
        style_classes,
        &std::collections::HashSet::new(),
    )
}

/// Generate code for a widget node with local variable context
fn generate_widget_with_locals(
    node: &crate::WidgetNode,
    model_ident: &syn::Ident,
    message_ident: &syn::Ident,
    style_classes: &HashMap<String, StyleClass>,
    local_vars: &std::collections::HashSet<String>,
) -> Result<TokenStream, super::CodegenError> {
    match node.kind {
        WidgetKind::Text => generate_text_with_locals(node, model_ident, style_classes, local_vars),
        WidgetKind::Button => {
            generate_button_with_locals(node, model_ident, message_ident, style_classes, local_vars)
        }
        WidgetKind::Column => generate_container_with_locals(
            node,
            "column",
            model_ident,
            message_ident,
            style_classes,
            local_vars,
        ),
        WidgetKind::Row => generate_container_with_locals(
            node,
            "row",
            model_ident,
            message_ident,
            style_classes,
            local_vars,
        ),
        WidgetKind::Container => generate_container_with_locals(
            node,
            "container",
            model_ident,
            message_ident,
            style_classes,
            local_vars,
        ),
        WidgetKind::Scrollable => generate_container_with_locals(
            node,
            "scrollable",
            model_ident,
            message_ident,
            style_classes,
            local_vars,
        ),
        WidgetKind::Stack => generate_stack(node, model_ident, message_ident, style_classes),
        WidgetKind::Space => generate_space(node),
        WidgetKind::Rule => generate_rule(node),
        WidgetKind::Checkbox => generate_checkbox_with_locals(
            node,
            model_ident,
            message_ident,
            style_classes,
            local_vars,
        ),
        WidgetKind::Toggler => generate_toggler(node, model_ident, message_ident, style_classes),
        WidgetKind::Slider => generate_slider(node, model_ident, message_ident, style_classes),
        WidgetKind::Radio => generate_radio(node, model_ident, message_ident, style_classes),
        WidgetKind::ProgressBar => generate_progress_bar(node, model_ident, style_classes),
        WidgetKind::TextInput => generate_text_input_with_locals(
            node,
            model_ident,
            message_ident,
            style_classes,
            local_vars,
        ),
        WidgetKind::Image => generate_image(node),
        WidgetKind::Svg => generate_svg(node),
        WidgetKind::PickList => generate_pick_list(node, model_ident, message_ident, style_classes),
        WidgetKind::ComboBox => generate_combo_box(node, model_ident, message_ident, style_classes),
        WidgetKind::Tooltip => generate_tooltip(node, model_ident, message_ident, style_classes),
        WidgetKind::Grid => generate_grid(node, model_ident, message_ident, style_classes),
        WidgetKind::Canvas => generate_canvas(node, model_ident, message_ident, style_classes),
        WidgetKind::Float => generate_float(node, model_ident, message_ident, style_classes),
        WidgetKind::For => {
            generate_for_with_locals(node, model_ident, message_ident, style_classes, local_vars)
        }
        WidgetKind::If => {
            generate_if_with_locals(node, model_ident, message_ident, style_classes, local_vars)
        }
        WidgetKind::Custom(ref name) => {
            generate_custom_widget(node, name, model_ident, message_ident, style_classes)
        }
        WidgetKind::DatePicker => {
            generate_date_picker(node, model_ident, message_ident, style_classes)
        }
        WidgetKind::TimePicker => {
            generate_time_picker(node, model_ident, message_ident, style_classes)
        }
        WidgetKind::ColorPicker => {
            generate_color_picker(node, model_ident, message_ident, style_classes)
        }
        WidgetKind::Menu => generate_menu(node, model_ident, message_ident, style_classes),
        WidgetKind::MenuItem | WidgetKind::MenuSeparator => {
            // These are handled by generate_menu and shouldn't appear as top-level widgets
            Err(super::CodegenError::InvalidWidget(format!(
                "{:?} must be inside a <menu>",
                node.kind
            )))
        }
        WidgetKind::ContextMenu => {
            generate_context_menu(node, model_ident, message_ident, style_classes, local_vars)
        }
        WidgetKind::DataTable => {
            generate_data_table(node, model_ident, message_ident, style_classes)
        }
        WidgetKind::DataColumn => {
            // These are handled by generate_data_table logic, shouldn't appear as top-level widgets
            Err(super::CodegenError::InvalidWidget(format!(
                "{:?} must be inside a <data_table>",
                node.kind
            )))
        }
        WidgetKind::TreeView => {
            generate_tree_view(node, model_ident, message_ident, style_classes, local_vars)
        }
        WidgetKind::TreeNode => {
            // These are handled by generate_tree_view logic, shouldn't appear as top-level widgets
            Err(super::CodegenError::InvalidWidget(format!(
                "{:?} must be inside a <tree_view>",
                node.kind
            )))
        }
        WidgetKind::CanvasRect
        | WidgetKind::CanvasCircle
        | WidgetKind::CanvasLine
        | WidgetKind::CanvasText
        | WidgetKind::CanvasGroup => {
            // These are handled by generate_canvas logic, shouldn't appear as top-level widgets
            Err(super::CodegenError::InvalidWidget(format!(
                "{:?} is not a top-level widget and must be inside a <canvas>",
                node.kind
            )))
        }
        WidgetKind::TabBar => generate_tab_bar_with_locals(
            node,
            model_ident,
            message_ident,
            style_classes,
            local_vars,
        ),
        WidgetKind::Tab => {
            // Tab must be inside TabBar, handled by generate_tab_bar
            Err(super::CodegenError::InvalidWidget(
                "Tab must be inside TabBar".to_string(),
            ))
        }
    }
}

// ============================================================================
// Style Application Functions
// ============================================================================

/// Apply inline styles or CSS classes to a widget
///
/// Priority order:
/// 1. Inline styles (node.style) - highest priority
/// 2. CSS classes (node.classes) - medium priority
/// 3. Default Iced styles - lowest priority (fallback)
fn apply_widget_style(
    widget: TokenStream,
    node: &crate::WidgetNode,
    widget_type: &str,
    style_classes: &HashMap<String, StyleClass>,
) -> Result<TokenStream, super::CodegenError> {
    // Check if widget has any styling
    let has_inline_style = node.style.is_some();
    let has_classes = !node.classes.is_empty();

    // Check for dynamic class binding (e.g., class="{if filter == 'All' then 'btn_primary' else 'btn_filter'}")
    let class_binding = node.attributes.get("class").and_then(|attr| match attr {
        AttributeValue::Binding(expr) => Some(expr),
        _ => None,
    });
    let has_class_binding = class_binding.is_some();

    if !has_inline_style && !has_classes && !has_class_binding {
        // No styling needed, return widget as-is
        return Ok(widget);
    }

    // Get style class if widget has classes
    let style_class = if let Some(class_name) = node.classes.first() {
        style_classes.get(class_name)
    } else {
        None
    };

    // Generate style closure based on priority
    if let Some(ref style_props) = node.style {
        // Priority 1: Inline styles
        let style_closure =
            generate_inline_style_closure(style_props, widget_type, &node.kind, style_class)?;
        Ok(quote! {
            #widget.style(#style_closure)
        })
    } else if let Some(class_name) = node.classes.first() {
        // Priority 2: CSS class (use first class for now)
        // Generate a wrapper closure that matches the widget's expected signature
        let style_fn_ident = format_ident!("style_{}", class_name.replace('-', "_"));

        match widget_type {
            "text_input" => {
                // text_input.style() expects fn(theme, status) -> text_input::Style
                // Style class functions return container::Style, so we need to convert
                Ok(quote! {
                    #widget.style(|theme: &iced::Theme, _status: iced::widget::text_input::Status| {
                        let container_style = #style_fn_ident(theme);
                        iced::widget::text_input::Style {
                            background: container_style.background.unwrap_or(iced::Background::Color(theme.extended_palette().background.base.color)),
                            border: container_style.border,
                            icon: theme.extended_palette().background.base.text,
                            placeholder: theme.extended_palette().background.weak.text,
                            value: container_style.text_color.unwrap_or(theme.extended_palette().background.base.text),
                            selection: theme.extended_palette().primary.weak.color,
                        }
                    })
                })
            }
            "checkbox" => {
                // checkbox.style() expects fn(theme, status) -> checkbox::Style
                // Check if the style class has state variants (needs 2-arg call with button::Status)
                let has_state_variants = style_class
                    .map(|sc| !sc.state_variants.is_empty())
                    .unwrap_or(false);

                if has_state_variants {
                    // Style function expects (theme, button::Status), map checkbox status to button status
                    Ok(quote! {
                        #widget.style(|theme: &iced::Theme, status: iced::widget::checkbox::Status| {
                            // Map checkbox status to button status for the style function
                            let button_status = match status {
                                iced::widget::checkbox::Status::Active { .. } => iced::widget::button::Status::Active,
                                iced::widget::checkbox::Status::Hovered { .. } => iced::widget::button::Status::Hovered,
                                iced::widget::checkbox::Status::Disabled { .. } => iced::widget::button::Status::Disabled,
                            };
                            let button_style = #style_fn_ident(theme, button_status);
                            iced::widget::checkbox::Style {
                                background: button_style.background.unwrap_or(iced::Background::Color(iced::Color::WHITE)),
                                icon_color: button_style.text_color,
                                border: button_style.border,
                                text_color: None,
                            }
                        })
                    })
                } else {
                    // Style function expects only theme (container style)
                    Ok(quote! {
                        #widget.style(|theme: &iced::Theme, _status: iced::widget::checkbox::Status| {
                            let container_style = #style_fn_ident(theme);
                            iced::widget::checkbox::Style {
                                background: container_style.background.unwrap_or(iced::Background::Color(iced::Color::WHITE)),
                                icon_color: container_style.text_color,
                                border: container_style.border,
                                text_color: None,
                            }
                        })
                    })
                }
            }
            "button" => {
                // button.style() expects fn(theme, status) -> button::Style
                // Style class functions for buttons already have the correct signature
                Ok(quote! {
                    #widget.style(#style_fn_ident)
                })
            }
            _ => {
                // Default: container-style widgets (container, row, column, etc.)
                Ok(quote! {
                    #widget.style(#style_fn_ident)
                })
            }
        }
    } else if let Some(binding_expr) = class_binding {
        // Priority 3: Dynamic class binding
        generate_dynamic_class_style(widget, binding_expr, widget_type, style_classes)
    } else {
        Ok(widget)
    }
}

/// Generate style application for dynamic class bindings
///
/// Generates code that evaluates the binding at runtime and dispatches
/// to the appropriate style function based on the class name.
fn generate_dynamic_class_style(
    widget: TokenStream,
    binding_expr: &crate::expr::BindingExpr,
    widget_type: &str,
    style_classes: &HashMap<String, StyleClass>,
) -> Result<TokenStream, super::CodegenError> {
    // Generate code to evaluate the binding
    let class_expr = super::bindings::generate_expr(&binding_expr.expr);

    match widget_type {
        "button" => {
            // Generate match arms only for button-compatible style classes
            // (those with state variants, which generate fn(theme, status) -> button::Style)
            let mut match_arms = Vec::new();
            for (class_name, style_class) in style_classes.iter() {
                // Only include classes that have state variants (button styles)
                if !style_class.state_variants.is_empty() {
                    let style_fn = format_ident!("style_{}", class_name.replace('-', "_"));
                    let class_lit = proc_macro2::Literal::string(class_name);
                    match_arms.push(quote! {
                        #class_lit => #style_fn(_theme, status),
                    });
                }
            }

            Ok(quote! {
                #widget.style({
                    let __class_name = #class_expr;
                    move |_theme: &iced::Theme, status: iced::widget::button::Status| {
                        match __class_name.as_str() {
                            #(#match_arms)*
                            _ => iced::widget::button::Style::default(),
                        }
                    }
                })
            })
        }
        "checkbox" => {
            // For checkboxes, map checkbox status to button status
            // Only use style classes with state variants
            let mut checkbox_match_arms = Vec::new();
            for (class_name, style_class) in style_classes.iter() {
                if !style_class.state_variants.is_empty() {
                    let style_fn = format_ident!("style_{}", class_name.replace('-', "_"));
                    let class_lit = proc_macro2::Literal::string(class_name);
                    checkbox_match_arms.push(quote! {
                        #class_lit => {
                            let button_style = #style_fn(_theme, button_status);
                            iced::widget::checkbox::Style {
                                background: button_style.background.unwrap_or(iced::Background::Color(iced::Color::WHITE)),
                                icon_color: button_style.text_color,
                                border: button_style.border,
                                text_color: None,
                            }
                        }
                    });
                }
            }
            Ok(quote! {
                #widget.style({
                    let __class_name = #class_expr;
                    move |_theme: &iced::Theme, status: iced::widget::checkbox::Status| {
                        let button_status = match status {
                            iced::widget::checkbox::Status::Active { .. } => iced::widget::button::Status::Active,
                            iced::widget::checkbox::Status::Hovered { .. } => iced::widget::button::Status::Hovered,
                            iced::widget::checkbox::Status::Disabled { .. } => iced::widget::button::Status::Disabled,
                        };
                        match __class_name.as_str() {
                            #(#checkbox_match_arms)*
                            _ => iced::widget::checkbox::Style::default(),
                        }
                    }
                })
            })
        }
        _ => {
            // For other widgets (container, etc.), use container style functions
            // Only include classes without state variants (container styles)
            let mut container_match_arms = Vec::new();
            for (class_name, style_class) in style_classes.iter() {
                if style_class.state_variants.is_empty() {
                    let style_fn = format_ident!("style_{}", class_name.replace('-', "_"));
                    let class_lit = proc_macro2::Literal::string(class_name);
                    container_match_arms.push(quote! {
                        #class_lit => #style_fn(_theme),
                    });
                }
            }
            Ok(quote! {
                #widget.style({
                    let __class_name = #class_expr;
                    move |_theme: &iced::Theme| {
                        match __class_name.as_str() {
                            #(#container_match_arms)*
                            _ => iced::widget::container::Style::default(),
                        }
                    }
                })
            })
        }
    }
}

/// Generate state-specific style application code
///
/// Creates a match expression that applies different styles based on widget state.
/// Merges base style with state-specific overrides.
///
/// # Arguments
/// * `base_style` - The base style struct (used when state is None)
/// * `style_class` - The style class containing state variants
/// * `widget_state_ident` - Identifier for the widget_state variable
/// * `style_struct_fn` - Function to generate style struct from StyleProperties
fn generate_state_style_match(
    base_style: TokenStream,
    style_class: &StyleClass,
    widget_state_ident: &syn::Ident,
    style_struct_fn: fn(&StyleProperties) -> Result<TokenStream, super::CodegenError>,
) -> Result<TokenStream, super::CodegenError> {
    use crate::ir::theme::WidgetState;

    // Collect all state variants
    let mut state_arms = Vec::new();

    for (state, state_props) in &style_class.state_variants {
        let state_variant = match state {
            WidgetState::Hover => quote! { dampen_core::ir::WidgetState::Hover },
            WidgetState::Focus => quote! { dampen_core::ir::WidgetState::Focus },
            WidgetState::Active => quote! { dampen_core::ir::WidgetState::Active },
            WidgetState::Disabled => quote! { dampen_core::ir::WidgetState::Disabled },
        };

        // Generate style struct for this state
        let state_style = style_struct_fn(state_props)?;

        state_arms.push(quote! {
            Some(#state_variant) => #state_style
        });
    }

    // Generate match expression
    Ok(quote! {
        match #widget_state_ident {
            #(#state_arms,)*
            None => #base_style
        }
    })
}

/// Generate inline style closure for a widget
///
/// Creates a closure like: |_theme: &iced::Theme, status| { ... }
///
/// # State-Aware Styling
///
/// When a widget has state variants (hover, focus, etc.), this generates
/// code that maps the status parameter to WidgetState and applies the
/// appropriate style.
///
/// # Arguments
/// * `style_props` - Base style properties for the widget
/// * `widget_type` - Type of widget ("button", "text_input", etc.)
/// * `widget_kind` - The WidgetKind enum (needed for status mapping)
/// * `style_class` - Optional style class with state variants
fn generate_inline_style_closure(
    style_props: &StyleProperties,
    widget_type: &str,
    widget_kind: &WidgetKind,
    style_class: Option<&StyleClass>,
) -> Result<TokenStream, super::CodegenError> {
    // Check if we have state-specific styling
    let has_state_variants = style_class
        .map(|sc| !sc.state_variants.is_empty())
        .unwrap_or(false);

    match widget_type {
        "button" => {
            let base_style = generate_button_style_struct(style_props)?;

            if has_state_variants {
                // Generate state-aware closure
                let status_ident = format_ident!("status");
                if let Some(status_mapping) =
                    super::status_mapping::generate_status_mapping(widget_kind, &status_ident)
                {
                    let widget_state_ident = format_ident!("widget_state");
                    // Safe: has_state_variants guarantees style_class.is_some()
                    let class = style_class.ok_or_else(|| {
                        super::CodegenError::InvalidWidget(
                            "Expected style class with state variants".to_string(),
                        )
                    })?;
                    let style_match = generate_state_style_match(
                        base_style,
                        class,
                        &widget_state_ident,
                        generate_button_style_struct,
                    )?;

                    Ok(quote! {
                        |_theme: &iced::Theme, #status_ident: iced::widget::button::Status| {
                            // Map Iced status to WidgetState
                            let #widget_state_ident = #status_mapping;

                            // Apply state-specific styling
                            #style_match
                        }
                    })
                } else {
                    // Widget kind doesn't support status mapping, fall back to simple closure
                    Ok(quote! {
                        |_theme: &iced::Theme, _status: iced::widget::button::Status| {
                            #base_style
                        }
                    })
                }
            } else {
                // No state variants, use simple closure
                Ok(quote! {
                    |_theme: &iced::Theme, _status: iced::widget::button::Status| {
                        #base_style
                    }
                })
            }
        }
        "container" => {
            let style_struct = generate_container_style_struct(style_props)?;
            Ok(quote! {
                |_theme: &iced::Theme| {
                    #style_struct
                }
            })
        }
        "text_input" => {
            let base_style = generate_text_input_style_struct(style_props)?;

            if has_state_variants {
                // Generate state-aware closure
                let status_ident = format_ident!("status");
                if let Some(status_mapping) =
                    super::status_mapping::generate_status_mapping(widget_kind, &status_ident)
                {
                    let widget_state_ident = format_ident!("widget_state");
                    let class = style_class.ok_or_else(|| {
                        super::CodegenError::InvalidWidget(
                            "Expected style class with state variants".to_string(),
                        )
                    })?;
                    let style_match = generate_state_style_match(
                        base_style,
                        class,
                        &widget_state_ident,
                        generate_text_input_style_struct,
                    )?;

                    Ok(quote! {
                        |_theme: &iced::Theme, #status_ident: iced::widget::text_input::Status| {
                            // Map Iced status to WidgetState
                            let #widget_state_ident = #status_mapping;

                            // Apply state-specific styling
                            #style_match
                        }
                    })
                } else {
                    // Widget kind doesn't support status mapping, fall back to simple closure
                    Ok(quote! {
                        |_theme: &iced::Theme, _status: iced::widget::text_input::Status| {
                            #base_style
                        }
                    })
                }
            } else {
                // No state variants, use simple closure
                Ok(quote! {
                    |_theme: &iced::Theme, _status: iced::widget::text_input::Status| {
                        #base_style
                    }
                })
            }
        }
        "checkbox" => {
            let base_style = generate_checkbox_style_struct(style_props)?;

            if has_state_variants {
                let status_ident = format_ident!("status");
                if let Some(status_mapping) =
                    super::status_mapping::generate_status_mapping(widget_kind, &status_ident)
                {
                    let widget_state_ident = format_ident!("widget_state");
                    let class = style_class.ok_or_else(|| {
                        super::CodegenError::InvalidWidget(
                            "Expected style class with state variants".to_string(),
                        )
                    })?;
                    let style_match = generate_state_style_match(
                        base_style,
                        class,
                        &widget_state_ident,
                        generate_checkbox_style_struct,
                    )?;

                    Ok(quote! {
                        |_theme: &iced::Theme, #status_ident: iced::widget::checkbox::Status| {
                            let #widget_state_ident = #status_mapping;
                            #style_match
                        }
                    })
                } else {
                    Ok(quote! {
                        |_theme: &iced::Theme, _status: iced::widget::checkbox::Status| {
                            #base_style
                        }
                    })
                }
            } else {
                Ok(quote! {
                    |_theme: &iced::Theme, _status: iced::widget::checkbox::Status| {
                        #base_style
                    }
                })
            }
        }
        "toggler" => {
            let base_style = generate_toggler_style_struct(style_props)?;

            if has_state_variants {
                let status_ident = format_ident!("status");
                if let Some(status_mapping) =
                    super::status_mapping::generate_status_mapping(widget_kind, &status_ident)
                {
                    let widget_state_ident = format_ident!("widget_state");
                    let class = style_class.ok_or_else(|| {
                        super::CodegenError::InvalidWidget(
                            "Expected style class with state variants".to_string(),
                        )
                    })?;
                    let style_match = generate_state_style_match(
                        base_style,
                        class,
                        &widget_state_ident,
                        generate_toggler_style_struct,
                    )?;

                    Ok(quote! {
                        |_theme: &iced::Theme, #status_ident: iced::widget::toggler::Status| {
                            let #widget_state_ident = #status_mapping;
                            #style_match
                        }
                    })
                } else {
                    Ok(quote! {
                        |_theme: &iced::Theme, _status: iced::widget::toggler::Status| {
                            #base_style
                        }
                    })
                }
            } else {
                Ok(quote! {
                    |_theme: &iced::Theme, _status: iced::widget::toggler::Status| {
                        #base_style
                    }
                })
            }
        }
        "slider" => {
            let base_style = generate_slider_style_struct(style_props)?;

            if has_state_variants {
                let status_ident = format_ident!("status");
                if let Some(status_mapping) =
                    super::status_mapping::generate_status_mapping(widget_kind, &status_ident)
                {
                    let widget_state_ident = format_ident!("widget_state");
                    let class = style_class.ok_or_else(|| {
                        super::CodegenError::InvalidWidget(
                            "Expected style class with state variants".to_string(),
                        )
                    })?;
                    let style_match = generate_state_style_match(
                        base_style,
                        class,
                        &widget_state_ident,
                        generate_slider_style_struct,
                    )?;

                    Ok(quote! {
                        |_theme: &iced::Theme, #status_ident: iced::widget::slider::Status| {
                            let #widget_state_ident = #status_mapping;
                            #style_match
                        }
                    })
                } else {
                    Ok(quote! {
                        |_theme: &iced::Theme, _status: iced::widget::slider::Status| {
                            #base_style
                        }
                    })
                }
            } else {
                Ok(quote! {
                    |_theme: &iced::Theme, _status: iced::widget::slider::Status| {
                        #base_style
                    }
                })
            }
        }
        _ => {
            // For unsupported widgets, return a no-op closure
            Ok(quote! {
                |_theme: &iced::Theme| iced::widget::container::Style::default()
            })
        }
    }
}

// ============================================================================
// Helper Functions (copied from theme.rs for reuse)
// ============================================================================

/// Generate iced::Color from Color IR
fn generate_color_expr(color: &Color) -> TokenStream {
    let r = color.r;
    let g = color.g;
    let b = color.b;
    let a = color.a;
    quote! {
        iced::Color::from_rgba(#r, #g, #b, #a)
    }
}

/// Generate iced::Background from Background IR
fn generate_background_expr(bg: &Background) -> TokenStream {
    match bg {
        Background::Color(color) => {
            let color_expr = generate_color_expr(color);
            quote! { iced::Background::Color(#color_expr) }
        }
        Background::Gradient(gradient) => generate_gradient_expr(gradient),
        Background::Image { .. } => {
            quote! { iced::Background::Color(iced::Color::TRANSPARENT) }
        }
    }
}

/// Generate iced::Gradient from Gradient IR
fn generate_gradient_expr(gradient: &Gradient) -> TokenStream {
    match gradient {
        Gradient::Linear { angle, stops } => {
            let radians = angle * (std::f32::consts::PI / 180.0);
            let color_exprs: Vec<_> = stops
                .iter()
                .map(|s| generate_color_expr(&s.color))
                .collect();
            let offsets: Vec<_> = stops.iter().map(|s| s.offset).collect();

            quote! {
                iced::Background::Gradient(iced::Gradient::Linear(
                    iced::gradient::Linear::new(#radians)
                        #(.add_stop(#offsets, #color_exprs))*
                ))
            }
        }
        Gradient::Radial { stops, .. } => {
            // Fallback to linear for radial (Iced limitation)
            let color_exprs: Vec<_> = stops
                .iter()
                .map(|s| generate_color_expr(&s.color))
                .collect();
            let offsets: Vec<_> = stops.iter().map(|s| s.offset).collect();

            quote! {
                iced::Background::Gradient(iced::Gradient::Linear(
                    iced::gradient::Linear::new(0.0)
                        #(.add_stop(#offsets, #color_exprs))*
                ))
            }
        }
    }
}

/// Generate iced::Border from Border IR
fn generate_border_expr(border: &Border) -> TokenStream {
    let width = border.width;
    let color_expr = generate_color_expr(&border.color);
    let radius_expr = generate_border_radius_expr(&border.radius);

    quote! {
        iced::Border {
            width: #width,
            color: #color_expr,
            radius: #radius_expr,
        }
    }
}

/// Generate iced::border::Radius from BorderRadius IR
fn generate_border_radius_expr(radius: &BorderRadius) -> TokenStream {
    let tl = radius.top_left;
    let tr = radius.top_right;
    let br = radius.bottom_right;
    let bl = radius.bottom_left;

    quote! {
        iced::border::Radius::from(#tl).top_right(#tr).bottom_right(#br).bottom_left(#bl)
    }
}

/// Generate iced::Shadow from Shadow IR
fn generate_shadow_expr(shadow: &Shadow) -> TokenStream {
    let offset_x = shadow.offset_x;
    let offset_y = shadow.offset_y;
    let blur = shadow.blur_radius;
    let color_expr = generate_color_expr(&shadow.color);

    quote! {
        iced::Shadow {
            offset: iced::Vector::new(#offset_x, #offset_y),
            blur_radius: #blur,
            color: #color_expr,
        }
    }
}

/// Generate iced::widget::button::Style struct from StyleProperties
///
/// When no explicit color is set, uses theme text color via _theme parameter
/// that's available in the style closure scope.
fn generate_button_style_struct(
    props: &StyleProperties,
) -> Result<TokenStream, super::CodegenError> {
    let background_expr = props
        .background
        .as_ref()
        .map(|bg| {
            let expr = generate_background_expr(bg);
            quote! { Some(#expr) }
        })
        .unwrap_or_else(|| quote! { None });

    // Use theme text color as fallback instead of hardcoded BLACK
    let text_color_expr = props
        .color
        .as_ref()
        .map(generate_color_expr)
        .unwrap_or_else(|| quote! { _theme.extended_palette().background.base.text });

    let border_expr = props
        .border
        .as_ref()
        .map(generate_border_expr)
        .unwrap_or_else(|| quote! { iced::Border::default() });

    let shadow_expr = props
        .shadow
        .as_ref()
        .map(generate_shadow_expr)
        .unwrap_or_else(|| quote! { iced::Shadow::default() });

    Ok(quote! {
        iced::widget::button::Style {
            background: #background_expr,
            text_color: #text_color_expr,
            border: #border_expr,
            shadow: #shadow_expr,
            snap: false,
        }
    })
}

/// Generate iced::widget::container::Style struct from StyleProperties
fn generate_container_style_struct(
    props: &StyleProperties,
) -> Result<TokenStream, super::CodegenError> {
    let background_expr = props
        .background
        .as_ref()
        .map(|bg| {
            let expr = generate_background_expr(bg);
            quote! { Some(#expr) }
        })
        .unwrap_or_else(|| quote! { None });

    let text_color_expr = props
        .color
        .as_ref()
        .map(|color| {
            let color_expr = generate_color_expr(color);
            quote! { Some(#color_expr) }
        })
        .unwrap_or_else(|| quote! { None });

    let border_expr = props
        .border
        .as_ref()
        .map(generate_border_expr)
        .unwrap_or_else(|| quote! { iced::Border::default() });

    let shadow_expr = props
        .shadow
        .as_ref()
        .map(generate_shadow_expr)
        .unwrap_or_else(|| quote! { iced::Shadow::default() });

    Ok(quote! {
        iced::widget::container::Style {
            background: #background_expr,
            text_color: #text_color_expr,
            border: #border_expr,
            shadow: #shadow_expr,
            snap: false,
        }
    })
}

/// Generate iced::widget::text_input::Style struct from StyleProperties
///
/// When no explicit colors are set, uses theme text colors via _theme parameter
/// that's available in the style closure scope.
fn generate_text_input_style_struct(
    props: &StyleProperties,
) -> Result<TokenStream, super::CodegenError> {
    let background_expr = props
        .background
        .as_ref()
        .map(|bg| {
            let expr = generate_background_expr(bg);
            quote! { #expr }
        })
        .unwrap_or_else(
            || quote! { iced::Background::Color(_theme.extended_palette().background.base.color) },
        );

    let border_expr = props
        .border
        .as_ref()
        .map(generate_border_expr)
        .unwrap_or_else(|| quote! { iced::Border::default() });

    // Use theme colors as fallback instead of hardcoded colors
    let value_color = props
        .color
        .as_ref()
        .map(generate_color_expr)
        .unwrap_or_else(|| quote! { _theme.extended_palette().background.base.text });

    Ok(quote! {
        iced::widget::text_input::Style {
            background: #background_expr,
            border: #border_expr,
            icon: _theme.extended_palette().background.base.text,
            placeholder: _theme.extended_palette().background.weak.text,
            value: #value_color,
            selection: _theme.extended_palette().primary.weak.color,
        }
    })
}

/// Generate checkbox style struct from StyleProperties
///
/// When no explicit colors are set, uses theme colors via _theme parameter
/// that's available in the style closure scope.
fn generate_checkbox_style_struct(
    props: &StyleProperties,
) -> Result<TokenStream, super::CodegenError> {
    let background_expr = props
        .background
        .as_ref()
        .map(|bg| {
            let expr = generate_background_expr(bg);
            quote! { #expr }
        })
        .unwrap_or_else(
            || quote! { iced::Background::Color(_theme.extended_palette().background.base.color) },
        );

    let border_expr = props
        .border
        .as_ref()
        .map(generate_border_expr)
        .unwrap_or_else(|| quote! { iced::Border::default() });

    // Use theme text color as fallback instead of hardcoded BLACK
    let text_color = props
        .color
        .as_ref()
        .map(generate_color_expr)
        .unwrap_or_else(|| quote! { _theme.extended_palette().primary.base.color });

    Ok(quote! {
        iced::widget::checkbox::Style {
            background: #background_expr,
            icon_color: #text_color,
            border: #border_expr,
            text_color: None,
        }
    })
}

/// Generate toggler style struct from StyleProperties
fn generate_toggler_style_struct(
    props: &StyleProperties,
) -> Result<TokenStream, super::CodegenError> {
    let background_expr = props
        .background
        .as_ref()
        .map(|bg| {
            let expr = generate_background_expr(bg);
            quote! { #expr }
        })
        .unwrap_or_else(
            || quote! { iced::Background::Color(iced::Color::from_rgb(0.5, 0.5, 0.5)) },
        );

    Ok(quote! {
        iced::widget::toggler::Style {
            background: #background_expr,
            background_border_width: 0.0,
            background_border_color: iced::Color::TRANSPARENT,
            foreground: iced::Background::Color(iced::Color::WHITE),
            foreground_border_width: 0.0,
            foreground_border_color: iced::Color::TRANSPARENT,
        }
    })
}

/// Generate slider style struct from StyleProperties
fn generate_slider_style_struct(
    props: &StyleProperties,
) -> Result<TokenStream, super::CodegenError> {
    let border_expr = props
        .border
        .as_ref()
        .map(generate_border_expr)
        .unwrap_or_else(|| quote! { iced::Border::default() });

    Ok(quote! {
        iced::widget::slider::Style {
            rail: iced::widget::slider::Rail {
                colors: (
                    iced::Color::from_rgb(0.6, 0.6, 0.6),
                    iced::Color::from_rgb(0.2, 0.6, 1.0),
                ),
                width: 4.0,
                border: #border_expr,
            },
            handle: iced::widget::slider::Handle {
                shape: iced::widget::slider::HandleShape::Circle { radius: 8.0 },
                color: iced::Color::WHITE,
                border_width: 1.0,
                border_color: iced::Color::from_rgb(0.6, 0.6, 0.6),
            },
        }
    })
}

/// Generate text widget
fn generate_text(
    node: &crate::WidgetNode,
    model_ident: &syn::Ident,
    _style_classes: &HashMap<String, StyleClass>,
) -> Result<TokenStream, super::CodegenError> {
    let value_attr = node.attributes.get("value").ok_or_else(|| {
        super::CodegenError::InvalidWidget("text requires value attribute".to_string())
    })?;

    let value_expr = generate_attribute_value(value_attr, model_ident);

    let mut text_widget = quote! {
        iced::widget::text(#value_expr)
    };

    // Apply size attribute
    if let Some(size) = node.attributes.get("size").and_then(|attr| {
        if let AttributeValue::Static(s) = attr {
            s.parse::<f32>().ok()
        } else {
            None
        }
    }) {
        text_widget = quote! { #text_widget.size(#size) };
    }

    // Apply weight attribute (bold, normal, etc.)
    if let Some(weight) = node.attributes.get("weight").and_then(|attr| {
        if let AttributeValue::Static(s) = attr {
            Some(s.clone())
        } else {
            None
        }
    }) {
        let weight_expr = match weight.to_lowercase().as_str() {
            "bold" => quote! { iced::font::Weight::Bold },
            "semibold" => quote! { iced::font::Weight::Semibold },
            "medium" => quote! { iced::font::Weight::Medium },
            "light" => quote! { iced::font::Weight::Light },
            _ => quote! { iced::font::Weight::Normal },
        };
        text_widget = quote! {
            #text_widget.font(iced::Font { weight: #weight_expr, ..Default::default() })
        };
    }

    // Apply inline style color if present
    if let Some(ref style_props) = node.style
        && let Some(ref color) = style_props.color
    {
        let color_expr = generate_color_expr(color);
        text_widget = quote! { #text_widget.color(#color_expr) };
    }

    // Use helper to wrap in container if layout attributes are present
    Ok(maybe_wrap_in_container(text_widget, node))
}

/// Generate Length expression from string
fn generate_length_expr(s: &str) -> TokenStream {
    let s = s.trim().to_lowercase();
    if s == "fill" {
        quote! { iced::Length::Fill }
    } else if s == "shrink" {
        quote! { iced::Length::Shrink }
    } else if let Some(pct) = s.strip_suffix('%') {
        if let Ok(p) = pct.parse::<f32>() {
            // Iced doesn't have a direct percentage, use FillPortion as approximation
            let portion = ((p / 100.0) * 16.0).round() as u16;
            let portion = portion.max(1);
            quote! { iced::Length::FillPortion(#portion) }
        } else {
            quote! { iced::Length::Shrink }
        }
    } else if let Ok(px) = s.parse::<f32>() {
        quote! { iced::Length::Fixed(#px) }
    } else {
        quote! { iced::Length::Shrink }
    }
}

/// Generate Length expression from LayoutLength IR type
fn generate_layout_length_expr(length: &LayoutLength) -> TokenStream {
    match length {
        LayoutLength::Fixed(px) => quote! { iced::Length::Fixed(#px) },
        LayoutLength::Fill => quote! { iced::Length::Fill },
        LayoutLength::Shrink => quote! { iced::Length::Shrink },
        LayoutLength::FillPortion(portion) => {
            let p = *portion as u16;
            quote! { iced::Length::FillPortion(#p) }
        }
        LayoutLength::Percentage(pct) => {
            // Iced doesn't have direct percentage, approximate with FillPortion
            let portion = ((pct / 100.0) * 16.0).round() as u16;
            let portion = portion.max(1);
            quote! { iced::Length::FillPortion(#portion) }
        }
    }
}

/// Generate horizontal alignment expression
fn generate_horizontal_alignment_expr(s: &str) -> TokenStream {
    match s.trim().to_lowercase().as_str() {
        "center" => quote! { iced::alignment::Horizontal::Center },
        "end" | "right" => quote! { iced::alignment::Horizontal::Right },
        _ => quote! { iced::alignment::Horizontal::Left },
    }
}

/// Generate vertical alignment expression
fn generate_vertical_alignment_expr(s: &str) -> TokenStream {
    match s.trim().to_lowercase().as_str() {
        "center" => quote! { iced::alignment::Vertical::Center },
        "end" | "bottom" => quote! { iced::alignment::Vertical::Bottom },
        _ => quote! { iced::alignment::Vertical::Top },
    }
}

/// Wraps a widget in a container if layout attributes are present.
///
/// This helper provides consistent layout attribute support across all widgets
/// by wrapping them in a container when needed.
///
/// # Arguments
///
/// * `widget` - The widget expression to potentially wrap
/// * `node` - The widget node containing attributes
///
/// # Returns
///
/// Returns the widget wrapped in a container if layout attributes are present,
/// otherwise returns the original widget.
fn maybe_wrap_in_container(widget: TokenStream, node: &crate::WidgetNode) -> TokenStream {
    // Check if we need to wrap in container for layout/alignment/classes
    let needs_container = node.layout.is_some()
        || !node.classes.is_empty()
        || node.attributes.contains_key("align_x")
        || node.attributes.contains_key("align_y")
        || node.attributes.contains_key("width")
        || node.attributes.contains_key("height")
        || node.attributes.contains_key("padding");

    if !needs_container {
        return quote! { #widget.into() };
    }

    let mut container = quote! {
        iced::widget::container(#widget)
    };

    // Apply width
    if let Some(width) = node.attributes.get("width").and_then(|attr| {
        if let AttributeValue::Static(s) = attr {
            Some(s.clone())
        } else {
            None
        }
    }) {
        let width_expr = generate_length_expr(&width);
        container = quote! { #container.width(#width_expr) };
    }

    // Apply height
    if let Some(height) = node.attributes.get("height").and_then(|attr| {
        if let AttributeValue::Static(s) = attr {
            Some(s.clone())
        } else {
            None
        }
    }) {
        let height_expr = generate_length_expr(&height);
        container = quote! { #container.height(#height_expr) };
    }

    // Apply padding
    if let Some(padding) = node.attributes.get("padding").and_then(|attr| {
        if let AttributeValue::Static(s) = attr {
            s.parse::<f32>().ok()
        } else {
            None
        }
    }) {
        container = quote! { #container.padding(#padding) };
    }

    // Apply align_x
    if let Some(align_x) = node.attributes.get("align_x").and_then(|attr| {
        if let AttributeValue::Static(s) = attr {
            Some(s.clone())
        } else {
            None
        }
    }) {
        let align_expr = generate_horizontal_alignment_expr(&align_x);
        container = quote! { #container.align_x(#align_expr) };
    }

    // Apply align_y
    if let Some(align_y) = node.attributes.get("align_y").and_then(|attr| {
        if let AttributeValue::Static(s) = attr {
            Some(s.clone())
        } else {
            None
        }
    }) {
        let align_expr = generate_vertical_alignment_expr(&align_y);
        container = quote! { #container.align_y(#align_expr) };
    }

    // Apply class style if present
    if let Some(class_name) = node.classes.first() {
        let style_fn_ident = format_ident!("style_{}", class_name.replace('-', "_"));
        container = quote! { #container.style(#style_fn_ident) };
    }

    quote! { #container.into() }
}

/// Generate button widget
fn generate_button(
    node: &crate::WidgetNode,
    model_ident: &syn::Ident,
    message_ident: &syn::Ident,
    style_classes: &HashMap<String, StyleClass>,
) -> Result<TokenStream, super::CodegenError> {
    let label_attr = node.attributes.get("label").ok_or_else(|| {
        super::CodegenError::InvalidWidget("button requires label attribute".to_string())
    })?;

    let label_expr = generate_attribute_value(label_attr, model_ident);

    let on_click = node
        .events
        .iter()
        .find(|e| e.event == crate::EventKind::Click);

    let mut button = quote! {
        iced::widget::button(iced::widget::text(#label_expr))
    };

    // Handle enabled attribute
    let enabled_condition = node.attributes.get("enabled").map(|attr| match attr {
        AttributeValue::Static(s) => {
            // Static enabled values
            match s.to_lowercase().as_str() {
                "true" | "1" | "yes" | "on" => quote! { true },
                "false" | "0" | "no" | "off" => quote! { false },
                _ => quote! { true }, // Default to enabled
            }
        }
        AttributeValue::Binding(binding_expr) => {
            // Dynamic binding expression - use generate_bool_expr for native boolean
            super::bindings::generate_bool_expr(&binding_expr.expr)
        }
        AttributeValue::Interpolated(_) => {
            // Interpolated strings treated as enabled if non-empty
            let expr_tokens = generate_attribute_value(attr, model_ident);
            quote! { !#expr_tokens.is_empty() && #expr_tokens != "false" && #expr_tokens != "0" }
        }
    });

    if let Some(event) = on_click {
        let variant_name = to_upper_camel_case(&event.handler);
        let handler_ident = format_ident!("{}", variant_name);

        let param_expr = if let Some(ref param) = event.param {
            let param_tokens = generate_expr(&param.expr);
            quote! { (#param_tokens) }
        } else {
            quote! {}
        };

        // Generate on_press call based on enabled condition
        button = match enabled_condition {
            None => {
                // No enabled attribute - always enabled
                quote! {
                    #button.on_press(#message_ident::#handler_ident #param_expr)
                }
            }
            Some(condition) => {
                // Conditional enabled - use on_press_maybe
                quote! {
                    #button.on_press_maybe(
                        if #condition {
                            Some(#message_ident::#handler_ident #param_expr)
                        } else {
                            None
                        }
                    )
                }
            }
        };
    }

    // Apply styles (inline or classes)
    button = apply_widget_style(button, node, "button", style_classes)?;

    Ok(quote! { #button.into() })
}

/// Helper function to convert snake_case to UpperCamelCase
fn to_upper_camel_case(s: &str) -> String {
    let mut result = String::new();
    let mut capitalize_next = true;
    for c in s.chars() {
        if c == '_' {
            capitalize_next = true;
        } else if capitalize_next {
            result.push(c.to_ascii_uppercase());
            capitalize_next = false;
        } else {
            result.push(c);
        }
    }
    result
}

/// Generate container widget (column, row, container, scrollable)
fn generate_container(
    node: &crate::WidgetNode,
    widget_type: &str,
    model_ident: &syn::Ident,
    message_ident: &syn::Ident,
    style_classes: &HashMap<String, StyleClass>,
) -> Result<TokenStream, super::CodegenError> {
    let children: Vec<TokenStream> = node
        .children
        .iter()
        .map(|child| generate_widget(child, model_ident, message_ident, style_classes))
        .collect::<Result<_, _>>()?;

    let widget_ident = format_ident!("{}", widget_type);

    // Get merged layout from node.layout and style classes
    let merged_layout = get_merged_layout(node, style_classes);

    // Get spacing from attributes or merged layout
    let spacing = node
        .attributes
        .get("spacing")
        .and_then(|attr| {
            if let AttributeValue::Static(s) = attr {
                s.parse::<f32>().ok()
            } else {
                None
            }
        })
        .or_else(|| merged_layout.as_ref().and_then(|l| l.spacing()));

    // Get padding from attributes or merged layout
    let padding = node
        .attributes
        .get("padding")
        .and_then(|attr| {
            if let AttributeValue::Static(s) = attr {
                s.parse::<f32>().ok()
            } else {
                None
            }
        })
        .or_else(|| merged_layout.as_ref().and_then(|l| l.padding()));

    let mut widget = if widget_type == "container" {
        // Container in Iced can only have one child
        // If multiple children provided, wrap them in a column automatically
        // Use let binding with explicit type to help inference when child is a column/row
        if children.is_empty() {
            quote! {
                iced::widget::container(iced::widget::Space::new())
            }
        } else if children.len() == 1 {
            let child = &children[0];
            quote! {
                {
                    let content: iced::Element<'_, _, _> = #child;
                    iced::widget::container(content)
                }
            }
        } else {
            // Multiple children - wrap in a column
            quote! {
                {
                    let content: iced::Element<'_, _, _> = iced::widget::column(vec![#(#children),*]).into();
                    iced::widget::container(content)
                }
            }
        }
    } else if widget_type == "scrollable" {
        // Scrollable in Iced can only have one child
        // If multiple children provided, wrap them in a column automatically
        // Use let binding with explicit type to help inference when child is a column/row
        if children.is_empty() {
            quote! {
                iced::widget::scrollable(iced::widget::Space::new())
            }
        } else if children.len() == 1 {
            let child = &children[0];
            quote! {
                {
                    let content: iced::Element<'_, _, _> = #child;
                    iced::widget::scrollable(content)
                }
            }
        } else {
            // Multiple children - wrap in a column
            quote! {
                {
                    let content: iced::Element<'_, _, _> = iced::widget::column(vec![#(#children),*]).into();
                    iced::widget::scrollable(content)
                }
            }
        }
    } else {
        quote! {
            iced::widget::#widget_ident(vec![#(#children),*])
        }
    };

    if let Some(s) = spacing {
        widget = quote! { #widget.spacing(#s) };
    }

    if let Some(p) = padding {
        widget = quote! { #widget.padding(#p) };
    }

    // Apply width from attributes or merged layout
    let width_from_attr = node.attributes.get("width").and_then(|attr| {
        if let AttributeValue::Static(s) = attr {
            Some(s.clone())
        } else {
            None
        }
    });
    let width_from_layout = merged_layout.as_ref().and_then(|l| l.width());

    if let Some(width) = width_from_attr {
        let width_expr = generate_length_expr(&width);
        widget = quote! { #widget.width(#width_expr) };
    } else if let Some(layout_width) = width_from_layout {
        let width_expr = generate_layout_length_expr(layout_width);
        widget = quote! { #widget.width(#width_expr) };
    }

    // Apply height from attributes or merged layout
    let height_from_attr = node.attributes.get("height").and_then(|attr| {
        if let AttributeValue::Static(s) = attr {
            Some(s.clone())
        } else {
            None
        }
    });
    let height_from_layout = merged_layout.as_ref().and_then(|l| l.height());

    if let Some(height) = height_from_attr {
        let height_expr = generate_length_expr(&height);
        widget = quote! { #widget.height(#height_expr) };
    } else if let Some(layout_height) = height_from_layout {
        let height_expr = generate_layout_length_expr(layout_height);
        widget = quote! { #widget.height(#height_expr) };
    }

    // Apply align_x attribute (for containers)
    if widget_type == "container" {
        if let Some(align_x) = node.attributes.get("align_x").and_then(|attr| {
            if let AttributeValue::Static(s) = attr {
                Some(s.clone())
            } else {
                None
            }
        }) {
            let align_expr = generate_horizontal_alignment_expr(&align_x);
            widget = quote! { #widget.align_x(#align_expr) };
        }

        // Apply align_y attribute
        if let Some(align_y) = node.attributes.get("align_y").and_then(|attr| {
            if let AttributeValue::Static(s) = attr {
                Some(s.clone())
            } else {
                None
            }
        }) {
            let align_expr = generate_vertical_alignment_expr(&align_y);
            widget = quote! { #widget.align_y(#align_expr) };
        }
    }

    // Apply align_items for column/row (vertical alignment of children)
    if (widget_type == "column" || widget_type == "row")
        && let Some(align) = node.attributes.get("align_items").and_then(|attr| {
            if let AttributeValue::Static(s) = attr {
                Some(s.clone())
            } else {
                None
            }
        })
    {
        let align_expr = match align.to_lowercase().as_str() {
            "center" => quote! { iced::Alignment::Center },
            "end" => quote! { iced::Alignment::End },
            _ => quote! { iced::Alignment::Start },
        };
        widget = quote! { #widget.align_items(#align_expr) };
    }

    // Apply styles (only for container, not column/row/scrollable)
    if widget_type == "container" {
        widget = apply_widget_style(widget, node, "container", style_classes)?;
    }

    // Check if Column/Row needs to be wrapped in a container for align_x/align_y
    // These attributes position the Column/Row itself within its parent,
    // which requires an outer container wrapper
    if (widget_type == "column" || widget_type == "row")
        && (node.attributes.contains_key("align_x") || node.attributes.contains_key("align_y"))
    {
        let mut container = quote! { iced::widget::container(#widget) };

        if let Some(align_x) = node.attributes.get("align_x").and_then(|attr| {
            if let AttributeValue::Static(s) = attr {
                Some(s.clone())
            } else {
                None
            }
        }) {
            let align_expr = generate_horizontal_alignment_expr(&align_x);
            container = quote! { #container.align_x(#align_expr) };
        }

        if let Some(align_y) = node.attributes.get("align_y").and_then(|attr| {
            if let AttributeValue::Static(s) = attr {
                Some(s.clone())
            } else {
                None
            }
        }) {
            let align_expr = generate_vertical_alignment_expr(&align_y);
            container = quote! { #container.align_y(#align_expr) };
        }

        // Container needs explicit width/height to enable alignment
        container = quote! { #container.width(iced::Length::Fill).height(iced::Length::Fill) };

        return Ok(quote! { #container.into() });
    }

    Ok(quote! { #widget.into() })
}

/// Generate stack widget
fn generate_stack(
    node: &crate::WidgetNode,
    model_ident: &syn::Ident,
    message_ident: &syn::Ident,
    style_classes: &HashMap<String, StyleClass>,
) -> Result<TokenStream, super::CodegenError> {
    let children: Vec<TokenStream> = node
        .children
        .iter()
        .map(|child| generate_widget(child, model_ident, message_ident, style_classes))
        .collect::<Result<_, _>>()?;

    Ok(quote! {
        iced::widget::stack(vec![#(#children),*]).into()
    })
}

/// Generate space widget
fn generate_space(node: &crate::WidgetNode) -> Result<TokenStream, super::CodegenError> {
    // Get width attribute
    let width = node.attributes.get("width").and_then(|attr| {
        if let AttributeValue::Static(s) = attr {
            Some(s.clone())
        } else {
            None
        }
    });

    // Get height attribute
    let height = node.attributes.get("height").and_then(|attr| {
        if let AttributeValue::Static(s) = attr {
            Some(s.clone())
        } else {
            None
        }
    });

    let mut space = quote! { iced::widget::Space::new() };

    // Apply width
    if let Some(w) = width {
        let width_expr = generate_length_expr(&w);
        space = quote! { #space.width(#width_expr) };
    }

    // Apply height
    if let Some(h) = height {
        let height_expr = generate_length_expr(&h);
        space = quote! { #space.height(#height_expr) };
    }

    Ok(quote! { #space.into() })
}

/// Generate rule (horizontal/vertical line) widget
fn generate_rule(node: &crate::WidgetNode) -> Result<TokenStream, super::CodegenError> {
    // Get direction (default to horizontal)
    let direction = node
        .attributes
        .get("direction")
        .and_then(|attr| {
            if let AttributeValue::Static(s) = attr {
                Some(s.clone())
            } else {
                None
            }
        })
        .unwrap_or_else(|| "horizontal".to_string());

    // Get thickness (default to 1)
    let thickness = node
        .attributes
        .get("thickness")
        .and_then(|attr| {
            if let AttributeValue::Static(s) = attr {
                s.parse::<f32>().ok()
            } else {
                None
            }
        })
        .unwrap_or(1.0);

    let rule = if direction.to_lowercase() == "vertical" {
        quote! { iced::widget::rule::vertical(#thickness) }
    } else {
        quote! { iced::widget::rule::horizontal(#thickness) }
    };

    Ok(quote! { #rule.into() })
}

/// Generate checkbox widget
fn generate_checkbox(
    node: &crate::WidgetNode,
    model_ident: &syn::Ident,
    message_ident: &syn::Ident,
    style_classes: &HashMap<String, StyleClass>,
) -> Result<TokenStream, super::CodegenError> {
    let label = node
        .attributes
        .get("label")
        .and_then(|attr| {
            if let AttributeValue::Static(s) = attr {
                Some(s.clone())
            } else {
                None
            }
        })
        .unwrap_or_default();
    let label_lit = proc_macro2::Literal::string(&label);
    let label_expr = quote! { #label_lit.to_string() };

    let checked_attr = node.attributes.get("checked");
    let checked_expr = checked_attr
        .map(|attr| generate_attribute_value(attr, model_ident))
        .unwrap_or(quote! { false });

    let on_toggle = node
        .events
        .iter()
        .find(|e| e.event == crate::EventKind::Toggle);

    let checkbox = if let Some(event) = on_toggle {
        let variant_name = to_upper_camel_case(&event.handler);
        let handler_ident = format_ident!("{}", variant_name);
        quote! {
            iced::widget::checkbox(#label_expr, #checked_expr)
                .on_toggle(#message_ident::#handler_ident)
        }
    } else {
        quote! {
            iced::widget::checkbox(#label_expr, #checked_expr)
        }
    };

    // Apply styles
    let checkbox = apply_widget_style(checkbox, node, "checkbox", style_classes)?;

    Ok(quote! { #checkbox.into() })
}

/// Generate toggler widget
fn generate_toggler(
    node: &crate::WidgetNode,
    model_ident: &syn::Ident,
    message_ident: &syn::Ident,
    style_classes: &HashMap<String, StyleClass>,
) -> Result<TokenStream, super::CodegenError> {
    let label = node
        .attributes
        .get("label")
        .and_then(|attr| {
            if let AttributeValue::Static(s) = attr {
                Some(s.clone())
            } else {
                None
            }
        })
        .unwrap_or_default();
    let label_lit = proc_macro2::Literal::string(&label);
    let label_expr = quote! { #label_lit.to_string() };

    let is_toggled_attr = node.attributes.get("toggled");
    let is_toggled_expr = is_toggled_attr
        .map(|attr| generate_attribute_value(attr, model_ident))
        .unwrap_or(quote! { false });

    let on_toggle = node
        .events
        .iter()
        .find(|e| e.event == crate::EventKind::Toggle);

    let toggler = if let Some(event) = on_toggle {
        let variant_name = to_upper_camel_case(&event.handler);
        let handler_ident = format_ident!("{}", variant_name);
        quote! {
            iced::widget::toggler(#label_expr, #is_toggled_expr, None)
                .on_toggle(|_| #message_ident::#handler_ident)
        }
    } else {
        quote! {
            iced::widget::toggler(#label_expr, #is_toggled_expr, None)
        }
    };

    // Apply styles
    let toggler = apply_widget_style(toggler, node, "toggler", style_classes)?;

    Ok(quote! { #toggler.into() })
}

/// Generate slider widget
fn generate_slider(
    node: &crate::WidgetNode,
    model_ident: &syn::Ident,
    message_ident: &syn::Ident,
    style_classes: &HashMap<String, StyleClass>,
) -> Result<TokenStream, super::CodegenError> {
    let min = node.attributes.get("min").and_then(|attr| {
        if let AttributeValue::Static(s) = attr {
            s.parse::<f32>().ok()
        } else {
            None
        }
    });

    let max = node.attributes.get("max").and_then(|attr| {
        if let AttributeValue::Static(s) = attr {
            s.parse::<f32>().ok()
        } else {
            None
        }
    });

    let value_attr = node.attributes.get("value").ok_or_else(|| {
        super::CodegenError::InvalidWidget("slider requires value attribute".to_string())
    })?;
    let value_expr = generate_attribute_value(value_attr, model_ident);

    let on_change = node
        .events
        .iter()
        .find(|e| e.event == crate::EventKind::Change);

    let mut slider = quote! {
        iced::widget::slider(0.0..=100.0, #value_expr, |v| {})
    };

    if let Some(m) = min {
        slider = quote! { #slider.min(#m) };
    }
    if let Some(m) = max {
        slider = quote! { #slider.max(#m) };
    }

    // Apply step attribute (increment size)
    let step = node.attributes.get("step").and_then(|attr| {
        if let AttributeValue::Static(s) = attr {
            s.parse::<f32>().ok()
        } else {
            None
        }
    });

    if let Some(s) = step {
        slider = quote! { #slider.step(#s) };
    }

    if let Some(event) = on_change {
        let variant_name = to_upper_camel_case(&event.handler);
        let handler_ident = format_ident!("{}", variant_name);
        slider = quote! {
            iced::widget::slider(0.0..=100.0, #value_expr, |v| #message_ident::#handler_ident(v))
        };
    }

    // Apply styles
    slider = apply_widget_style(slider, node, "slider", style_classes)?;

    Ok(quote! { #slider.into() })
}

/// Generate radio widget
fn generate_radio(
    node: &crate::WidgetNode,
    _model_ident: &syn::Ident,
    message_ident: &syn::Ident,
    _style_classes: &HashMap<String, StyleClass>,
) -> Result<TokenStream, super::CodegenError> {
    let label = node
        .attributes
        .get("label")
        .and_then(|attr| {
            if let AttributeValue::Static(s) = attr {
                Some(s.clone())
            } else {
                None
            }
        })
        .unwrap_or_default();
    let label_lit = proc_macro2::Literal::string(&label);
    let label_expr = quote! { #label_lit.to_string() };

    let value_attr = node.attributes.get("value").ok_or_else(|| {
        super::CodegenError::InvalidWidget("radio requires value attribute".to_string())
    })?;
    let value_expr = match value_attr {
        AttributeValue::Binding(expr) => generate_expr(&expr.expr),
        _ => quote! { String::new() },
    };

    let selected_attr = node.attributes.get("selected");
    let selected_expr = match selected_attr {
        Some(AttributeValue::Binding(expr)) => generate_expr(&expr.expr),
        _ => quote! { None },
    };

    let on_select = node
        .events
        .iter()
        .find(|e| e.event == crate::EventKind::Select);

    if let Some(event) = on_select {
        let variant_name = to_upper_camel_case(&event.handler);
        let handler_ident = format_ident!("{}", variant_name);
        Ok(quote! {
            iced::widget::radio(#label_expr, #value_expr, #selected_expr, |v| #message_ident::#handler_ident(v)).into()
        })
    } else {
        Ok(quote! {
            iced::widget::radio(#label_expr, #value_expr, #selected_expr, |_| ()).into()
        })
    }
}

/// Generate progress bar widget
fn generate_progress_bar(
    node: &crate::WidgetNode,
    model_ident: &syn::Ident,
    _style_classes: &HashMap<String, StyleClass>,
) -> Result<TokenStream, super::CodegenError> {
    let value_attr = node.attributes.get("value").ok_or_else(|| {
        super::CodegenError::InvalidWidget("progress_bar requires value attribute".to_string())
    })?;
    let value_expr = generate_attribute_value(value_attr, model_ident);

    let max_attr = node.attributes.get("max").and_then(|attr| {
        if let AttributeValue::Static(s) = attr {
            s.parse::<f32>().ok()
        } else {
            None
        }
    });

    // Parse style attribute (default to "primary")
    let style_str = node
        .attributes
        .get("style")
        .and_then(|attr| {
            if let AttributeValue::Static(s) = attr {
                Some(s.clone())
            } else {
                None
            }
        })
        .unwrap_or_else(|| "primary".to_string());

    // Parse custom colors
    let bar_color = node.attributes.get("bar_color").and_then(|attr| {
        if let AttributeValue::Static(s) = attr {
            parse_color_to_tokens(s)
        } else {
            None
        }
    });

    let background_color = node.attributes.get("background_color").and_then(|attr| {
        if let AttributeValue::Static(s) = attr {
            parse_color_to_tokens(s)
        } else {
            None
        }
    });

    // Parse border radius
    let border_radius = node.attributes.get("border_radius").and_then(|attr| {
        if let AttributeValue::Static(s) = attr {
            s.parse::<f32>().ok()
        } else {
            None
        }
    });

    // Parse height (girth)
    let height = node.attributes.get("height").and_then(|attr| {
        if let AttributeValue::Static(s) = attr {
            s.parse::<f32>().ok()
        } else {
            None
        }
    });

    // Generate style closure based on style attribute
    let bar_color_expr = if let Some(color_tokens) = bar_color {
        quote! { #color_tokens }
    } else {
        match style_str.as_str() {
            "success" => quote! { palette.success.base.color },
            "warning" => quote! { palette.warning.base.color },
            "danger" => quote! { palette.danger.base.color },
            "secondary" => quote! { palette.secondary.base.color },
            _ => quote! { palette.primary.base.color }, // default to primary
        }
    };

    // Generate background color expression
    let background_color_expr = if let Some(color_tokens) = background_color {
        quote! { #color_tokens }
    } else {
        quote! { palette.background.weak.color }
    };

    // Generate border expression
    let border_expr = if let Some(radius) = border_radius {
        quote! { iced::Border::default().rounded(#radius) }
    } else {
        quote! { iced::Border::default() }
    };

    // Generate height/girth expression
    let girth_expr = if let Some(h) = height {
        quote! { .girth(#h) }
    } else {
        quote! {}
    };

    if let Some(max) = max_attr {
        Ok(quote! {
            iced::widget::progress_bar(0.0..=#max, #value_expr)
                #girth_expr
                .style(|theme: &iced::Theme| {
                    let palette = theme.extended_palette();
                    iced::widget::progress_bar::Style {
                        background: iced::Background::Color(#background_color_expr),
                        bar: iced::Background::Color(#bar_color_expr),
                        border: #border_expr,
                    }
                })
                .into()
        })
    } else {
        Ok(quote! {
            iced::widget::progress_bar(0.0..=100.0, #value_expr)
                #girth_expr
                .style(|theme: &iced::Theme| {
                    let palette = theme.extended_palette();
                    iced::widget::progress_bar::Style {
                        background: iced::Background::Color(#background_color_expr),
                        bar: iced::Background::Color(#bar_color_expr),
                        border: #border_expr,
                    }
                })
                .into()
        })
    }
}

/// Parse a color string into TokenStream for code generation
fn parse_color_to_tokens(color_str: &str) -> Option<TokenStream> {
    // Try hex color (#RRGGBB or #RRGGBBAA)
    if color_str.starts_with('#') {
        let hex = &color_str[1..];
        if hex.len() == 6 {
            if let (Ok(r), Ok(g), Ok(b)) = (
                u8::from_str_radix(&hex[0..2], 16),
                u8::from_str_radix(&hex[2..4], 16),
                u8::from_str_radix(&hex[4..6], 16),
            ) {
                let rf = r as f32 / 255.0;
                let gf = g as f32 / 255.0;
                let bf = b as f32 / 255.0;
                return Some(quote! { iced::Color::from_rgb(#rf, #gf, #bf) });
            }
        } else if hex.len() == 8 {
            if let (Ok(r), Ok(g), Ok(b), Ok(a)) = (
                u8::from_str_radix(&hex[0..2], 16),
                u8::from_str_radix(&hex[2..4], 16),
                u8::from_str_radix(&hex[4..6], 16),
                u8::from_str_radix(&hex[6..8], 16),
            ) {
                let rf = r as f32 / 255.0;
                let gf = g as f32 / 255.0;
                let bf = b as f32 / 255.0;
                let af = a as f32 / 255.0;
                return Some(quote! { iced::Color::from_rgba(#rf, #gf, #bf, #af) });
            }
        }
    }

    // Try RGB format: rgb(r,g,b)
    if color_str.starts_with("rgb(") && color_str.ends_with(')') {
        let inner = &color_str[4..color_str.len() - 1];
        let parts: Vec<&str> = inner.split(',').map(|s| s.trim()).collect();
        if parts.len() == 3 {
            if let (Ok(r), Ok(g), Ok(b)) = (
                parts[0].parse::<u8>(),
                parts[1].parse::<u8>(),
                parts[2].parse::<u8>(),
            ) {
                let rf = r as f32 / 255.0;
                let gf = g as f32 / 255.0;
                let bf = b as f32 / 255.0;
                return Some(quote! { iced::Color::from_rgb(#rf, #gf, #bf) });
            }
        }
    }

    // Try RGBA format: rgba(r,g,b,a)
    if color_str.starts_with("rgba(") && color_str.ends_with(')') {
        let inner = &color_str[5..color_str.len() - 1];
        let parts: Vec<&str> = inner.split(',').map(|s| s.trim()).collect();
        if parts.len() == 4 {
            if let (Ok(r), Ok(g), Ok(b), Ok(a)) = (
                parts[0].parse::<u8>(),
                parts[1].parse::<u8>(),
                parts[2].parse::<u8>(),
                parts[3].parse::<f32>(),
            ) {
                let rf = r as f32 / 255.0;
                let gf = g as f32 / 255.0;
                let bf = b as f32 / 255.0;
                return Some(quote! { iced::Color::from_rgba(#rf, #gf, #bf, #a) });
            }
        }
    }

    None
}

/// Generate text input widget
fn generate_text_input(
    node: &crate::WidgetNode,
    model_ident: &syn::Ident,
    message_ident: &syn::Ident,
    style_classes: &HashMap<String, StyleClass>,
) -> Result<TokenStream, super::CodegenError> {
    let value_expr = node
        .attributes
        .get("value")
        .map(|attr| generate_attribute_value(attr, model_ident))
        .unwrap_or(quote! { String::new() });

    let placeholder = node.attributes.get("placeholder").and_then(|attr| {
        if let AttributeValue::Static(s) = attr {
            Some(s.clone())
        } else {
            None
        }
    });

    let on_input = node
        .events
        .iter()
        .find(|e| e.event == crate::EventKind::Input);

    let on_submit = node
        .events
        .iter()
        .find(|e| e.event == crate::EventKind::Submit);

    let mut text_input = match placeholder {
        Some(ph) => {
            let ph_lit = proc_macro2::Literal::string(&ph);
            quote! {
                iced::widget::text_input(#ph_lit, &#value_expr)
            }
        }
        None => quote! {
            iced::widget::text_input("", &#value_expr)
        },
    };

    if let Some(event) = on_input {
        let variant_name = to_upper_camel_case(&event.handler);
        let handler_ident = format_ident!("{}", variant_name);
        text_input = quote! {
            #text_input.on_input(|v| #message_ident::#handler_ident(v))
        };
    }

    if let Some(event) = on_submit {
        let variant_name = to_upper_camel_case(&event.handler);
        let handler_ident = format_ident!("{}", variant_name);
        text_input = quote! {
            #text_input.on_submit(#message_ident::#handler_ident)
        };
    }

    // Apply password/secure attribute (masks input)
    let is_password = node
        .attributes
        .get("password")
        .or_else(|| node.attributes.get("secure"))
        .and_then(|attr| {
            if let AttributeValue::Static(s) = attr {
                Some(s.to_lowercase() == "true" || s == "1")
            } else {
                None
            }
        })
        .unwrap_or(false);

    if is_password {
        text_input = quote! { #text_input.password() };
    }

    // Apply styles
    text_input = apply_widget_style(text_input, node, "text_input", style_classes)?;

    Ok(quote! { #text_input.into() })
}

/// Generate image widget
fn generate_image(node: &crate::WidgetNode) -> Result<TokenStream, super::CodegenError> {
    let src_attr = node.attributes.get("src").ok_or_else(|| {
        super::CodegenError::InvalidWidget("image requires src attribute".to_string())
    })?;

    let src = match src_attr {
        AttributeValue::Static(s) => s.clone(),
        _ => String::new(),
    };
    let src_lit = proc_macro2::Literal::string(&src);

    let width = node.attributes.get("width").and_then(|attr| {
        if let AttributeValue::Static(s) = attr {
            s.parse::<u32>().ok()
        } else {
            None
        }
    });

    let height = node.attributes.get("height").and_then(|attr| {
        if let AttributeValue::Static(s) = attr {
            s.parse::<u32>().ok()
        } else {
            None
        }
    });

    let mut image = quote! {
        iced::widget::image::Image::new(iced::widget::image::Handle::from_memory(std::fs::read(#src_lit).unwrap_or_default()))
    };

    // Apply native width/height if specified with integer values
    if let (Some(w), Some(h)) = (width, height) {
        image = quote! { #image.width(#w).height(#h) };
    } else if let Some(w) = width {
        image = quote! { #image.width(#w) };
    } else if let Some(h) = height {
        image = quote! { #image.height(#h) };
    }

    // Check if we need container for NON-native layout attributes
    // (padding, alignment, classes - NOT width/height since those are native)
    // For Image, only wrap if there are alignment/padding/classes
    let needs_container = !node.classes.is_empty()
        || node.attributes.contains_key("align_x")
        || node.attributes.contains_key("align_y")
        || node.attributes.contains_key("padding");

    if needs_container {
        // Wrap with container for layout attributes, but skip width/height (already applied)
        let mut container = quote! { iced::widget::container(#image) };

        if let Some(padding) = node.attributes.get("padding").and_then(|attr| {
            if let AttributeValue::Static(s) = attr {
                s.parse::<f32>().ok()
            } else {
                None
            }
        }) {
            container = quote! { #container.padding(#padding) };
        }

        if let Some(align_x) = node.attributes.get("align_x").and_then(|attr| {
            if let AttributeValue::Static(s) = attr {
                Some(s.clone())
            } else {
                None
            }
        }) {
            let align_expr = generate_horizontal_alignment_expr(&align_x);
            container = quote! { #container.align_x(#align_expr) };
        }

        if let Some(align_y) = node.attributes.get("align_y").and_then(|attr| {
            if let AttributeValue::Static(s) = attr {
                Some(s.clone())
            } else {
                None
            }
        }) {
            let align_expr = generate_vertical_alignment_expr(&align_y);
            container = quote! { #container.align_y(#align_expr) };
        }

        if let Some(class_name) = node.classes.first() {
            let style_fn_ident = format_ident!("style_{}", class_name.replace('-', "_"));
            container = quote! { #container.style(#style_fn_ident) };
        }

        Ok(quote! { #container.into() })
    } else {
        Ok(quote! { #image.into() })
    }
}

/// Generate SVG widget
fn generate_svg(node: &crate::WidgetNode) -> Result<TokenStream, super::CodegenError> {
    // Support both "src" (standard) and "path" (legacy) for backward compatibility
    let path_attr = node
        .attributes
        .get("src")
        .or_else(|| node.attributes.get("path"))
        .ok_or_else(|| {
            super::CodegenError::InvalidWidget("svg requires src attribute".to_string())
        })?;

    let path = match path_attr {
        AttributeValue::Static(s) => s.clone(),
        _ => String::new(),
    };
    let path_lit = proc_macro2::Literal::string(&path);

    let width = node.attributes.get("width").and_then(|attr| {
        if let AttributeValue::Static(s) = attr {
            s.parse::<u32>().ok()
        } else {
            None
        }
    });

    let height = node.attributes.get("height").and_then(|attr| {
        if let AttributeValue::Static(s) = attr {
            s.parse::<u32>().ok()
        } else {
            None
        }
    });

    let mut svg = quote! {
        iced::widget::svg::Svg::new(iced::widget::svg::Handle::from_path(#path_lit))
    };

    // Apply native width/height if specified with integer values
    if let (Some(w), Some(h)) = (width, height) {
        svg = quote! { #svg.width(#w).height(#h) };
    } else if let Some(w) = width {
        svg = quote! { #svg.width(#w) };
    } else if let Some(h) = height {
        svg = quote! { #svg.height(#h) };
    }

    // Check if we need container for NON-native layout attributes
    // (padding, alignment, classes - NOT width/height since those are native)
    // For SVG, only wrap if there are alignment/padding/classes
    let needs_container = !node.classes.is_empty()
        || node.attributes.contains_key("align_x")
        || node.attributes.contains_key("align_y")
        || node.attributes.contains_key("padding");

    if needs_container {
        // Wrap with container for layout attributes, but skip width/height (already applied)
        let mut container = quote! { iced::widget::container(#svg) };

        if let Some(padding) = node.attributes.get("padding").and_then(|attr| {
            if let AttributeValue::Static(s) = attr {
                s.parse::<f32>().ok()
            } else {
                None
            }
        }) {
            container = quote! { #container.padding(#padding) };
        }

        if let Some(align_x) = node.attributes.get("align_x").and_then(|attr| {
            if let AttributeValue::Static(s) = attr {
                Some(s.clone())
            } else {
                None
            }
        }) {
            let align_expr = generate_horizontal_alignment_expr(&align_x);
            container = quote! { #container.align_x(#align_expr) };
        }

        if let Some(align_y) = node.attributes.get("align_y").and_then(|attr| {
            if let AttributeValue::Static(s) = attr {
                Some(s.clone())
            } else {
                None
            }
        }) {
            let align_expr = generate_vertical_alignment_expr(&align_y);
            container = quote! { #container.align_y(#align_expr) };
        }

        if let Some(class_name) = node.classes.first() {
            let style_fn_ident = format_ident!("style_{}", class_name.replace('-', "_"));
            container = quote! { #container.style(#style_fn_ident) };
        }

        Ok(quote! { #container.into() })
    } else {
        Ok(quote! { #svg.into() })
    }
}

/// Generate pick list widget
fn generate_pick_list(
    node: &crate::WidgetNode,
    model_ident: &syn::Ident,
    message_ident: &syn::Ident,
    _style_classes: &HashMap<String, StyleClass>,
) -> Result<TokenStream, super::CodegenError> {
    let options_attr = node.attributes.get("options").ok_or_else(|| {
        super::CodegenError::InvalidWidget("pick_list requires options attribute".to_string())
    })?;

    let options: Vec<String> = match options_attr {
        AttributeValue::Static(s) => s.split(',').map(|s| s.trim().to_string()).collect(),
        _ => Vec::new(),
    };
    let options_ref: Vec<&str> = options.iter().map(|s| s.as_str()).collect();

    let selected_attr = node.attributes.get("selected");
    let selected_expr = selected_attr
        .map(|attr| generate_attribute_value(attr, model_ident))
        .unwrap_or(quote! { None });

    let on_select = node
        .events
        .iter()
        .find(|e| e.event == crate::EventKind::Select);

    if let Some(event) = on_select {
        let variant_name = to_upper_camel_case(&event.handler);
        let handler_ident = format_ident!("{}", variant_name);
        Ok(quote! {
            iced::widget::pick_list(&[#(#options_ref),*], #selected_expr, |v| #message_ident::#handler_ident(v)).into()
        })
    } else {
        Ok(quote! {
            iced::widget::pick_list(&[#(#options_ref),*], #selected_expr, |_| ()).into()
        })
    }
}

/// Generate combo box widget
fn generate_combo_box(
    node: &crate::WidgetNode,
    model_ident: &syn::Ident,
    message_ident: &syn::Ident,
    _style_classes: &HashMap<String, StyleClass>,
) -> Result<TokenStream, super::CodegenError> {
    let options_attr = node.attributes.get("options").ok_or_else(|| {
        super::CodegenError::InvalidWidget("combobox requires options attribute".to_string())
    })?;

    let options: Vec<String> = match options_attr {
        AttributeValue::Static(s) => s.split(',').map(|s| s.trim().to_string()).collect(),
        _ => Vec::new(),
    };
    let options_ref: Vec<&str> = options.iter().map(|s| s.as_str()).collect();

    let selected_attr = node.attributes.get("selected");
    let selected_expr = selected_attr
        .map(|attr| generate_attribute_value(attr, model_ident))
        .unwrap_or(quote! { None });

    let on_select = node
        .events
        .iter()
        .find(|e| e.event == crate::EventKind::Select);

    if let Some(event) = on_select {
        let variant_name = to_upper_camel_case(&event.handler);
        let handler_ident = format_ident!("{}", variant_name);
        Ok(quote! {
            iced::widget::combo_box(&[#(#options_ref),*], "", #selected_expr, |v, _| #message_ident::#handler_ident(v)).into()
        })
    } else {
        Ok(quote! {
            iced::widget::combo_box(&[#(#options_ref),*], "", #selected_expr, |_, _| ()).into()
        })
    }
}

/// Generate tooltip widget
fn generate_tooltip(
    node: &crate::WidgetNode,
    model_ident: &syn::Ident,
    message_ident: &syn::Ident,
    style_classes: &HashMap<String, StyleClass>,
) -> Result<TokenStream, super::CodegenError> {
    let child = node.children.first().ok_or_else(|| {
        super::CodegenError::InvalidWidget("tooltip must have exactly one child".to_string())
    })?;
    let child_widget = generate_widget(child, model_ident, message_ident, style_classes)?;

    let message_attr = node.attributes.get("message").ok_or_else(|| {
        super::CodegenError::InvalidWidget("tooltip requires message attribute".to_string())
    })?;
    let message_expr = generate_attribute_value(message_attr, model_ident);

    Ok(quote! {
        iced::widget::tooltip(#child_widget, #message_expr, iced::widget::tooltip::Position::FollowCursor).into()
    })
}

/// Generate grid widget
fn generate_grid(
    node: &crate::WidgetNode,
    model_ident: &syn::Ident,
    message_ident: &syn::Ident,
    style_classes: &HashMap<String, StyleClass>,
) -> Result<TokenStream, super::CodegenError> {
    let children: Vec<TokenStream> = node
        .children
        .iter()
        .map(|child| generate_widget(child, model_ident, message_ident, style_classes))
        .collect::<Result<_, _>>()?;

    let columns = node
        .attributes
        .get("columns")
        .and_then(|attr| {
            if let AttributeValue::Static(s) = attr {
                s.parse::<u32>().ok()
            } else {
                None
            }
        })
        .unwrap_or(1);

    let spacing = node.attributes.get("spacing").and_then(|attr| {
        if let AttributeValue::Static(s) = attr {
            s.parse::<f32>().ok()
        } else {
            None
        }
    });

    let padding = node.attributes.get("padding").and_then(|attr| {
        if let AttributeValue::Static(s) = attr {
            s.parse::<f32>().ok()
        } else {
            None
        }
    });

    let grid = quote! {
        iced::widget::grid::Grid::new_with_children(vec![#(#children),*], #columns)
    };

    let grid = if let Some(s) = spacing {
        quote! { #grid.spacing(#s) }
    } else {
        grid
    };

    let grid = if let Some(p) = padding {
        quote! { #grid.padding(#p) }
    } else {
        grid
    };

    Ok(quote! { #grid.into() })
}

/// Generate canvas widget
fn generate_canvas(
    node: &crate::WidgetNode,
    model_ident: &syn::Ident,
    message_ident: &syn::Ident,
    _style_classes: &HashMap<String, StyleClass>,
) -> Result<TokenStream, super::CodegenError> {
    let width = node.attributes.get("width").and_then(|attr| {
        if let AttributeValue::Static(s) = attr {
            s.parse::<f32>().ok()
        } else {
            None
        }
    });

    let height = node.attributes.get("height").and_then(|attr| {
        if let AttributeValue::Static(s) = attr {
            s.parse::<f32>().ok()
        } else {
            None
        }
    });

    let width_expr = match width {
        Some(w) => quote! { iced::Length::Fixed(#w) },
        None => quote! { iced::Length::Fixed(400.0) },
    };

    let height_expr = match height {
        Some(h) => quote! { iced::Length::Fixed(#h) },
        None => quote! { iced::Length::Fixed(300.0) },
    };

    // Check for custom program binding
    let content_expr = if let Some(program_attr) = node.attributes.get("program") {
        let program_binding = match program_attr {
            AttributeValue::Binding(expr) => super::bindings::generate_bool_expr(&expr.expr),
            _ => quote! { None },
        };

        // Generate declarative canvas for the 'else' case
        let shape_exprs = generate_canvas_shapes(&node.children, model_ident)?;
        let handlers_expr = generate_canvas_handlers(node, model_ident, message_ident)?;
        let prog_init = quote! {
            dampen_iced::canvas::DeclarativeProgram::new(vec![#(#shape_exprs),*])
        };
        let prog_with_handlers = if let Some(handlers) = handlers_expr {
            quote! { #prog_init.with_handlers(#handlers) }
        } else {
            prog_init
        };

        quote! {
            if let Some(container) = &#program_binding {
                 let canvas = iced::widget::canvas(dampen_iced::canvas::CanvasProgramWrapper::new(
                     dampen_iced::canvas::CanvasContent::Custom(container.0.clone())
                 ))
                 .width(#width_expr)
                 .height(#height_expr);

                 iced::Element::from(canvas).map(|()| unreachable!("Custom program action not supported in codegen"))
            } else {
                 let canvas = iced::widget::canvas(dampen_iced::canvas::CanvasProgramWrapper::new(
                     dampen_iced::canvas::CanvasContent::Declarative(#prog_with_handlers)
                 ))
                 .width(#width_expr)
                 .height(#height_expr);

                 iced::Element::from(canvas)
            }
        }
    } else {
        // Generate declarative canvas
        let shape_exprs = generate_canvas_shapes(&node.children, model_ident)?;

        // Parse event handlers
        let handlers_expr = generate_canvas_handlers(node, model_ident, message_ident)?;

        let prog_init = quote! {
            dampen_iced::canvas::DeclarativeProgram::new(vec![#(#shape_exprs),*])
        };

        let prog_with_handlers = if let Some(handlers) = handlers_expr {
            quote! { #prog_init.with_handlers(#handlers) }
        } else {
            prog_init
        };

        quote! {
            iced::widget::canvas(dampen_iced::canvas::CanvasProgramWrapper::new(
                dampen_iced::canvas::CanvasContent::Declarative(#prog_with_handlers)
            ))
            .width(#width_expr)
            .height(#height_expr)
            .into()
        }
    };

    Ok(content_expr)
}

/// Generate float widget
fn generate_float(
    node: &crate::WidgetNode,
    model_ident: &syn::Ident,
    message_ident: &syn::Ident,
    style_classes: &HashMap<String, StyleClass>,
) -> Result<TokenStream, super::CodegenError> {
    let child = node.children.first().ok_or_else(|| {
        super::CodegenError::InvalidWidget("float must have exactly one child".to_string())
    })?;
    let child_widget = generate_widget(child, model_ident, message_ident, style_classes)?;

    let position = node
        .attributes
        .get("position")
        .and_then(|attr| {
            if let AttributeValue::Static(s) = attr {
                Some(s.clone())
            } else {
                None
            }
        })
        .unwrap_or_else(|| "TopRight".to_string());

    let offset_x = node.attributes.get("offset_x").and_then(|attr| {
        if let AttributeValue::Static(s) = attr {
            s.parse::<f32>().ok()
        } else {
            None
        }
    });

    let offset_y = node.attributes.get("offset_y").and_then(|attr| {
        if let AttributeValue::Static(s) = attr {
            s.parse::<f32>().ok()
        } else {
            None
        }
    });

    let float = match position.as_str() {
        "TopLeft" => quote! { iced::widget::float::float_top_left(#child_widget) },
        "TopRight" => quote! { iced::widget::float::float_top_right(#child_widget) },
        "BottomLeft" => quote! { iced::widget::float::float_bottom_left(#child_widget) },
        "BottomRight" => quote! { iced::widget::float::float_bottom_right(#child_widget) },
        _ => quote! { iced::widget::float::float_top_right(#child_widget) },
    };

    let float = if let (Some(ox), Some(oy)) = (offset_x, offset_y) {
        quote! { #float.offset_x(#ox).offset_y(#oy) }
    } else if let Some(ox) = offset_x {
        quote! { #float.offset_x(#ox) }
    } else if let Some(oy) = offset_y {
        quote! { #float.offset_y(#oy) }
    } else {
        float
    };

    Ok(quote! { #float.into() })
}

/// Generate for loop widget (iterates over collection)
///
/// Expects attributes:
/// - `each`: variable name for each item (e.g., "task")
/// - `in`: binding expression for the collection (e.g., "{filtered_tasks}")
fn generate_for(
    node: &crate::WidgetNode,
    model_ident: &syn::Ident,
    message_ident: &syn::Ident,
    style_classes: &HashMap<String, StyleClass>,
) -> Result<TokenStream, super::CodegenError> {
    // Get the 'in' attribute (collection to iterate)
    let in_attr = node.attributes.get("in").ok_or_else(|| {
        super::CodegenError::InvalidWidget("for requires 'in' attribute".to_string())
    })?;

    // Get the 'each' attribute (loop variable name)
    let var_name = node
        .attributes
        .get("each")
        .and_then(|attr| {
            if let AttributeValue::Static(s) = attr {
                Some(s.clone())
            } else {
                None
            }
        })
        .unwrap_or_else(|| "item".to_string());

    let var_ident = format_ident!("{}", var_name);

    // Generate the collection expression (raw, without .to_string())
    let collection_expr = generate_attribute_value_raw(in_attr, model_ident);

    // Generate children widgets
    let children: Vec<TokenStream> = node
        .children
        .iter()
        .map(|child| generate_widget(child, model_ident, message_ident, style_classes))
        .collect::<Result<_, _>>()?;

    // Generate the for loop that builds widgets
    Ok(quote! {
        {
            let items: Vec<_> = #collection_expr;
            let widgets: Vec<iced::Element<'_, #message_ident>> = items
                .iter()
                .enumerate()
                .flat_map(|(index, #var_ident)| {
                    let _ = index; // Suppress unused warning if not used
                    vec![#(#children),*]
                })
                .collect();
            iced::widget::column(widgets).into()
        }
    })
}

/// Generate if widget
fn generate_if(
    node: &crate::WidgetNode,
    model_ident: &syn::Ident,
    message_ident: &syn::Ident,
    style_classes: &HashMap<String, StyleClass>,
) -> Result<TokenStream, super::CodegenError> {
    let condition_attr = node.attributes.get("condition").ok_or_else(|| {
        super::CodegenError::InvalidWidget("if requires condition attribute".to_string())
    })?;

    let children: Vec<TokenStream> = node
        .children
        .iter()
        .map(|child| generate_widget(child, model_ident, message_ident, style_classes))
        .collect::<Result<_, _>>()?;

    let condition_expr = generate_attribute_value(condition_attr, model_ident);

    Ok(quote! {
        if #condition_expr.parse::<bool>().unwrap_or(false) {
            iced::widget::column(vec![#(#children),*]).into()
        } else {
            iced::widget::column(vec![]).into()
        }
    })
}

/// Generate custom widget
/// Generate DatePicker widget
fn generate_date_picker(
    node: &crate::WidgetNode,
    model_ident: &syn::Ident,
    message_ident: &syn::Ident,
    style_classes: &HashMap<String, StyleClass>,
) -> Result<TokenStream, super::CodegenError> {
    let show = node
        .attributes
        .get("show")
        .map(|attr| match attr {
            AttributeValue::Binding(b) => super::bindings::generate_bool_expr(&b.expr),
            AttributeValue::Static(s) => {
                let v = s == "true";
                quote! { #v }
            }
            _ => quote! { false },
        })
        .unwrap_or(quote! { false });

    let date = if let Some(attr) = node.attributes.get("value") {
        match attr {
            AttributeValue::Binding(b) => {
                let expr = super::bindings::generate_bool_expr(&b.expr);
                quote! { iced_aw::date_picker::Date::from(#expr) }
            }
            AttributeValue::Static(s) => {
                let format = node
                    .attributes
                    .get("format")
                    .map(|f| match f {
                        AttributeValue::Static(fs) => fs.as_str(),
                        _ => "%Y-%m-%d",
                    })
                    .unwrap_or("%Y-%m-%d");
                quote! {
                    iced_aw::date_picker::Date::from(
                        chrono::NaiveDate::parse_from_str(#s, #format).unwrap_or_default()
                    )
                }
            }
            _ => quote! { iced_aw::date_picker::Date::today() },
        }
    } else {
        quote! { iced_aw::date_picker::Date::today() }
    };

    let on_cancel = if let Some(h) = node
        .events
        .iter()
        .find(|e| e.event == crate::EventKind::Cancel)
    {
        let msg = format_ident!("{}", h.handler);
        quote! { #message_ident::#msg }
    } else {
        quote! { #message_ident::None }
    };

    let on_submit = if let Some(h) = node
        .events
        .iter()
        .find(|e| e.event == crate::EventKind::Submit)
    {
        let msg = format_ident!("{}", h.handler);
        quote! {
            |date| {
                let s = chrono::NaiveDate::from(date).format("%Y-%m-%d").to_string();
                #message_ident::#msg(s)
            }
        }
    } else {
        quote! { |_| #message_ident::None }
    };

    let underlay = if let Some(child) = node.children.first() {
        generate_widget(child, model_ident, message_ident, style_classes)?
    } else {
        quote! { iced::widget::text("Missing child") }
    };

    Ok(quote! {
        iced_aw::widgets::date_picker::DatePicker::new(
            #show,
            #date,
            #underlay,
            #on_cancel,
            #on_submit
        )
    })
}

/// Generate ColorPicker widget
fn generate_color_picker(
    node: &crate::WidgetNode,
    model_ident: &syn::Ident,
    message_ident: &syn::Ident,
    style_classes: &HashMap<String, StyleClass>,
) -> Result<TokenStream, super::CodegenError> {
    let show = node
        .attributes
        .get("show")
        .map(|attr| match attr {
            AttributeValue::Binding(b) => super::bindings::generate_bool_expr(&b.expr),
            AttributeValue::Static(s) => {
                let v = s == "true";
                quote! { #v }
            }
            _ => quote! { false },
        })
        .unwrap_or(quote! { false });

    let color = if let Some(attr) = node.attributes.get("value") {
        match attr {
            AttributeValue::Binding(b) => {
                let expr = super::bindings::generate_expr(&b.expr);
                quote! { iced::Color::from_hex(&#expr.to_string()).unwrap_or(iced::Color::BLACK) }
            }
            AttributeValue::Static(s) => {
                quote! { iced::Color::from_hex(#s).unwrap_or(iced::Color::BLACK) }
            }
            _ => quote! { iced::Color::BLACK },
        }
    } else {
        quote! { iced::Color::BLACK }
    };

    let on_cancel = if let Some(h) = node
        .events
        .iter()
        .find(|e| e.event == crate::EventKind::Cancel)
    {
        let msg = format_ident!("{}", h.handler);
        quote! { #message_ident::#msg }
    } else {
        quote! { #message_ident::None }
    };

    let on_submit = if let Some(h) = node
        .events
        .iter()
        .find(|e| e.event == crate::EventKind::Submit)
    {
        let msg = format_ident!("{}", h.handler);
        quote! {
            |color| {
                let s = iced::color!(color).to_string();
                #message_ident::#msg(s)
            }
        }
    } else {
        quote! { |_| #message_ident::None }
    };

    let underlay = if let Some(child) = node.children.first() {
        generate_widget(child, model_ident, message_ident, style_classes)?
    } else {
        quote! { iced::widget::text("Missing child") }
    };

    Ok(quote! {
        iced_aw::widgets::color_picker::ColorPicker::new(
            #show,
            #color,
            #underlay,
            #on_cancel,
            #on_submit
        )
    })
}

/// Generate TimePicker widget
fn generate_time_picker(
    node: &crate::WidgetNode,
    model_ident: &syn::Ident,
    message_ident: &syn::Ident,
    style_classes: &HashMap<String, StyleClass>,
) -> Result<TokenStream, super::CodegenError> {
    let show = node
        .attributes
        .get("show")
        .map(|attr| match attr {
            AttributeValue::Binding(b) => super::bindings::generate_bool_expr(&b.expr),
            AttributeValue::Static(s) => {
                let v = s == "true";
                quote! { #v }
            }
            _ => quote! { false },
        })
        .unwrap_or(quote! { false });

    let time = if let Some(attr) = node.attributes.get("value") {
        match attr {
            AttributeValue::Binding(b) => {
                let expr = super::bindings::generate_bool_expr(&b.expr);
                quote! { iced_aw::time_picker::Time::from(#expr) }
            }
            AttributeValue::Static(s) => {
                let format = node
                    .attributes
                    .get("format")
                    .map(|f| match f {
                        AttributeValue::Static(fs) => fs.as_str(),
                        _ => "%H:%M:%S",
                    })
                    .unwrap_or("%H:%M:%S");
                quote! {
                    iced_aw::time_picker::Time::from(
                        chrono::NaiveTime::parse_from_str(#s, #format).unwrap_or_default()
                    )
                }
            }
            _ => {
                quote! { iced_aw::time_picker::Time::from(chrono::Local::now().naive_local().time()) }
            }
        }
    } else {
        quote! { iced_aw::time_picker::Time::from(chrono::Local::now().naive_local().time()) }
    };

    let use_24h = node.attributes.get("use_24h").map(|attr| match attr {
        AttributeValue::Binding(b) => super::bindings::generate_bool_expr(&b.expr),
        AttributeValue::Static(s) => {
            let v = s == "true";
            quote! { #v }
        }
        _ => quote! { false },
    });

    let show_seconds = node.attributes.get("show_seconds").map(|attr| match attr {
        AttributeValue::Binding(b) => super::bindings::generate_bool_expr(&b.expr),
        AttributeValue::Static(s) => {
            let v = s == "true";
            quote! { #v }
        }
        _ => quote! { false },
    });

    let on_cancel = if let Some(h) = node
        .events
        .iter()
        .find(|e| e.event == crate::EventKind::Cancel)
    {
        let msg = format_ident!("{}", h.handler);
        quote! { #message_ident::#msg }
    } else {
        quote! { #message_ident::None }
    };

    let on_submit = if let Some(h) = node
        .events
        .iter()
        .find(|e| e.event == crate::EventKind::Submit)
    {
        let msg = format_ident!("{}", h.handler);
        quote! {
            |time| {
                let s = chrono::NaiveTime::from(time).format("%H:%M:%S").to_string();
                #message_ident::#msg(s)
            }
        }
    } else {
        quote! { |_| #message_ident::None }
    };

    let underlay = if let Some(child) = node.children.first() {
        generate_widget(child, model_ident, message_ident, style_classes)?
    } else {
        quote! { iced::widget::text("Missing child") }
    };

    let mut picker_setup = quote! {
        let mut picker = iced_aw::widgets::time_picker::TimePicker::new(
            #show,
            #time,
            #underlay,
            #on_cancel,
            #on_submit
        );
    };

    if let Some(use_24h_expr) = use_24h {
        picker_setup.extend(quote! {
            if #use_24h_expr {
                picker = picker.use_24h();
            }
        });
    }

    if let Some(show_seconds_expr) = show_seconds {
        picker_setup.extend(quote! {
            if #show_seconds_expr {
                picker = picker.show_seconds();
            }
        });
    }

    Ok(quote! {
        {
            #picker_setup
            picker
        }
    })
}

fn generate_custom_widget(
    node: &crate::WidgetNode,
    name: &str,
    model_ident: &syn::Ident,
    message_ident: &syn::Ident,
    style_classes: &HashMap<String, StyleClass>,
) -> Result<TokenStream, super::CodegenError> {
    let widget_ident = format_ident!("{}", name);
    let children: Vec<TokenStream> = node
        .children
        .iter()
        .map(|child| generate_widget(child, model_ident, message_ident, style_classes))
        .collect::<Result<_, _>>()?;

    Ok(quote! {
        #widget_ident(vec![#(#children),*]).into()
    })
}

/// Generate attribute value expression with inlined bindings
fn generate_attribute_value(attr: &AttributeValue, _model_ident: &syn::Ident) -> TokenStream {
    match attr {
        AttributeValue::Static(s) => {
            let lit = proc_macro2::Literal::string(s);
            quote! { #lit.to_string() }
        }
        AttributeValue::Binding(expr) => generate_expr(&expr.expr),
        AttributeValue::Interpolated(parts) => {
            let parts_str: Vec<String> = parts
                .iter()
                .map(|part| match part {
                    InterpolatedPart::Literal(s) => s.clone(),
                    InterpolatedPart::Binding(_) => "{}".to_string(),
                })
                .collect();
            let binding_exprs: Vec<TokenStream> = parts
                .iter()
                .filter_map(|part| {
                    if let InterpolatedPart::Binding(expr) = part {
                        Some(generate_expr(&expr.expr))
                    } else {
                        None
                    }
                })
                .collect();

            let format_string = parts_str.join("");
            let lit = proc_macro2::Literal::string(&format_string);

            quote! { format!(#lit, #(#binding_exprs),*) }
        }
    }
}

/// Generate attribute value without `.to_string()` conversion
/// Used for collections in for loops where we need the raw value
fn generate_attribute_value_raw(attr: &AttributeValue, _model_ident: &syn::Ident) -> TokenStream {
    match attr {
        AttributeValue::Static(s) => {
            let lit = proc_macro2::Literal::string(s);
            quote! { #lit }
        }
        AttributeValue::Binding(expr) => super::bindings::generate_bool_expr(&expr.expr),
        AttributeValue::Interpolated(parts) => {
            // For interpolated, we still need to generate a string
            let parts_str: Vec<String> = parts
                .iter()
                .map(|part| match part {
                    InterpolatedPart::Literal(s) => s.clone(),
                    InterpolatedPart::Binding(_) => "{}".to_string(),
                })
                .collect();
            let binding_exprs: Vec<TokenStream> = parts
                .iter()
                .filter_map(|part| {
                    if let InterpolatedPart::Binding(expr) = part {
                        Some(generate_expr(&expr.expr))
                    } else {
                        None
                    }
                })
                .collect();

            let format_string = parts_str.join("");
            let lit = proc_macro2::Literal::string(&format_string);

            quote! { format!(#lit, #(#binding_exprs),*) }
        }
    }
}

// ============================================================================
// Functions with local variable context support (for loops)
// ============================================================================

/// Generate text widget with local variable context
fn generate_text_with_locals(
    node: &crate::WidgetNode,
    model_ident: &syn::Ident,
    _style_classes: &HashMap<String, StyleClass>,
    local_vars: &std::collections::HashSet<String>,
) -> Result<TokenStream, super::CodegenError> {
    let value_attr = node.attributes.get("value").ok_or_else(|| {
        super::CodegenError::InvalidWidget("text requires value attribute".to_string())
    })?;

    let value_expr = generate_attribute_value_with_locals(value_attr, model_ident, local_vars);

    let mut text_widget = quote! {
        iced::widget::text(#value_expr)
    };

    // Apply size attribute
    if let Some(size) = node.attributes.get("size").and_then(|attr| {
        if let AttributeValue::Static(s) = attr {
            s.parse::<f32>().ok()
        } else {
            None
        }
    }) {
        text_widget = quote! { #text_widget.size(#size) };
    }

    // Apply weight attribute
    if let Some(weight) = node.attributes.get("weight").and_then(|attr| {
        if let AttributeValue::Static(s) = attr {
            Some(s.clone())
        } else {
            None
        }
    }) {
        let weight_expr = match weight.to_lowercase().as_str() {
            "bold" => quote! { iced::font::Weight::Bold },
            "semibold" => quote! { iced::font::Weight::Semibold },
            "medium" => quote! { iced::font::Weight::Medium },
            "light" => quote! { iced::font::Weight::Light },
            _ => quote! { iced::font::Weight::Normal },
        };
        text_widget = quote! {
            #text_widget.font(iced::Font { weight: #weight_expr, ..Default::default() })
        };
    }

    // Apply inline style color if present
    if let Some(ref style_props) = node.style
        && let Some(ref color) = style_props.color
    {
        let color_expr = generate_color_expr(color);
        text_widget = quote! { #text_widget.color(#color_expr) };
    }

    Ok(maybe_wrap_in_container(text_widget, node))
}

/// Generate button widget with local variable context
fn generate_button_with_locals(
    node: &crate::WidgetNode,
    model_ident: &syn::Ident,
    message_ident: &syn::Ident,
    style_classes: &HashMap<String, StyleClass>,
    local_vars: &std::collections::HashSet<String>,
) -> Result<TokenStream, super::CodegenError> {
    let label_attr = node.attributes.get("label").ok_or_else(|| {
        super::CodegenError::InvalidWidget("button requires label attribute".to_string())
    })?;

    let label_expr = generate_attribute_value_with_locals(label_attr, model_ident, local_vars);

    let on_click = node
        .events
        .iter()
        .find(|e| e.event == crate::EventKind::Click);

    let mut button = quote! {
        iced::widget::button(iced::widget::text(#label_expr))
    };

    if let Some(event) = on_click {
        let variant_name = to_upper_camel_case(&event.handler);
        let handler_ident = format_ident!("{}", variant_name);

        let param_expr = if let Some(ref param) = event.param {
            let param_tokens = super::bindings::generate_expr_with_locals(&param.expr, local_vars);
            quote! { (#param_tokens) }
        } else {
            quote! {}
        };

        button = quote! {
            #button.on_press(#message_ident::#handler_ident #param_expr)
        };
    }

    // Apply styles
    button = apply_widget_style(button, node, "button", style_classes)?;

    Ok(quote! { Into::<Element<'_, #message_ident>>::into(#button) })
}

/// Generate container widget with local variable context
fn generate_container_with_locals(
    node: &crate::WidgetNode,
    widget_type: &str,
    model_ident: &syn::Ident,
    message_ident: &syn::Ident,
    style_classes: &HashMap<String, StyleClass>,
    local_vars: &std::collections::HashSet<String>,
) -> Result<TokenStream, super::CodegenError> {
    let children: Vec<TokenStream> = node
        .children
        .iter()
        .map(|child| {
            generate_widget_with_locals(
                child,
                model_ident,
                message_ident,
                style_classes,
                local_vars,
            )
        })
        .collect::<Result<_, _>>()?;

    let mut container = match widget_type {
        "column" => {
            quote! { iced::widget::column({ let children: Vec<Element<'_, #message_ident>> = vec![#(#children),*]; children }) }
        }
        "row" => {
            quote! { iced::widget::row({ let children: Vec<Element<'_, #message_ident>> = vec![#(#children),*]; children }) }
        }
        "scrollable" => {
            quote! { iced::widget::scrollable(iced::widget::column({ let children: Vec<Element<'_, #message_ident>> = vec![#(#children),*]; children })) }
        }
        _ => {
            // container wraps a single child
            if children.len() == 1 {
                let child = &children[0];
                quote! { iced::widget::container(#child) }
            } else {
                quote! { iced::widget::container(iced::widget::column({ let children: Vec<Element<'_, #message_ident>> = vec![#(#children),*]; children })) }
            }
        }
    };

    // Get merged layout from node.layout and style classes
    let merged_layout = get_merged_layout(node, style_classes);

    // Get spacing from attributes or merged layout
    let spacing = node
        .attributes
        .get("spacing")
        .and_then(|attr| {
            if let AttributeValue::Static(s) = attr {
                s.parse::<f32>().ok()
            } else {
                None
            }
        })
        .or_else(|| merged_layout.as_ref().and_then(|l| l.spacing()));

    // Apply spacing for column/row
    if let Some(s) = spacing
        && (widget_type == "column" || widget_type == "row")
    {
        container = quote! { #container.spacing(#s) };
    }

    // Get padding from attributes or merged layout
    let padding = node
        .attributes
        .get("padding")
        .and_then(|attr| {
            if let AttributeValue::Static(s) = attr {
                s.parse::<f32>().ok()
            } else {
                None
            }
        })
        .or_else(|| merged_layout.as_ref().and_then(|l| l.padding()));

    // Apply padding
    if let Some(p) = padding {
        container = quote! { #container.padding(#p) };
    }

    // Apply width from attributes or merged layout
    let width_from_attr = node.attributes.get("width").and_then(|attr| {
        if let AttributeValue::Static(s) = attr {
            Some(s.clone())
        } else {
            None
        }
    });
    let width_from_layout = merged_layout.as_ref().and_then(|l| l.width());

    if let Some(width) = width_from_attr {
        let width_expr = generate_length_expr(&width);
        container = quote! { #container.width(#width_expr) };
    } else if let Some(layout_width) = width_from_layout {
        let width_expr = generate_layout_length_expr(layout_width);
        container = quote! { #container.width(#width_expr) };
    }

    // Apply height from attributes or merged layout
    let height_from_attr = node.attributes.get("height").and_then(|attr| {
        if let AttributeValue::Static(s) = attr {
            Some(s.clone())
        } else {
            None
        }
    });
    let height_from_layout = merged_layout.as_ref().and_then(|l| l.height());

    if let Some(height) = height_from_attr {
        let height_expr = generate_length_expr(&height);
        container = quote! { #container.height(#height_expr) };
    } else if let Some(layout_height) = height_from_layout {
        let height_expr = generate_layout_length_expr(layout_height);
        container = quote! { #container.height(#height_expr) };
    }

    // Apply alignment for row/column
    if let Some(align_y) = node.attributes.get("align_y").and_then(|attr| {
        if let AttributeValue::Static(s) = attr {
            Some(s.clone())
        } else {
            None
        }
    }) && widget_type == "row"
    {
        let alignment_expr = match align_y.to_lowercase().as_str() {
            "top" | "start" => quote! { iced::alignment::Vertical::Top },
            "bottom" | "end" => quote! { iced::alignment::Vertical::Bottom },
            _ => quote! { iced::alignment::Vertical::Center },
        };
        container = quote! { #container.align_y(#alignment_expr) };
    }

    // Apply styles
    if widget_type == "container" {
        container = apply_widget_style(container, node, "container", style_classes)?;
    }

    // Use explicit into() conversion to help type inference with nested containers
    Ok(quote! { Into::<Element<'_, #message_ident>>::into(#container) })
}

/// Generate for loop widget with local variable context
fn generate_for_with_locals(
    node: &crate::WidgetNode,
    model_ident: &syn::Ident,
    message_ident: &syn::Ident,
    style_classes: &HashMap<String, StyleClass>,
    local_vars: &std::collections::HashSet<String>,
) -> Result<TokenStream, super::CodegenError> {
    // Get the 'in' attribute (collection to iterate)
    let in_attr = node.attributes.get("in").ok_or_else(|| {
        super::CodegenError::InvalidWidget("for requires 'in' attribute".to_string())
    })?;

    // Get the 'each' attribute (loop variable name)
    let var_name = node
        .attributes
        .get("each")
        .and_then(|attr| {
            if let AttributeValue::Static(s) = attr {
                Some(s.clone())
            } else {
                None
            }
        })
        .unwrap_or_else(|| "item".to_string());

    let var_ident = format_ident!("{}", var_name);

    // Generate the collection expression (raw, without .to_string())
    let collection_expr =
        generate_attribute_value_raw_with_locals(in_attr, model_ident, local_vars);

    // Create new local vars set including the loop variable
    let mut new_local_vars = local_vars.clone();
    new_local_vars.insert(var_name.clone());
    new_local_vars.insert("index".to_string());

    // Generate children widgets with the new local context
    let children: Vec<TokenStream> = node
        .children
        .iter()
        .map(|child| {
            generate_widget_with_locals(
                child,
                model_ident,
                message_ident,
                style_classes,
                &new_local_vars,
            )
        })
        .collect::<Result<_, _>>()?;

    // Generate the for loop that builds widgets
    // Use explicit type annotations to help Rust's type inference
    Ok(quote! {
        {
            let mut widgets: Vec<Element<'_, #message_ident>> = Vec::new();
            for (index, #var_ident) in (#collection_expr).iter().enumerate() {
                let _ = index;
                #(
                    let child_widget: Element<'_, #message_ident> = #children;
                    widgets.push(child_widget);
                )*
            }
            Into::<Element<'_, #message_ident>>::into(iced::widget::column(widgets))
        }
    })
}

/// Generate if widget with local variable context
fn generate_if_with_locals(
    node: &crate::WidgetNode,
    model_ident: &syn::Ident,
    message_ident: &syn::Ident,
    style_classes: &HashMap<String, StyleClass>,
    local_vars: &std::collections::HashSet<String>,
) -> Result<TokenStream, super::CodegenError> {
    let condition_attr = node.attributes.get("condition").ok_or_else(|| {
        super::CodegenError::InvalidWidget("if requires condition attribute".to_string())
    })?;

    let children: Vec<TokenStream> = node
        .children
        .iter()
        .map(|child| {
            generate_widget_with_locals(
                child,
                model_ident,
                message_ident,
                style_classes,
                local_vars,
            )
        })
        .collect::<Result<_, _>>()?;

    let condition_expr =
        generate_attribute_value_with_locals(condition_attr, model_ident, local_vars);

    Ok(quote! {
        if #condition_expr.parse::<bool>().unwrap_or(false) {
            Into::<Element<'_, #message_ident>>::into(iced::widget::column({ let children: Vec<Element<'_, #message_ident>> = vec![#(#children),*]; children }))
        } else {
            Into::<Element<'_, #message_ident>>::into(iced::widget::column({ let children: Vec<Element<'_, #message_ident>> = vec![]; children }))
        }
    })
}

/// Generate checkbox widget with local variable context
fn generate_checkbox_with_locals(
    node: &crate::WidgetNode,
    model_ident: &syn::Ident,
    message_ident: &syn::Ident,
    style_classes: &HashMap<String, StyleClass>,
    local_vars: &std::collections::HashSet<String>,
) -> Result<TokenStream, super::CodegenError> {
    // Get checked attribute
    let checked_attr = node.attributes.get("checked");
    let checked_expr = if let Some(attr) = checked_attr {
        generate_attribute_value_raw_with_locals(attr, model_ident, local_vars)
    } else {
        quote! { false }
    };

    // Get on_change event
    let on_change = node
        .events
        .iter()
        .find(|e| e.event == crate::EventKind::Change);

    let mut checkbox = quote! {
        iced::widget::checkbox(#checked_expr)
    };

    if let Some(event) = on_change {
        let variant_name = to_upper_camel_case(&event.handler);
        let handler_ident = format_ident!("{}", variant_name);

        let param_expr = if let Some(ref param) = event.param {
            let param_tokens = super::bindings::generate_expr_with_locals(&param.expr, local_vars);
            quote! { (#param_tokens) }
        } else {
            quote! {}
        };

        checkbox = quote! {
            #checkbox.on_toggle(move |_| #message_ident::#handler_ident #param_expr)
        };
    }

    // Apply size
    if let Some(size) = node.attributes.get("size").and_then(|attr| {
        if let AttributeValue::Static(s) = attr {
            s.parse::<f32>().ok()
        } else {
            None
        }
    }) {
        checkbox = quote! { #checkbox.size(#size) };
    }

    // Apply styles
    checkbox = apply_widget_style(checkbox, node, "checkbox", style_classes)?;

    Ok(quote! { Into::<Element<'_, #message_ident>>::into(#checkbox) })
}

/// Generate text_input widget with local variable context
fn generate_text_input_with_locals(
    node: &crate::WidgetNode,
    model_ident: &syn::Ident,
    message_ident: &syn::Ident,
    style_classes: &HashMap<String, StyleClass>,
    local_vars: &std::collections::HashSet<String>,
) -> Result<TokenStream, super::CodegenError> {
    // Get placeholder
    let placeholder = node
        .attributes
        .get("placeholder")
        .and_then(|attr| {
            if let AttributeValue::Static(s) = attr {
                Some(s.clone())
            } else {
                None
            }
        })
        .unwrap_or_default();
    let placeholder_lit = proc_macro2::Literal::string(&placeholder);

    // Get value attribute
    let value_attr = node.attributes.get("value");
    let value_expr = if let Some(attr) = value_attr {
        generate_attribute_value_with_locals(attr, model_ident, local_vars)
    } else {
        quote! { String::new() }
    };

    let on_input = node
        .events
        .iter()
        .find(|e| e.event == crate::EventKind::Input);

    let on_submit = node
        .events
        .iter()
        .find(|e| e.event == crate::EventKind::Submit);

    let mut text_input = quote! {
        iced::widget::text_input(#placeholder_lit, &#value_expr)
    };

    // Apply on_input
    if let Some(event) = on_input {
        let variant_name = to_upper_camel_case(&event.handler);
        let handler_ident = format_ident!("{}", variant_name);
        text_input = quote! { #text_input.on_input(|v| #message_ident::#handler_ident(v)) };
    }

    // Apply on_submit
    if let Some(event) = on_submit {
        let variant_name = to_upper_camel_case(&event.handler);
        let handler_ident = format_ident!("{}", variant_name);
        text_input = quote! { #text_input.on_submit(#message_ident::#handler_ident) };
    }

    // Apply size
    if let Some(size) = node.attributes.get("size").and_then(|attr| {
        if let AttributeValue::Static(s) = attr {
            s.parse::<f32>().ok()
        } else {
            None
        }
    }) {
        text_input = quote! { #text_input.size(#size) };
    }

    // Apply padding
    if let Some(padding) = node.attributes.get("padding").and_then(|attr| {
        if let AttributeValue::Static(s) = attr {
            s.parse::<f32>().ok()
        } else {
            None
        }
    }) {
        text_input = quote! { #text_input.padding(#padding) };
    }

    // Apply width
    if let Some(width) = node.attributes.get("width").and_then(|attr| {
        if let AttributeValue::Static(s) = attr {
            Some(generate_length_expr(s))
        } else {
            None
        }
    }) {
        text_input = quote! { #text_input.width(#width) };
    }

    // Apply styles
    text_input = apply_widget_style(text_input, node, "text_input", style_classes)?;

    Ok(quote! { Into::<Element<'_, #message_ident>>::into(#text_input) })
}

/// Generate attribute value expression with local variable context
fn generate_attribute_value_with_locals(
    attr: &AttributeValue,
    _model_ident: &syn::Ident,
    local_vars: &std::collections::HashSet<String>,
) -> TokenStream {
    match attr {
        AttributeValue::Static(s) => {
            let lit = proc_macro2::Literal::string(s);
            quote! { #lit.to_string() }
        }
        AttributeValue::Binding(expr) => {
            super::bindings::generate_expr_with_locals(&expr.expr, local_vars)
        }
        AttributeValue::Interpolated(parts) => {
            let parts_str: Vec<String> = parts
                .iter()
                .map(|part| match part {
                    InterpolatedPart::Literal(s) => s.clone(),
                    InterpolatedPart::Binding(_) => "{}".to_string(),
                })
                .collect();
            let binding_exprs: Vec<TokenStream> = parts
                .iter()
                .filter_map(|part| {
                    if let InterpolatedPart::Binding(expr) = part {
                        Some(super::bindings::generate_expr_with_locals(
                            &expr.expr, local_vars,
                        ))
                    } else {
                        None
                    }
                })
                .collect();

            let format_string = parts_str.join("");
            let lit = proc_macro2::Literal::string(&format_string);

            quote! { format!(#lit, #(#binding_exprs),*) }
        }
    }
}

/// Generate attribute value without `.to_string()` conversion with local variable context
fn generate_attribute_value_raw_with_locals(
    attr: &AttributeValue,
    _model_ident: &syn::Ident,
    local_vars: &std::collections::HashSet<String>,
) -> TokenStream {
    match attr {
        AttributeValue::Static(s) => {
            let lit = proc_macro2::Literal::string(s);
            quote! { #lit }
        }
        AttributeValue::Binding(expr) => {
            super::bindings::generate_bool_expr_with_locals(&expr.expr, local_vars)
        }
        AttributeValue::Interpolated(parts) => {
            let parts_str: Vec<String> = parts
                .iter()
                .map(|part| match part {
                    InterpolatedPart::Literal(s) => s.clone(),
                    InterpolatedPart::Binding(_) => "{}".to_string(),
                })
                .collect();
            let binding_exprs: Vec<TokenStream> = parts
                .iter()
                .filter_map(|part| {
                    if let InterpolatedPart::Binding(expr) = part {
                        Some(super::bindings::generate_expr_with_locals(
                            &expr.expr, local_vars,
                        ))
                    } else {
                        None
                    }
                })
                .collect();

            let format_string = parts_str.join("");
            let lit = proc_macro2::Literal::string(&format_string);

            quote! { format!(#lit, #(#binding_exprs),*) }
        }
    }
}

fn generate_canvas_shapes(
    nodes: &[crate::WidgetNode],
    model_ident: &syn::Ident,
) -> Result<Vec<TokenStream>, super::CodegenError> {
    let mut shape_exprs = Vec::new();
    for node in nodes {
        match node.kind {
            WidgetKind::CanvasRect => shape_exprs.push(generate_rect_shape(node, model_ident)?),
            WidgetKind::CanvasCircle => shape_exprs.push(generate_circle_shape(node, model_ident)?),
            WidgetKind::CanvasLine => shape_exprs.push(generate_line_shape(node, model_ident)?),
            WidgetKind::CanvasText => shape_exprs.push(generate_text_shape(node, model_ident)?),
            WidgetKind::CanvasGroup => shape_exprs.push(generate_group_shape(node, model_ident)?),
            _ => {}
        }
    }
    Ok(shape_exprs)
}

fn generate_rect_shape(
    node: &crate::WidgetNode,
    model_ident: &syn::Ident,
) -> Result<TokenStream, super::CodegenError> {
    let x = generate_f32_attr(node, "x", 0.0, model_ident);
    let y = generate_f32_attr(node, "y", 0.0, model_ident);
    let width = generate_f32_attr(node, "width", 0.0, model_ident);
    let height = generate_f32_attr(node, "height", 0.0, model_ident);
    let fill = generate_color_option_attr(node, "fill", model_ident);
    let stroke = generate_color_option_attr(node, "stroke", model_ident);
    let stroke_width = generate_f32_attr(node, "stroke_width", 1.0, model_ident);
    let radius = generate_f32_attr(node, "radius", 0.0, model_ident);

    Ok(quote! {
        dampen_iced::canvas::CanvasShape::Rect(dampen_iced::canvas::RectShape {
            x: #x,
            y: #y,
            width: #width,
            height: #height,
            fill: #fill,
            stroke: #stroke,
            stroke_width: #stroke_width,
            radius: #radius,
        })
    })
}

fn generate_circle_shape(
    node: &crate::WidgetNode,
    model_ident: &syn::Ident,
) -> Result<TokenStream, super::CodegenError> {
    let cx = generate_f32_attr(node, "cx", 0.0, model_ident);
    let cy = generate_f32_attr(node, "cy", 0.0, model_ident);
    let radius = generate_f32_attr(node, "radius", 0.0, model_ident);
    let fill = generate_color_option_attr(node, "fill", model_ident);
    let stroke = generate_color_option_attr(node, "stroke", model_ident);
    let stroke_width = generate_f32_attr(node, "stroke_width", 1.0, model_ident);

    Ok(quote! {
        dampen_iced::canvas::CanvasShape::Circle(dampen_iced::canvas::CircleShape {
            cx: #cx,
            cy: #cy,
            radius: #radius,
            fill: #fill,
            stroke: #stroke,
            stroke_width: #stroke_width,
        })
    })
}

fn generate_line_shape(
    node: &crate::WidgetNode,
    model_ident: &syn::Ident,
) -> Result<TokenStream, super::CodegenError> {
    let x1 = generate_f32_attr(node, "x1", 0.0, model_ident);
    let y1 = generate_f32_attr(node, "y1", 0.0, model_ident);
    let x2 = generate_f32_attr(node, "x2", 0.0, model_ident);
    let y2 = generate_f32_attr(node, "y2", 0.0, model_ident);
    let stroke = generate_color_option_attr(node, "stroke", model_ident);
    let stroke_width = generate_f32_attr(node, "stroke_width", 1.0, model_ident);

    Ok(quote! {
        dampen_iced::canvas::CanvasShape::Line(dampen_iced::canvas::LineShape {
            x1: #x1,
            y1: #y1,
            x2: #x2,
            y2: #y2,
            stroke: #stroke,
            stroke_width: #stroke_width,
        })
    })
}

fn generate_text_shape(
    node: &crate::WidgetNode,
    model_ident: &syn::Ident,
) -> Result<TokenStream, super::CodegenError> {
    let x = generate_f32_attr(node, "x", 0.0, model_ident);
    let y = generate_f32_attr(node, "y", 0.0, model_ident);
    let content = generate_attribute_value(
        node.attributes
            .get("content")
            .unwrap_or(&AttributeValue::Static(String::new())),
        model_ident,
    );
    let size = generate_f32_attr(node, "size", 16.0, model_ident);
    let color = generate_color_option_attr(node, "color", model_ident);

    Ok(quote! {
        dampen_iced::canvas::CanvasShape::Text(dampen_iced::canvas::TextShape {
            x: #x,
            y: #y,
            content: #content,
            size: #size,
            color: #color,
        })
    })
}

fn generate_group_shape(
    node: &crate::WidgetNode,
    model_ident: &syn::Ident,
) -> Result<TokenStream, super::CodegenError> {
    let children = generate_canvas_shapes(&node.children, model_ident)?;
    let transform = generate_transform_attr(node, model_ident);

    Ok(quote! {
        dampen_iced::canvas::CanvasShape::Group(dampen_iced::canvas::GroupShape {
            transform: #transform,
            children: vec![#(#children),*],
        })
    })
}

fn generate_f32_attr(
    node: &crate::WidgetNode,
    name: &str,
    default: f32,
    _model_ident: &syn::Ident,
) -> TokenStream {
    if let Some(attr) = node.attributes.get(name) {
        match attr {
            AttributeValue::Static(s) => {
                let val = s.parse::<f32>().unwrap_or(default);
                quote! { #val }
            }
            AttributeValue::Binding(expr) => {
                let tokens = super::bindings::generate_bool_expr(&expr.expr);
                quote! { (#tokens) as f32 }
            }
            AttributeValue::Interpolated(_) => quote! { #default },
        }
    } else {
        quote! { #default }
    }
}

fn generate_color_option_attr(
    node: &crate::WidgetNode,
    name: &str,
    _model_ident: &syn::Ident,
) -> TokenStream {
    if let Some(attr) = node.attributes.get(name) {
        match attr {
            AttributeValue::Static(s) => {
                if let Ok(c) = crate::parser::style_parser::parse_color_attr(s) {
                    let r = c.r;
                    let g = c.g;
                    let b = c.b;
                    let a = c.a;
                    quote! { Some(iced::Color::from_rgba(#r, #g, #b, #a)) }
                } else {
                    quote! { None }
                }
            }
            AttributeValue::Binding(expr) => {
                let tokens = generate_expr(&expr.expr);
                quote! {
                    dampen_iced::convert::parse_color_maybe(&(#tokens).to_string())
                        .map(|c| iced::Color::from_rgba(c.r, c.g, c.b, c.a))
                }
            }
            _ => quote! { None },
        }
    } else {
        quote! { None }
    }
}

fn generate_transform_attr(node: &crate::WidgetNode, _model_ident: &syn::Ident) -> TokenStream {
    if let Some(AttributeValue::Static(s)) = node.attributes.get("transform") {
        let s = s.trim();
        if let Some(inner) = s
            .strip_prefix("translate(")
            .and_then(|s| s.strip_suffix(")"))
        {
            let parts: Vec<f32> = inner
                .split(',')
                .filter_map(|p| p.trim().parse().ok())
                .collect();
            if parts.len() == 2 {
                let x = parts[0];
                let y = parts[1];
                return quote! { Some(dampen_iced::canvas::Transform::Translate(#x, #y)) };
            }
        }
        if let Some(inner) = s.strip_prefix("rotate(").and_then(|s| s.strip_suffix(")"))
            && let Ok(angle) = inner.trim().parse::<f32>()
        {
            return quote! { Some(dampen_iced::canvas::Transform::Rotate(#angle)) };
        }
        if let Some(inner) = s.strip_prefix("scale(").and_then(|s| s.strip_suffix(")")) {
            let parts: Vec<f32> = inner
                .split(',')
                .filter_map(|p| p.trim().parse().ok())
                .collect();
            if parts.len() == 1 {
                let s = parts[0];
                return quote! { Some(dampen_iced::canvas::Transform::Scale(#s)) };
            } else if parts.len() == 2 {
                let x = parts[0];
                let y = parts[1];
                return quote! { Some(dampen_iced::canvas::Transform::ScaleXY(#x, #y)) };
            }
        }
        if let Some(inner) = s.strip_prefix("matrix(").and_then(|s| s.strip_suffix(")")) {
            let parts: Vec<f32> = inner
                .split(',')
                .filter_map(|p| p.trim().parse().ok())
                .collect();
            if parts.len() == 6 {
                return quote! { Some(dampen_iced::canvas::Transform::Matrix([#(#parts),*])) };
            }
        }
        quote! { None }
    } else {
        quote! { None }
    }
}

fn generate_canvas_handlers(
    node: &crate::WidgetNode,
    _model_ident: &syn::Ident,
    message_ident: &syn::Ident,
) -> Result<Option<TokenStream>, super::CodegenError> {
    let on_click = node
        .events
        .iter()
        .find(|e| e.event == crate::EventKind::CanvasClick);
    let on_drag = node
        .events
        .iter()
        .find(|e| e.event == crate::EventKind::CanvasDrag);
    let on_move = node
        .events
        .iter()
        .find(|e| e.event == crate::EventKind::CanvasMove);
    let on_release = node
        .events
        .iter()
        .find(|e| e.event == crate::EventKind::CanvasRelease);

    if on_click.is_none() && on_drag.is_none() && on_move.is_none() && on_release.is_none() {
        return Ok(None);
    }

    let mut match_arms = Vec::new();

    if let Some(e) = on_click {
        let variant = format_ident!("{}", to_upper_camel_case(&e.handler));
        let name = &e.handler;
        match_arms.push(quote! { #name => #message_ident :: #variant(event) });
    }
    if let Some(e) = on_drag {
        let variant = format_ident!("{}", to_upper_camel_case(&e.handler));
        let name = &e.handler;
        match_arms.push(quote! { #name => #message_ident :: #variant(event) });
    }
    if let Some(e) = on_move {
        let variant = format_ident!("{}", to_upper_camel_case(&e.handler));
        let name = &e.handler;
        match_arms.push(quote! { #name => #message_ident :: #variant(event) });
    }
    if let Some(e) = on_release {
        let variant = format_ident!("{}", to_upper_camel_case(&e.handler));
        let name = &e.handler;
        match_arms.push(quote! { #name => #message_ident :: #variant(event) });
    }

    let click_name = on_click.map(|e| e.handler.as_str()).unwrap_or("");
    let drag_name = on_drag.map(|e| e.handler.as_str()).unwrap_or("");
    let move_name = on_move.map(|e| e.handler.as_str()).unwrap_or("");
    let release_name = on_release.map(|e| e.handler.as_str()).unwrap_or("");

    Ok(Some(quote! {
        dampen_iced::canvas::CanvasEventHandlers {
            handler_names: dampen_iced::canvas::CanvasHandlerNames {
                on_click: if #click_name != "" { Some(#click_name.to_string()) } else { None },
                on_drag: if #drag_name != "" { Some(#drag_name.to_string()) } else { None },
                on_move: if #move_name != "" { Some(#move_name.to_string()) } else { None },
                on_release: if #release_name != "" { Some(#release_name.to_string()) } else { None },
            },
            msg_factory: |name, event| {
                 match name {
                     #(#match_arms,)*
                     _ => panic!("Unknown canvas handler: {}", name),
                 }
            }
        }
    }))
}

/// Generate Menu widget (MenuBar)
fn generate_menu(
    node: &crate::WidgetNode,
    model_ident: &syn::Ident,
    message_ident: &syn::Ident,
    style_classes: &HashMap<String, StyleClass>,
) -> Result<TokenStream, super::CodegenError> {
    let items = generate_menu_items(&node.children, model_ident, message_ident, style_classes)?;
    // TODO: Handle layout attributes via container wrapper if needed
    Ok(quote! {
        iced_aw::menu::MenuBar::new(#items).into()
    })
}

/// Generate items for Menu/MenuBar
fn generate_menu_items(
    children: &[crate::WidgetNode],
    model_ident: &syn::Ident,
    message_ident: &syn::Ident,
    style_classes: &HashMap<String, StyleClass>,
) -> Result<TokenStream, super::CodegenError> {
    let mut item_exprs = Vec::new();

    for child in children {
        match child.kind {
            WidgetKind::MenuItem => {
                item_exprs.push(generate_menu_item_struct(
                    child,
                    model_ident,
                    message_ident,
                    style_classes,
                )?);
            }
            WidgetKind::MenuSeparator => {
                item_exprs.push(generate_menu_separator_struct(child)?);
            }
            _ => {}
        }
    }

    Ok(quote! {
        vec![#(#item_exprs),*]
    })
}

/// Generate Item struct for MenuItem
fn generate_menu_item_struct(
    node: &crate::WidgetNode,
    model_ident: &syn::Ident,
    message_ident: &syn::Ident,
    style_classes: &HashMap<String, StyleClass>,
) -> Result<TokenStream, super::CodegenError> {
    let label_attr = node.attributes.get("label").ok_or_else(|| {
        super::CodegenError::InvalidWidget("MenuItem requires label attribute".to_string())
    })?;

    let label_expr = generate_attribute_value(label_attr, model_ident);

    // Content is a button
    let mut btn = quote! {
        iced::widget::button(iced::widget::text(#label_expr))
            .width(iced::Length::Shrink) // Use Shrink to avoid layout collapse in MenuBar
            .style(iced::widget::button::text)
    };

    if let Some(event) = node
        .events
        .iter()
        .find(|e| e.event == crate::EventKind::Click)
    {
        let variant_name = to_upper_camel_case(&event.handler);
        let variant_ident = syn::Ident::new(&variant_name, proc_macro2::Span::call_site());

        let msg = if let Some(param) = &event.param {
            let param_expr = crate::codegen::bindings::generate_expr(&param.expr);
            quote! { #message_ident::#variant_ident(#param_expr) }
        } else {
            quote! { #message_ident::#variant_ident }
        };

        btn = quote! { #btn.on_press(#msg) };
    }

    let content = quote! { #btn };

    // Check for submenu
    if let Some(submenu) = node.children.iter().find(|c| c.kind == WidgetKind::Menu) {
        let items =
            generate_menu_items(&submenu.children, model_ident, message_ident, style_classes)?;
        Ok(quote! {
            iced_aw::menu::Item::with_menu(#content, iced_aw::menu::Menu::new(#items))
        })
    } else {
        Ok(quote! {
            iced_aw::menu::Item::new(#content)
        })
    }
}

fn generate_menu_separator_struct(
    _node: &crate::WidgetNode,
) -> Result<TokenStream, super::CodegenError> {
    Ok(quote! {
        iced_aw::menu::Item::new(iced::widget::rule::horizontal(1))
    })
}

fn generate_context_menu(
    node: &crate::WidgetNode,
    model_ident: &syn::Ident,
    message_ident: &syn::Ident,
    style_classes: &HashMap<String, StyleClass>,
    local_vars: &std::collections::HashSet<String>,
) -> Result<TokenStream, super::CodegenError> {
    let underlay = node
        .children
        .first()
        .ok_or(super::CodegenError::InvalidWidget(
            "ContextMenu requires underlay".into(),
        ))?;
    let underlay_expr = generate_widget_with_locals(
        underlay,
        model_ident,
        message_ident,
        style_classes,
        local_vars,
    )?;

    let menu_node = node
        .children
        .get(1)
        .ok_or(super::CodegenError::InvalidWidget(
            "ContextMenu requires menu".into(),
        ))?;

    if menu_node.kind != WidgetKind::Menu {
        return Err(super::CodegenError::InvalidWidget(
            "Second child of ContextMenu must be <menu>".into(),
        ));
    }

    // Generate menu content (column of buttons)
    let mut buttons = Vec::new();
    for child in &menu_node.children {
        match child.kind {
            WidgetKind::MenuItem => {
                let label =
                    child
                        .attributes
                        .get("label")
                        .ok_or(super::CodegenError::InvalidWidget(
                            "MenuItem requires label".into(),
                        ))?;
                let label_expr =
                    generate_attribute_value_with_locals(label, model_ident, local_vars);

                let mut btn = quote! {
                    iced::widget::button(iced::widget::text(#label_expr))
                        .width(iced::Length::Fill)
                        .style(iced::widget::button::text)
                };

                if let Some(event) = child
                    .events
                    .iter()
                    .find(|e| e.event == crate::EventKind::Click)
                {
                    let variant_name = to_upper_camel_case(&event.handler);
                    let variant_ident =
                        syn::Ident::new(&variant_name, proc_macro2::Span::call_site());

                    let msg = if let Some(param) = &event.param {
                        let param_expr = crate::codegen::bindings::generate_expr(&param.expr);
                        quote! { #message_ident::#variant_ident(#param_expr) }
                    } else {
                        quote! { #message_ident::#variant_ident }
                    };
                    btn = quote! { #btn.on_press(#msg) };
                }

                buttons.push(quote! { #btn.into() });
            }
            WidgetKind::MenuSeparator => {
                buttons.push(quote! { iced::widget::rule::horizontal(1).into() });
            }
            _ => {}
        }
    }

    let overlay_content = quote! {
        iced::widget::container(
            iced::widget::column(vec![#(#buttons),*])
                .spacing(2)
        )
        .padding(5)
        .style(iced::widget::container::bordered_box)
        .into()
    };

    Ok(quote! {
        iced_aw::ContextMenu::new(
            #underlay_expr,
            move || #overlay_content
        )
        .into()
    })
}

fn generate_data_table(
    node: &crate::WidgetNode,
    model_ident: &syn::Ident,
    message_ident: &syn::Ident,
    style_classes: &HashMap<String, StyleClass>,
) -> Result<TokenStream, super::CodegenError> {
    let data_attr = node.attributes.get("data").ok_or_else(|| {
        super::CodegenError::InvalidWidget("data_table requires data attribute".to_string())
    })?;
    let data_expr = generate_attribute_value_raw(data_attr, model_ident);

    let mut column_exprs = Vec::new();
    for child in &node.children {
        if child.kind == WidgetKind::DataColumn {
            let header_attr = child.attributes.get("header").ok_or_else(|| {
                super::CodegenError::InvalidWidget(
                    "data_column requires header attribute".to_string(),
                )
            })?;
            let header_expr = generate_attribute_value(header_attr, model_ident);
            let header = quote! { iced::widget::text(#header_expr) };

            let field = child.attributes.get("field");

            let view_closure = if let Some(AttributeValue::Static(field_name)) = field {
                let field_ident = syn::Ident::new(field_name, proc_macro2::Span::call_site());
                // Assuming item is the model type or struct
                quote! {
                    |item| iced::widget::text(item.#field_ident.to_string()).into()
                }
            } else {
                // Template handling
                // Find template content
                let template_content = if let Some(tmpl) = child
                    .children
                    .iter()
                    .find(|c| matches!(c.kind, WidgetKind::Custom(ref s) if s == "template"))
                {
                    &tmpl.children
                } else {
                    &child.children
                };

                if let Some(root) = template_content.first() {
                    let mut locals = std::collections::HashSet::new();
                    locals.insert("index".to_string());
                    locals.insert("item".to_string());

                    let widget_expr = generate_widget_with_locals(
                        root,
                        model_ident,
                        message_ident,
                        style_classes,
                        &locals,
                    )?;

                    quote! {
                        |(index, item)| {
                            let _ = index; // Suppress unused warning
                            #widget_expr.into()
                        }
                    }
                } else {
                    quote! { |(_index, _item)| iced::widget::text("").into() }
                }
            };

            let mut col = quote! {
                iced::widget::table::column(#header, #view_closure)
            };

            if let Some(width) = child.attributes.get("width") {
                let width_expr = match width {
                    AttributeValue::Static(s) => generate_length_expr(s),
                    _ => quote! { iced::Length::Fill },
                };
                col = quote! { #col.width(#width_expr) };
            }

            column_exprs.push(col);
        }
    }

    let table = quote! {
        iced::widget::table::Table::new(vec![#(#column_exprs),*], #data_expr)
    };

    // Handle on_row_click
    // TODO: Re-enable when Table API for row clicks is identified
    /*
    if let Some(event) = node.events.iter().find(|e| e.event == crate::ir::EventKind::RowClick) {
        let variant_name = to_upper_camel_case(&event.handler);
        let handler_ident = format_ident!("{}", variant_name);

        // Generate parameter expression
        let param_expr = if let Some(ref binding) = event.param {
            let mut locals = std::collections::HashSet::new();
            locals.insert("index".to_string());
            locals.insert("item".to_string());

            let expr = crate::codegen::bindings::generate_expr_with_locals(
                &binding.expr,
                &locals,
            );
            quote! { (#expr) }
        } else {
            quote! {}
        };

        // We assume data_expr evaluates to something indexable (Vec, slice)
        // We define `item` as reference to element at index
        table = quote! {
            #table.on_row_click(move |index: usize| {
                let item = &(#data_expr)[index];
                #message_ident::#handler_ident #param_expr
            })
        };
    }
    */

    // Apply layout
    Ok(maybe_wrap_in_container(table, node))
}

/// Generate TreeView widget code
fn generate_tree_view(
    node: &crate::WidgetNode,
    model_ident: &syn::Ident,
    message_ident: &syn::Ident,
    style_classes: &HashMap<String, StyleClass>,
    local_vars: &std::collections::HashSet<String>,
) -> Result<TokenStream, super::CodegenError> {
    // Get tree configuration from attributes
    let indent_size = node
        .attributes
        .get("indent_size")
        .and_then(|attr| match attr {
            AttributeValue::Static(s) => s.parse::<f32>().ok(),
            _ => None,
        })
        .unwrap_or(20.0);

    let node_height = node
        .attributes
        .get("node_height")
        .and_then(|attr| match attr {
            AttributeValue::Static(s) => s.parse::<f32>().ok(),
            _ => None,
        })
        .unwrap_or(30.0);

    let _icon_size = node
        .attributes
        .get("icon_size")
        .and_then(|attr| match attr {
            AttributeValue::Static(s) => s.parse::<f32>().ok(),
            _ => None,
        })
        .unwrap_or(16.0);

    let expand_icon = node
        .attributes
        .get("expand_icon")
        .and_then(|attr| match attr {
            AttributeValue::Static(s) => Some(s.clone()),
            _ => None,
        })
        .unwrap_or_else(|| "▶".to_string());

    let collapse_icon = node
        .attributes
        .get("collapse_icon")
        .and_then(|attr| match attr {
            AttributeValue::Static(s) => Some(s.clone()),
            _ => None,
        })
        .unwrap_or_else(|| "▼".to_string());

    // Check if we have a nodes binding (dynamic tree) or inline children (static tree)
    let has_nodes_binding = node.attributes.contains_key("nodes");

    if has_nodes_binding {
        // Dynamic tree from binding - generate code that builds tree at runtime
        let nodes_binding = node.attributes.get("nodes").ok_or_else(|| {
            super::CodegenError::InvalidWidget("nodes attribute is required".into())
        })?;
        let nodes_expr = generate_attribute_value_raw(nodes_binding, model_ident);

        // Get expanded IDs binding
        let expanded_binding = node.attributes.get("expanded");
        let expanded_expr =
            expanded_binding.map(|attr| generate_attribute_value_raw(attr, model_ident));

        // Get selected ID binding
        let selected_binding = node.attributes.get("selected");
        let selected_expr =
            selected_binding.map(|attr| generate_attribute_value_raw(attr, model_ident));

        // Generate the tree view using a recursive helper function
        let tree_view = quote! {
            {
                let tree_nodes = #nodes_expr;
                let expanded_ids: std::collections::HashSet<String> = #expanded_expr
                    .map(|v: Vec<String>| v.into_iter().collect())
                    .unwrap_or_default();
                let selected_id: Option<String> = #selected_expr;

                // Build tree recursively
                fn build_tree_nodes(
                    nodes: &[TreeNode],
                    expanded_ids: &std::collections::HashSet<String>,
                    selected_id: &Option<String>,
                    depth: usize,
                ) -> Vec<iced::Element<'static, #message_ident>> {
                    let mut elements = Vec::new();
                    for node in nodes {
                        let is_expanded = expanded_ids.contains(&node.id);
                        let is_selected = selected_id.as_ref() == Some(&node.id);
                        let has_children = !node.children.is_empty();

                        // Build node row
                        let indent = (depth as f32) * #indent_size;
                        let node_element = build_tree_node_row(
                            node,
                            is_expanded,
                            is_selected,
                            has_children,
                            indent,
                            #node_height,
                            #expand_icon,
                            #collapse_icon,
                        );
                        elements.push(node_element);

                        // Add children if expanded
                        if is_expanded && has_children {
                            let child_elements = build_tree_nodes(
                                &node.children,
                                expanded_ids,
                                selected_id,
                                depth + 1,
                            );
                            elements.extend(child_elements);
                        }
                    }
                    elements
                }

                iced::widget::column(build_tree_nodes(&tree_nodes, &expanded_ids, &selected_id, 0))
                    .spacing(2)
                    .into()
            }
        };

        Ok(tree_view)
    } else {
        // Static tree from inline XML children
        let tree_elements: Vec<TokenStream> = node
            .children
            .iter()
            .filter(|c| c.kind == WidgetKind::TreeNode)
            .map(|child| {
                generate_tree_node(
                    child,
                    model_ident,
                    message_ident,
                    style_classes,
                    local_vars,
                    indent_size,
                    node_height,
                    &expand_icon,
                    &collapse_icon,
                    0,
                    node,
                )
            })
            .collect::<Result<_, _>>()?;

        Ok(quote! {
            iced::widget::column(vec![#(#tree_elements),*])
                .spacing(2)
                .into()
        })
    }
}

/// Generate a single tree node element (recursive for children)
#[allow(clippy::too_many_arguments)]
fn generate_tree_node(
    node: &crate::WidgetNode,
    _model_ident: &syn::Ident,
    message_ident: &syn::Ident,
    _style_classes: &HashMap<String, StyleClass>,
    _local_vars: &std::collections::HashSet<String>,
    indent_size: f32,
    node_height: f32,
    expand_icon: &str,
    collapse_icon: &str,
    depth: usize,
    parent_node: &crate::WidgetNode,
) -> Result<TokenStream, super::CodegenError> {
    // T068, T069: Prevent infinite recursion during code generation
    if depth > 50 {
        return Ok(quote! {
            iced::widget::text("... max depth reached").size(12).into()
        });
    }

    let id = node.id.clone().unwrap_or_else(|| "unknown".to_string());

    let label = node
        .attributes
        .get("label")
        .and_then(|attr| match attr {
            AttributeValue::Static(s) => Some(s.clone()),
            _ => None,
        })
        .unwrap_or_else(|| id.clone());

    let icon = node.attributes.get("icon").and_then(|attr| match attr {
        AttributeValue::Static(s) => Some(s.clone()),
        _ => None,
    });

    let expanded = node.attributes.get("expanded").and_then(|attr| match attr {
        AttributeValue::Static(s) => s.parse::<bool>().ok(),
        _ => None,
    });

    let selected = node.attributes.get("selected").and_then(|attr| match attr {
        AttributeValue::Static(s) => s.parse::<bool>().ok(),
        _ => None,
    });

    let _disabled = node.attributes.get("disabled").and_then(|attr| match attr {
        AttributeValue::Static(s) => s.parse::<bool>().ok(),
        _ => None,
    });

    let has_children = !node.children.is_empty();
    let is_expanded = expanded.unwrap_or(false);
    let is_selected = selected.unwrap_or(false);

    let indent = (depth as f32) * indent_size;

    // Build label text with optional icon
    let label_text = if let Some(ref icon_str) = icon {
        format!("{} {}", icon_str, label)
    } else {
        label
    };

    // Generate expand/collapse button or spacer
    let toggle_button = if has_children {
        let icon = if is_expanded {
            collapse_icon
        } else {
            expand_icon
        };

        // Check for on_toggle event handler
        if let Some(event) = parent_node
            .events
            .iter()
            .find(|e| matches!(e.event, crate::ir::node::EventKind::Toggle))
        {
            let variant_name = to_upper_camel_case(&event.handler);
            let handler_ident = format_ident!("{}", variant_name);

            quote! {
                iced::widget::button(iced::widget::text(#icon).size(14))
                    .on_press(#message_ident::#handler_ident)
                    .width(iced::Length::Fixed(20.0))
                    .height(iced::Length::Fixed(#node_height))
            }
        } else {
            quote! {
                iced::widget::text(#icon).size(14)
            }
        }
    } else {
        quote! {
            iced::widget::container(iced::widget::text(""))
                .width(iced::Length::Fixed(20.0))
        }
    };

    // Generate label element with selection handling
    let label_element = if let Some(event) = parent_node
        .events
        .iter()
        .find(|e| matches!(e.event, crate::ir::node::EventKind::Select))
    {
        let variant_name = to_upper_camel_case(&event.handler);
        let handler_ident = format_ident!("{}", variant_name);

        quote! {
            iced::widget::button(iced::widget::text(#label_text).size(14))
                .on_press(#message_ident::#handler_ident)
                .style(|_theme: &iced::Theme, _status: iced::widget::button::Status| {
                    if #is_selected {
                        iced::widget::button::Style {
                            background: Some(iced::Background::Color(
                                iced::Color::from_rgb(0.0, 0.48, 0.8),
                            )),
                            text_color: iced::Color::WHITE,
                            ..Default::default()
                        }
                    } else {
                        iced::widget::button::Style::default()
                    }
                })
        }
    } else {
        quote! {
            iced::widget::text(#label_text).size(14)
        }
    };

    // Build node row
    let node_row = quote! {
        iced::widget::row(vec![#toggle_button.into(), #label_element.into()])
            .spacing(4)
            .padding(iced::Padding::from([0.0, 0.0, 0.0, #indent]))
    };

    // If expanded and has children, render them recursively
    if is_expanded && has_children {
        let child_elements: Vec<TokenStream> = node
            .children
            .iter()
            .filter(|c| c.kind == WidgetKind::TreeNode)
            .map(|child| {
                generate_tree_node(
                    child,
                    _model_ident,
                    message_ident,
                    _style_classes,
                    _local_vars,
                    indent_size,
                    node_height,
                    expand_icon,
                    collapse_icon,
                    depth + 1,
                    parent_node,
                )
            })
            .collect::<Result<_, _>>()?;

        Ok(quote! {
            iced::widget::column(vec![
                #node_row.into(),
                iced::widget::column(vec![#(#child_elements),*])
                    .spacing(2)
                    .into(),
            ])
            .spacing(2)
        })
    } else {
        Ok(node_row)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse;

    #[test]
    fn test_view_generation() {
        let xml = r#"<column><text value="Hello" /></column>"#;
        let doc = parse(xml).unwrap();

        let result = generate_view(&doc, "Model", "Message").unwrap();
        let code = result.to_string();

        assert!(code.contains("text"));
        assert!(code.contains("column"));
    }

    #[test]
    fn test_view_generation_with_binding() {
        let xml = r#"<column><text value="{name}" /></column>"#;
        let doc = parse(xml).unwrap();

        let result = generate_view(&doc, "Model", "Message").unwrap();
        let code = result.to_string();

        assert!(code.contains("name"));
        assert!(code.contains("to_string"));
    }

    #[test]
    fn test_button_with_handler() {
        let xml = r#"<column><button label="Click" on_click="handle_click" /></column>"#;
        let doc = parse(xml).unwrap();

        let result = generate_view(&doc, "Model", "Message").unwrap();
        let code = result.to_string();

        assert!(code.contains("button"));
        assert!(code.contains("HandleClick"));
    }

    #[test]
    fn test_container_with_children() {
        let xml = r#"<column spacing="10"><text value="A" /><text value="B" /></column>"#;
        let doc = parse(xml).unwrap();

        let result = generate_view(&doc, "Model", "Message").unwrap();
        let code = result.to_string();

        assert!(code.contains("column"));
        assert!(code.contains("spacing"));
    }

    #[test]
    fn test_button_with_inline_style() {
        use crate::ir::node::WidgetNode;
        use crate::ir::style::{Background, Color, StyleProperties};
        use std::collections::HashMap;

        // Manually construct a button with inline style
        let button_node = WidgetNode {
            kind: WidgetKind::Button,
            id: None,
            attributes: {
                let mut attrs = HashMap::new();
                attrs.insert(
                    "label".to_string(),
                    AttributeValue::Static("Test".to_string()),
                );
                attrs
            },
            events: vec![],
            children: vec![],
            span: Default::default(),
            style: Some(StyleProperties {
                background: Some(Background::Color(Color::from_rgb8(52, 152, 219))),
                color: Some(Color::from_rgb8(255, 255, 255)),
                border: None,
                shadow: None,
                opacity: None,
                transform: None,
            }),
            layout: None,
            theme_ref: None,
            classes: vec![],
            breakpoint_attributes: HashMap::new(),
            inline_state_variants: HashMap::new(),
        };

        let model_ident = syn::Ident::new("model", proc_macro2::Span::call_site());
        let message_ident = syn::Ident::new("Message", proc_macro2::Span::call_site());
        let style_classes = HashMap::new();

        let result =
            generate_button(&button_node, &model_ident, &message_ident, &style_classes).unwrap();
        let code = result.to_string();

        // Should contain style closure (note: quote! adds spaces)
        assert!(code.contains("style"));
        assert!(code.contains("button :: Status"));
        assert!(code.contains("button :: Style"));
        assert!(code.contains("background"));
        assert!(code.contains("text_color"));
    }

    #[test]
    fn test_button_with_css_class() {
        use crate::ir::node::WidgetNode;
        use crate::ir::theme::StyleClass;
        use std::collections::HashMap;

        // Manually construct a button with CSS class
        let button_node = WidgetNode {
            kind: WidgetKind::Button,
            id: None,
            attributes: {
                let mut attrs = HashMap::new();
                attrs.insert(
                    "label".to_string(),
                    AttributeValue::Static("Test".to_string()),
                );
                attrs
            },
            events: vec![],
            children: vec![],
            span: Default::default(),
            style: None,
            layout: None,
            theme_ref: None,
            classes: vec!["primary-button".to_string()],
            breakpoint_attributes: HashMap::new(),
            inline_state_variants: HashMap::new(),
        };

        let model_ident = syn::Ident::new("model", proc_macro2::Span::call_site());
        let message_ident = syn::Ident::new("Message", proc_macro2::Span::call_site());
        let style_classes: HashMap<String, StyleClass> = HashMap::new();

        let result =
            generate_button(&button_node, &model_ident, &message_ident, &style_classes).unwrap();
        let code = result.to_string();

        // Should call style function (note: quote! adds spaces)
        assert!(code.contains("style"));
        assert!(code.contains("style_primary_button"));
    }

    #[test]
    fn test_container_with_inline_style() {
        use crate::ir::node::WidgetNode;
        use crate::ir::style::{
            Background, Border, BorderRadius, BorderStyle, Color, StyleProperties,
        };
        use crate::ir::theme::StyleClass;
        use std::collections::HashMap;

        let container_node = WidgetNode {
            kind: WidgetKind::Container,
            id: None,
            attributes: HashMap::new(),
            events: vec![],
            children: vec![],
            span: Default::default(),
            style: Some(StyleProperties {
                background: Some(Background::Color(Color::from_rgb8(240, 240, 240))),
                color: None,
                border: Some(Border {
                    width: 2.0,
                    color: Color::from_rgb8(200, 200, 200),
                    radius: BorderRadius {
                        top_left: 8.0,
                        top_right: 8.0,
                        bottom_right: 8.0,
                        bottom_left: 8.0,
                    },
                    style: BorderStyle::Solid,
                }),
                shadow: None,
                opacity: None,
                transform: None,
            }),
            layout: None,
            theme_ref: None,
            classes: vec![],
            breakpoint_attributes: HashMap::new(),
            inline_state_variants: HashMap::new(),
        };

        let model_ident = syn::Ident::new("model", proc_macro2::Span::call_site());
        let message_ident = syn::Ident::new("Message", proc_macro2::Span::call_site());
        let style_classes: HashMap<String, StyleClass> = HashMap::new();

        let result = generate_container(
            &container_node,
            "container",
            &model_ident,
            &message_ident,
            &style_classes,
        )
        .unwrap();
        let code = result.to_string();

        // Should contain style closure (note: quote! adds spaces)
        assert!(code.contains("style"));
        assert!(code.contains("container :: Style"));
        assert!(code.contains("background"));
        assert!(code.contains("border"));
    }
}

/// Generate TabBar widget code with content
fn generate_tab_bar_with_locals(
    node: &crate::WidgetNode,
    model_ident: &syn::Ident,
    message_ident: &syn::Ident,
    style_classes: &HashMap<String, StyleClass>,
    local_vars: &std::collections::HashSet<String>,
) -> Result<TokenStream, super::CodegenError> {
    use proc_macro2::Span;
    use quote::quote;

    // Get selected index attribute
    let selected_attr = node.attributes.get("selected").ok_or_else(|| {
        super::CodegenError::InvalidWidget("TabBar requires 'selected' attribute".to_string())
    })?;

    // Generate selected index expression
    let selected_expr = match selected_attr {
        AttributeValue::Static(s) => {
            let idx: usize = s.parse().map_err(|_| {
                super::CodegenError::InvalidWidget(format!("Invalid selected index: {}", s))
            })?;
            quote! { #idx }
        }
        AttributeValue::Binding(binding) => {
            // Generate binding expression - generate_expr returns a TokenStream that produces a String
            let binding_expr = generate_expr(&binding.expr);
            quote! { (#binding_expr).parse::<usize>().unwrap_or(0) }
        }
        _ => quote! { 0usize },
    };

    // Find on_select event handler
    let on_select_handler = node
        .events
        .iter()
        .find(|e| matches!(e.event, crate::ir::EventKind::Select))
        .map(|e| syn::Ident::new(&e.handler, Span::call_site()));

    // Generate tab labels and content
    let _tab_count = node.children.len();
    let tab_labels: Vec<_> = node
        .children
        .iter()
        .enumerate()
        .map(|(idx, child)| {
            let idx_lit = proc_macro2::Literal::usize_unsuffixed(idx);

            // Get label from tab
            let label_expr = if let Some(label_attr) = child.attributes.get("label") {
                match label_attr {
                    AttributeValue::Static(s) => Some(quote! { #s.to_string() }),
                    _ => None,
                }
            } else {
                None
            };

            // Get icon from tab
            let icon_expr = if let Some(icon_attr) = child.attributes.get("icon") {
                match icon_attr {
                    AttributeValue::Static(s) => {
                        let icon_char = resolve_icon_for_codegen(s);
                        Some(quote! { #icon_char })
                    }
                    _ => None,
                }
            } else {
                None
            };

            // Build TabLabel expression based on what we have
            let tab_label_expr = match (icon_expr, label_expr) {
                (Some(icon), Some(label)) => {
                    quote! { iced_aw::tab_bar::TabLabel::IconText(#icon, #label) }
                }
                (Some(icon), None) => {
                    quote! { iced_aw::tab_bar::TabLabel::Icon(#icon) }
                }
                (None, Some(label)) => {
                    quote! { iced_aw::tab_bar::TabLabel::Text(#label) }
                }
                (None, None) => {
                    quote! { iced_aw::tab_bar::TabLabel::Text("Tab".to_string()) }
                }
            };

            quote! {
                tab_bar = tab_bar.push(#idx_lit, #tab_label_expr);
            }
        })
        .collect();

    // Generate content for each tab
    let tab_content_arms: Vec<_> = node
        .children
        .iter()
        .enumerate()
        .map(|(idx, child)| {
            let idx_lit = proc_macro2::Literal::usize_unsuffixed(idx);

            // Generate content for this tab's children
            let content_widgets: Vec<_> = child
                .children
                .iter()
                .map(|child_node| {
                    generate_widget_with_locals(
                        child_node,
                        model_ident,
                        message_ident,
                        style_classes,
                        local_vars,
                    )
                })
                .collect::<Result<Vec<_>, _>>()?;

            Ok::<_, super::CodegenError>(quote! {
                #idx_lit => iced::widget::column(vec![#(#content_widgets),*]).into()
            })
        })
        .collect::<Result<Vec<_>, super::CodegenError>>()?;

    // Generate on_select callback if handler exists
    let on_select_expr = if let Some(handler) = on_select_handler {
        quote! {
            .on_select(|idx| #message_ident::#handler(idx))
        }
    } else {
        quote! {}
    };

    // Generate icon_size if specified
    let icon_size_expr = if let Some(icon_size_attr) = node.attributes.get("icon_size") {
        match icon_size_attr {
            AttributeValue::Static(s) => {
                if let Ok(icon_size) = s.parse::<f32>() {
                    Some(quote! { .icon_size(#icon_size) })
                } else {
                    None
                }
            }
            _ => None,
        }
    } else {
        None
    };

    // Generate text_size if specified
    let text_size_expr = if let Some(text_size_attr) = node.attributes.get("text_size") {
        match text_size_attr {
            AttributeValue::Static(s) => {
                if let Ok(text_size) = s.parse::<f32>() {
                    Some(quote! { .text_size(#text_size) })
                } else {
                    None
                }
            }
            _ => None,
        }
    } else {
        None
    };

    // Build the complete TabBar widget with content
    let tab_bar_widget = quote! {
        {
            let mut tab_bar = iced_aw::TabBar::new(#selected_expr)
                #on_select_expr
                #icon_size_expr
                #text_size_expr;

            #(#tab_labels)*

            tab_bar
        }
    };

    // Build content element using match on selected index
    let content_element = if tab_content_arms.is_empty() {
        quote! { iced::widget::column(vec![]).into() }
    } else {
        quote! {
            match #selected_expr {
                #(#tab_content_arms,)*
                _ => iced::widget::column(vec![]).into(),
            }
        }
    };

    // Combine TabBar and content in a column
    let result = quote! {
        iced::widget::column![
            #tab_bar_widget,
            #content_element
        ]
    };

    Ok(result)
}

/// Resolve icon name to Unicode character for codegen
fn resolve_icon_for_codegen(name: &str) -> char {
    match name {
        "home" => '\u{F015}',
        "settings" => '\u{F013}',
        "user" => '\u{F007}',
        "search" => '\u{F002}',
        "add" => '\u{F067}',
        "delete" => '\u{F1F8}',
        "edit" => '\u{F044}',
        "save" => '\u{F0C7}',
        "close" => '\u{F00D}',
        "back" => '\u{F060}',
        "forward" => '\u{F061}',
        _ => '\u{F111}', // Circle as fallback
    }
}

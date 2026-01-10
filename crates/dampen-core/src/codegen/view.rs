//! View function generation
//!
//! This module generates static Rust code for widget trees with inlined bindings.

use crate::DampenDocument;
use crate::codegen::bindings::generate_expr;
use crate::ir::node::{AttributeValue, InterpolatedPart, WidgetKind};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

/// Generate the view function body from a Dampen document
pub fn generate_view(
    document: &DampenDocument,
    _model_name: &str,
    message_name: &str,
) -> Result<TokenStream, super::CodegenError> {
    let message_ident = syn::Ident::new(message_name, proc_macro2::Span::call_site());
    let count_ident = syn::Ident::new("count", proc_macro2::Span::call_site());

    let root_widget = generate_widget(&document.root, &count_ident, &message_ident)?;

    Ok(quote! {
        #root_widget
    })
}

/// Generate code for a widget node
fn generate_widget(
    node: &crate::WidgetNode,
    model_ident: &syn::Ident,
    message_ident: &syn::Ident,
) -> Result<TokenStream, super::CodegenError> {
    match node.kind {
        WidgetKind::Text => generate_text(node, model_ident),
        WidgetKind::Button => generate_button(node, model_ident, message_ident),
        WidgetKind::Column => generate_container(node, "column", model_ident, message_ident),
        WidgetKind::Row => generate_container(node, "row", model_ident, message_ident),
        WidgetKind::Container => generate_container(node, "container", model_ident, message_ident),
        WidgetKind::Scrollable => {
            generate_container(node, "scrollable", model_ident, message_ident)
        }
        WidgetKind::Stack => generate_stack(node, model_ident, message_ident),
        WidgetKind::Space => generate_space(node),
        WidgetKind::Rule => generate_rule(node),
        WidgetKind::Checkbox => generate_checkbox(node, model_ident, message_ident),
        WidgetKind::Toggler => generate_toggler(node, model_ident, message_ident),
        WidgetKind::Slider => generate_slider(node, model_ident, message_ident),
        WidgetKind::Radio => generate_radio(node, model_ident, message_ident),
        WidgetKind::ProgressBar => generate_progress_bar(node, model_ident),
        WidgetKind::TextInput => generate_text_input(node, model_ident, message_ident),
        WidgetKind::Image => generate_image(node),
        WidgetKind::Svg => generate_svg(node),
        WidgetKind::PickList => generate_pick_list(node, model_ident, message_ident),
        WidgetKind::ComboBox => generate_combo_box(node, model_ident, message_ident),
        WidgetKind::Tooltip => generate_tooltip(node, model_ident, message_ident),
        WidgetKind::Grid => generate_grid(node, model_ident, message_ident),
        WidgetKind::Canvas => generate_canvas(node, model_ident, message_ident),
        WidgetKind::Float => generate_float(node, model_ident, message_ident),
        WidgetKind::For => generate_for(node, model_ident, message_ident),
        WidgetKind::Custom(ref name) => {
            generate_custom_widget(node, name, model_ident, message_ident)
        }
    }
}

/// Generate text widget
fn generate_text(
    node: &crate::WidgetNode,
    model_ident: &syn::Ident,
) -> Result<TokenStream, super::CodegenError> {
    let value_attr = node.attributes.get("value").ok_or_else(|| {
        super::CodegenError::InvalidWidget("text requires value attribute".to_string())
    })?;

    let value_expr = generate_attribute_value(value_attr, model_ident);

    Ok(quote! {
        iced::widget::text(#value_expr)
    })
}

/// Generate button widget
fn generate_button(
    node: &crate::WidgetNode,
    model_ident: &syn::Ident,
    message_ident: &syn::Ident,
) -> Result<TokenStream, super::CodegenError> {
    let label_attr = node.attributes.get("label").ok_or_else(|| {
        super::CodegenError::InvalidWidget("button requires label attribute".to_string())
    })?;

    let label_expr = generate_attribute_value(label_attr, model_ident);

    let on_click = node
        .events
        .iter()
        .find(|e| e.event == crate::EventKind::Click);

    if let Some(event) = on_click {
        let handler_ident = format_ident!("{}", event.handler);

        let param_expr = if let Some(ref param) = event.param {
            let param_tokens = generate_expr(&param.expr);
            quote! { Some(#param_tokens) }
        } else {
            quote! { None }
        };

        Ok(quote! {
            iced::widget::button(#label_expr)
                .on_press(#message_ident::#handler_ident(#param_expr))
        })
    } else {
        Ok(quote! {
            iced::widget::button(#label_expr)
        })
    }
}

/// Generate container widget (column, row, container, scrollable)
fn generate_container(
    node: &crate::WidgetNode,
    widget_type: &str,
    model_ident: &syn::Ident,
    message_ident: &syn::Ident,
) -> Result<TokenStream, super::CodegenError> {
    let children: Vec<TokenStream> = node
        .children
        .iter()
        .map(|child| generate_widget(child, model_ident, message_ident))
        .collect::<Result<_, _>>()?;

    let widget_ident = format_ident!("{}", widget_type);

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

    let mut widget = quote! {
        iced::widget::#widget_ident(vec![#(#children),*])
    };

    if let Some(s) = spacing {
        widget = quote! { #widget.spacing(#s) };
    }

    if let Some(p) = padding {
        widget = quote! { #widget.padding(#p) };
    }

    Ok(widget)
}

/// Generate stack widget
fn generate_stack(
    node: &crate::WidgetNode,
    model_ident: &syn::Ident,
    message_ident: &syn::Ident,
) -> Result<TokenStream, super::CodegenError> {
    let children: Vec<TokenStream> = node
        .children
        .iter()
        .map(|child| generate_widget(child, model_ident, message_ident))
        .collect::<Result<_, _>>()?;

    Ok(quote! {
        iced::widget::stack(vec![#(#children),*])
    })
}

/// Generate space widget
fn generate_space(_node: &crate::WidgetNode) -> Result<TokenStream, super::CodegenError> {
    Ok(quote! {
        iced::widget::Space::default()
    })
}

/// Generate rule (horizontal line) widget
fn generate_rule(node: &crate::WidgetNode) -> Result<TokenStream, super::CodegenError> {
    let width = node.attributes.get("width").and_then(|attr| {
        if let AttributeValue::Static(s) = attr {
            s.parse::<f32>().ok()
        } else {
            None
        }
    });

    if let Some(w) = width {
        Ok(quote! {
            iced::widget::rule::Rule::default().width(#w)
        })
    } else {
        Ok(quote! {
            iced::widget::Rule::default()
        })
    }
}

/// Generate checkbox widget
fn generate_checkbox(
    node: &crate::WidgetNode,
    _model_ident: &syn::Ident,
    message_ident: &syn::Ident,
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
        .map(|attr| generate_attribute_value(attr, _model_ident))
        .unwrap_or(quote! { false });

    let on_toggle = node
        .events
        .iter()
        .find(|e| e.event == crate::EventKind::Toggle);

    if let Some(event) = on_toggle {
        let handler_ident = format_ident!("{}", event.handler);
        Ok(quote! {
            iced::widget::checkbox(#label_expr, #checked_expr)
                .on_toggle(#message_ident::#handler_ident)
        })
    } else {
        Ok(quote! {
            iced::widget::checkbox(#label_expr, #checked_expr)
        })
    }
}

/// Generate toggler widget
fn generate_toggler(
    node: &crate::WidgetNode,
    _model_ident: &syn::Ident,
    message_ident: &syn::Ident,
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
        .map(|attr| generate_attribute_value(attr, _model_ident))
        .unwrap_or(quote! { false });

    let on_toggle = node
        .events
        .iter()
        .find(|e| e.event == crate::EventKind::Toggle);

    if let Some(event) = on_toggle {
        let handler_ident = format_ident!("{}", event.handler);
        Ok(quote! {
            iced::widget::toggler(#label_expr, #is_toggled_expr, None)
                .on_toggle(|_| #message_ident::#handler_ident)
        })
    } else {
        Ok(quote! {
            iced::widget::toggler(#label_expr, #is_toggled_expr, None)
        })
    }
}

/// Generate slider widget
fn generate_slider(
    node: &crate::WidgetNode,
    model_ident: &syn::Ident,
    message_ident: &syn::Ident,
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

    if let Some(event) = on_change {
        let handler_ident = format_ident!("{}", event.handler);
        slider = quote! {
            iced::widget::slider(0.0..=100.0, #value_expr, |v| #message_ident::#handler_ident(v))
        };
    }

    Ok(slider)
}

/// Generate radio widget
fn generate_radio(
    node: &crate::WidgetNode,
    _model_ident: &syn::Ident,
    message_ident: &syn::Ident,
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
        let handler_ident = format_ident!("{}", event.handler);
        Ok(quote! {
            iced::widget::radio(#label_expr, #value_expr, #selected_expr, |v| #message_ident::#handler_ident(v))
        })
    } else {
        Ok(quote! {
            iced::widget::radio(#label_expr, #value_expr, #selected_expr, |_| ())
        })
    }
}

/// Generate progress bar widget
fn generate_progress_bar(
    node: &crate::WidgetNode,
    model_ident: &syn::Ident,
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

    if let Some(max) = max_attr {
        Ok(quote! {
            iced::widget::progress_bar(0.0..=#max, #value_expr)
        })
    } else {
        Ok(quote! {
            iced::widget::progress_bar(0.0..=100.0, #value_expr)
        })
    }
}

/// Generate text input widget
fn generate_text_input(
    node: &crate::WidgetNode,
    model_ident: &syn::Ident,
    message_ident: &syn::Ident,
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
        let handler_ident = format_ident!("{}", event.handler);
        text_input = quote! {
            #text_input.on_input(|v| #message_ident::#handler_ident(v))
        };
    }

    Ok(text_input)
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

    let image = quote! {
        iced::widget::image::Image::new(iced::widget::image::Handle::from_memory(std::fs::read(#src_lit).unwrap_or_default()))
    };

    if let (Some(w), Some(h)) = (width, height) {
        Ok(quote! { #image.width(#w).height(#h) })
    } else if let Some(w) = width {
        Ok(quote! { #image.width(#w) })
    } else if let Some(h) = height {
        Ok(quote! { #image.height(#h) })
    } else {
        Ok(image)
    }
}

/// Generate SVG widget
fn generate_svg(node: &crate::WidgetNode) -> Result<TokenStream, super::CodegenError> {
    let path_attr = node.attributes.get("path").ok_or_else(|| {
        super::CodegenError::InvalidWidget("svg requires path attribute".to_string())
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

    let svg = quote! {
        iced::widget::svg::Svg::new(iced::widget::svg::Handle::from_path(#path_lit))
    };

    if let (Some(w), Some(h)) = (width, height) {
        Ok(quote! { #svg.width(#w).height(#h) })
    } else if let Some(w) = width {
        Ok(quote! { #svg.width(#w) })
    } else if let Some(h) = height {
        Ok(quote! { #svg.height(#h) })
    } else {
        Ok(svg)
    }
}

/// Generate pick list widget
fn generate_pick_list(
    node: &crate::WidgetNode,
    model_ident: &syn::Ident,
    message_ident: &syn::Ident,
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
        let handler_ident = format_ident!("{}", event.handler);
        Ok(quote! {
            iced::widget::pick_list(&[#(#options_ref),*], #selected_expr, |v| #message_ident::#handler_ident(v))
        })
    } else {
        Ok(quote! {
            iced::widget::pick_list(&[#(#options_ref),*], #selected_expr, |_| ())
        })
    }
}

/// Generate combo box widget
fn generate_combo_box(
    node: &crate::WidgetNode,
    model_ident: &syn::Ident,
    message_ident: &syn::Ident,
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
        let handler_ident = format_ident!("{}", event.handler);
        Ok(quote! {
            iced::widget::combo_box(&[#(#options_ref),*], "", #selected_expr, |v, _| #message_ident::#handler_ident(v))
        })
    } else {
        Ok(quote! {
            iced::widget::combo_box(&[#(#options_ref),*], "", #selected_expr, |_, _| ())
        })
    }
}

/// Generate tooltip widget
fn generate_tooltip(
    node: &crate::WidgetNode,
    model_ident: &syn::Ident,
    message_ident: &syn::Ident,
) -> Result<TokenStream, super::CodegenError> {
    let child = node.children.first().ok_or_else(|| {
        super::CodegenError::InvalidWidget("tooltip must have exactly one child".to_string())
    })?;
    let child_widget = generate_widget(child, model_ident, message_ident)?;

    let message_attr = node.attributes.get("message").ok_or_else(|| {
        super::CodegenError::InvalidWidget("tooltip requires message attribute".to_string())
    })?;
    let message_expr = generate_attribute_value(message_attr, model_ident);

    Ok(quote! {
        iced::widget::tooltip(#child_widget, #message_expr, iced::widget::tooltip::Position::FollowCursor)
    })
}

/// Generate grid widget
fn generate_grid(
    node: &crate::WidgetNode,
    model_ident: &syn::Ident,
    message_ident: &syn::Ident,
) -> Result<TokenStream, super::CodegenError> {
    let children: Vec<TokenStream> = node
        .children
        .iter()
        .map(|child| generate_widget(child, model_ident, message_ident))
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

    Ok(if let Some(p) = padding {
        quote! { #grid.padding(#p) }
    } else {
        grid
    })
}

/// Generate canvas widget
fn generate_canvas(
    node: &crate::WidgetNode,
    _model_ident: &syn::Ident,
    _message_ident: &syn::Ident,
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

    let size = match (width, height) {
        (Some(w), Some(h)) => quote! { iced::Size::new(#w, #h) },
        (Some(w), None) => quote! { iced::Size::new(#w, 100.0) },
        (None, Some(h)) => quote! { iced::Size::new(100.0, #h) },
        _ => quote! { iced::Size::new(100.0, 100.0) },
    };

    Ok(quote! {
        iced::widget::canvas(#size)
    })
}

/// Generate float widget
fn generate_float(
    node: &crate::WidgetNode,
    model_ident: &syn::Ident,
    message_ident: &syn::Ident,
) -> Result<TokenStream, super::CodegenError> {
    let child = node.children.first().ok_or_else(|| {
        super::CodegenError::InvalidWidget("float must have exactly one child".to_string())
    })?;
    let child_widget = generate_widget(child, model_ident, message_ident)?;

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

    if let (Some(ox), Some(oy)) = (offset_x, offset_y) {
        Ok(quote! { #float.offset_x(#ox).offset_y(#oy) })
    } else if let Some(ox) = offset_x {
        Ok(quote! { #float.offset_x(#ox) })
    } else if let Some(oy) = offset_y {
        Ok(quote! { #float.offset_y(#oy) })
    } else {
        Ok(float)
    }
}

/// Generate for loop widget (iterates over collection)
fn generate_for(
    node: &crate::WidgetNode,
    model_ident: &syn::Ident,
    message_ident: &syn::Ident,
) -> Result<TokenStream, super::CodegenError> {
    let items_attr = node.attributes.get("items").ok_or_else(|| {
        super::CodegenError::InvalidWidget("for requires items attribute".to_string())
    })?;

    let item_name = node
        .attributes
        .get("item")
        .and_then(|attr| {
            if let AttributeValue::Static(s) = attr {
                Some(s.clone())
            } else {
                None
            }
        })
        .unwrap_or_else(|| "item".to_string());

    let _children: Vec<TokenStream> = node
        .children
        .iter()
        .map(|child| generate_widget(child, model_ident, message_ident))
        .collect::<Result<_, _>>()?;

    let _items_expr = generate_attribute_value(items_attr, model_ident);
    let _item_ident = format_ident!("{}", item_name);

    Ok(quote! {
        {
            let _items = _items_expr;
            iced::widget::column(vec![])
        }
    })
}

/// Generate custom widget
fn generate_custom_widget(
    node: &crate::WidgetNode,
    name: &str,
    model_ident: &syn::Ident,
    message_ident: &syn::Ident,
) -> Result<TokenStream, super::CodegenError> {
    let widget_ident = format_ident!("{}", name);
    let children: Vec<TokenStream> = node
        .children
        .iter()
        .map(|child| generate_widget(child, model_ident, message_ident))
        .collect::<Result<_, _>>()?;

    Ok(quote! {
        #widget_ident(vec![#(#children),*])
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
        assert!(code.contains("handle_click"));
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
}

//! Completion request handler.
//!
//! Provides context-aware autocompletion for widgets, attributes, and values.

use dampen_core::ir::WidgetKind;
use dampen_core::schema::get_widget_schema;
use tower_lsp::lsp_types::{
    CompletionItem, CompletionItemKind, CompletionParams, CompletionResponse, InsertTextFormat,
};

use crate::analyzer::{Analyzer, CompletionContext};
use crate::document::DocumentState;

/// Handles completion requests.
///
/// Returns completion items based on cursor position and document context.
///
/// # Arguments
///
/// * `doc` - The document state
/// * `params` - Completion parameters
///
/// # Returns
///
/// Optional completion list
pub fn completion(doc: &DocumentState, params: CompletionParams) -> Option<CompletionResponse> {
    let analyzer = Analyzer::new();
    let context = analyzer.get_completion_context(doc, params.text_document_position.position);

    let items = match context {
        CompletionContext::WidgetName => complete_widget_names(),
        CompletionContext::AttributeName { widget } => complete_attributes(&widget),
        CompletionContext::AttributeValue { widget, attribute } => {
            complete_values(&widget, &attribute)
        }
        _ => vec![],
    };

    Some(CompletionResponse::Array(items))
}

fn complete_widget_names() -> Vec<CompletionItem> {
    WidgetKind::all_standard()
        .iter()
        .map(|name| CompletionItem {
            label: name.to_string(),
            kind: Some(CompletionItemKind::CLASS),
            detail: Some("Widget".to_string()),
            ..Default::default()
        })
        .collect()
}

fn complete_attributes(widget_name: &str) -> Vec<CompletionItem> {
    let kind = match widget_kind_from_str(widget_name) {
        Some(k) => k,
        None => return vec![],
    };

    let schema = get_widget_schema(&kind);
    let mut items = Vec::new();

    // Helper to add attributes
    let mut add_attrs = |attrs: &[&str], detail: &str, kind: CompletionItemKind| {
        for attr in attrs {
            items.push(CompletionItem {
                label: attr.to_string(),
                kind: Some(kind),
                detail: Some(detail.to_string()),
                insert_text: Some(format!("{}=\"$0\"", attr)),
                insert_text_format: Some(InsertTextFormat::SNIPPET),
                ..Default::default()
            });
        }
    };

    add_attrs(
        schema.required,
        "Required Attribute",
        CompletionItemKind::PROPERTY,
    );
    add_attrs(
        schema.optional,
        "Optional Attribute",
        CompletionItemKind::PROPERTY,
    );
    add_attrs(schema.events, "Event", CompletionItemKind::EVENT);
    add_attrs(
        schema.style_attributes,
        "Style Attribute",
        CompletionItemKind::FIELD,
    );
    add_attrs(
        schema.layout_attributes,
        "Layout Attribute",
        CompletionItemKind::PROPERTY,
    );

    items
}

fn complete_values(_widget: &str, attr: &str) -> Vec<CompletionItem> {
    // Basic heuristics for common attribute types
    match attr {
        "enabled" | "checked" | "visible" | "show" | "toggled" | "selected" | "active"
        | "password" | "close_on_select" | "use_24h" | "show_seconds" | "show_alpha" => vec![
            CompletionItem {
                label: "true".to_string(),
                kind: Some(CompletionItemKind::VALUE),
                ..Default::default()
            },
            CompletionItem {
                label: "false".to_string(),
                kind: Some(CompletionItemKind::VALUE),
                ..Default::default()
            },
        ],
        "align" | "align_items" | "align_self" | "align_x" | "align_y" => vec![
            CompletionItem {
                label: "start".to_string(),
                kind: Some(CompletionItemKind::ENUM_MEMBER),
                ..Default::default()
            },
            CompletionItem {
                label: "center".to_string(),
                kind: Some(CompletionItemKind::ENUM_MEMBER),
                ..Default::default()
            },
            CompletionItem {
                label: "end".to_string(),
                kind: Some(CompletionItemKind::ENUM_MEMBER),
                ..Default::default()
            },
            CompletionItem {
                label: "stretch".to_string(),
                kind: Some(CompletionItemKind::ENUM_MEMBER),
                ..Default::default()
            },
        ],
        "justify_content" => vec![
            CompletionItem {
                label: "start".to_string(),
                kind: Some(CompletionItemKind::ENUM_MEMBER),
                ..Default::default()
            },
            CompletionItem {
                label: "center".to_string(),
                kind: Some(CompletionItemKind::ENUM_MEMBER),
                ..Default::default()
            },
            CompletionItem {
                label: "end".to_string(),
                kind: Some(CompletionItemKind::ENUM_MEMBER),
                ..Default::default()
            },
            CompletionItem {
                label: "space_between".to_string(),
                kind: Some(CompletionItemKind::ENUM_MEMBER),
                ..Default::default()
            },
            CompletionItem {
                label: "space_around".to_string(),
                kind: Some(CompletionItemKind::ENUM_MEMBER),
                ..Default::default()
            },
            CompletionItem {
                label: "space_evenly".to_string(),
                kind: Some(CompletionItemKind::ENUM_MEMBER),
                ..Default::default()
            },
        ],
        "direction" => vec![
            CompletionItem {
                label: "row".to_string(),
                kind: Some(CompletionItemKind::ENUM_MEMBER),
                ..Default::default()
            },
            CompletionItem {
                label: "column".to_string(),
                kind: Some(CompletionItemKind::ENUM_MEMBER),
                ..Default::default()
            },
            CompletionItem {
                label: "row_reverse".to_string(),
                kind: Some(CompletionItemKind::ENUM_MEMBER),
                ..Default::default()
            },
            CompletionItem {
                label: "column_reverse".to_string(),
                kind: Some(CompletionItemKind::ENUM_MEMBER),
                ..Default::default()
            },
        ],
        _ => {
            // Check for color attributes
            if attr.contains("color") || attr == "background" || attr == "fill" || attr == "stroke"
            {
                vec![
                    CompletionItem {
                        label: "#000000".to_string(),
                        kind: Some(CompletionItemKind::COLOR),
                        ..Default::default()
                    },
                    CompletionItem {
                        label: "#FFFFFF".to_string(),
                        kind: Some(CompletionItemKind::COLOR),
                        ..Default::default()
                    },
                    CompletionItem {
                        label: "transparent".to_string(),
                        kind: Some(CompletionItemKind::COLOR),
                        ..Default::default()
                    },
                ]
            } else {
                vec![]
            }
        }
    }
}

fn widget_kind_from_str(name: &str) -> Option<WidgetKind> {
    match name {
        "column" => Some(WidgetKind::Column),
        "row" => Some(WidgetKind::Row),
        "container" => Some(WidgetKind::Container),
        "scrollable" => Some(WidgetKind::Scrollable),
        "stack" => Some(WidgetKind::Stack),
        "text" => Some(WidgetKind::Text),
        "image" => Some(WidgetKind::Image),
        "svg" => Some(WidgetKind::Svg),
        "button" => Some(WidgetKind::Button),
        "text_input" => Some(WidgetKind::TextInput),
        "checkbox" => Some(WidgetKind::Checkbox),
        "slider" => Some(WidgetKind::Slider),
        "pick_list" => Some(WidgetKind::PickList),
        "toggler" => Some(WidgetKind::Toggler),
        "space" => Some(WidgetKind::Space),
        "rule" => Some(WidgetKind::Rule),
        "radio" => Some(WidgetKind::Radio),
        "combobox" => Some(WidgetKind::ComboBox),
        "progress_bar" => Some(WidgetKind::ProgressBar),
        "tooltip" => Some(WidgetKind::Tooltip),
        "grid" => Some(WidgetKind::Grid),
        "canvas" => Some(WidgetKind::Canvas),
        "rect" => Some(WidgetKind::CanvasRect),
        "circle" => Some(WidgetKind::CanvasCircle),
        "line" => Some(WidgetKind::CanvasLine),
        "canvas_text" => Some(WidgetKind::CanvasText),
        "group" => Some(WidgetKind::CanvasGroup),
        "date_picker" => Some(WidgetKind::DatePicker),
        "time_picker" => Some(WidgetKind::TimePicker),
        "color_picker" => Some(WidgetKind::ColorPicker),
        "menu" => Some(WidgetKind::Menu),
        "menu_item" => Some(WidgetKind::MenuItem),
        "menu_separator" => Some(WidgetKind::MenuSeparator),
        "context_menu" => Some(WidgetKind::ContextMenu),
        "float" => Some(WidgetKind::Float),
        "data_table" => Some(WidgetKind::DataTable),
        "data_column" => Some(WidgetKind::DataColumn),
        "tree_view" => Some(WidgetKind::TreeView),
        "tree_node" => Some(WidgetKind::TreeNode),
        "for" => Some(WidgetKind::For),
        "if" => Some(WidgetKind::If),
        _ => Some(WidgetKind::Custom(name.to_string())),
    }
}

//! Widget and attribute documentation data.
//!
//! Provides documentation strings for hover functionality.

#![allow(dead_code)]

use std::collections::HashMap;

use once_cell::sync::Lazy;

/// Documentation for all widgets.
pub static WIDGET_DOCUMENTATION: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
    let mut docs = HashMap::new();

    docs.insert(
        "column",
        "# Column Widget\n\nA vertical layout container that arranges children in a column.",
    );
    docs.insert(
        "row",
        "# Row Widget\n\nA horizontal layout container that arranges children in a row.",
    );
    docs.insert(
        "container",
        "# Container Widget\n\nA generic container widget with padding and styling options.",
    );
    docs.insert("text", "# Text Widget\n\nDisplays text content.\n\n**Required Attributes:**\n- `value`: The text to display");
    docs.insert("button", "# Button Widget\n\nAn interactive button that can trigger events.\n\n**Optional Attributes:**\n- `label`: Button text\n- `enabled`: Whether the button is clickable");
    docs.insert("image", "# Image Widget\n\nDisplays an image.\n\n**Required Attributes:**\n- `src`: Path to the image file");
    docs.insert(
        "text_input",
        "# TextInput Widget\n\nA text input field for user input.",
    );
    docs.insert(
        "checkbox",
        "# Checkbox Widget\n\nA checkbox for boolean input.",
    );
    docs.insert(
        "slider",
        "# Slider Widget\n\nA slider for numeric input within a range.",
    );
    docs.insert(
        "scrollable",
        "# Scrollable Widget\n\nA container that allows scrolling when content overflows.",
    );
    docs.insert(
        "stack",
        "# Stack Widget\n\nA container that stacks children on top of each other.",
    );
    docs.insert(
        "pick_list",
        "# PickList Widget\n\nA dropdown list for selecting from options.",
    );
    docs.insert(
        "toggler",
        "# Toggler Widget\n\nA toggle switch for boolean input.",
    );
    docs.insert(
        "space",
        "# Space Widget\n\nAn empty widget that takes up space in layouts.",
    );
    docs.insert(
        "rule",
        "# Rule Widget\n\nA horizontal or vertical divider line.",
    );
    docs.insert(
        "radio",
        "# Radio Widget\n\nA radio button for single selection from a group.",
    );
    docs.insert(
        "combobox",
        "# ComboBox Widget\n\nA combination of text input and dropdown list.",
    );
    docs.insert(
        "progress_bar",
        "# ProgressBar Widget\n\nDisplays progress as a horizontal bar.",
    );
    docs.insert(
        "tooltip",
        "# Tooltip Widget\n\nShows a tooltip message when hovering over its child.",
    );
    docs.insert(
        "grid",
        "# Grid Widget\n\nA container that arranges children in a grid layout.",
    );
    docs.insert(
        "canvas",
        "# Canvas Widget\n\nA 2D drawing canvas for custom graphics.",
    );
    docs.insert("svg", "# Svg Widget\n\nDisplays an SVG image.");
    docs.insert(
        "date_picker",
        "# DatePicker Widget\n\nA widget for selecting dates.",
    );
    docs.insert(
        "time_picker",
        "# TimePicker Widget\n\nA widget for selecting times.",
    );
    docs.insert(
        "color_picker",
        "# ColorPicker Widget\n\nA widget for selecting colors.",
    );
    docs.insert("menu", "# Menu Widget\n\nA dropdown menu container.");
    docs.insert("menu_item", "# MenuItem Widget\n\nAn individual menu item.");
    docs.insert(
        "menu_separator",
        "# MenuSeparator Widget\n\nA horizontal separator line in a menu.",
    );
    docs.insert(
        "context_menu",
        "# ContextMenu Widget\n\nA context menu that appears on right-click.",
    );
    docs.insert(
        "float",
        "# Float Widget\n\nA floating container that can be positioned freely.",
    );
    docs.insert(
        "data_table",
        "# DataTable Widget\n\nA table for displaying tabular data.",
    );
    docs.insert(
        "data_column",
        "# DataColumn Widget\n\nA column definition for DataTable.",
    );
    docs.insert(
        "tree_view",
        "# TreeView Widget\n\nA hierarchical tree view widget.",
    );
    docs.insert("tree_node", "# TreeNode Widget\n\nA node in a TreeView.");

    docs
});

/// Gets documentation for a widget.
pub fn get_widget_documentation(name: &str) -> Option<&str> {
    WIDGET_DOCUMENTATION.get(name).copied()
}

/// Gets documentation for a widget attribute.
pub fn get_attribute_documentation(_widget: &str, _attribute: &str) -> Option<&'static str> {
    // TODO: Implement attribute documentation lookup
    None
}

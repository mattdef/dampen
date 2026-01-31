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
        "# Column Widget\n\n\
A vertical layout container that arranges children in a column.\n\n\
## Description\n\n\
The `column` widget creates a vertical stack of child widgets, placing them one below another. \
It's one of the fundamental layout containers in Dampen.\n\n\
## Attributes\n\n\
**Layout Attributes:**\n\
- `spacing` - Space between children (e.g., \"10px\")\n\
- `align_items` - Horizontal alignment: \"start\", \"center\", \"end\", \"stretch\"\n\
- `padding` - Inner padding (e.g., \"20px\" or \"10px 20px\")\n\
- `width`, `height` - Dimensions\n\n\
**Style Attributes:**\n\
- `background` - Background color (hex or CSS color name)\n\
- `border_radius` - Corner rounding\n\n\
## Example\n\n\
```xml\n\
<column spacing=\"10px\" padding=\"20px\" align_items=\"center\">\n\
    <text value=\"First Item\"/>\n\
    <text value=\"Second Item\"/>\n\
    <button label=\"Click Me\"/>\n\
</column>\n\
```\n\n\
## See Also\n\n\
- `row` - Horizontal layout counterpart\n\
- `container` - Generic single-child container",
    );

    docs.insert(
        "row",
        "# Row Widget\n\n\
A horizontal layout container that arranges children in a row.\n\n\
## Description\n\n\
The `row` widget creates a horizontal arrangement of child widgets, placing them side by side. \
Use it for toolbars, button groups, or any horizontal layout needs.\n\n\
## Attributes\n\n\
**Layout Attributes:**\n\
- `spacing` - Space between children\n\
- `align_items` - Vertical alignment: \"start\", \"center\", \"end\", \"stretch\"\n\
- `justify_content` - Horizontal distribution: \"start\", \"center\", \"end\", \"space_between\", \"space_around\"\n\
- `padding` - Inner padding\n\
- `width`, `height` - Dimensions\n\n\
**Style Attributes:**\n\
- `background` - Background color\n\
- `border_radius` - Corner rounding\n\n\
## Example\n\n\
```xml\n\
<row spacing=\"10px\" align_items=\"center\">\n\
    <button label=\"Save\"/>\n\
    <button label=\"Cancel\"/>\n\
    <button label=\"Delete\"/>\n\
</row>\n\
```\n\n\
## See Also\n\n\
- `column` - Vertical layout counterpart\n\
- `stack` - Overlapping layout",
    );

    docs.insert(
        "container",
        "# Container Widget\n\n\
A generic container widget with padding and styling options.\n\n\
## Description\n\n\
The `container` widget wraps a single child and applies styling, padding, and alignment. \
It's useful for adding visual styling or spacing around content.\n\n\
## Attributes\n\n\
**Layout Attributes:**\n\
- `padding` - Inner padding around child\n\
- `align_x` - Horizontal alignment: \"left\", \"center\", \"right\"\n\
- `align_y` - Vertical alignment: \"top\", \"center\", \"bottom\"\n\
- `width`, `height` - Container dimensions\n\
- `max_width`, `max_height` - Maximum dimensions\n\n\
**Style Attributes:**\n\
- `background` - Background color\n\
- `border_color`, `border_width`, `border_radius` - Border styling\n\
- `shadow` - Box shadow effect\n\n\
## Example\n\n\
```xml\n\
<container \n\
    padding=\"20px\"\n\
    background=\"#f0f0f0\"\n\
    border_radius=\"8px\"\n\
    align_x=\"center\"\n\
    align_y=\"center\">\n\
    <text value=\"Centered Content\"/>\n\
</container>\n\
```\n\n\
## See Also\n\n\
- `column`, `row` - Multi-child containers\n\
- `scrollable` - Scrollable container",
    );

    docs.insert(
        "text",
        "# Text Widget\n\n\
Displays text content with customizable styling.\n\n\
## Description\n\n\
The `text` widget renders a string of text with support for styling, sizing, and color customization.\n\n\
## Required Attributes\n\n\
- `value` - The text content to display\n\n\
## Optional Attributes\n\n\
- `size` - Font size (e.g., \"16px\", \"1.2em\")\n\
- `weight` - Font weight: \"normal\", \"bold\", \"100\"-\"900\"\n\
- `color` - Text color (hex or CSS color name)\n\
- `font_family` - Font family name\n\
- `line_height` - Line height multiplier\n\
- `text_align` - Text alignment: \"left\", \"center\", \"right\", \"justify\"\n\n\
## Style Attributes\n\n\
- `background` - Background color behind text\n\
- `opacity` - Transparency (0.0 - 1.0)\n\n\
## Example\n\n\
```xml\n\
<text \n\
    value=\"Hello, World!\"\n\
    size=\"24px\"\n\
    weight=\"bold\"\n\
    color=\"#333333\"/>\n\
```\n\n\
## See Also\n\n\
- `text_input` - Editable text field",
    );

    docs.insert(
        "button",
        "# Button Widget\n\n\
An interactive button that can trigger events.\n\n\
## Description\n\n\
The `button` widget creates a clickable button that can display text and respond to user interactions.\n\n\
## Optional Attributes\n\n\
- `label` - Button text label\n\
- `enabled` - Whether the button is clickable (true/false)\n\n\
## Event Attributes\n\n\
- `on_click` - Triggered when button is clicked\n\
- `on_press` - Triggered when button is pressed down\n\
- `on_release` - Triggered when button is released\n\n\
## Style Attributes\n\n\
- `background` - Button background color\n\
- `color` - Text color\n\
- `border_radius` - Corner rounding\n\
- `padding` - Inner padding\n\
- `width`, `height` - Button dimensions\n\n\
## Example\n\n\
```xml\n\
<button \n\
    label=\"Click Me\"\n\
    enabled=\"true\"\n\
    on_click=\"handle_click\"\n\
    background=\"#007bff\"\n\
    color=\"white\"/>\n\
```\n\n\
## See Also\n\n\
- `text_input` - Text input field\n\
- `checkbox` - Boolean toggle",
    );

    docs.insert(
        "image",
        "# Image Widget\n\n\
Displays an image from a file or URL.\n\n\
## Description\n\n\
The `image` widget renders an image file. Supported formats depend on the backend (typically PNG, JPG, SVG).\n\n\
## Required Attributes\n\n\
- `src` - Path to the image file (relative or absolute)\n\n\
## Optional Attributes\n\n\
- `width` - Display width\n\
- `height` - Display height\n\
- `fit` - How image fits: \"fill\", \"contain\", \"cover\", \"scale_down\", \"none\"\n\
- `filter_method` - Scaling filter: \"nearest\", \"linear\"\n\n\
## Style Attributes\n\n\
- `opacity` - Image transparency\n\
- `border_radius` - Corner clipping\n\n\
## Example\n\n\
```xml\n\
<image \n\
    src=\"assets/logo.png\"\n\
    width=\"200px\"\n\
    height=\"auto\"\n\
    fit=\"contain\"/>\n\
```\n\n\
## See Also\n\n\
- `svg` - SVG image display",
    );

    docs.insert(
        "text_input",
        "# TextInput Widget\n\n\
A text input field for user input.\n\n\
## Description\n\n\
The `text_input` widget provides an editable text field where users can type and edit text.\n\n\
## Optional Attributes\n\n\
- `placeholder` - Hint text shown when empty\n\
- `value` - Initial text value\n\
- `password` - Mask input as password (true/false)\n\
- `icon` - Icon to display in the input\n\
- `size` - Font size\n\n\
## Event Attributes\n\n\
- `on_input` - Triggered on every keystroke\n\
- `on_submit` - Triggered when user presses Enter\n\
- `on_change` - Triggered when value changes\n\
- `on_paste` - Triggered when text is pasted\n\n\
## Style Attributes\n\n\
- `background` - Input background\n\
- `color` - Text color\n\
- `border_color`, `border_width`, `border_radius` - Border styling\n\n\
## Example\n\n\
```xml\n\
<text_input \n\
    placeholder=\"Enter your name...\"\n\
    value=\"{|user_name|}\"\n\
    on_input=\"update_name\"\n\
    size=\"16px\"/>\n\
```\n\n\
## See Also\n\n\
- `text` - Display-only text\n\
- `checkbox` - Boolean input",
    );

    docs.insert(
        "checkbox",
        "# Checkbox Widget\n\n\
A checkbox for boolean input.\n\n\
## Description\n\n\
The `checkbox` widget provides a toggleable checkbox with an optional label.\n\n\
## Optional Attributes\n\n\
- `checked` - Initial checked state (true/false)\n\
- `label` - Text label next to checkbox\n\
- `icon` - Custom checkmark icon\n\
- `size` - Checkbox size\n\n\
## Event Attributes\n\n\
- `on_toggle` - Triggered when checked state changes\n\n\
## Style Attributes\n\n\
- `background` - Checkbox background\n\
- `color` - Checkmark color\n\
- `border_color`, `border_radius` - Border styling\n\n\
## Example\n\n\
```xml\n\
<checkbox \n\
    checked=\"{|is_enabled|}\"\n\
    label=\"Enable notifications\"\n\
    on_toggle=\"toggle_notifications\"/>\n\
```\n\n\
## See Also\n\n\
- `toggler` - Toggle switch variant\n\
- `radio` - Single-select option",
    );

    docs.insert(
        "slider",
        "# Slider Widget\n\n\
A slider for numeric input within a range.\n\n\
## Description\n\n\
The `slider` widget provides a draggable slider for selecting numeric values within a defined range.\n\n\
## Optional Attributes\n\n\
- `min` - Minimum value (default: 0)\n\
- `max` - Maximum value (default: 100)\n\
- `value` - Current value\n\
- `step` - Step increment for discrete values\n\n\
## Event Attributes\n\n\
- `on_change` - Triggered when value changes\n\
- `on_release` - Triggered when user releases the slider\n\n\
## Style Attributes\n\n\
- `background` - Track background\n\
- `color` - Thumb/handle color\n\n\
## Example\n\n\
```xml\n\
<slider \n\
    min=\"0\"\n\
    max=\"100\"\n\
    value=\"{|volume|}\"\n\
    step=\"1\"\n\
    on_change=\"update_volume\"/>\n\
```\n\n\
## See Also\n\n\
- `progress_bar` - Non-interactive progress indicator",
    );

    docs.insert(
        "scrollable",
        "# Scrollable Widget\n\n\
A container that allows scrolling when content overflows.\n\n\
## Description\n\n\
The `scrollable` widget wraps content and enables scrolling when the content exceeds the available space.\n\n\
## Optional Attributes\n\n\
- `direction` - Scroll direction: \"vertical\", \"horizontal\", \"both\"\n\n\
## Event Attributes\n\n\
- `on_scroll` - Triggered when scroll position changes\n\n\
## Style Attributes\n\n\
- All standard layout and style attributes\n\n\
## Example\n\n\
```xml\n\
<scrollable direction=\"vertical\" height=\"400px\">\n\
    <column spacing=\"10px\">\n\
        <text value=\"Item 1\"/>\n\
        <text value=\"Item 2\"/>\n\
        <!-- Many more items... -->\n\
    </column>\n\
</scrollable>\n\
```\n\n\
## See Also\n\n\
- `container` - Non-scrolling container\n\
- `column` - Vertical layout",
    );

    docs.insert(
        "stack",
        "# Stack Widget\n\n\
A container that stacks children on top of each other.\n\n\
## Description\n\n\
The `stack` widget overlays its children, with later children appearing on top of earlier ones. \
Useful for creating layered UIs.\n\n\
## Attributes\n\n\
- Standard layout attributes\n\n\
## Example\n\n\
```xml\n\
<stack width=\"300px\" height=\"200px\">\n\
    <image src=\"background.png\"/>\n\
    <container align_x=\"center\" align_y=\"center\">\n\
        <text value=\"Overlay Text\"/>\n\
    </container>\n\
</stack>\n\
```\n\n\
## See Also\n\n\
- `column`, `row` - Sequential layouts\n\
- `float` - Positioned overlay",
    );

    docs.insert(
        "pick_list",
        "# PickList Widget\n\n\
A dropdown list for selecting from options.\n\n\
## Description\n\n\
The `pick_list` widget provides a dropdown menu where users can select one option from a list.\n\n\
## Optional Attributes\n\n\
- `placeholder` - Text shown when no selection\n\
- `selected` - Currently selected value\n\
- `options` - List of available options\n\n\
## Event Attributes\n\n\
- `on_select` - Triggered when selection changes\n\n\
## Style Attributes\n\n\
- All standard layout and style attributes\n\n\
## Example\n\n\
```xml\n\
<pick_list \n\
    placeholder=\"Select a color...\"\n\
    selected=\"{|selected_color|}\"\n\
    options=\"{|colors|}\"\n\
    on_select=\"update_color\"/>\n\
```\n\n\
## See Also\n\n\
- `combobox` - Editable dropdown\n\
- `radio` - Radio button selection",
    );

    docs.insert(
        "toggler",
        "# Toggler Widget\n\n\
A toggle switch for boolean input.\n\n\
## Description\n\n\
The `toggler` widget provides a sliding toggle switch, an alternative to checkbox for boolean values.\n\n\
## Optional Attributes\n\n\
- `checked` - Initial state (true/false)\n\
- `active` - Alias for checked\n\
- `toggled` - Current state binding\n\
- `label` - Text label\n\n\
## Event Attributes\n\n\
- `on_toggle` - Triggered when state changes\n\n\
## Style Attributes\n\n\
- All standard layout and style attributes\n\n\
## Example\n\n\
```xml\n\
<toggler \n\
    checked=\"{|dark_mode|}\"\n\
    label=\"Dark Mode\"\n\
    on_toggle=\"toggle_theme\"/>\n\
```\n\n\
## See Also\n\n\
- `checkbox` - Checkbox variant\n\
- `radio` - Single-select option",
    );

    docs.insert(
        "space",
        "# Space Widget\n\n\
An empty widget that takes up space in layouts.\n\n\
## Description\n\n\
The `space` widget is invisible and simply occupies the specified dimensions. \
Useful for creating gaps or pushing content to edges.\n\n\
## Optional Attributes\n\n\
- `width` - Horizontal space\n\
- `height` - Vertical space\n\n\
## Example\n\n\
```xml\n\
<row>\n\
    <button label=\"Left\"/>\n\
    <space width=\"fill\"/>\n\
    <button label=\"Right\"/>\n\
</row>\n\
```\n\n\
## See Also\n\n\
- `container` - Visible wrapper\n\
- `rule` - Visual separator",
    );

    docs.insert(
        "rule",
        "# Rule Widget\n\n\
A horizontal or vertical divider line.\n\n\
## Description\n\n\
The `rule` widget draws a line, useful for visually separating content sections.\n\n\
## Optional Attributes\n\n\
- `direction` - \"horizontal\" or \"vertical\"\n\n\
## Style Attributes\n\n\
- `color` - Line color\n\
- `border_width` - Line thickness\n\n\
## Example\n\n\
```xml\n\
<column spacing=\"20px\">\n\
    <text value=\"Section 1\"/>\n\
    <rule/>\n\
    <text value=\"Section 2\"/>\n\
</column>\n\
```\n\n\
## See Also\n\n\
- `space` - Invisible spacing\n\
- `container` - Visual grouping",
    );

    docs.insert(
        "radio",
        "# Radio Widget\n\n\
A radio button for single selection from a group.\n\n\
## Description\n\n\
The `radio` widget provides a radio button for selecting one option from a mutually exclusive group.\n\n\
## Required Attributes\n\n\
- `label` - Display text\n\
- `value` - The value this radio represents\n\n\
## Optional Attributes\n\n\
- `id` - Unique identifier\n\
- `selected` - Whether this radio is selected\n\
- `disabled` - Whether this radio is disabled\n\
- `size` - Radio button size\n\
- `text_size` - Label text size\n\n\
## Event Attributes\n\n\
- `on_select` - Triggered when selected\n\n\
## Style Attributes\n\n\
- All standard layout and style attributes\n\n\
## Example\n\n\
```xml\n\
<column>\n\
    <radio label=\"Option A\" value=\"a\" selected=\"{|selection == 'a'|}\"/>\n\
    <radio label=\"Option B\" value=\"b\" selected=\"{|selection == 'b'|}\"/>\n\
</column>\n\
```\n\n\
## See Also\n\n\
- `checkbox` - Multi-select toggle\n\
- `pick_list` - Dropdown selection",
    );

    docs.insert(
        "combobox",
        "# ComboBox Widget\n\n\
A combination of text input and dropdown list.\n\n\
## Description\n\n\
The `combobox` widget combines a text input with a dropdown, allowing users to either type a value or select from predefined options.\n\n\
## Optional Attributes\n\n\
- `placeholder` - Hint text\n\
- `value` - Current text value\n\
- `selected` - Currently selected option\n\
- `options` - Available options list\n\n\
## Event Attributes\n\n\
- `on_input` - Triggered when text changes\n\
- `on_select` - Triggered when option selected\n\n\
## Style Attributes\n\n\
- All standard layout and style attributes\n\n\
## Example\n\n\
```xml\n\
<combobox \n\
    placeholder=\"Type or select...\"\n\
    value=\"{|search_text|}\"\n\
    options=\"{|suggestions|}\"\n\
    on_input=\"update_search\"\n\
    on_select=\"select_option\"/>\n\
```\n\n\
## See Also\n\n\
- `pick_list` - Dropdown only\n\
- `text_input` - Text only",
    );

    docs.insert(
        "progress_bar",
        "# ProgressBar Widget\n\n\
Displays progress as a horizontal bar.\n\n\
## Description\n\n\
The `progress_bar` widget shows a visual representation of progress, typically from 0% to 100%.\n\n\
## Optional Attributes\n\n\
- `value` - Current progress value\n\
- `min` - Minimum value (default: 0)\n\
- `max` - Maximum value (default: 100)\n\
- `style` - Visual style variant\n\n\
## Style Attributes\n\n\
- `background` - Track background\n\
- `color` - Progress fill color\n\n\
## Example\n\n\
```xml\n\
<progress_bar \n\
    value=\"{|download_progress|}\"\n\
    min=\"0\"\n\
    max=\"100\"\n\
    color=\"#4CAF50\"/>\n\
```\n\n\
## See Also\n\n\
- `slider` - Interactive value selector",
    );

    docs.insert(
        "tooltip",
        "# Tooltip Widget\n\n\
Shows a tooltip message when hovering over its child.\n\n\
## Description\n\n\
The `tooltip` widget wraps another widget and displays a popup message when the user hovers over it.\n\n\
## Optional Attributes\n\n\
- `message` - Tooltip text content\n\
- `position` - Tooltip position: \"top\", \"bottom\", \"left\", \"right\"\n\
- `delay` - Delay before showing (in milliseconds)\n\n\
## Event Attributes\n\n\
- All standard events from child\n\n\
## Example\n\n\
```xml\n\
<tooltip message=\"Click to save\" position=\"top\">\n\
    <button label=\"Save\" on_click=\"save\"/>\n\
</tooltip>\n\
```\n\n\
## See Also\n\n\
- `container` - Visual wrapper\n\
- `float` - Positioned overlay",
    );

    docs.insert(
        "grid",
        "# Grid Widget\n\n\
A container that arranges children in a grid layout.\n\n\
## Description\n\n\
The `grid` widget creates a two-dimensional grid layout with rows and columns.\n\n\
## Optional Attributes\n\n\
- `columns` - Number of columns\n\n\
## Event Attributes\n\n\
- All standard events\n\n\
## Style Attributes\n\n\
- All standard layout and style attributes\n\n\
## Example\n\n\
```xml\n\
<grid columns=\"3\" spacing=\"10px\">\n\
    <text value=\"Cell 1\"/>\n\
    <text value=\"Cell 2\"/>\n\
    <text value=\"Cell 3\"/>\n\
    <text value=\"Cell 4\"/>\n\
    <text value=\"Cell 5\"/>\n\
    <text value=\"Cell 6\"/>\n\
</grid>\n\
```\n\n\
## See Also\n\n\
- `column`, `row` - Linear layouts\n\
- `stack` - Overlapping layout",
    );

    docs.insert(
        "canvas",
        "# Canvas Widget\n\n\
A 2D drawing canvas for custom graphics.\n\n\
## Description\n\n\
The `canvas` widget provides a drawing surface for custom 2D graphics using shapes like rectangles, circles, lines, and text.\n\n\
## Optional Attributes\n\n\
- `width` - Canvas width\n\
- `height` - Canvas height\n\
- `program` - Drawing program/state\n\
- `cache` - Whether to cache the canvas\n\n\
## Event Attributes\n\n\
- `on_click` - Click on canvas\n\
- `on_drag` - Drag on canvas\n\
- `on_move` - Mouse move on canvas\n\
- `on_release` - Mouse release on canvas\n\n\
## Style Attributes\n\n\
- All standard layout attributes\n\n\
## Child Elements\n\n\
- `rect` - Rectangle shape\n\
- `circle` - Circle shape\n\
- `line` - Line shape\n\
- `canvas_text` - Text in canvas\n\
- `group` - Group of shapes\n\n\
## Example\n\n\
```xml\n\
<canvas width=\"400\" height=\"300\">\n\
    <rect x=\"10\" y=\"10\" width=\"100\" height=\"50\" fill=\"red\"/>\n\
    <circle cx=\"200\" cy=\"150\" radius=\"50\" fill=\"blue\"/>\n\
</canvas>\n\
```\n\n\
## See Also\n\n\
- `image` - Static image display\n\
- `svg` - SVG graphics",
    );

    docs.insert(
        "svg",
        "# Svg Widget\n\n\
Displays an SVG image.\n\n\
## Description\n\n\
The `svg` widget renders Scalable Vector Graphics files.\n\n\
## Required Attributes\n\n\
- `src` - Path to SVG file\n\n\
## Optional Attributes\n\n\
- `width` - Display width\n\
- `height` - Display height\n\
- `path` - Alternative to src for inline SVG\n\n\
## Style Attributes\n\n\
- All standard layout attributes\n\n\
## Example\n\n\
```xml\n\
<svg src=\"assets/icon.svg\" width=\"64px\" height=\"64px\"/>\n\
```\n\n\
## See Also\n\n\
- `image` - Raster image display\n\
- `canvas` - Custom drawing",
    );

    docs.insert(
        "date_picker",
        "# DatePicker Widget\n\n\
A widget for selecting dates.\n\n\
## Description\n\n\
The `date_picker` widget provides a calendar interface for selecting dates.\n\n\
## Optional Attributes\n\n\
- `value` - Selected date\n\
- `format` - Date format string\n\
- `show` - Whether picker is visible\n\
- `min_date` - Minimum selectable date\n\
- `max_date` - Maximum selectable date\n\n\
## Event Attributes\n\n\
- `on_submit` - Date confirmed\n\
- `on_cancel` - Selection cancelled\n\n\
## Style Attributes\n\n\
- All standard layout and style attributes\n\n\
## Example\n\n\
```xml\n\
<date_picker \n\
    value=\"{|selected_date|}\"\n\
    format=\"YYYY-MM-DD\"\n\
    on_submit=\"update_date\"/>\n\
```\n\n\
## See Also\n\n\
- `time_picker` - Time selection\n\
- `text_input` - Manual date entry",
    );

    docs.insert(
        "time_picker",
        "# TimePicker Widget\n\n\
A widget for selecting times.\n\n\
## Description\n\n\
The `time_picker` widget provides a time selection interface.\n\n\
## Optional Attributes\n\n\
- `value` - Selected time\n\
- `format` - Time format string\n\
- `show` - Whether picker is visible\n\
- `use_24h` - Use 24-hour format (true/false)\n\
- `show_seconds` - Show seconds (true/false)\n\n\
## Event Attributes\n\n\
- `on_submit` - Time confirmed\n\
- `on_cancel` - Selection cancelled\n\n\
## Style Attributes\n\n\
- All standard layout and style attributes\n\n\
## Example\n\n\
```xml\n\
<time_picker \n\
    value=\"{|selected_time|}\"\n\
    use_24h=\"true\"\n\
    show_seconds=\"false\"\n\
    on_submit=\"update_time\"/>\n\
```\n\n\
## See Also\n\n\
- `date_picker` - Date selection\n\
- `text_input` - Manual time entry",
    );

    docs.insert(
        "color_picker",
        "# ColorPicker Widget\n\n\
A widget for selecting colors.\n\n\
## Description\n\n\
The `color_picker` widget provides a color selection interface with visual picker.\n\n\
## Optional Attributes\n\n\
- `value` - Selected color (hex format)\n\
- `show` - Whether picker is visible\n\
- `show_alpha` - Show alpha channel (true/false)\n\
- `enabled` - Whether picker is enabled\n\n\
## Event Attributes\n\n\
- `on_submit` - Color confirmed\n\
- `on_cancel` - Selection cancelled\n\
- `on_change` - Color changed (live)\n\n\
## Style Attributes\n\n\
- All standard layout and style attributes\n\n\
## Example\n\n\
```xml\n\
<color_picker \n\
    value=\"{|selected_color|}\"\n\
    show_alpha=\"true\"\n\
    on_change=\"preview_color\"\n\
    on_submit=\"apply_color\"/>\n\
```\n\n\
## See Also\n\n\
- `text_input` - Manual color entry",
    );

    docs.insert(
        "menu",
        "# Menu Widget\n\n\
A dropdown menu container.\n\n\
## Description\n\n\
The `menu` widget creates a dropdown menu that can contain menu items and separators.\n\n\
## Optional Attributes\n\n\
- `position` - Menu position\n\
- `close_on_select` - Close after selection (true/false)\n\
- `width` - Menu width\n\
- `spacing` - Spacing between items\n\
- `class` - CSS class\n\n\
## Event Attributes\n\n\
- `on_open` - Menu opened\n\
- `on_close` - Menu closed\n\n\
## Style Attributes\n\n\
- All standard layout and style attributes\n\n\
## Child Elements\n\n\
- `menu_item` - Individual menu items\n\
- `menu_separator` - Separator lines\n\n\
## Example\n\n\
```xml\n\
<menu close_on_select=\"true\">\n\
    <menu_item label=\"New\" on_click=\"new_file\"/>\n\
    <menu_item label=\"Open\" on_click=\"open_file\"/>\n\
    <menu_separator/>\n\
    <menu_item label=\"Exit\" on_click=\"exit\"/>\n\
</menu>\n\
```\n\n\
## See Also\n\n\
- `context_menu` - Right-click menu\n\
- `pick_list` - Single selection dropdown",
    );

    docs.insert(
        "menu_item",
        "# MenuItem Widget\n\n\
An individual menu item.\n\n\
## Description\n\n\
The `menu_item` widget represents a single selectable item within a menu.\n\n\
## Required Attributes\n\n\
- `label` - Display text\n\n\
## Optional Attributes\n\n\
- `icon` - Icon to display\n\
- `shortcut` - Keyboard shortcut text\n\
- `disabled` - Whether item is disabled\n\
- `class` - CSS class\n\n\
## Event Attributes\n\n\
- `on_click` - Item clicked\n\n\
## Style Attributes\n\n\
- All standard layout and style attributes\n\n\
## Example\n\n\
```xml\n\
<menu_item \n\
    label=\"Save\"\n\
    shortcut=\"Ctrl+S\"\n\
    on_click=\"save_file\"/>\n\
```\n\n\
## See Also\n\n\
- `menu` - Menu container\n\
- `menu_separator` - Menu divider",
    );

    docs.insert(
        "menu_separator",
        "# MenuSeparator Widget\n\n\
A horizontal separator line in a menu.\n\n\
## Description\n\n\
The `menu_separator` widget draws a horizontal line to visually separate groups of menu items.\n\n\
## Style Attributes\n\n\
- `color` - Line color\n\
- `opacity` - Line opacity\n\n\
## Layout Attributes\n\n\
- `height` - Line thickness\n\n\
## Example\n\n\
```xml\n\
<menu>\n\
    <menu_item label=\"Cut\"/>\n\
    <menu_item label=\"Copy\"/>\n\
    <menu_item label=\"Paste\"/>\n\
    <menu_separator/>\n\
    <menu_item label=\"Select All\"/>\n\
</menu>\n\
```\n\n\
## See Also\n\n\
- `menu` - Menu container\n\
- `menu_item` - Menu item\n\
- `rule` - General separator",
    );

    docs.insert(
        "context_menu",
        "# ContextMenu Widget\n\n\
A context menu that appears on right-click.\n\n\
## Description\n\n\
The `context_menu` widget provides a popup menu that appears when the user right-clicks on its child widget.\n\n\
## Optional Attributes\n\n\
- `context` - Context data for the menu\n\n\
## Event Attributes\n\n\
- `on_open` - Menu opened\n\
- `on_close` - Menu closed\n\n\
## Style Attributes\n\n\
- All standard layout and style attributes\n\n\
## Child Elements\n\n\
- `menu_item` - Menu items\n\
- `menu_separator` - Separators\n\n\
## Example\n\n\
```xml\n\
<context_menu>\n\
    <menu_item label=\"Copy\" on_click=\"copy\"/>\n\
    <menu_item label=\"Paste\" on_click=\"paste\"/>\n\
</context_menu>\n\
```\n\n\
## See Also\n\n\
- `menu` - Regular dropdown menu\n\
- `tooltip` - Hover popup",
    );

    docs.insert(
        "float",
        "# Float Widget\n\n\
A floating container that can be positioned freely.\n\n\
## Description\n\n\
The `float` widget creates a positioned overlay that floats above other content. \
Useful for modals, popups, and positioned elements.\n\n\
## Attributes\n\n\
- Standard layout attributes\n\n\
## Example\n\n\
```xml\n\
<float top=\"100px\" left=\"100px\">\n\
    <container padding=\"20px\" background=\"white\">\n\
        <text value=\"Floating Content\"/>\n\
    </container>\n\
</float>\n\
```\n\n\
## See Also\n\n\
- `stack` - Layered layout\n\
- `tooltip` - Hover popup",
    );

    docs.insert(
        "data_table",
        "# DataTable Widget\n\n\
A table for displaying tabular data.\n\n\
## Description\n\n\
The `data_table` widget displays data in a table format with sortable columns.\n\n\
## Required Attributes\n\n\
- `data` - Table data source\n\n\
## Optional Attributes\n\n\
- `width` - Table width\n\
- `height` - Table height\n\
- `min_width` - Minimum width\n\
- `max_width` - Maximum width\n\
- `scrollbar_width` - Scrollbar width\n\n\
## Event Attributes\n\n\
- `on_row_click` - Row clicked\n\n\
## Style Attributes\n\n\
- All standard layout and style attributes\n\n\
## Child Elements\n\n\
- `data_column` - Column definitions\n\n\
## Example\n\n\
```xml\n\
<data_table data=\"{|users|}\" width=\"100%\">\n\
    <data_column header=\"Name\" field=\"name\"/>\n\
    <data_column header=\"Email\" field=\"email\"/>\n\
</data_table>\n\
```\n\n\
## See Also\n\n\
- `data_column` - Column definition\n\
- `grid` - Grid layout",
    );

    docs.insert(
        "data_column",
        "# DataColumn Widget\n\n\
A column definition for DataTable.\n\n\
## Description\n\n\
The `data_column` widget defines a column within a `data_table`, specifying how data should be displayed.\n\n\
## Required Attributes\n\n\
- `header` - Column header text\n\n\
## Optional Attributes\n\n\
- `field` - Data field name\n\
- `width` - Column width\n\
- `min_width` - Minimum column width\n\
- `max_width` - Maximum column width\n\
- `align` - Text alignment: \"left\", \"center\", \"right\"\n\n\
## Example\n\n\
```xml\n\
<data_column \n\
    header=\"Price\"\n\
    field=\"price\"\n\
    width=\"100px\"\n\
    align=\"right\"/>\n\
```\n\n\
## See Also\n\n\
- `data_table` - Table container\n\
- `text` - Text display",
    );

    docs.insert(
        "tree_view",
        "# TreeView Widget\n\n\
A hierarchical tree view widget.\n\n\
## Description\n\n\
The `tree_view` widget displays hierarchical data in an expandable tree structure.\n\n\
## Optional Attributes\n\n\
- `nodes` - Tree node data\n\
- `expanded` - Expanded node IDs\n\
- `selected` - Selected node ID\n\
- `indent_size` - Indentation per level\n\
- `node_height` - Height of each node\n\
- `icon_size` - Size of expand/collapse icons\n\
- `expand_icon` - Icon for expanded nodes\n\
- `collapse_icon` - Icon for collapsed nodes\n\
- `leaf_icon` - Icon for leaf nodes\n\n\
## Event Attributes\n\n\
- `on_toggle` - Node expanded/collapsed\n\
- `on_select` - Node selected\n\
- `on_double_click` - Node double-clicked\n\n\
## Style Attributes\n\n\
- All standard layout and style attributes\n\n\
## Child Elements\n\n\
- `tree_node` - Individual tree nodes\n\n\
## Example\n\n\
```xml\n\
<tree_view \n\
    nodes=\"{|file_tree|}\"\n\
    on_select=\"select_file\"\n\
    on_toggle=\"toggle_folder\">\n\
    <!-- tree_node children generated dynamically -->\n\
</tree_view>\n\
```\n\n\
## See Also\n\n\
- `tree_node` - Tree node\n\
- `column` - Vertical layout",
    );

    docs.insert(
        "tree_node",
        "# TreeNode Widget\n\n\
A node in a TreeView.\n\n\
## Description\n\n\
The `tree_node` widget represents a single node within a tree view hierarchy.\n\n\
## Required Attributes\n\n\
- `id` - Unique node identifier\n\
- `label` - Display text\n\n\
## Optional Attributes\n\n\
- `icon` - Node icon\n\
- `expanded` - Whether node is expanded\n\
- `selected` - Whether node is selected\n\
- `disabled` - Whether node is disabled\n\
- `class` - CSS class\n\n\
## Example\n\n\
```xml\n\
<tree_node \n\
    id=\"folder1\"\n\
    label=\"Documents\"\n\
    expanded=\"true\">\n\
    <tree_node id=\"file1\" label=\"readme.txt\"/>\n\
</tree_node>\n\
```\n\n\
## See Also\n\n\
- `tree_view` - Tree container\n\
- `text` - Text display",
    );

    docs.insert(
        "rect",
        "# Rect Widget (Canvas)\n\n\
A rectangle shape for canvas drawing.\n\n\
## Description\n\n\
The `rect` widget draws a rectangle within a `canvas` element.\n\n\
## Required Attributes\n\n\
- `x` - X coordinate\n\
- `y` - Y coordinate\n\
- `width` - Rectangle width\n\
- `height` - Rectangle height\n\n\
## Optional Attributes\n\n\
- `fill` - Fill color\n\
- `stroke` - Stroke color\n\
- `stroke_width` - Stroke thickness\n\
- `radius` - Corner radius\n\n\
## Example\n\n\
```xml\n\
<canvas width=\"400\" height=\"300\">\n\
    <rect x=\"10\" y=\"10\" width=\"100\" height=\"50\" fill=\"red\"/>\n\
</canvas>\n\
```\n\n\
## See Also\n\n\
- `canvas` - Drawing surface\n\
- `circle` - Circle shape\n\
- `line` - Line shape",
    );

    docs.insert(
        "circle",
        "# Circle Widget (Canvas)\n\n\
A circle shape for canvas drawing.\n\n\
## Description\n\n\
The `circle` widget draws a circle within a `canvas` element.\n\n\
## Required Attributes\n\n\
- `cx` - Center X coordinate\n\
- `cy` - Center Y coordinate\n\
- `radius` - Circle radius\n\n\
## Optional Attributes\n\n\
- `fill` - Fill color\n\
- `stroke` - Stroke color\n\
- `stroke_width` - Stroke thickness\n\n\
## Example\n\n\
```xml\n\
<canvas width=\"400\" height=\"300\">\n\
    <circle cx=\"200\" cy=\"150\" radius=\"50\" fill=\"blue\"/>\n\
</canvas>\n\
```\n\n\
## See Also\n\n\
- `canvas` - Drawing surface\n\
- `rect` - Rectangle shape\n\
- `line` - Line shape",
    );

    docs.insert(
        "line",
        "# Line Widget (Canvas)\n\n\
A line shape for canvas drawing.\n\n\
## Description\n\n\
The `line` widget draws a straight line within a `canvas` element.\n\n\
## Required Attributes\n\n\
- `x1` - Start X coordinate\n\
- `y1` - Start Y coordinate\n\
- `x2` - End X coordinate\n\
- `y2` - End Y coordinate\n\n\
## Optional Attributes\n\n\
- `stroke` - Line color\n\
- `stroke_width` - Line thickness\n\n\
## Example\n\n\
```xml\n\
<canvas width=\"400\" height=\"300\">\n\
    <line x1=\"0\" y1=\"0\" x2=\"400\" y2=\"300\" stroke=\"black\" stroke_width=\"2\"/>\n\
</canvas>\n\
```\n\n\
## See Also\n\n\
- `canvas` - Drawing surface\n\
- `rect` - Rectangle shape\n\
- `circle` - Circle shape",
    );

    docs.insert(
        "canvas_text",
        "# CanvasText Widget (Canvas)\n\n\
Text rendering for canvas drawing.\n\n\
## Description\n\n\
The `canvas_text` widget renders text within a `canvas` element.\n\n\
## Required Attributes\n\n\
- `x` - X coordinate\n\
- `y` - Y coordinate\n\
- `content` - Text content\n\n\
## Optional Attributes\n\n\
- `size` - Font size\n\
- `color` - Text color\n\n\
## Example\n\n\
```xml\n\
<canvas width=\"400\" height=\"300\">\n\
    <canvas_text x=\"50\" y=\"50\" content=\"Hello Canvas\" size=\"24px\" color=\"black\"/>\n\
</canvas>\n\
```\n\n\
## See Also\n\n\
- `canvas` - Drawing surface\n\
- `text` - Regular text widget",
    );

    docs.insert(
        "group",
        "# Group Widget (Canvas)\n\n\
A group of canvas shapes with shared transform.\n\n\
## Description\n\n\
The `group` widget groups multiple canvas shapes together, applying a shared transformation.\n\n\
## Optional Attributes\n\n\
- `transform` - Transformation matrix or operations\n\n\
## Child Elements\n\n\
- `rect` - Rectangle shapes\n\
- `circle` - Circle shapes\n\
- `line` - Line shapes\n\
- `canvas_text` - Text elements\n\n\
## Example\n\n\
```xml\n\
<canvas width=\"400\" height=\"300\">\n\
    <group transform=\"translate(50, 50)\">\n\
        <rect x=\"0\" y=\"0\" width=\"50\" height=\"50\" fill=\"red\"/>\n\
        <circle cx=\"75\" cy=\"25\" radius=\"20\" fill=\"blue\"/>\n\
    </group>\n\
</canvas>\n\
```\n\n\
## See Also\n\n\
- `canvas` - Drawing surface",
    );

    docs.insert(
        "for",
        "# For Widget\n\n\
A loop construct for rendering lists.\n\n\
## Description\n\n\
The `for` widget iterates over a collection and renders its children for each item.\n\n\
## Required Attributes\n\n\
- `each` - Variable name for current item\n\
- `in` - Collection to iterate over\n\n\
## Optional Attributes\n\n\
- `template` - Optional template reference\n\n\
## Example\n\n\
```xml\n\
<for each=\"item\" in=\"{|items|}\">\n\
    <text value=\"{|item.name|}\"/>\n\
</for>\n\
```\n\n\
## See Also\n\n\
- `if` - Conditional rendering\n\
- `column` - Container for list items",
    );

    docs.insert(
        "if",
        "# If Widget\n\n\
A conditional rendering construct.\n\n\
## Description\n\n\
The `if` widget conditionally renders its children based on a boolean expression.\n\n\
## Required Attributes\n\n\
- `condition` - Boolean expression or binding\n\n\
## Example\n\n\
```xml\n\
<if condition=\"{|is_logged_in|}\">\n\
    <text value=\"Welcome back!\"/>\n\
</if>\n\
```\n\n\
## See Also\n\n\
- `for` - Loop construct\n\
- Container widgets for grouping",
    );

    docs
});

/// Documentation for common attributes.
pub static ATTRIBUTE_DOCUMENTATION: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
    let mut docs = HashMap::new();

    // Layout attributes
    docs.insert(
        "id",
        "**id** - Unique identifier for the widget\n\n\
Type: `string`\n\n\
Used to reference the widget from code or apply specific styling.\n\n\
Example: `id=\"submit_button\"`",
    );

    docs.insert(
        "width",
        "**width** - Widget width\n\n\
Type: `length` (e.g., \"100px\", \"50%\", \"auto\")\n\n\
Sets the horizontal size of the widget.\n\n\
Examples:\n\
- `width=\"200px\"` - Fixed 200 pixels\n\
- `width=\"50%\"` - Half of parent width\n\
- `width=\"auto\"` - Size to content",
    );

    docs.insert(
        "height",
        "**height** - Widget height\n\n\
Type: `length` (e.g., \"100px\", \"50%\", \"auto\")\n\n\
Sets the vertical size of the widget.\n\n\
Examples:\n\
- `height=\"100px\"` - Fixed 100 pixels\n\
- `height=\"50%\"` - Half of parent height\n\
- `height=\"auto\"` - Size to content",
    );

    docs.insert(
        "min_width",
        "**min_width** - Minimum width constraint\n\n\
Type: `length`\n\n\
Prevents the widget from shrinking below this width.",
    );

    docs.insert(
        "max_width",
        "**max_width** - Maximum width constraint\n\n\
Type: `length`\n\n\
Prevents the widget from growing beyond this width.",
    );

    docs.insert(
        "min_height",
        "**min_height** - Minimum height constraint\n\n\
Type: `length`\n\n\
Prevents the widget from shrinking below this height.",
    );

    docs.insert(
        "max_height",
        "**max_height** - Maximum height constraint\n\n\
Type: `length`\n\n\
Prevents the widget from growing beyond this height.",
    );

    docs.insert(
        "padding",
        "**padding** - Inner spacing\n\n\
Type: `length` or `length[]` (e.g., \"10px\", \"10px 20px\", \"10px 20px 15px\")\n\n\
Space between widget border and content.\n\n\
Examples:\n\
- `padding=\"10px\"` - 10px on all sides\n\
- `padding=\"10px 20px\"` - 10px vertical, 20px horizontal\n\
- `padding=\"5px 10px 15px 20px\"` - Top, right, bottom, left",
    );

    docs.insert(
        "spacing",
        "**spacing** - Space between children\n\n\
Type: `length`\n\n\
Used in container widgets (column, row, menu) to set gap between children.\n\n\
Example: `spacing=\"10px\"`",
    );

    docs.insert(
        "align_items",
        "**align_items** - Cross-axis alignment for children\n\n\
Type: `enum` (\"start\", \"center\", \"end\", \"stretch\")\n\n\
Aligns children perpendicular to the main axis.\n\n\
- `start` - Align to start\n\
- `center` - Center align\n\
- `end` - Align to end\n\
- `stretch` - Stretch to fill",
    );

    docs.insert(
        "justify_content",
        "**justify_content** - Main-axis content distribution\n\n\
Type: `enum` (\"start\", \"center\", \"end\", \"space_between\", \"space_around\", \"space_evenly\")\n\n\
Distributes space along the main axis.\n\n\
- `start` - Pack at start\n\
- `center` - Pack at center\n\
- `end` - Pack at end\n\
- `space_between` - Even space between items\n\
- `space_around` - Even space around items\n\
- `space_evenly` - Truly even spacing",
    );

    docs.insert(
        "align",
        "**align** - General alignment shorthand\n\n\
Type: `enum` (\"start\", \"center\", \"end\")\n\n\
Quick alignment setting for single-axis alignment.",
    );

    docs.insert(
        "align_x",
        "**align_x** - Horizontal alignment\n\n\
Type: `enum` (\"left\", \"center\", \"right\")\n\n\
Horizontal positioning within parent.\n\n\
- `left` - Align to left\n\
- `center` - Center horizontally\n\
- `right` - Align to right",
    );

    docs.insert(
        "align_y",
        "**align_y** - Vertical alignment\n\n\
Type: `enum` (\"top\", \"center\", \"bottom\")\n\n\
Vertical positioning within parent.\n\n\
- `top` - Align to top\n\
- `center` - Center vertically\n\
- `bottom` - Align to bottom",
    );

    docs.insert(
        "align_self",
        "**align_self** - Override parent alignment for this widget\n\n\
Type: `enum` (\"start\", \"center\", \"end\", \"stretch\")\n\n\
Allows a child to override the parent's `align_items` setting.",
    );

    docs.insert(
        "direction",
        "**direction** - Layout direction\n\n\
Type: `enum` (\"row\", \"column\", \"row_reverse\", \"column_reverse\")\n\n\
Sets the direction of flex layout.\n\n\
- `row` - Left to right\n\
- `column` - Top to bottom\n\
- `row_reverse` - Right to left\n\
- `column_reverse` - Bottom to top",
    );

    docs.insert(
        "position",
        "**position** - Positioning mode\n\n\
Type: `enum` (\"static\", \"relative\", \"absolute\")\n\n\
Controls how the widget is positioned.\n\n\
- `static` - Normal flow\n\
- `relative` - Offset from normal position\n\
- `absolute` - Positioned relative to parent",
    );

    docs.insert(
        "top",
        "**top** - Top offset for positioned widgets\n\n\
Type: `length`\n\n\
Distance from top edge when using absolute positioning.",
    );

    docs.insert(
        "right",
        "**right** - Right offset for positioned widgets\n\n\
Type: `length`\n\n\
Distance from right edge when using absolute positioning.",
    );

    docs.insert(
        "bottom",
        "**bottom** - Bottom offset for positioned widgets\n\n\
Type: `length`\n\n\
Distance from bottom edge when using absolute positioning.",
    );

    docs.insert(
        "left",
        "**left** - Left offset for positioned widgets\n\n\
Type: `length`\n\n\
Distance from left edge when using absolute positioning.",
    );

    docs.insert(
        "z_index",
        "**z_index** - Stack order for overlapping widgets\n\n\
Type: `integer`\n\n\
Higher values appear on top of lower values.\n\n\
Example: `z_index=\"10\"`",
    );

    docs.insert(
        "class",
        "**class** - CSS class name(s)\n\n\
Type: `string`\n\n\
Applies styling classes to the widget. Multiple classes can be space-separated.\n\n\
Example: `class=\"btn btn-primary\"`",
    );

    docs.insert(
        "theme",
        "**theme** - Theme name\n\n\
Type: `string`\n\n\
Applies a named theme to the widget.\n\n\
Example: `theme=\"dark\"`",
    );

    docs.insert(
        "theme_ref",
        "**theme_ref** - Theme reference\n\n\
Type: `string`\n\n\
References a theme definition for styling.",
    );

    // Style attributes
    docs.insert(
        "background",
        "**background** - Background color\n\n\
Type: `color` (hex, CSS name, or rgba)\n\n\
Sets the background color of the widget.\n\n\
Examples:\n\
- `background=\"#FF5733\"` - Hex color\n\
- `background=\"red\"` - CSS color name\n\
- `background=\"rgba(255,0,0,0.5)\"` - With transparency",
    );

    docs.insert(
        "color",
        "**color** - Foreground/text color\n\n\
Type: `color`\n\n\
Sets the text or foreground color.\n\n\
Example: `color=\"#333333\"`",
    );

    docs.insert(
        "text_color",
        "**text_color** - Text color (alias for color)\n\n\
Type: `color`\n\n\
Explicit text color setting.\n\n\
Example: `text_color=\"white\"`",
    );

    docs.insert(
        "border_color",
        "**border_color** - Border color\n\n\
Type: `color`\n\n\
Sets the color of the widget border.\n\n\
Example: `border_color=\"#CCCCCC\"`",
    );

    docs.insert(
        "border_width",
        "**border_width** - Border thickness\n\n\
Type: `length`\n\n\
Sets the width of the widget border.\n\n\
Example: `border_width=\"2px\"`",
    );

    docs.insert(
        "border_radius",
        "**border_radius** - Corner rounding\n\n\
Type: `length` or `length[]`\n\n\
Rounds the corners of the widget.\n\n\
Examples:\n\
- `border_radius=\"8px\"` - Uniform rounding\n\
- `border_radius=\"8px 0\"` - Top-left/bottom-right, top-right/bottom-left\n\
- `border_radius=\"4px 8px 12px 16px\"` - Per-corner",
    );

    docs.insert(
        "border_style",
        "**border_style** - Border style\n\n\
Type: `enum` (\"solid\", \"dashed\", \"dotted\", \"none\")\n\n\
Sets the style of the border line.\n\n\
Example: `border_style=\"dashed\"`",
    );

    docs.insert(
        "shadow",
        "**shadow** - Box shadow\n\n\
Type: `shadow` (e.g., \"2px 2px 4px rgba(0,0,0,0.3)\")\n\n\
Adds a shadow effect to the widget.\n\n\
Format: `offset-x offset-y blur-radius color`\n\n\
Example: `shadow=\"0 4px 8px rgba(0,0,0,0.2)\"`",
    );

    docs.insert(
        "shadow_color",
        "**shadow_color** - Shadow color\n\n\
Type: `color`\n\n\
Sets the color of the widget shadow.\n\n\
Example: `shadow_color=\"rgba(0,0,0,0.3)\"`",
    );

    docs.insert(
        "shadow_offset",
        "**shadow_offset** - Shadow offset\n\n\
Type: `length[]` (e.g., \"2px 2px\")\n\n\
Sets the horizontal and vertical shadow offset.\n\n\
Example: `shadow_offset=\"2px 4px\"`",
    );

    docs.insert(
        "shadow_blur_radius",
        "**shadow_blur_radius** - Shadow blur amount\n\n\
Type: `length`\n\n\
Sets how much the shadow is blurred.\n\n\
Example: `shadow_blur_radius=\"8px\"`",
    );

    docs.insert(
        "opacity",
        "**opacity** - Transparency\n\n\
Type: `number` (0.0 to 1.0)\n\n\
Sets the widget transparency.\n\n\
- `0.0` - Fully transparent\n\
- `1.0` - Fully opaque\n\n\
Example: `opacity=\"0.5\"`",
    );

    docs.insert(
        "transform",
        "**transform** - Visual transformation\n\n\
Type: `transform`\n\n\
Applies visual transformations like rotation, scaling, or translation.\n\n\
Examples:\n\
- `transform=\"rotate(45deg)\"`\n\
- `transform=\"scale(1.5)\"`\n\
- `transform=\"translate(10px, 20px)\"`",
    );

    docs.insert(
        "style",
        "**style** - Inline style or style reference\n\n\
Type: `string`\n\n\
Applies inline CSS or references a style definition.\n\n\
Example: `style=\"font-weight: bold;\"`",
    );

    // Widget-specific attributes
    docs.insert(
        "value",
        "**value** - Widget value or content\n\n\
Type: `string` or `binding`\n\n\
The main content or value of the widget.\n\n\
- For `text`: The text to display\n\
- For `text_input`: The input value\n\
- For `radio`: The option value\n\n\
Examples:\n\
- `value=\"Hello World\"` - Static text\n\
- `value=\"{|user_name|}\"` - Bound value",
    );

    docs.insert(
        "label",
        "**label** - Display label\n\n\
Type: `string` or `binding`\n\n\
A text label for the widget.\n\n\
Used by: `button`, `checkbox`, `radio`, `menu_item`, `tree_node`\n\n\
Example: `label=\"Click Me\"`",
    );

    docs.insert(
        "src",
        "**src** - Source path\n\n\
Type: `string`\n\n\
Path to a resource file.\n\n\
Used by: `image`, `svg`\n\n\
Example: `src=\"assets/logo.png\"`",
    );

    docs.insert(
        "placeholder",
        "**placeholder** - Hint text\n\n\
Type: `string`\n\n\
Text displayed when a field is empty.\n\n\
Used by: `text_input`, `pick_list`, `combobox`\n\n\
Example: `placeholder=\"Enter your name...\"`",
    );

    docs.insert(
        "enabled",
        "**enabled** - Enabled state\n\n\
Type: `boolean` (true/false)\n\n\
Whether the widget is interactive.\n\n\
Example: `enabled=\"false\"`",
    );

    docs.insert(
        "checked",
        "**checked** - Checked state\n\n\
Type: `boolean` (true/false)\n\n\
Whether a checkbox or toggler is checked.\n\n\
Used by: `checkbox`, `toggler`\n\n\
Example: `checked=\"{|is_selected|}\"`",
    );

    // Event attributes
    docs.insert(
        "on_click",
        "**on_click** - Click event handler\n\n\
Type: `event_binding`\n\n\
Triggered when the widget is clicked.\n\n\
Example: `on_click=\"handle_click\"`",
    );

    docs.insert(
        "on_press",
        "**on_press** - Press event handler\n\n\
Type: `event_binding`\n\n\
Triggered when the widget is pressed down.\n\n\
Example: `on_press=\"handle_press\"`",
    );

    docs.insert(
        "on_release",
        "**on_release** - Release event handler\n\n\
Type: `event_binding`\n\n\
Triggered when the widget is released.\n\n\
Example: `on_release=\"handle_release\"`",
    );

    docs.insert(
        "on_change",
        "**on_change** - Change event handler\n\n\
Type: `event_binding`\n\n\
Triggered when the widget's value changes.\n\n\
Example: `on_change=\"handle_change\"`",
    );

    docs.insert(
        "on_input",
        "**on_input** - Input event handler\n\n\
Type: `event_binding`\n\n\
Triggered on every keystroke or input change.\n\n\
Example: `on_input=\"handle_input\"`",
    );

    docs.insert(
        "on_submit",
        "**on_submit** - Submit event handler\n\n\
Type: `event_binding`\n\n\
Triggered when the user submits (e.g., presses Enter).\n\n\
Example: `on_submit=\"handle_submit\"`",
    );

    docs.insert(
        "on_select",
        "**on_select** - Select event handler\n\n\
Type: `event_binding`\n\n\
Triggered when an item is selected.\n\n\
Example: `on_select=\"handle_select\"`",
    );

    docs.insert(
        "on_toggle",
        "**on_toggle** - Toggle event handler\n\n\
Type: `event_binding`\n\n\
Triggered when a toggleable widget changes state.\n\n\
Example: `on_toggle=\"handle_toggle\"`",
    );

    docs.insert(
        "on_scroll",
        "**on_scroll** - Scroll event handler\n\n\
Type: `event_binding`\n\n\
Triggered when the widget is scrolled.\n\n\
Example: `on_scroll=\"handle_scroll\"`",
    );

    docs.insert(
        "on_paste",
        "**on_paste** - Paste event handler\n\n\
Type: `event_binding`\n\n\
Triggered when content is pasted into the widget.\n\n\
Example: `on_paste=\"handle_paste\"`",
    );

    docs
});

/// Gets documentation for a widget.
pub fn get_widget_documentation(name: &str) -> Option<&'static str> {
    WIDGET_DOCUMENTATION.get(name).copied()
}

/// Gets documentation for an attribute.
pub fn get_attribute_documentation(_widget: &str, attribute: &str) -> Option<&'static str> {
    ATTRIBUTE_DOCUMENTATION.get(attribute).copied()
}

/// Gets a list of all documented widget names.
pub fn get_all_widget_names() -> Vec<&'static str> {
    WIDGET_DOCUMENTATION.keys().copied().collect()
}

/// Gets a list of all documented attribute names.
pub fn get_all_attribute_names() -> Vec<&'static str> {
    ATTRIBUTE_DOCUMENTATION.keys().copied().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_widget_documentation_exists() {
        assert!(get_widget_documentation("button").is_some());
        assert!(get_widget_documentation("column").is_some());
        assert!(get_widget_documentation("text").is_some());
    }

    #[test]
    fn test_widget_documentation_not_found() {
        assert!(get_widget_documentation("nonexistent").is_none());
    }

    #[test]
    fn test_attribute_documentation_exists() {
        assert!(get_attribute_documentation("button", "on_click").is_some());
        assert!(get_attribute_documentation("column", "spacing").is_some());
        assert!(get_attribute_documentation("text", "value").is_some());
    }

    #[test]
    fn test_all_widget_names() {
        let names = get_all_widget_names();
        assert!(!names.is_empty());
        assert!(names.contains(&"button"));
        assert!(names.contains(&"column"));
    }

    #[test]
    fn test_all_attribute_names() {
        let names = get_all_attribute_names();
        assert!(!names.is_empty());
        assert!(names.contains(&"on_click"));
        assert!(names.contains(&"width"));
    }
}

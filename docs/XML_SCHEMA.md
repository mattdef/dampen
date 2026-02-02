# XML Schema Reference: Dampen UI Markup

**Version**: 1.0.0  
**Last Updated**: 2025-12-30

This document defines the complete XML schema for Dampen UI markup files (`.dampen`).

---

## Document Structure

### Root Element

Every Dampen file must have a single root element:

```xml
<dampen version="1.0">
    <!-- UI content here -->
</dampen>
```

**Attributes:**

| Attribute | Type | Required | Description |
|-----------|------|----------|-------------|
| `version` | string | **Yes** | Schema version in `major.minor` format (e.g., "1.0"). Specifies which Dampen schema version the file uses. See [Schema Versioning](#schema-versioning) for details. |
| `encoding` | string | No | Character encoding (e.g., "utf-8"). Defaults to UTF-8 if not specified. |

**Version Attribute Details:**
- **Format**: `major.minor` (e.g., "1.0", "1.1", "2.0")
- **Current Supported**: 1.0 (all core widgets)
- **Validation**: Parser rejects files with unsupported future versions
- **Default Behavior**: Files without `version` attribute default to 1.0 for backward compatibility

**Valid Examples:**
```xml
<!-- Explicit version (recommended) -->
<dampen version="1.0">
    <column><text value="Hello!" /></column>
</dampen>

<!-- With encoding -->
<dampen version="1.0" encoding="utf-8">
    <column><text value="Hello!" /></column>
</dampen>
```

**Alternative (simple files without `<dampen>` wrapper):**
```xml
<!-- Implicit version 1.0 -->
<column>
    <text value="Hello, World!" />
</column>
```
Note: Files starting directly with widgets (without `<dampen>` root) implicitly use version 1.0.

**Note on XML Declaration:** Dampen does not use standard XML declarations (`<?xml version="1.0"?>`) or XML namespaces (`xmlns`). The `<dampen>` element serves as both the root element and version declaration.

---

## Layout Widgets

### `<column>` - Vertical Layout

Stacks children top-to-bottom.

```xml
<column spacing="10" padding="20" align_x="center">
    <text value="First" />
    <text value="Second" />
</column>
```

**Attributes:**
| Attribute | Type | Default | Description |
|-----------|------|---------|-------------|
| `spacing` | length | 0 | Space between children |
| `padding` | length/box | 0 | Inner padding |
| `align_x` | align | start | Horizontal alignment of children: start, center, end |
| `align_y` | align | start | Vertical alignment within container: start, center, end |
| `width` | length | auto | Width constraint |
| `height` | length | auto | Height constraint |

### `<row>` - Horizontal Layout

Places children left-to-right.

```xml
<row spacing="10" align_y="center">
    <button label="Cancel" />
    <button label="OK" />
</row>
```

**Attributes:**
| Attribute | Type | Default | Description |
|-----------|------|---------|-------------|
| `spacing` | length | 0 | Space between children |
| `padding` | length/box | 0 | Inner padding |
| `align_x` | align | start | Horizontal alignment within container: start, center, end |
| `align_y` | align | start | Vertical alignment of children: start, center, end |
| `width` | length | auto | Width constraint |
| `height` | length | auto | Height constraint |

### `<container>` - Single Child Container

Wraps a single child with padding and alignment.

```xml
<container padding="20" align_x="center" align_y="center">
    <text value="Centered content" />
</container>
```

**Attributes:**
| Attribute | Type | Default | Description |
|-----------|------|---------|-------------|
| `padding` | length/box | 0 | Inner padding |
| `align_x` | align | start | Horizontal alignment |
| `align_y` | align | start | Vertical alignment |
| `width` | length | auto | Width constraint |
| `height` | length | auto | Height constraint |

### `<scrollable>` - Scrollable Container

For overflow content.

```xml
<scrollable direction="vertical" height="300">
    <column>
        <!-- Long content -->
    </column>
</scrollable>
```

**Attributes:**
| Attribute | Type | Default | Description |
|-----------|------|---------|-------------|
| `direction` | direction | vertical | vertical, horizontal, both |
| `width` | length | auto | Width constraint |
| `height` | length | auto | Height constraint |

### `<stack>` - Layered Container

Children overlap in layers.

```xml
<stack>
    <image src="background.png" />
    <text value="Overlay text" />
</stack>
```

**Attributes:**
| Attribute | Type | Default | Description |
|-----------|------|---------|-------------|
| `width` | length | auto | Width constraint |
| `height` | length | auto | Height constraint |

---

## Content Widgets

### `<text>` - Text Display

Displays text content.

```xml
<text value="Hello, World!" size="20" color="#333333" />
<text value="Count: {counter}" />
```

**Attributes:**
| Attribute | Type | Default | Description |
|-----------|------|---------|-------------|
| `value` | string/binding | "" | Text content |
| `size` | number | 16 | Font size in pixels |
| `color` | color | inherit | Text color |
| `font` | font-ref | default | Font family |
| `weight` | weight | normal | normal, bold, light |
| `align_x` | align | start | Horizontal alignment: start, center, end |
| `align_y` | align | start | Vertical alignment: start, center, end |
| `style` | style-ref | - | Style reference |

### `<image>` - Image Display

Displays an image from file or URL.

```xml
<image src="logo.png" width="100" height="100" />
<image src="{user.avatar}" fit="cover" />
```

**Attributes:**
| Attribute | Type | Default | Description |
|-----------|------|---------|-------------|
| `src` | string/binding | required | Image path or URL |
| `width` | length | auto | Width constraint |
| `height` | length | auto | Height constraint |
| `fit` | fit | contain | contain, cover, fill, none, scale_down |
| `filter_method` | string | - | Filter method for scaling |
| `path` | string | - | Alternative path specification |

### `<svg>` - SVG Display

Displays SVG content.

```xml
<svg src="icon.svg" width="24" height="24" />
```

**Attributes:**
| Attribute | Type | Default | Description |
|-----------|------|---------|-------------|
| `src` | string | required | SVG file path |
| `width` | length | auto | Width constraint |
| `height` | length | auto | Height constraint |
| `color` | color | inherit | Fill color override |

---

## Interactive Widgets

### `<button>` - Clickable Button

```xml
<button label="Click me" on_click="handle_click" />
<button on_click="submit" style="primary">
    <text value="Submit" />
</button>
```

**Attributes:**
| Attribute | Type | Default | Description |
|-----------|------|---------|-------------|
| `label` | string/binding | - | Button text |
| `on_click` | handler | - | Click handler name |
| `on_press` | handler | - | Press handler (mouse down / touch start) |
| `on_release` | handler | - | Release handler (mouse up / touch end) |
| `enabled` | bool/binding | true | Interactive state |
| `style` | style-ref | default | Style reference |
| `width` | length | auto | Width constraint |

### `<text_input>` - Text Input Field

```xml
<text_input 
    placeholder="Enter name..."
    value="{name}"
    on_input="update_name"
    on_submit="save_name"
/>
```

**Attributes:**
| Attribute | Type | Default | Description |
|-----------|------|---------|-------------|
| `value` | string/binding | "" | Current value |
| `placeholder` | string | "" | Placeholder text |
| `on_input` | handler | - | Keystroke handler |
| `on_submit` | handler | - | Enter key handler |
| `on_change` | handler | - | Value change handler |
| `on_paste` | handler | - | Paste handler |
| `password` | bool | false | Mask as password |
| `enabled` | bool/binding | true | Editable state |
| `width` | length | auto | Width constraint |
| `size` | number | - | Input size |
| `icon` | string | - | Input icon |

### `<checkbox>` - Toggle Checkbox

```xml
<checkbox 
    label="Accept terms"
    checked="{accepted}"
    on_toggle="toggle_accepted"
/>
```

**Attributes:**
| Attribute | Type | Default | Description |
|-----------|------|---------|-------------|
| `label` | string/binding | "" | Label text |
| `checked` | bool/binding | false | Checked state |
| `on_toggle` | handler | - | Toggle handler |
| `enabled` | bool/binding | true | Interactive state |
| `icon` | string | - | Checkbox icon |
| `size` | number | - | Checkbox size |

### `<slider>` - Numeric Slider

```xml
<slider 
    min="0" 
    max="100" 
    value="{volume}"
    on_change="set_volume"
    step="1"
/>
```

**Attributes:**
| Attribute | Type | Default | Description |
|-----------|------|---------|-------------|
| `min` | number | 0 | Minimum value |
| `max` | number | 100 | Maximum value |
| `value` | number/binding | min | Current value |
| `step` | number | 1 | Step increment |
| `on_change` | handler | - | Change handler |
| `on_release` | handler | - | Release handler |
| `enabled` | bool/binding | true | Interactive state |
| `width` | length | auto | Width constraint |

### `<pick_list>` - Dropdown Selection

```xml
<pick_list
    options="{items}"
    selected="{current_item}"
    on_select="select_item"
    placeholder="Choose..."
/>
```

**Attributes:**
| Attribute | Type | Default | Description |
|-----------|------|---------|-------------|
| `options` | list/binding | required | List of options |
| `selected` | any/binding | none | Selected value |
| `on_select` | handler | - | Selection handler |
| `placeholder` | string | "" | Placeholder |
| `enabled` | bool/binding | true | Interactive state |
| `width` | length | auto | Width constraint |

### `<radio>` - Radio Button

Individual radio button widget.

```xml
<column>
    <radio label="Small" value="small" selected="{size}" on_select="set_size" />
    <radio label="Medium" value="medium" selected="{size}" on_select="set_size" />
    <radio label="Large" value="large" selected="{size}" on_select="set_size" />
</column>
```

**Attributes:**
| Attribute | Type | Default | Description |
|-----------|------|---------|-------------|
| `label` | string/binding | "" | Radio button label |
| `value` | string/binding | required | Value when selected |
| `id` | string | - | Radio button ID |
| `selected` | any/binding | none | Currently selected value |
| `disabled` | bool/binding | false | Disabled state |
| `size` | number | - | Radio button size |
| `text_size` | number | - | Label text size |
| `text_line_height` | number | - | Label line height |
| `text_shaping` | string | - | Text shaping option |

**Events:**
| Event | Description |
|-------|-------------|
| `on_select` | Selection handler |

### `<toggler>` - Toggle Switch

```xml
<toggler
    label="Dark mode"
    toggled="{dark_mode}"
    on_toggle="toggle_dark_mode"
/>
```

**Attributes:**
| Attribute | Type | Default | Description |
|-----------|------|---------|-------------|
| `label` | string/binding | "" | Label text |
| `toggled` | bool/binding | false | Toggle state |
| `checked` | bool/binding | false | Alias for toggled |
| `active` | bool/binding | false | Alias for toggled |
| `on_toggle` | handler | - | Toggle handler |
| `enabled` | bool/binding | true | Interactive state |

### `<progress_bar>` - Progress Indicator

```xml
<progress_bar
    min="0"
    max="100"
    value="{progress}"
/>
```

**Attributes:**
| Attribute | Type | Default | Description |
|-----------|------|---------|-------------|
| `min` | number | 0 | Minimum value |
| `max` | number | 100 | Maximum value |
| `value` | number/binding | 0 | Current progress |

### `<for>` - Iteration Widget

Render a list of items by iterating over a collection.

```xml
<column>
    <for each="item" in="items">
        <text value="{item}" />
    </for>
</column>
```

**Attributes:**
| Attribute | Type | Default | Description |
|-----------|------|---------|-------------|
| `each` | string | required | Iterator variable name |
| `in` | binding | required | Collection to iterate over |
| `template` | string | - | Optional template name for rendering |

**Note:** The `in` attribute uses a binding expression (e.g., `{items}`) to reference the collection.

---

## Decorative Widgets

### `<space>` - Empty Space

```xml
<row>
    <text value="Left" />
    <space width="fill" />
    <text value="Right" />
</row>
```

**Attributes:**
| Attribute | Type | Default | Description |
|-----------|------|---------|-------------|
| `width` | length | 0 | Horizontal space |
| `height` | length | 0 | Vertical space |

### `<rule>` - Divider Line

```xml
<rule direction="horizontal" />
```

**Attributes:**
| Attribute | Type | Default | Description |
|-----------|------|---------|-------------|
| `direction` | direction | horizontal | Line direction |
| `color` | color | inherit | Line color |
| `thickness` | number | 1 | Line thickness |

---

## Advanced Widgets

### `<combobox>` - Combo Box Selection

Combines a text input with a dropdown list for selection.

```xml
<combobox
    options="Option1,Option2,Option3"
    selected="{current_option}"
    placeholder="Choose an option..."
    on_select="handle_select"
/>
```

**Attributes:**
| Attribute | Type | Default | Description |
|-----------|------|---------|-------------|
| `options` | string/binding | - | List of options (comma-separated string or binding) |
| `selected` | string/binding | - | Currently selected value |
| `value` | string/binding | - | Current input value |
| `placeholder` | string | "" | Placeholder text |
| `on_select` | handler | - | Selection handler (receives selected value) |
| `on_input` | handler | - | Input handler (for text input) |
| `enabled` | bool/binding | true | Interactive state |
| `width` | length | auto | Width constraint |

### `<grid>` - Grid Layout

Arranges children in a grid with a specified number of columns.

```xml
<grid columns="3" spacing="10">
    <text value="Cell 1" />
    <text value="Cell 2" />
    <text value="Cell 3" />
    <text value="Cell 4" />
    <text value="Cell 5" />
    <text value="Cell 6" />
</grid>
```

**Attributes:**
| Attribute | Type | Default | Description |
|-----------|------|---------|-------------|
| `columns` | number | required | Number of columns (1-20) |
| `spacing` | number | 10 | Spacing between cells |
| `width` | length | auto | Width constraint |
| `height` | length | auto | Height constraint |

### `<tooltip>` - Tooltip Overlay

Displays a tooltip when hovering over its child element.

```xml
<tooltip message="Help text here" position="top">
    <button label="Hover me" />
</tooltip>
```

**Attributes:**
| Attribute | Type | Default | Description |
|-----------|------|---------|-------------|
| `message` | string | required | Tooltip text content |
| `position` | position | follow_cursor | Tooltip position: `top`, `bottom`, `left`, `right`, `follow_cursor` |
| `delay` | number | 300 | Delay in ms before tooltip appears |
| `style` | style-ref | - | Style reference for tooltip |

### `<float>` - Floating Container

Positions a child element absolutely relative to its parent.

```xml
<container>
    <text value="Main content" />
    <float position="TopRight" offset_x="10" offset_y="10">
        <button label="X" on_click="close" />
    </float>
</container>
```

**Attributes:**
| Attribute | Type | Default | Description |
|-----------|------|---------|-------------|
| `position` | position | TopLeft | Anchor position: `TopLeft`, `TopRight`, `BottomLeft`, `BottomRight` |
| `offset_x` | number | 0 | Horizontal offset in pixels |
| `offset_y` | number | 0 | Vertical offset in pixels |
| `z_index` | number | 0 | Stacking order |

### `<canvas>` - Canvas Drawing

Renders custom graphics using a drawing program.

```xml
<canvas width="400" height="300" program="{drawing_program}" />
```

**Attributes:**
| Attribute | Type | Default | Description |
|-----------|------|---------|-------------|
| `width` | number | required | Canvas width (50-4000px) |
| `height` | number | required | Canvas height (50-4000px) |
| `program` | binding | required | Reference to a `CanvasProgram` |
| `cache` | bool | false | Enable caching for better performance |
| `style` | style-ref | - | Style reference |

**Events:**
| Event | Description |
|-------|-------------|
| `on_click` | Mouse click on canvas |
| `on_drag` | Mouse drag on canvas |
| `on_move` | Mouse move over canvas |
| `on_release` | Mouse release on canvas |

---

### `<date_picker>` - Date Selection

Date picker widget for selecting dates.

```xml
<date_picker 
    value="{selected_date}" 
    format="YYYY-MM-DD"
    on_submit="handle_date_submit"
    on_cancel="handle_date_cancel"
/>
```

**Attributes:**
| Attribute | Type | Default | Description |
|-----------|------|---------|-------------|
| `value` | string/binding | - | Selected date value |
| `format` | string | "YYYY-MM-DD" | Date format string |
| `show` | bool/binding | false | Show/hide picker |
| `min_date` | string | - | Minimum selectable date |
| `max_date` | string | - | Maximum selectable date |

**Events:**
| Event | Description |
|-------|-------------|
| `on_submit` | Date selected |
| `on_cancel` | Selection cancelled |

---

### `<time_picker>` - Time Selection

Time picker widget for selecting times.

```xml
<time_picker 
    value="{selected_time}" 
    format="HH:mm"
    use_24h="true"
    on_submit="handle_time_submit"
/>
```

**Attributes:**
| Attribute | Type | Default | Description |
|-----------|------|---------|-------------|
| `value` | string/binding | - | Selected time value |
| `format` | string | "HH:mm" | Time format string |
| `show` | bool/binding | false | Show/hide picker |
| `use_24h` | bool | true | Use 24-hour format |
| `show_seconds` | bool | false | Show seconds selector |

**Events:**
| Event | Description |
|-------|-------------|
| `on_submit` | Time selected |
| `on_cancel` | Selection cancelled |

---

### `<color_picker>` - Color Selection

Color picker widget for selecting colors.

```xml
<color_picker 
    value="{selected_color}" 
    show_alpha="true"
    on_change="handle_color_change"
    on_submit="handle_color_submit"
/>
```

**Attributes:**
| Attribute | Type | Default | Description |
|-----------|------|---------|-------------|
| `value` | string/binding | - | Selected color value (hex) |
| `show` | bool/binding | false | Show/hide picker |
| `show_alpha` | bool | false | Show alpha channel selector |
| `enabled` | bool/binding | true | Enable/disable picker |

**Events:**
| Event | Description |
|-------|-------------|
| `on_change` | Color changed |
| `on_submit` | Color selected |
| `on_cancel` | Selection cancelled |

---

### `<menu>` - Menu Widget

Menu container for dropdown or context menus.

```xml
<menu position="bottom" close_on_select="true">
    <menu_item label="New" on_click="handle_new" />
    <menu_item label="Open" on_click="handle_open" />
    <menu_separator />
    <menu_item label="Exit" on_click="handle_exit" />
</menu>
```

**Attributes:**
| Attribute | Type | Default | Description |
|-----------|------|---------|-------------|
| `position` | string | "bottom" | Menu position: "top", "bottom", "left", "right" |
| `close_on_select` | bool | true | Close menu when item selected |
| `width` | length | auto | Menu width |
| `spacing` | number | 0 | Spacing between items |
| `class` | string | - | CSS class for styling |

**Events:**
| Event | Description |
|-------|-------------|
| `on_open` | Menu opened |
| `on_close` | Menu closed |

---

### `<menu_item>` - Menu Item

Individual menu item within a `<menu>`.

```xml
<menu_item 
    label="Save" 
    icon="save_icon"
    shortcut="Ctrl+S"
    on_click="handle_save"
/>
```

**Attributes:**
| Attribute | Type | Default | Description |
|-----------|------|---------|-------------|
| `label` | string | **required** | Menu item text |
| `icon` | string | - | Icon reference |
| `shortcut` | string | - | Keyboard shortcut display |
| `disabled` | bool | false | Disable the item |
| `class` | string | - | CSS class for styling |

**Events:**
| Event | Description |
|-------|-------------|
| `on_click` | Item clicked |

---

### `<menu_separator>` - Menu Separator

Horizontal separator line between menu items.

```xml
<menu_separator />
```

**Attributes:**
| Attribute | Type | Default | Description |
|-----------|------|---------|-------------|
| `color` | color | inherit | Separator color |
| `opacity` | number | 1.0 | Separator opacity |
| `height` | length | 1 | Separator height |

---

### `<context_menu>` - Context Menu

Right-click context menu.

```xml
<context_menu context="{menu_items}">
    <!-- Menu items or use binding -->
</context_menu>
```

**Attributes:**
| Attribute | Type | Default | Description |
|-----------|------|---------|-------------|
| `context` | binding | - | Context data binding |

**Events:**
| Event | Description |
|-------|-------------|
| `on_open` | Menu opened |
| `on_close` | Menu closed |

---

### `<data_table>` - Data Table

Table widget for displaying tabular data.

```xml
<data_table 
    data="{table_data}"
    width="fill"
    height="400"
    on_row_click="handle_row_click"
>
    <data_column header="Name" field="name" width="200" />
    <data_column header="Email" field="email" width="fill" />
    <data_column header="Age" field="age" width="80" align="center" />
</data_table>
```

**Attributes:**
| Attribute | Type | Default | Description |
|-----------|------|---------|-------------|
| `data` | binding | **required** | Table data source |
| `width` | length | auto | Table width |
| `height` | length | auto | Table height |
| `min_width` | length | - | Minimum width |
| `max_width` | length | - | Maximum width |
| `scrollbar_width` | number | 10 | Scrollbar width |

**Events:**
| Event | Description |
|-------|-------------|
| `on_row_click` | Row clicked |

---

### `<data_column>` - Data Table Column

Column definition for `<data_table>`.

```xml
<data_column 
    header="Column Name" 
    field="data_field"
    width="150"
    min_width="100"
    align="left"
/>
```

**Attributes:**
| Attribute | Type | Default | Description |
|-----------|------|---------|-------------|
| `header` | string | **required** | Column header text |
| `field` | string | - | Data field name |
| `width` | length | auto | Column width |
| `min_width` | length | - | Minimum column width |
| `max_width` | length | - | Maximum column width |
| `align` | align | left | Text alignment: left, center, right |

---

### `<tree_view>` - Tree View

Hierarchical tree widget for displaying nested data.

```xml
<tree_view 
    nodes="{tree_nodes}"
    expanded="{expanded_nodes}"
    selected="{selected_node}"
    indent_size="20"
    node_height="30"
    on_toggle="handle_expand_toggle"
    on_select="handle_node_select"
    on_double_click="handle_node_double_click"
/>
```

**Attributes:**
| Attribute | Type | Default | Description |
|-----------|------|---------|-------------|
| `nodes` | binding | - | Tree nodes data |
| `expanded` | binding | - | Set of expanded node IDs |
| `selected` | binding | - | Currently selected node ID |
| `indent_size` | number | 20 | Indentation per level (px) |
| `node_height` | number | 30 | Height of each node (px) |
| `icon_size` | number | 16 | Size of expand/collapse icons |
| `expand_icon` | string | - | Custom expand icon |
| `collapse_icon` | string | - | Custom collapse icon |
| `leaf_icon` | string | - | Custom leaf node icon |

**Events:**
| Event | Description |
|-------|-------------|
| `on_toggle` | Node expanded/collapsed |
| `on_select` | Node selected |
| `on_double_click` | Node double-clicked |

---

### `<tree_node>` - Tree Node

Individual node within a `<tree_view>` (used in data structure).

```xml
<!-- Tree nodes are typically defined in your model, not XML -->
<!-- Example structure expected by tree_view: -->
TreeNode {
    id: "node_1",
    label: "Parent Node",
    icon: "folder_icon",
    expanded: true,
    selected: false,
    children: [...]
}
```

**Attributes:**
| Attribute | Type | Default | Description |
|-----------|------|---------|-------------|
| `id` | string | **required** | Unique node identifier |
| `label` | string | **required** | Node display text |
| `icon` | string | - | Node icon |
| `expanded` | bool | false | Expanded state |
| `selected` | bool | false | Selected state |
| `disabled` | bool | false | Disabled state |
| `class` | string | - | CSS class |

---

### `<tab_bar>` - Tab Bar Widget

A tab bar widget that displays a row of selectable tabs with associated content.

```xml
<tab_bar selected="{selected_tab}" on_select="on_tab_selected" icon_size="20" text_size="14">
    <tab label="General" icon="settings">
        <column>
            <text value="General Settings" />
            <checkbox label="Enable auto-save" checked="{auto_save}" on_toggle="toggle_auto_save" />
        </column>
    </tab>
    <tab label="Appearance" icon="user">
        <column>
            <text value="Appearance Settings" />
            <pick_list selected="{theme}" options="Light,Dark,System" on_select="change_theme" />
        </column>
    </tab>
    <tab label="Notifications" icon="add">
        <column>
            <text value="Notification Settings" />
            <checkbox label="Email notifications" checked="{email_notifications}" on_toggle="toggle_email" />
        </column>
    </tab>
</tab_bar>
```

**Attributes:**
| Attribute | Type | Default | Description |
|-----------|------|---------|-------------|
| `selected` | number/binding | **required** | Index of the currently selected tab (0-based) |
| `icon_size` | number | 20 | Size of tab icons in pixels |
| `text_size` | number | 14 | Size of tab text in pixels |
| `spacing` | number | 0 | Spacing between tabs |
| `padding` | length/box | 0 | Padding around the tab bar |
| `width` | length | auto | Width constraint |
| `height` | length | auto | Height constraint |
| `class` | string | - | CSS class for styling |

**Events:**
| Event | Description |
|-------|-------------|
| `on_select` | Called when a tab is selected, receives the tab index |

**Child Elements:**

The `<tab_bar>` must contain one or more `<tab>` elements as children.

### `<tab>` - Tab Item

Individual tab within a `<tab_bar>`. Each tab defines its label, optional icon, and content.

```xml
<tab label="Settings" icon="settings">
    <!-- Tab content widgets here -->
    <column>
        <text value="Settings Content" />
    </column>
</tab>
```

**Attributes:**
| Attribute | Type | Default | Description |
|-----------|------|---------|-------------|
| `label` | string | **required** | Tab label text |
| `icon` | string | - | Icon name (e.g., "settings", "user", "home") |
| `enabled` | bool | true | Whether the tab is clickable |

**Content:**

Each `<tab>` can contain any number of child widgets. The content of the selected tab is automatically displayed below the tab bar.

**Notes:**
- The `selected` attribute of `<tab_bar>` should be bound to a numeric field in your model
- When `on_select` is triggered, update the bound field to change the displayed tab
- Tab content is rendered dynamically based on the selected index
- Icons are resolved using the framework's icon mapping system

---

### `<if>` - Conditional Rendering

Conditionally renders content based on a condition.

```xml
<if condition="{show_welcome}">
    <text value="Welcome back!" />
</if>
```

**Attributes:**
| Attribute | Type | Default | Description |
|-----------|------|---------|-------------|
| `condition` | bool/binding | **required** | Condition to evaluate |

---

## Binding Expression Syntax

### Basic Field Binding

```xml
<text value="{counter}" />
```

Binds to `model.counter`.

### Nested Field Access

```xml
<text value="{user.profile.name}" />
```

Binds to `model.user.profile.name`.

### Formatted Binding

```xml
<text value="Count: {counter}" />
<text value="Hello, {name}! You have {messages.len()} messages." />
```

Interpolates multiple values.

### Method Calls

```xml
<text value="{items.len()}" />
<text value="{name.to_uppercase()}" />
```

Calls methods on bound values.

### Conditional Binding

```xml
<button enabled="{counter > 0}" />
<text value="{if is_loading then 'Loading...' else 'Ready'}" />
<container style="{if is_error then 'error' else 'default'}" />
```

### Shared State Bindings

**NEW in v0.2.4!** Access application-wide shared state from any view.

**Syntax:**
```xml
<text value="{shared.field}" />
```

**Local vs Shared Bindings:**

```xml
<!-- Local model binding (view-specific state) -->
<text value="{message}" />
<text value="{user.email}" />

<!-- Shared state binding (cross-view state) -->
<text value="{shared.theme}" />
<text value="{shared.username}" />

<!-- Mixed usage -->
<column>
    <text value="Welcome, {shared.username}!" />
    <text value="{local_status}" />
</column>
```

**Common Use Cases:**

```xml
<!-- User preferences -->
<text value="Theme: {shared.theme}" />
<text value="Language: {shared.language}" />

<!-- Session data -->
<text value="Logged in as: {shared.current_user}" />

<!-- Application settings -->
<toggler 
    label="Dark Mode"
    toggled="{shared.dark_mode}"
    on_toggle="toggle_dark_mode"
/>
```

**Nested Field Access:**

```xml
<!-- Access nested shared fields -->
<text value="{shared.user.profile.name}" />
<text value="{shared.settings.theme.primary_color}" />
```

**Requirements:**

1. **Shared model** must derive `UiModel`:
   ```rust
   #[derive(Clone, UiModel, Serialize, Deserialize)]
   pub struct SharedState {
       pub theme: String,
       pub username: String,
   }
   ```

2. **AppState** must have shared context:
   ```rust
   let shared = SharedContext::new(SharedState::default());
   let state = AppState::with_handlers(document, handlers)
       .with_shared_context(shared);
   ```

3. **Handlers** can modify shared state:
   ```rust
   registry.register_with_value_and_shared(
       "update_theme",
       |model, shared, theme| {
           if let Some(s) = shared.downcast_ref::<SharedContext<SharedState>>() {
               s.write(|state| state.theme = theme);
           }
       }
   );
   ```

**Behavior:**

- **Thread-safe**: Multiple views can read simultaneously
- **Hot-reload preserved**: Shared state survives XML reloads
- **Type-safe**: Compile-time verification via UiModel trait
- **Null-safe**: Missing fields render as empty string

**See also:** `docs/USAGE.md` "Shared State for Multi-View Applications" section

---

### Supported Operators

| Operator | Description | Example |
|----------|-------------|---------|
| `==` | Equality | `{status == 'active'}` |
| `!=` | Inequality | `{count != 0}` |
| `<` | Less than | `{age < 18}` |
| `<=` | Less or equal | `{score <= 100}` |
| `>` | Greater than | `{items.len() > 0}` |
| `>=` | Greater or equal | `{progress >= 100}` |
| `+` | Addition | `{a + b}` |
| `-` | Subtraction | `{price - discount}` |
| `*` | Multiplication | `{quantity * unit_price}` |
| `/` | Division | `{total / count}` |
| `&&` | Logical AND | `{active && visible}` |
| `\|\|` | Logical OR | `{error \|\| warning}` |
| `!` | Logical NOT | `{!is_valid}` |

---

## Event Handler Syntax

### Handler Reference

```xml
<button on_click="increment" />
```

References handler function `increment` in Rust code.

### Handler Signatures

```rust
// Simple handler
fn increment(model: &mut Model) {
    model.counter += 1;
}

// Handler with value
fn update_name(model: &mut Model, value: String) {
    model.name = value;
}

// Handler returning Command
fn fetch_data(model: &mut Model) -> Command<Message> {
    Command::perform(async { fetch_api().await }, Message::DataReceived)
}
```

---

## Attribute Value Types

| Type | Format | Examples |
|------|--------|----------|
| `string` | Quoted text or binding | `"Hello"`, `{name}` |
| `number` | Integer or float | `10`, `3.14` |
| `length` | Number with optional unit | `100`, `50%`, `fill`, `auto` |
| `color` | Hex or named color | `#FF0000`, `red`, `{theme.primary}` |
| `bool` | Boolean or binding | `true`, `false`, `{is_enabled}` |
| `binding` | Expression in braces | `{counter}`, `{user.name}` |
| `handler` | Handler function name | `"handle_click"` |
| `style-ref` | Style name | `"primary"`, `"danger"` |
| `align` | Alignment value | `start`, `center`, `end` |
| `direction` | Direction value | `horizontal`, `vertical` |
| `fit` | Content fit mode | `contain`, `cover`, `fill`, `none` |

### Length Units

| Value | Description |
|-------|-------------|
| `100` | Fixed pixels |
| `50%` | Percentage of parent |
| `fill` | Fill remaining space |
| `auto` | Automatic sizing |

---

## Styling System

### Theme Definition

```xml
<themes>
    <theme name="custom">
        <palette 
            primary="#3498db" 
            secondary="#2ecc71"
            success="#27ae60"
            warning="#f39c12"
            danger="#e74c3c"
            background="#ecf0f1"
            surface="#ffffff"
            text="#2c3e50"
            text_secondary="#7f8c8d" />
        <typography 
            font_family="Inter, sans-serif"
            font_size_base="16"
            font_size_small="12"
            font_size_large="24"
            font_weight="normal"
            line_height="1.5" />
        <spacing unit="8" />
    </theme>
</themes>

<global_theme name="custom" />
```

### Style Classes

```xml
<style_classes>
    <style name="button_primary"
        extends="button_base"
        background="#3498db"
        color="#ffffff"
        padding="12 24"
        border_radius="6"
        hover:background="#2980b9"
        active:background="#21618c"
        disabled:opacity="0.5" />
</style_classes>
```

### Inline Style Attributes

**Layout:**
- `width`: fixed, fill, shrink, fill_portion(n), percentage
- `height`: fixed, fill, shrink, fill_portion(n), percentage
- `padding`: spacing value (e.g., "10 20" or "10 20 30 40")
- `spacing`: child spacing value
- `align_items`: start, center, end, stretch
- `justify_content`: start, center, end, space_between, space_around, space_evenly
- `position`: relative, absolute
- `top`, `right`, `bottom`, `left`: offset values
- `z_index`: integer

**Style:**
- `background`: color or gradient
- `color`: text color
- `border_width`: thickness
- `border_color`: color
- `border_radius`: corner rounding
- `shadow`: "offset_x offset_y blur color"
- `opacity`: 0.0-1.0
- `transform`: transform operations

**State Variants (prefixed with `:`):**
- `hover:*`: hover state (e.g., `hover:background="#2980b9"`, `hover:color="#fff"`)
- `focus:*`: focus state (e.g., `focus:border_color="#3498db"`)
- `active:*`: active state (e.g., `active:background="#21618c"`)
- `disabled:*`: disabled state (e.g., `disabled:opacity="0.5"`)

**Responsive (prefixed):**
- `mobile:*`: < 640px
- `tablet:*`: 640px - 1024px
- `desktop:*`: > 1024px

### Widget Attributes

**All Widgets:**
- `class`: space-separated style class names
- `theme_ref`: apply local theme
- `disabled`: boolean
- `id`: identifier for state tracking

**Interactive Widgets:**
- `on_click`: handler name
- `on_input`: handler name (text_input)
- `on_change`: handler name (checkbox, slider, pick_list)
- `on_toggle`: handler name (toggler)
- `on_submit`: handler name (text_input)

### State-Based Styling

State variants can be defined using child elements or prefixed attributes:

```xml
<!-- Prefixed attributes (recommended) -->
<style name="btn"
    background="#3498db"
    hover:background="#2980b9"
    active:background="#21618c"
    disabled:opacity="0.5" />
```

### Responsive Design

Breakpoint-prefixed attributes override base values:

```xml
<column mobile:spacing="10" desktop:spacing="20">
    <text mobile:size="18" desktop:size="32" value="Responsive" />
</column>
```

---

## Complete Example

```xml
<dampen version="1.0">
    <column padding="20" spacing="10">
        <text value="Todo List" size="24" weight="bold" />
        
        <row spacing="10">
            <text_input 
                placeholder="New todo..."
                value="{new_todo}"
                on_input="update_new_todo"
                on_submit="add_todo"
                width="fill"
            />
            <button label="Add" on_click="add_todo" enabled="{new_todo.len() > 0}" />
        </row>
        
        <rule />
        
        <scrollable height="300">
            <column spacing="5">
                <text value="{items.len()} items" color="#888888" />
            </column>
        </scrollable>
        
        <row spacing="10">
            <text value="{completed_count} completed" />
            <space width="fill" />
            <button 
                label="Clear completed" 
                on_click="clear_completed"
                enabled="{completed_count > 0}"
            />
        </row>
    </column>
</dampen>
```

---

## Schema Versioning

Dampen uses semantic versioning for its XML schema to ensure compatibility and enable evolution of the UI framework.

### Version Format

Version numbers follow the `major.minor` format:
- **Major version**: Breaking changes or removal of features (e.g., 2.0)
- **Minor version**: Backward-compatible additions (e.g., 1.1)

### Supported Versions

**Version 1.0** (Current): Initial release with core widgets
- Layout: column, row, container, scrollable, stack, grid, float
- Content: text, image, svg
- Interactive: button, text_input, checkbox, slider, pick_list, toggler, radio, progress_bar, combobox
- Control flow: for, if
- Decorative: space, rule, tooltip
- Bindings: field access, method calls, conditionals, formatting, shared state
- Events: click, input, change, toggle, submit, select, scroll, drag, move, release, open, close

**Version 1.1** (Experimental): Additional widgets (not fully functional)
- Pickers: date_picker, time_picker, color_picker
- Menu: menu, menu_item, menu_separator, context_menu
- Data: data_table, data_column
- Tree: tree_view, tree_node
- Canvas: canvas (with shapes: canvas_rect, canvas_circle, canvas_line, canvas_text, canvas_group)

**Note**: Widgets requiring v1.1+ are experimental and may not be fully functional. Use `dampen check --show-widget-versions` to see the full list.

### Version Validation

The parser validates schema versions at parse time:

1. **Declared Version Check**: Parser reads the `version` attribute from `<dampen>` root
2. **Support Validation**: Compares against maximum supported version (currently 1.0)
3. **Error Handling**: Rejects files declaring unsupported future versions with clear error messages
4. **Widget Version Warnings**: `dampen check` warns about widgets requiring higher versions than declared

### Backward Compatibility

- Files without a `version` attribute default to version 1.0
- All version 1.0 files will continue to work when version 1.1 is released
- Future versions will maintain backward compatibility within the same major version

### Best Practices

1. **Always declare version explicitly**: `<dampen version="1.0">` makes intent clear
2. **Use the latest supported version**: Currently 1.0
3. **Validate before commits**: Run `dampen check` to catch version errors early
4. **Don't mix versions**: All `.dampen` files in a project should use the same version

---

## Troubleshooting

### Version-Related Errors

#### Error: "Schema version X.Y is not supported"

**Cause**: Your file declares a version newer than your installed Dampen framework supports.

**Example**:
```
Error: Schema version 2.0 is not supported. Maximum supported version: 1.0
  --> src/ui/window.dampen:1:9
   |
 1 | <dampen version="2.0">
   |         ^^^^^^^^^^^^^^ unsupported version
   |
Suggestion: Upgrade dampen-core to support v2.0, or use version="1.0"
```

**Solutions**:
1. **Upgrade Dampen**: Update to the latest version that supports the schema version:
   ```bash
   cargo update dampen-core dampen-iced
   ```
2. **Downgrade Schema Version**: Change the version attribute to a supported version:
   ```xml
   <dampen version="1.0">
   ```

#### Error: "Invalid version format"

**Cause**: Version attribute is not in the correct `major.minor` format.

**Example**:
```
Error: Invalid version format '1'. Expected 'major.minor' (e.g., '1.0')
  --> src/ui/window.dampen:1:9
   |
 1 | <dampen version="1">
   |         ^^^^^^^^^^^ invalid format
   |
Suggestion: Use format: version="1.0"
```

**Common Invalid Formats**:
```xml
<!-- ❌ Missing minor version -->
<dampen version="1">

<!-- ❌ Version prefix -->
<dampen version="v1.0">

<!-- ❌ Patch version not supported -->
<dampen version="1.0.5">

<!-- ❌ Prerelease suffix -->
<dampen version="1.0-beta">

<!-- ❌ Non-numeric -->
<dampen version="one.zero">

<!-- ✅ Correct format -->
<dampen version="1.0">
```

**Solutions**:
- Use the format `major.minor`: `version="1.0"`
- Don't include prefixes (v), suffixes (-beta), or patch numbers (.5)
- Both parts must be numeric

#### Warning: File has no version attribute

**Note**: This is currently allowed (defaults to v1.0), but explicit versioning is recommended.

**Example**:
```xml
<!-- Implicit version 1.0 (works but not recommended) -->
<dampen>
    <column><text value="Hello" /></column>
</dampen>
```

**Recommendation**:
```xml
<!-- Explicit version (recommended) -->
<dampen version="1.0">
    <column><text value="Hello" /></column>
</dampen>
```

**Why explicit is better**:
- Makes schema version intent clear
- Easier migration when new versions are released
- Consistent with best practices
- Future versions may warn about missing version

#### Warning: "Widget requires higher schema version"

**Cause**: You're using a widget that was introduced in a newer schema version than your document declares.

**Example**:
```
Warning: Widget 'experimental_widget' requires schema v1.1 but document declares v1.0 in src/ui/window.dampen:153:21
  Suggestion: Update to <dampen version="1.1"> or remove this widget
```

**What this means**:
- The widget you're using requires a schema version newer than what your document declares
- Currently affects: **Canvas**, **date_picker**, **time_picker**, **color_picker**, **menu**, **menu_item**, **menu_separator**, **context_menu**, **data_table**, **data_column** (all require v1.1, experimental/non-functional)
- All other widgets are v1.0 and fully functional

**Solutions**:

1. **Upgrade schema version** (when a newer version is officially supported):
   ```xml
   <dampen version="1.1">
       <!-- Use newer widgets here -->
   </dampen>
   ```

2. **Replace with compatible widget**:
   Use v1.0 alternatives

3. **Ignore warning** (if you know what you're doing):
   The warning is informational for experimental features.

**Why warnings instead of errors**:
- Allows gradual migration to new schema versions
- Developers can test experimental widgets before official release
- Non-blocking for development workflows

**Check widget versions**:
```bash
dampen check --show-widget-versions
```

This displays a table of all widgets with their minimum required versions:
```
Widget               Min Version Status
-------------------- ---------- ------------------------------
column               1.0        Stable
row                  1.0        Stable
container            1.0        Stable
scrollable           1.0        Stable
stack                1.0        Stable
text                 1.0        Stable
image                1.0        Stable
svg                  1.0        Stable
button               1.0        Stable
text_input           1.0        Stable
checkbox             1.0        Stable
slider               1.0        Stable
pick_list            1.0        Stable
toggler              1.0        Stable
radio                1.0        Stable
space                1.0        Stable
rule                 1.0        Stable
progress_bar         1.0        Stable
combobox             1.0        Stable
tooltip              1.0        Stable
grid                 1.0        Stable
canvas               1.1        Experimental (not fully functional)
date_picker          1.1        Experimental (not fully functional)
time_picker          1.1        Experimental (not fully functional)
color_picker         1.1        Experimental (not fully functional)
menu                 1.1        Experimental (not fully functional)
menu_item            1.1        Experimental (not fully functional)
menu_separator       1.1        Experimental (not fully functional)
context_menu         1.1        Experimental (not fully functional)
float                1.0        Stable
data_table           1.1        Experimental (not fully functional)
data_column          1.1        Experimental (not fully functional)
```

### Validation Commands

**Check all `.dampen` files in your project**:
```bash
dampen check
```

**Check a specific file**:
```bash
dampen check src/ui/window.dampen
```

**Check with verbose output**:
```bash
dampen check --verbose
```

**Show widget version requirements**:
```bash
dampen check --show-widget-versions
```

### Common Issues

#### Issue: "File works in development but fails in CI"

**Possible Causes**:
1. Different Dampen versions between local and CI
2. Missing version attribute causing version mismatch

**Solutions**:
- Pin Dampen version in `Cargo.toml`:
  ```toml
  [dependencies]
  dampen-core = "=0.2.0"
  dampen-iced = "=0.2.0"
  ```
- Always declare `version` explicitly in all `.dampen` files
- Run `dampen check` as part of CI pipeline

#### Issue: "Mixed version errors across files"

**Cause**: Different `.dampen` files using different schema versions.

**Solution**: Standardize all files to the same version:
```bash
# Find all .dampen files without version 1.0
grep -r '<dampen' src/ examples/ | grep -v 'version="1.0"'

# Update all to use version 1.0
# (Manual edit or search-replace in your editor)
```

### Getting Help

If you encounter version-related issues not covered here:

1. **Check the version**: Verify your Dampen installation version:
   ```bash
   cargo tree | grep dampen-core
   ```

2. **Read the quickstart**: See `docs/QUICKSTART.md` for version usage examples

3. **Report bugs**: If you believe the version validation is incorrect:
   - GitHub Issues: https://github.com/dampen-ui/dampen/issues
   - Include: Error message, `.dampen` file content, Dampen version

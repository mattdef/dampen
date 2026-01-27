<div align="center">

# Dampen Roadmap

**Declarative UI framework for Rust with Iced backend, hot-reloading and advanced styling**

[![Version](https://img.shields.io/badge/version-0.2.0--alpha-blue)](CHANGELOG.md)
[![Status](https://img.shields.io/badge/status-Active%20Development-yellow)](README.md)
[![Rust](https://img.shields.io/badge/rust-1.85+-orange)](https://rust-lang.org)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-green)](LICENSE-MIT)

</div>

---

## Table of Contents

1. [Vision](#vision)
2. [Development Phases](#development-phases)
3. [Planned Features](#planned-features)
4. [Technical Improvements](#technical-improvements)
5. [Ecosystem and Tools](#ecosystem-and-tools)
6. [Quality Goals](#quality-goals)
7. [Estimated Timeline](#estimated-timeline)
8. [Contributing](#contributing)

---

## Vision

Dampen aims to become the reference declarative UI framework for Rust, offering an exceptional developer experience combining:

- **Simplicity**: User interface definition via declarative XML
- **Performance**: Dual-mode architecture (interpreted for development, codegen for production)
- **Flexibility**: Backend-agnostic with complete Iced implementation
- **Productivity**: Instant hot-reloading and complete CLI tooling

The long-term goal is to enable Rust developers to create modern desktop applications with minimal learning curve and maximum productivity.

---

## Development Phases

### v0.1.0 - Foundations (Completed ✓)

**Objective**: Establish framework basics with essential features

| Milestone | Status | Description |
|-----------|--------|-------------|
| XML Parser | ✅ | Parsing of `.dampen` files |
| IR (Intermediate Representation) | ✅ | Internal data structure for UI |
| Iced Backend | ✅ | Complete widget mapping |
| Proc Macros | ✅ | `#[derive(UiModel)]`, `#[dampen_ui]` |
| Basic CLI | ✅ | Commands `new`, `check`, `build` |
| Examples | ✅ | hello-world, counter, todo-app |

### v0.2.0 - Validation and Dual-Mode Architecture (Completed ✓)

**Objective**: Improve robustness and prepare for production

| Milestone | Status | Description |
|-----------|--------|-------------|
| Version Validation | ✅ | XML schema validation |
| Interpreted Mode | ✅ | Runtime parsing with hot-reload |
| Codegen Mode | ✅ | Static code generation |
| Parity Tests | ✅ | Ensure identical behavior |
| Benchmarks | ✅ | Performance metrics |

### v0.2.1 - Add versioning to the XML schema (Completed ✓)

**Objective**: Improve robustness and prepare for production

| Milestone | Status | Description |
|-----------|--------|-------------|
| Versioning | ✅ | Add versioning to the XML schema |
| Backward Compatibility | ✅ | Ensure backward compatibility |
| Forward Compatibility | ✅ | Ensure forward compatibility |
| Version Migration | ✅ | Provide migration tools |
| Version Documentation | ✅ | Document version changes |

### v0.2.2 - Enhanced multi-windows application (Completed ✓)

**Objective**: Improve user experience and functionality

| Milestone | Status | Description |
|-----------|--------|-------------|
| Code refactoring | ✅ | Auto-Discovery Multi-View Application |

### v0.2.3 - Enhanced multi-windows application (Completed ✓)

**Objective**: Improve user experience and functionality

| Milestone | Status | Description |
|-----------|--------|-------------|
| CLI - Create window UI  | ✅ | Create a new window with the command `dampen add --ui <window_name>` |

### v0.2.4 - Enhanced multi-windows application (Completed ✓)

**Objective**: Improve user experience and functionality

| Milestone | Status | Description |
|-----------|--------|-------------|
| Window Communication | ✅ | Inter-window messaging |

### v0.2.5 - Enhanced multi-windows application (Completed ✓)

**Objective**: Improve user experience and functionality

| Milestone | Status | Description |
|-----------|--------|-------------|
| Window Theming | ✅ | Customizable window appearance |

### v0.2.6 - Enhanced multi-windows application (Completed ✓)

**Objective**: Improve user experience and functionality

| Milestone | Status | Description |
|-----------|--------|-------------|
| Window Layouts | ✅ | Predefined window arrangements |

### v0.2.7 - Add Widgets schema system (Completed ✓)

**Objective**: Introduce a new schema module et expose it as API

| Milestone | Status | Description |
|-----------|--------|-------------|
| Widgets Schema | ✅ | Expose get_widget_schema(kind) and a WidgetKind.schema() helper |

### v0.2.8 - Enhanced multi-windows application (Completed ✓)

**Objective**: Improve user experience and functionality

| Milestone | Status | Description |
|-----------|--------|-------------|
| Window Persistence | ✅ | Save and restore window states |

### v0.3.0 - Advanced Widgets (In progress)

**Objective**: Enrich available widget library

| Milestone | Status | Priority | Description |
|-----------|--------|----------|-------------|
| Canvas | ✅ | Medium | Vector drawing widget |
| DatePicker | ✅ | High | Date selector |
| TimePicker | ✅ | High | Time selector |
| Menus | ✅ | High | Menu bars and context menus |
| DataTable | 🔲 | Medium | Table with sort/filter |
| ColorPicker | 🔲 | Low | Color selector |
| TreeView | 🔲 | Low | Hierarchical display |

### Developer Experience (planned)

**Objective**: Improve tooling and DX

| Milestone | Status | Priority | Description |
|-----------|--------|----------|-------------|
| Language Server | 🔲 | High | LSP support for `.dampen` files |
| VS Code Extension | 🔲 | High | Official VS Code extension |
| Interactive CLI | 🔲 | Low | Interactive mode for `dampen new` |
| Visual Hot Reload | 🔲 | Medium | Improved error overlay |
| Debugger Integration | 🔲 | Low | IDE debugging support |

---

## Planned Features

### UI Widgets

#### High Priority

```
⏳ Menu / MenuBar
   ├── Attributes : items, enabled, class
   ├── Events : on_select(action)
   └── Structure :
       <menubar>
           <menu label="File">
               <item label="Open" on_click="open_file" />
               <item label="Save" on_click="save_file" />
           </menu>
       </menubar>
```

```
⏳ ContextMenu
   ├── Attributes : items, position
   ├── Events : on_select(action)
   └── Example :
       <container on_right_click="show_menu">
           <context_menu id="main_menu" items="{menu_items}" />
       </container>
```

```
⏳ DatePicker / TimePicker
   ├── Attributes : value, min, max, format
   └── Example :
       <date_picker value="{date}" on_change="set_date" />
```

```
⏳ Tooltip (enhancement)
   ├── Attributes : delay, position (auto), max_width
   └── Example :
       <button label="Help" tooltip="Click for help" tooltip_delay="500" />
```

#### Medium Priority

```
⏳ DataTable
   ├── Attributes : columns, data, sortable, pagination
   ├── Events : on_sort(column), on_page_change
   └── Example :
       <data_table columns="{cols}" data="{rows}" sortable="true" />
```

```
⏳ ProgressRing
   ├── Attributes : min, max, value, stroke_width
   └── Example :
       <progress_ring value="{progress}" stroke_width="10" />
```

```
⏳ Tabs
   ├── Attributes : tabs, active_tab, on_change
   └── Example :
       <tabs tabs="{tab_titles}" active_tab="{active}" on_change="switch_tab" />
```

```
⏳ Canvas
   ├── Attributes : width, height
   ├── Commands : draw_line, draw_rect, draw_circle, draw_text
   └── Example :
       <canvas width="800" height="600" on_draw="render_canvas" />
```

#### Low Priority

```
⏳ TreeView
   ├── Attributes : nodes, expanded, on_toggle
   └── Example :
       <tree_view nodes="{tree_data}" on_toggle="expand_node" />
```

```
⏳ ColorPicker
   ├── Attributes : value, alpha, palettes
   └── Example :
       <color_picker value="{color}" on_change="set_color" />
```

### Advanced Binding System

```
⏳ Conditional Binding
   ├── Syntax : visible="{condition}", enabled="{condition}"
   └── Example :
       <button label="Delete" visible="{has_selection}" />
```

```
⏳ Dynamic Style Binding
   ├── Syntax : style_class="{condition ? 'class1' : 'class2'}"
   └── Example :
       <text value="{status}" style_class="{is_error ? 'error' : 'success'}" />
```

```
⏳ Computed Properties
   ├── Syntax : computed { full_name = first_name + ' ' + last_name }
   └── Example in XML :
       <text value="{computed.full_name}" />
```

```
⏳ Two-way Binding (TextInput)
   └── Example :
       <text_input value="{model.name}" />
       <!-- Modifications automatically update model.name -->
```

### Navigation and Views

```
⏳ Navigation Router
   ├── Attributes : routes, default_route, on_navigate
   └── Example :
       <router routes="/home,/settings,/profile" on_navigate="handle_route" />
```

```
⏳ Include
   ├── Attributes : src, binding
   └── Example :
       <include src="components/header.dampen" />
       <include src="components/footer.dampen" model="{footer_model}" />
```

```
⏳ Slots / Children
   ├── Attributes : slot, children
   └── Example :
       <card>
           <header>My Card Title</header>
           <body>Card content here</body>
       </card>
```

---

## Technical Improvements

### Performance

| Improvement | Priority | Target |
|-------------|----------|--------|
| Persistent IR Cache | High | < 50ms for re-parse |
| Parallel Parsing | Medium | Use all CPU cores |
| Widget Pooling | Medium | Reduce dynamic allocations |
| Incremental Updates | Low | Partial DOM update |

### Code Quality

| Improvement | Priority | Description |
|--------------|----------|-------------|
| 100% Test Coverage | High | Current > 90% → 100% |
| API Documentation | High | All public items documented |
| Fuzzing Tests | Medium | Parser fuzzing tests |
| Property-based Tests | Medium | proptest tests |

### Backend Agnostic

```
⏳ Backend Abstraction Layer (enhancement)
   ├── Common interface for all backends
   ├── Iced backend support (complete)
   └── Alternative backend planning :
       ├── iced-x86_64-unknown-linux-gnu (current)
       ├── iced (wasm32-unknown-unknown) → Phase 5
       └── winit + raw_window_handle (future)
```

---

## Ecosystem and Tools

### CLI Enhancements

```
⏳ dampen init
   └── Initialize Dampen in existing Rust project

⏳ dampen add widget <name>
   └── Add new widget to project

⏳ dampen generate component <name>
   └── Generate component boilerplate

⏳ dampen validate
   └── Validate complete project (XML + Rust)

⏳ dampen doc
   └── Generate project documentation
```

### Supported Editors

| Editor | Status | Support |
|--------|--------|---------|
| VS Code | 🔲 | Extension planned |
| RustRover | 🔲 | IDEA plugin |
| Zed | 🔲 | LSP configuration |
| Emacs | 🔲 | dampen mode |

### Templates

```
⏳ Template Application
   └── Complete application with navigation

⏳ Template Dashboard
   └── Application with sidebar dashboard

⏳ Template Settings
   └── System preferences application

⏳ Template TodoApp
   └── Complete todo list application
```

---

## Quality Goals

### Performance

| Metric | Target | Current |
|--------|--------|---------|
| Boot time (codegen) | < 100ms | ⏳ |
| Boot time (interpreted) | < 50ms | ⏳ |
| Hot-reload latency | < 300ms | ✓ |
| Memory footprint | < 50MB | ⏳ |
| Binary size (release) | < 10MB | ⏳ |

### Stability

| Criterion | Target | Current |
|-----------|--------|---------|
| Unit tests | > 95% coverage | > 90% |
| Integration tests | 100% passing | 100% |
| Breaking changes / version | 0 | ⏳ |
| Critical bugs | 0 | 0 |

### Compatibility

| Criterion | Target |
|-----------|--------|
| MSRV Rust | 1.85+ |
| Desktop platforms | Linux, macOS, Windows |

---

## Estimated Timeline

```
Version 0.2.0 (Q1 2026)

Version 0.3.0 (Q2 2026)

Version 0.4.0 (Q3 2026)

Version 0.5.0 (Q4 2026)

Version 1.0.0 (2027)
```

> **Note**: This timeline is indicative and subject to adjustments based on community feedback and emerging priorities.

---

## Contributing

Would you like to contribute to Dampen? Here's how to get started:

### Before You Start

1. Read the [contribution guide](docs/CONTRIBUTING.md)
2. Check [open issues](https://github.com/mattdef/dampen/issues)
3. Join our [GitHub discussions](https://github.com/mattdef/dampen/discussions)

### Getting Started

```bash
# Clone the repository
git clone https://github.com/mattdef/dampen.git
cd dampen

# Fetch dependencies
cargo fetch

# Build the project
cargo build --workspace

# Run tests
cargo test --workspace

# Run examples
cargo run -p hello-world
```

### Suggestions

Features not listed can be proposed via:

- [GitHub Discussions](https://github.com/mattdef/dampen/discussions) for ideas
- [GitHub Issues](https://github.com/mattdef/dampen/issues) for bugs
- [Pull Requests](https://github.com/mattdef/dampen/pulls) for code

---

<div align="center">

**Thank you for contributing to Dampen!** 🙏

*Together, let's build the future of UI development in Rust.*

</div>

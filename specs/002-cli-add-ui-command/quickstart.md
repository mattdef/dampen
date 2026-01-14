# Quickstart Guide: CLI Add UI Command

**Feature**: 002-cli-add-ui-command  
**Audience**: Dampen application developers  
**Last Updated**: 2026-01-13

## Overview

The `dampen add --ui` command generates boilerplate code for new UI windows in your Dampen application. It creates two files:
- **`.rs` file**: Rust module with Model, handlers, and auto-loading setup
- **`.dampen` file**: XML UI definition with example widgets

## Prerequisites

- Dampen project (created with `dampen new`)
- Current directory must be within a Dampen project
- `src/ui/` directory recommended (created automatically if missing)

## Basic Usage

### Generate a UI Window

```bash
# From project root
dampen add --ui settings
```

**Output**:
```
Created new UI window: settings
  - src/ui/settings.rs
  - src/ui/settings.dampen

Next steps:
  1. Add 'pub mod settings;' to src/ui/mod.rs
  2. Use in your application's view function
  3. Run 'cargo build' to compile
```

**Generated files**:

`src/ui/settings.rs`:
```rust
// Auto-loaded UI module for settings.
use dampen_core::{AppState, HandlerRegistry};
use dampen_macros::{UiModel, dampen_ui};
use serde::{Deserialize, Serialize};

#[dampen_ui("settings.dampen")]
mod _app {}

#[derive(Default, UiModel, Serialize, Deserialize, Clone, Debug)]
pub struct Model {
    pub message: String,
}

pub fn create_app_state() -> AppState<Model> {
    let document = _app::document();
    let handler_registry = create_handler_registry();
    AppState::with_handlers(document, handler_registry)
}

pub fn create_handler_registry() -> HandlerRegistry {
    let registry = HandlerRegistry::new();
    registry.register_simple("example_handler", |model: &mut dyn std::any::Any| {
        if let Some(m) = model.downcast_mut::<Model>() {
            m.message = "Handler triggered!".to_string();
        }
    });
    registry
}
```

`src/ui/settings.dampen`:
```xml
<?xml version="1.0" encoding="UTF-8" ?>
<dampen version="1.0">
    <column padding="40" spacing="20">
        <text value="Settings" size="32" weight="bold" />
        <button label="Click me!" on_click="example_handler" />
        <text value="{message}" size="24" />
    </column>
</dampen>
```

### Integrate into Application

1. **Export the module** in `src/ui/mod.rs`:
   ```rust
   pub mod window;  // existing
   pub mod settings; // add this line
   ```

2. **Use in your application** (e.g., `src/main.rs`):
   ```rust
   use crate::ui::settings;
   
   fn view(&self) -> Element<Message> {
       let settings_state = settings::create_app_state();
       // ... render settings view
   }
   ```

3. **Build and run**:
   ```bash
   cargo build
   cargo run
   ```

## Advanced Usage

### Custom Output Directory

Create windows in subdirectories for better organization:

```bash
# Create in src/ui/admin/
dampen add --ui user_management --path "src/ui/admin/"
```

**Output**:
```
Created new UI window: user_management
  - src/ui/admin/user_management.rs
  - src/ui/admin/user_management.dampen
```

**Directory structure**:
```
src/ui/
├── mod.rs
├── window.rs
├── window.dampen
└── admin/
    ├── user_management.rs
    └── user_management.dampen
```

**Module export** (`src/ui/mod.rs`):
```rust
pub mod window;

pub mod admin {
    pub mod user_management;
}
```

### Naming Conventions

The command automatically converts names to `snake_case`:

```bash
# Various input styles
dampen add --ui MyWindow        # → my_window.rs
dampen add --ui UserProfile     # → user_profile.rs
dampen add --ui my-window       # → my_window.rs
dampen add --ui user_profile    # → user_profile.rs (unchanged)
```

**In templates**:
- Rust files use `snake_case`: `user_profile`
- Struct names use `PascalCase`: `UserProfile`
- UI text uses `Title Case`: `User Profile`

### Multiple Windows Example

Create a multi-view application:

```bash
# Main dashboard
dampen add --ui dashboard

# Settings view
dampen add --ui settings

# User profile view
dampen add --ui user_profile
```

**Module structure** (`src/ui/mod.rs`):
```rust
pub mod window;
pub mod dashboard;
pub mod settings;
pub mod user_profile;
```

**View switching** (`src/main.rs`):
```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CurrentView {
    Dashboard,
    Settings,
    UserProfile,
}

fn view(&self) -> Element<Message> {
    match self.current_view {
        CurrentView::Dashboard => dashboard::create_app_state().view(),
        CurrentView::Settings => settings::create_app_state().view(),
        CurrentView::UserProfile => user_profile::create_app_state().view(),
    }
}
```

## Common Workflows

### Workflow 1: Add Feature Module

**Scenario**: Adding a new feature with its own UI.

```bash
# Create feature directory structure
mkdir -p src/ui/orders

# Generate order creation window
dampen add --ui new_order --path "src/ui/orders/"

# Generate order listing window
dampen add --ui order_list --path "src/ui/orders/"
```

**Result**:
```
src/ui/orders/
├── new_order.rs
├── new_order.dampen
├── order_list.rs
└── order_list.dampen
```

### Workflow 2: Rapid Prototyping

**Scenario**: Quickly scaffold multiple screens for prototyping.

```bash
# Dashboard views
dampen add --ui home
dampen add --ui analytics
dampen add --ui reports

# User management
dampen add --ui users --path "src/ui/admin/"
dampen add --ui roles --path "src/ui/admin/"
```

**Time savings**: ~5 minutes per window → ~10 seconds per window

### Workflow 3: Team Onboarding

**Scenario**: New team member adding their first UI component.

```bash
# Step 1: Clone project
git clone <repo>
cd project

# Step 2: Generate new component
dampen add --ui my_feature

# Step 3: Verify it compiles
cargo build

# Step 4: Customize
vim src/ui/my_feature.rs
vim src/ui/my_feature.dampen
```

**Benefits**: Consistent structure, working template, faster onboarding.

## Error Handling

### Error: Not a Dampen Project

**Symptom**:
```
Error: Not a Dampen project: Cargo.toml not found
help: Run 'dampen new <project_name>' to create a new Dampen project
```

**Cause**: Command run outside a Rust project.

**Solution**: Navigate to project root (directory with `Cargo.toml`).

```bash
cd /path/to/my-dampen-project
dampen add --ui settings
```

### Error: Missing Dampen Dependency

**Symptom**:
```
Error: Not a Dampen project: dampen-core dependency not found in Cargo.toml
help: Add dampen-core to [dependencies] or run 'dampen new' to create a new project
```

**Cause**: In a Rust project but not a Dampen project.

**Solution**: Add Dampen dependencies to `Cargo.toml`:
```toml
[dependencies]
dampen-core = "0.1"
dampen-macros = "0.1"
dampen-iced = "0.1"
```

### Error: Window Already Exists

**Symptom**:
```
Error: Window 'settings' already exists at src/ui/settings.rs
help: Choose a different name or remove the existing file first
```

**Cause**: Attempting to create a window with an existing name.

**Solution 1** - Use a different name:
```bash
dampen add --ui settings_v2
```

**Solution 2** - Remove existing file:
```bash
rm src/ui/settings.rs src/ui/settings.dampen
dampen add --ui settings
```

### Error: Invalid Window Name

**Symptom**:
```
Error: Invalid window name 'my window': contains invalid characters
help: Use only letters, numbers, and underscores. Examples: settings, user_profile, my_window
```

**Cause**: Window name contains spaces or special characters.

**Solution**: Use valid identifier syntax:
```bash
# Invalid
dampen add --ui "my window"
dampen add --ui "user-profile!"

# Valid
dampen add --ui my_window
dampen add --ui user_profile
```

### Error: Path Outside Project

**Symptom**:
```
Error: Path '../outside/' is outside the project directory
help: Use a relative path within the project, e.g., 'src/ui/orders/'
```

**Cause**: Custom path escapes project root.

**Solution**: Use relative paths within project:
```bash
# Invalid
dampen add --ui settings --path "../other-project/"

# Valid
dampen add --ui settings --path "src/ui/"
dampen add --ui settings --path "ui/admin/"
```

## Tips & Best Practices

### Naming Conventions

✅ **Good**:
- `dashboard` - Simple, clear
- `user_profile` - Descriptive, snake_case
- `order_details` - Feature-specific

❌ **Avoid**:
- `my_window` - Too generic
- `temp` - Unclear purpose
- `test123` - Non-descriptive

### Project Organization

**Small projects** (1-5 windows):
```
src/ui/
├── mod.rs
├── window.rs         # Main window
├── settings.rs       # Settings
└── about.rs          # About
```

**Medium projects** (6-15 windows):
```
src/ui/
├── mod.rs
├── dashboard/
│   ├── home.rs
│   └── analytics.rs
├── user/
│   ├── profile.rs
│   └── settings.rs
└── admin/
    ├── users.rs
    └── roles.rs
```

**Large projects** (15+ windows):
```
src/
├── ui/
│   ├── mod.rs
│   ├── common/        # Shared components
│   ├── dashboard/     # Dashboard feature
│   ├── orders/        # Orders feature
│   ├── inventory/     # Inventory feature
│   └── admin/         # Admin feature
```

### Template Customization

After generation, customize the templates:

1. **Model fields**: Replace `message: String` with your actual model
2. **Handlers**: Replace `example_handler` with real business logic
3. **UI layout**: Modify XML to match your design
4. **Validation**: Add field validation logic

**Example customization**:

Before (generated):
```rust
pub struct Model {
    pub message: String,
}
```

After (customized):
```rust
pub struct Model {
    pub username: String,
    pub email: String,
    pub age: Option<u32>,
    pub is_active: bool,
}
```

### Version Control

**Commit generated files** with meaningful message:
```bash
dampen add --ui settings
git add src/ui/settings.rs src/ui/settings.dampen
git commit -m "feat: add settings UI window"
```

**Modify in separate commit**:
```bash
# Edit files...
git add src/ui/settings.rs src/ui/settings.dampen
git commit -m "feat(settings): implement user preferences"
```

## Integration with Other Commands

### With `dampen check`

Validate generated XML:
```bash
dampen add --ui settings
dampen check src/ui/settings.dampen
```

### With `dampen build`

Build project with new window:
```bash
dampen add --ui settings
dampen build
```

### With `dampen run`

Run application with new window:
```bash
dampen add --ui settings
# ... integrate into app ...
dampen run
```

## Troubleshooting

**Q: Command not found**  
A: Ensure `dampen` CLI is installed and in PATH:
```bash
cargo install dampen-cli
```

**Q: Files not compiling**  
A: Check that you've added module export in `mod.rs`:
```rust
pub mod settings; // Add this
```

**Q: XML validation fails**  
A: Ensure XML is well-formed and uses valid Dampen widgets. Run:
```bash
dampen check src/ui/yourwindow.dampen
```

**Q: Path not created**  
A: Command automatically creates directories. Check permissions:
```bash
ls -la src/ui/
```

## Next Steps

- Customize Model struct with your application state
- Add handlers for user interactions
- Modify XML layout to match your design
- Run `dampen check` to validate XML
- Build and test your application

## References

- [Dampen User Guide](../../docs/USAGE.md)
- [Feature Specification](./spec.md)
- [Data Model](./data-model.md)
- [Template Contracts](./contracts/)

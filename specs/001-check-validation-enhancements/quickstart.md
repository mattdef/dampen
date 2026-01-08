# Quickstart: Enhanced gravity check Command

**Feature**: Check Validation Enhancements  
**Date**: 2026-01-08

This guide explains how to use the enhanced `gravity check` command with its new validation features.

## Installation

The enhanced check command is part of gravity-cli. Ensure you have the latest version:

```bash
cargo install gravity-cli
```

Or build from source:

```bash
cd crates/gravity-cli
cargo build --release
```

## Basic Usage

### Validate UI Files

```bash
gravity check
# Validates all .gravity files in ./ui directory

gravity check --input ./src/ui
# Validates files in specified directory

gravity check --input ./src/ui --verbose
# Shows detailed progress
```

### Enable Strict Mode

```bash
gravity check --strict
# Fails on warnings (recommended for CI/CD)
```

## Advanced Usage

### Handler Registry Validation

Validate that event handlers referenced in XML exist in your registered handlers:

```bash
gravity check --handlers ./target/handlers.json
```

**Generate handler registry** (from your Rust code):

```rust
// Use the #[ui_handler] macro to automatically generate handler registry
use gravity_macros::ui_handler;

#[ui_handler]
fn increment(model: &mut Model) {
    model.count += 1;
}

#[ui_handler]
fn decrement(model: &mut Model) {
    model.count -= 1;
}
```

This generates a `handlers.json` file that `gravity check` uses for validation.

---

### Model Binding Validation

Validate that data bindings reference existing model fields:

```bash
gravity check --model ./target/model.json
```

**Generate model info** (from your Rust code):

```rust
use gravity_macros::derive_uimodel;

#[derive(UiModel)]
pub struct Model {
    pub count: i32,
    pub user: User,
}

#[derive(UiModel)]
pub struct User {
    pub name: String,
    pub email: String,
}
```

This generates a `model.json` file describing all available binding paths.

---

### Custom Widget Attributes

Allow custom widgets to have user-defined attributes via config:

```bash
gravity check --custom-widgets ./custom-widget-config.json
```

**Config file format** (`custom-widget-config.json`):

```json
{
  "MyCustomWidget": {
    "allowed_attributes": ["value", "mode", "format"]
  },
  "DataGrid": {
    "allowed_attributes": ["columns", "rows", "sortable"]
  }
}
```

---

### Combined Validation

Use all validation features together:

```bash
gravity check \
  --input ./ui \
  --handlers ./target/handlers.json \
  --model ./target/model.json \
  --custom-widgets ./custom-widget-config.json \
  --strict \
  --verbose
```

## Error Examples

### Unknown Attribute (with suggestion)

```bash
$ gravity check
Error: Unknown attribute 'on_clik' for button in ui/main.gravity:10:5
  Did you mean 'on_click'? (distance: 1)
```

### Missing Required Attribute

```bash
$ gravity check
Error: Missing required attribute 'value' for Text in ui/main.gravity:15:2
```

### Unknown Handler

```bash
$ gravity check --handlers handlers.json
Error: Unknown handler 'incremnt' in ui/main.gravity:20:8
  Available handlers: increment, decrement, setValue (3 total)
```

### Invalid Binding

```bash
$ gravity check --model model.json
Error: Invalid binding field 'user.nme' in ui/profile.gravity:7:12
  Available fields: user.name, user.email, count, enabled (4 total)
```

### Radio Group Issues

```bash
$ gravity check
Error: Duplicate radio value 'option1' in group 'size' at ui/form.gravity:25:9
  First occurrence: ui/form.gravity:20:5
```

## Exit Codes

| Code | Meaning |
|------|---------|
| 0 | Validation passed (no errors) |
| 1 | Validation failed (one or more errors found) |
| 2 | Invalid arguments or missing files |

## Integration with CI/CD

Add to your CI pipeline:

```yaml
# GitHub Actions example
- name: Validate UI Files
  run: |
    gravity check \
      --handlers target/handlers.json \
      --model target/model.json \
      --strict
```

## Performance

Validation completes in under 1 second for typical projects (100-500 widgets). For very large projects, a warning is displayed if processing exceeds this threshold.

## Troubleshooting

### "handlers.json not found"

Ensure you've generated the handler registry. Run your build first, or generate manually:

```bash
gravity check --generate-handlers > handlers.json
```

### "model.json not found"

Similar to handlers, ensure your model is generated. The `#[derive(UiModel)]` macro outputs this file during compilation.

### Custom widget attributes flagged as unknown

Create a custom widget config file and pass it with `--custom-widgets`.

### Validation seems slow

For large codebases, consider:
- Using `--no-cache` to skip cached results
- Running validation only on changed files

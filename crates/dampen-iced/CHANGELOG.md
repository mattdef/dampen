# Changelog: dampen-iced

All notable changes to the dampen-iced crate will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.2.7] - 2026-01-21

### Bug Fixes

- **CRITICAL**: Fixed conditional attributes (e.g., `enabled="{count > 0}"`) not evaluating binding expressions. The `resolve_boolean_attribute()` helper was returning the default value instead of evaluating bindings, causing buttons and other widgets to ignore dynamic enabled/disabled states.

## [0.2.7-beta] - 2026-01-21

### Added

- **Helper Functions**: Three new helper functions extracted to eliminate code duplication:
  - `resolve_boolean_attribute()`: Parse boolean attributes with multiple format support ("true", "1", "yes", "on", etc.)
  - `resolve_handler_param()`: Resolve event handler parameters with rich error context
  - `create_state_aware_style_fn()`: Generic state-aware styling for hover, focus, active, disabled states

- **State-Aware Styling**: Slider widget now supports state-aware styling (hover, focus, active, disabled)

- **Performance Optimization**: StyleClass wrapped in `Rc` for efficient cloning in style closures

### Changed

- **Deprecated `IcedBackend`**: The legacy `IcedBackend` trait is now deprecated and will be removed in v0.3.0. Use `DampenWidgetBuilder` instead.

- **Verbose Logging**: Logging is now compile-time gated via `#[cfg(debug_assertions)]`. Debug output is automatically stripped from release builds, eliminating all logging overhead in production.

- **Improved Error Messages**: Handler resolution errors now include helpful suggestions for common mistakes.

### Removed

- **Verbose Runtime Flag**: The `with_verbose()` method and `verbose` field have been removed. Debug logging is now automatic in debug builds.

### Performance

- **Code Reduction**: ~370+ lines of duplicated code eliminated
- **Memory**: Rc-wrapped StyleClass reduces clone overhead (~47x faster clone)
- **Binary Size**: No logging code in release builds

### Migration from v0.2.6

If you're using `IcedBackend`:

```rust
// v0.2.6 (deprecated)
let backend = IcedBackend::new(|name, param| {
    Message::Handler(name, param)
});
let widget = backend.render(&document);
```

```rust
// v0.2.7 (recommended)
use dampen_iced::DampenWidgetBuilder;

let builder = DampenWidgetBuilder::new(
    &document,
    &model,
    Some(&handler_registry),
);
let element = builder.build();
```

### Breaking Changes

- `IcedBackend` is deprecated (still works but will be removed in v0.3.0)
- `with_verbose()` method removed (debug logging is automatic in debug builds)

### Bug Fixes

- Fixed collapsible if statement lint errors across the crate
- Fixed dead code warnings for unused helper methods

## [0.2.6] - 2025-12-15

### Initial Release

- Initial dampen-iced crate implementation
- Basic widget building support
- Binding and event handling
- Style and layout application

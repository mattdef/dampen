# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

#### Dual-Mode Architecture (Feature 001)
- **Interpreted Mode**: Runtime XML parsing with hot-reload support for rapid development
  - `dampen-dev` crate with file watching, hot-reload coordination, and error overlays
  - `FileWatcher` with 100ms debouncing for efficient file change detection
  - `HotReloadContext` with AST caching and performance metrics tracking
  - Async XML parsing via `tokio::spawn_blocking` for non-blocking UI
  - State preservation across reloads (model data maintained)
  - <300ms hot-reload latency for typical UIs (<1000 widgets)
- **Codegen Mode**: Build-time code generation for zero runtime overhead in production
  - Static IR construction via `build.rs` integration
  - No runtime XML parser included in release binaries
  - <1ms initialization time for all UI sizes
  - ~7MB memory savings vs interpreted mode
- **Automatic Mode Selection**: Profile-based feature flag configuration
  - `cargo run` → interpreted mode (development with hot-reload)
  - `cargo build --release` → codegen mode (production optimized)
  - Manual override via `--features` flag
- **Mode Parity**: Comprehensive testing ensures identical UI behavior across modes
  - Contract tests for parse, binding, handler, and widget parity
  - Integration test suite in `tests/integration/mode_parity_tests.rs`
- **Performance Optimizations**:
  - LRU AST cache with 100-entry limit
  - Async parsing for large XML files
  - Performance metrics tracking (reload count, latency, cache hit rate)
  - Comprehensive benchmark suite (`benchmarks/`)
- **Edge Case Handling**:
  - Graceful handling of deleted files during watch
  - Permission change detection
  - Simultaneous multi-file changes with debouncing
  - Circular dependency validation (placeholder for future)
  - 19 edge case integration tests
- **Documentation**:
  - Migration guide (`docs/migration/dual-mode.md`)
  - Developer guide (`docs/development/dual-mode.md`)
  - Performance documentation (`docs/performance.md`)
  - 10 common workflow examples in quickstart
  - Updated README with dual-mode section

#### Workspace & Tooling
- Workspace dependencies management for consistent versioning
- Automatic version injection in `dampen new` templates
- GitHub Actions workflow for automatic crates.io publishing
- Release script (`scripts/release.sh`) for automated releases
- Comprehensive release documentation (`docs/RELEASE.md`)

### Changed
- All crates now use `{ workspace = true }` for dependencies
- Template versions are auto-generated from workspace versions
- `#[dampen_ui]` macro now supports both interpreted and codegen modes via feature flags
- Examples migrated to dual-mode architecture (hello-world, counter, todo-app, etc.)
- Code quality improvements: all clippy warnings fixed, rustfmt applied

### Fixed
- Version consistency across workspace crates
- Clippy warnings in build scripts, macros, and examples
- Unused variable warnings via targeted `#[allow(dead_code)]`
- Clone performance issues (removed unnecessary `.clone()` calls)

## [0.1.0] - 2025-01-09

### Added
- Initial release
- XML-based declarative UI framework
- Iced backend implementation
- `#[derive(UiModel)]` macro for data binding
- `#[dampen_ui]` macro for automatic XML loading
- `dampen-core`: Parser, IR, and trait definitions
- `dampen-macros`: Proc macros for code generation
- `dampen-iced`: Iced widget mapping
- `dampen-cli`: Developer CLI tool
  - `dampen new` - Create new projects
  - `dampen check` - Validate XML syntax
  - `dampen inspect` - Inspect IR
  - `dampen build` - Build projects
- Widget support:
  - Text, Button, Column, Row, Container
  - TextInput, Checkbox, Radio
  - Scrollable, Image, SVG
- Expression system for data binding
- Handler system for events
- Examples: hello-world, counter, todo-app, widget-showcase, styling, responsive, settings

### Documentation
- Comprehensive README
- Project structure guidelines (CLAUDE.md)
- Getting started guide
- API documentation

[Unreleased]: https://github.com/mattdef/dampen/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/mattdef/dampen/releases/tag/v0.1.0

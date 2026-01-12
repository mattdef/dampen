# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

#### Multi-View Application Macro (Feature 001-dampen-app-macro)
- **#[dampen_app] Procedural Macro**: Eliminates 90% of boilerplate code in multi-view applications
  - Auto-discovers `.dampen` files in specified UI directory via file system scanning
  - Generates `CurrentView` enum with variants for each discovered view
  - Generates application struct with typed `AppState<T>` fields for each view
  - Generates `init()` and `new()` methods with automatic view initialization
  - Generates `update()` method with handler dispatch and view switching logic
  - Generates `view()` method with per-view rendering and error overlay support
  - Generates `subscription()` method with hot-reload file watching
  - Generates `switch_to_*()` helper methods for type-safe view transitions
- **Automatic View Discovery**: Zero manual registration required
  - Recursively scans `ui_dir` for `.dampen` files at compile-time
  - Validates matching `.rs` files with `Model` struct and `create_handlers()` function
  - Preserves nested directory structure in module paths
  - Deterministic ordering (alphabetical) for consistent builds
- **Selective Exclusion**: Fine-grained control over view discovery
  - `exclude` attribute with glob pattern support (e.g., `["debug/*", "experimental/**"]`)
  - Compile-time validation of glob patterns with actionable error messages
- **Hot-Reload Integration**: Seamless development workflow
  - Automatic file watching for all discovered views in debug builds
  - Error overlay with dismissible UI in development mode
  - Conditional compilation via `#[cfg(debug_assertions)]` for zero overhead in production
- **View Switching**: Type-safe navigation without manual routing
  - Optional `switch_view_variant` attribute for multi-view applications
  - Automatic generation of view transition methods
  - Optional `default_view` parameter to control startup view (defaults to first alphabetically)
- **Comprehensive Error Messages**: Clear, actionable compile-time diagnostics
  - File path and line number in all errors
  - Suggested fixes for common issues (missing .rs file, naming conflicts, invalid paths)
  - Validated at compile-time: required attributes, valid Rust identifiers, glob patterns
- **Production Optimizations**:
  - Hot-reload code automatically stripped in release builds
  - Zero runtime overhead (compile-time code generation only)
  - <200ms compilation overhead for 20-view applications
- **Documentation**:
  - Comprehensive quickstart guide (`specs/001-dampen-app-macro/quickstart.md`)
  - Migration guide from manual boilerplate pattern
  - Rustdoc comments for all public API
  - Updated USAGE.md with multi-view section
- **Testing**: 77 tests for macro functionality
  - Unit tests for view discovery, validation, and code generation
  - Compile-fail tests with trybuild for error message validation
  - Integration tests with real multi-view applications
  - Snapshot tests for generated code quality

#### Schema Version Validation (Feature 001-schema-version-validation)
- **Version Attribute Parsing**: Parser now reads and validates `version` attribute on `<dampen>` root element
  - Support for `major.minor` format (e.g., "1.0")
  - Maximum supported version: 1.0 (defined in `MAX_SUPPORTED_VERSION` constant)
  - Backward compatible: Files without version default to 1.0
- **Version Validation**: Parser rejects unsupported future versions with clear error messages
  - New `UnsupportedVersion` error variant in `ParseErrorKind`
  - Error messages include declared version, max supported version, and upgrade suggestions
  - Span information for precise error location in source files
- **Format Validation**: Strict validation of version string format
  - Rejects invalid formats (e.g., "1", "v1.0", "1.0.5", "1.0-beta")
  - Clear error messages with expected format guidance
  - Handles edge cases: whitespace, leading zeros, empty strings
- **Widget Version Infrastructure**: Foundation for version-gated widgets
  - `WidgetKind::minimum_version()` method returns required schema version for each widget
  - Canvas widget marked as v1.1 (experimental, non-functional)
  - All other widgets return v1.0
- **Widget Version Validation**: `dampen check` warns about widget-version compatibility
  - New `validate_widget_versions()` function with recursive tree traversal
  - `ValidationWarning` struct with detailed error messages and suggestions
  - Warnings displayed by default (non-blocking for development workflows)
  - Example: "Widget 'canvas' requires schema v1.1 but document declares v1.0"
- **CLI Enhancements**:
  - `dampen check --show-widget-versions` displays widget version requirements table
  - Shows minimum version and status (Stable/Experimental) for all widgets
- **File Updates**: All example files and templates now explicitly declare `version="1.0"`
  - 26+ `.dampen` files updated across examples/ and templates/
  - CLI `dampen new` generates files with version attribute
  - Test fixtures and inline test XML updated with proper escaping
- **Documentation**:
  - Updated XML_SCHEMA.md with comprehensive version attribute documentation
  - Added troubleshooting section with version error examples and solutions
  - Widget version warning documentation with clear examples
  - Quickstart guide (`specs/001-schema-version-validation/quickstart.md`)
  - Version validation contract tests (34+ tests in `version_tests.rs`)

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

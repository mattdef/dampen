# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Workspace dependencies management for consistent versioning
- Automatic version injection in `dampen new` templates
- GitHub Actions workflow for automatic crates.io publishing
- Release script (`scripts/release.sh`) for automated releases
- Comprehensive release documentation (`docs/RELEASE.md`)

### Changed
- All crates now use `{ workspace = true }` for dependencies
- Template versions are auto-generated from workspace versions

### Fixed
- Version consistency across workspace crates

## [0.1.0] - 2025-01-09

### Added
- Initial release
- XML-based declarative UI framework
- Iced backend implementation
- `#[derive(UiModel)]` macro for data binding
- `#[dampen_ui]` macro for automatic XML loading
- `dampen-core`: Parser, IR, and trait definitions
- `dampen-macros`: Proc macros for code generation
- `dampen-runtime`: Runtime interpreter and state management
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

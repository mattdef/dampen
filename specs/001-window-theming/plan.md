# Implementation Plan: Window Theming

**Branch**: `001-window-theming` | **Date**: 2026-01-16 | **Spec**: [spec.md](./spec.md)
**Input**: Feature specification from `/specs/001-window-theming/spec.md`

## Summary

Add a complete theming system to Dampen applications with a global `theme.dampen` file at `src/ui/theme/theme.dampen`. The system must be fully compatible with both interpreted (hot-reload) and codegen modes, match Iced's Theme properties, and maintain backward compatibility for applications without theming.

## Technical Context

**Language/Version**: Rust Edition 2024, MSRV 1.85  
**Primary Dependencies**: 
- `iced` 0.14+ (UI backend with Theme support)
- `roxmltree` 0.19+ (XML parsing)
- `dampen-core` (IR types, parser, codegen)
- `dampen-iced` (Iced backend adapter)
- `dampen-dev` (hot-reload, file watcher)
- `dampen-macros` (proc macros for `#[dampen_app]`)

**Storage**: File-based (`src/ui/theme/theme.dampen`)  
**Testing**: `cargo test --workspace`, contract tests, integration tests  
**Target Platform**: Cross-platform desktop (Linux, macOS, Windows)  
**Project Type**: Multi-crate workspace  
**Performance Goals**: 
- Theme switching < 200ms
- Hot-reload update < 500ms
- XML parse < 10ms for theme file

**Constraints**: 
- Zero breaking changes to existing applications
- No `unsafe` code in generated output
- Must work in both interpreted and codegen modes

**Scale/Scope**: 
- Affects all built-in widgets (~25 widget types)
- Single global theme file per application
- Multi-window support (theme shared across all windows)

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

| Principle | Status | Notes |
|-----------|--------|-------|
| **I. Declarative-First** | ✅ PASS | Theme defined in XML (`theme.dampen`), parsed to IR |
| **II. Type Safety Preservation** | ✅ PASS | Theme types fully typed in `ir::theme`, no runtime erasure |
| **III. Production Mode** | ✅ PASS | Codegen compiles theme to Rust code at build time |
| **IV. Backend Abstraction** | ✅ PASS | Theme IR in `dampen-core`, Iced adapter in `dampen-iced` |
| **V. Test-First Development** | ⚠️ PENDING | Contract tests must be written before implementation |

**Gate Status**: PASS - All principles satisfied or pending test implementation.

## Project Structure

### Documentation (this feature)

```text
specs/001-window-theming/
├── plan.md              # This file
├── research.md          # Phase 0 output
├── data-model.md        # Phase 1 output
├── quickstart.md        # Phase 1 output
├── contracts/           # Phase 1 output
│   └── theme-api.md     # Theme API contracts
└── tasks.md             # Phase 2 output (via /speckit.tasks)
```

### Source Code (repository root)

```text
crates/
├── dampen-core/
│   └── src/
│       ├── ir/
│       │   └── theme.rs          # Theme IR types (EXISTS - extend)
│       ├── parser/
│       │   └── theme_parser.rs   # Theme XML parser (EXISTS - extend)
│       └── codegen/
│           └── theme.rs          # Theme codegen (NEW)
├── dampen-iced/
│   └── src/
│       ├── theme_adapter.rs      # Iced theme adapter (EXISTS - implement)
│       └── builder/
│           └── widgets/*.rs      # Widget theming (MODIFY)
├── dampen-dev/
│   └── src/
│       └── watcher.rs            # File watcher (EXISTS - extend for theme.dampen)
├── dampen-macros/
│   └── src/
│       └── dampen_app.rs         # Macro (MODIFY for theme loading)
└── dampen-cli/
    └── templates/
        └── new/
            └── src/ui/theme/
                └── theme.dampen.template  # New project template (NEW)

tests/
├── contract/
│   └── theme_contracts.rs        # Theme contract tests (NEW)
└── integration/
    └── theme_e2e.rs              # Theme E2E tests (NEW)

examples/
└── styling/
    └── src/ui/
        └── theme/
            └── theme.dampen      # Move theme from window.dampen (REFACTOR)
```

**Structure Decision**: Single project (multi-crate workspace). Theme feature spans core, iced adapter, dev tools, and macros crates following existing architecture.

## Existing Infrastructure Analysis

### Already Implemented (Partial)

1. **Theme IR Types** (`dampen-core/src/ir/theme.rs`):
   - `Theme`, `ThemePalette`, `Typography`, `SpacingScale` structs
   - `StyleClass`, `WidgetState`, `StateSelector` for state-based styling
   - Validation methods for all theme components

2. **Theme Parser** (`dampen-core/src/parser/theme_parser.rs`):
   - `parse_theme()`, `parse_palette()`, `parse_typography()`
   - `parse_theme_from_node()` for XML parsing
   - `parse_style_class_from_node()` for style classes

3. **Theme Adapter** (`dampen-iced/src/theme_adapter.rs`):
   - `ThemeAdapter::to_iced()` - PLACEHOLDER (returns `IcedTheme::Light`)
   - `ThemeAdapter::text_style()` - PLACEHOLDER
   - `ThemeAdapter::font_size()` - PLACEHOLDER

4. **File Watcher** (`dampen-dev/src/watcher.rs`):
   - Full implementation with debouncing
   - Watches `.dampen` files in `src/ui/`
   - Channel-based event notification

### Not Yet Implemented

1. **Global Theme File Loading**: No support for `src/ui/theme/theme.dampen`
2. **Theme Context Propagation**: Widgets don't receive theme context
3. **Runtime Theme Switching**: No API for changing themes at runtime
4. **Theme Codegen**: No code generation for themes in production mode
5. **Iced Theme Mapping**: `ThemeAdapter` is placeholder only
6. **System Theme Detection**: No OS dark/light mode detection
7. **Theme Persistence**: No preference storage

## Complexity Tracking

No constitution violations requiring justification.

## Key Technical Decisions

### 1. Theme File Discovery

The theme file must be at a fixed location: `src/ui/theme/theme.dampen`. This enables:
- Predictable discovery without configuration
- Single source of truth for all windows
- Simple hot-reload watching

### 2. Theme Context Architecture

```
theme.dampen → Parser → Theme IR → ThemeContext → Widgets
                                        ↓
                              ThemeAdapter::to_iced()
                                        ↓
                              iced::Theme (for rendering)
```

### 3. Backward Compatibility Strategy

- If `src/ui/theme/theme.dampen` doesn't exist → use Iced default theme
- Existing `.dampen` files with inline themes → continue working (local scope)
- No changes required to existing applications

### 4. Dual-Mode Support

| Mode | Theme Behavior |
|------|---------------|
| Interpreted (`dampen run`) | Parse theme at startup, watch for changes, hot-reload |
| Codegen (`dampen build`) | Compile theme to Rust code, embed in binary |

### 5. Iced Theme Mapping

Map Dampen palette to Iced's `Theme::custom()`:
- `palette.primary` → `iced::theme::Palette::primary`
- `palette.background` → `iced::theme::Palette::background`
- `palette.text` → `iced::theme::Palette::text`
- etc.

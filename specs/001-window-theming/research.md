# Research: Window Theming

**Feature**: 001-window-theming  
**Date**: 2026-01-16  
**Status**: Complete

## Research Questions

### 1. Iced Theme API Structure

**Question**: How does Iced's Theme system work and what properties can be customized?

**Findings**:

Iced's theme system is built around:

1. **`iced::Theme` enum** - Built-in themes (Light, Dark, Dracula, Nord, etc.) plus Custom variant
2. **`iced::theme::Palette` struct** - Core color definitions with 6 fields:
   - `background: Color` - Application background
   - `text: Color` - Primary text color
   - `primary: Color` - Primary accent color
   - `success: Color` - Success state color
   - `warning: Color` - Warning state color  
   - `danger: Color` - Error/danger state color

3. **`iced::theme::palette::Extended`** - Extended palette with additional shades/variants

4. **Custom theme creation**:
   ```rust
   let palette = Palette {
       background: Color::from_rgb8(0xEC, 0xF0, 0xF1),
       text: Color::from_rgb8(0x2C, 0x3E, 0x50),
       primary: Color::from_rgb8(0x34, 0x98, 0xDB),
       success: Color::from_rgb8(0x27, 0xAE, 0x60),
       warning: Color::from_rgb8(0xF3, 0x9C, 0x12),
       danger: Color::from_rgb8(0xE7, 0x4C, 0x3C),
   };
   let theme = Theme::custom("MyTheme".to_string(), palette);
   ```

**Decision**: Map Dampen's `ThemePalette` to Iced's `Palette`. Dampen has additional colors (`secondary`, `surface`, `text_secondary`) that will be stored in extended metadata.

**Rationale**: Direct mapping ensures full Iced compatibility while Dampen's extra colors provide richer theming options.

**Alternatives Considered**:
- Use only Iced's 6 colors → Too limiting for branded apps
- Create completely custom style system → Incompatible with Iced widgets

---

### 2. Global Theme File Location

**Question**: Where should the global theme file be located?

**Findings**:

Current file watcher configuration (`dampen-dev/src/watcher.rs:22`):
```rust
pub fn default() -> Self {
    Self {
        watch_paths: vec![PathBuf::from("src/ui")],
        extension_filter: ".dampen".to_string(),
        recursive: true,
    }
}
```

**Decision**: `src/ui/theme/theme.dampen`

**Rationale**:
1. Within `src/ui/` - Already watched by file watcher (hot-reload works automatically)
2. Dedicated `theme/` subdirectory - Clear separation from window files
3. Single `theme.dampen` file - Simple discovery, no configuration needed
4. Matches user's explicit requirement from clarification session

**Alternatives Considered**:
- `dampen.theme` at project root → Outside watched directory, breaks hot-reload
- `src/theme.dampen` → Mixes with source code, less organized
- Configuration in `Cargo.toml` → Not declarative, can't hot-reload

---

### 3. Theme Context Propagation

**Question**: How should theme be propagated to widgets?

**Findings**:

Current `DampenWidgetBuilder` architecture (`dampen-iced/src/builder/mod.rs:65-86`):
- Builder holds references to document, model, and style_classes
- `build()` returns `Element<'a, HandlerMessage, Theme, Renderer>`
- Iced's `Theme` is already a type parameter in the widget tree

Iced propagates theme via:
1. Application-level: `iced::application(...).theme(|_| Theme::Dark)`
2. Widget-level: Each widget's `draw()` receives `&Theme`

**Decision**: Two-level propagation:
1. **Application level**: Set `iced::Theme` from global `theme.dampen`
2. **Widget level**: Pass `ThemeContext` through builder for Dampen-specific colors

**Rationale**: Leverages Iced's native theme system while allowing Dampen extensions.

**Alternatives Considered**:
- Pass theme to every widget explicitly → Too verbose, breaks existing API
- Global static → Not testable, can't have multiple themes in tests

---

### 4. Runtime Theme Switching API

**Question**: How should developers switch themes at runtime?

**Findings**:

From clarification session: Both bindings and handler actions required.

Existing handler patterns:
- `on_click="handler_name"` - Simple handler invocation
- `value="{model.field}"` - Binding expression

**Decision**: Dual API approach:

1. **Binding expression**:
   ```xml
   <window theme="{model.current_theme}">
   ```
   Model field determines active theme name.

2. **Handler action**:
   ```xml
   <button on_click="set_theme('dark')" />
   ```
   Built-in `set_theme(name)` pseudo-handler.

**Rationale**: Bindings for reactive state, handlers for imperative actions.

**Alternatives Considered**:
- Rust API only → Poor DX for simple use cases
- Custom message type → Breaks existing message patterns

---

### 5. System Theme Detection

**Question**: How to detect OS dark/light mode preference?

**Findings**:

Iced 0.14 provides system theme detection via `iced::window::settings::PlatformSpecific` or through the subscription system:
- Linux: `XDG_CURRENT_DESKTOP` + dbus for portal settings
- macOS: `NSAppearance` observation  
- Windows: Registry key monitoring

**Decision**: Use `dark_light` crate for cross-platform detection, fallback to "light".

**Rationale**: 
- `dark_light` is lightweight (~50 lines of code per platform)
- Works synchronously at startup
- Can poll for changes if needed

**Alternatives Considered**:
- Iced's built-in detection → Not exposed in public API as of 0.14
- Platform-specific code → Too much maintenance burden

---

### 6. Hot-Reload Integration

**Question**: How to integrate theme hot-reload with existing file watcher?

**Findings**:

Current flow (`dampen-dev/src/watcher.rs`, `dampen-dev/src/reload.rs`):
1. `FileWatcher` watches `src/ui/**/*.dampen`
2. On change, sends path via channel
3. `HotReloadSubscription` receives and triggers UI refresh

**Decision**: Extend existing watcher to:
1. Watch `src/ui/theme/theme.dampen` (automatic, already in `src/ui/`)
2. On theme file change, re-parse theme and update `ThemeContext`
3. Trigger full UI rebuild (theme affects all widgets)

**Rationale**: Minimal changes to existing infrastructure.

**Alternatives Considered**:
- Separate theme watcher → Unnecessary complexity
- Partial UI updates → Theme affects everything, not worth the complexity

---

### 7. Codegen Integration

**Question**: How should themes be compiled in production builds?

**Findings**:

Current codegen flow (`dampen-core/src/codegen/`):
1. `generate_application()` produces Rust code from `DampenDocument`
2. Generated code includes view function and update handlers
3. No theme handling currently

**Decision**: Add theme codegen:
1. Parse `theme.dampen` at build time
2. Generate `fn app_theme() -> iced::Theme` function
3. Include palette as compile-time constants

Generated code pattern:
```rust
pub fn app_theme() -> iced::Theme {
    iced::Theme::custom(
        "app_theme".to_string(),
        iced::theme::Palette {
            background: iced::Color::from_rgb8(0xEC, 0xF0, 0xF1),
            // ... other colors
        }
    )
}
```

**Rationale**: Zero runtime overhead, theme baked into binary.

**Alternatives Considered**:
- Embed XML and parse at startup → Adds runtime overhead
- External config file → Deployment complexity

---

### 8. Backward Compatibility

**Question**: How to ensure existing apps without theming continue to work?

**Findings**:

Current behavior:
- Apps have `.dampen` files with optional inline `<themes>` sections
- `DampenDocument` has `themes` and `global_theme` fields
- If no theme specified, Iced uses its default (Light)

**Decision**: Graceful degradation:
1. If `src/ui/theme/theme.dampen` missing → Use Iced default theme
2. If present but invalid → Log error, use Iced default theme
3. Existing inline themes in `.dampen` files → Continue working (local scope only)

**Rationale**: No breaking changes, additive feature only.

---

## Palette Mapping

| Dampen `ThemePalette` | Iced `Palette` | Notes |
|-----------------------|----------------|-------|
| `primary` | `primary` | Direct map |
| `background` | `background` | Direct map |
| `text` | `text` | Direct map |
| `success` | `success` | Direct map |
| `warning` | `warning` | Direct map |
| `danger` | `danger` | Direct map |
| `secondary` | (extended) | Stored in ThemeContext |
| `surface` | (extended) | Stored in ThemeContext |
| `text_secondary` | (extended) | Stored in ThemeContext |

## Sources

- [Iced Theme Documentation](https://docs.rs/iced/latest/iced/enum.Theme.html)
- [Iced Palette Struct](https://docs.rs/iced/latest/iced/theme/struct.Palette.html)
- [Iced Custom Theme](https://docs.rs/iced/latest/iced/widget/theme/struct.Custom.html)

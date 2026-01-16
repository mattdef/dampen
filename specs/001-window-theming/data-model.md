# Data Model: Window Theming

**Feature**: 001-window-theming  
**Date**: 2026-01-16

## Entity Overview

```
┌─────────────────┐      ┌──────────────────┐      ┌─────────────────┐
│  ThemeDocument  │──────│      Theme       │──────│  ThemePalette   │
└─────────────────┘      └──────────────────┘      └─────────────────┘
        │                        │                         │
        │                        │                         ├── primary
        │                        ├── name                  ├── secondary
        │                        ├── palette ─────────────►├── success
        │                        ├── typography            ├── warning
        │                        └── spacing               ├── danger
        │                                                  ├── background
        │                                                  ├── surface
        │                ┌──────────────────┐              ├── text
        │                │   Typography     │              └── text_secondary
        │                └──────────────────┘
        │                        │
        │                        ├── font_family
        │                        ├── font_size_base
        │                        ├── font_size_small
        │                        ├── font_size_large
        │                        ├── font_weight
        │                        └── line_height
        │
        │                ┌──────────────────┐
        │                │  SpacingScale    │
        │                └──────────────────┘
        │                        │
        │                        └── unit
        │
        ▼
┌─────────────────┐
│  ThemeContext   │ (Runtime state)
└─────────────────┘
        │
        ├── active_theme: String
        ├── themes: HashMap<String, Theme>
        ├── system_preference: Option<String>
        └── user_preference: Option<String>
```

## Entities

### ThemeDocument (NEW)

Represents the parsed content of `src/ui/theme/theme.dampen`.

```rust
/// Root document for theme.dampen file
/// Location: dampen-core/src/ir/theme.rs
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ThemeDocument {
    /// All defined themes (light, dark, custom, etc.)
    pub themes: HashMap<String, Theme>,
    
    /// Default theme name to use on startup
    /// If None, follows system preference
    pub default_theme: Option<String>,
    
    /// Whether to auto-detect system dark/light mode
    pub follow_system: bool,
}

impl ThemeDocument {
    /// Validate the document
    pub fn validate(&self) -> Result<(), String>;
    
    /// Get the effective default theme name
    /// Priority: user_preference > default_theme > system_preference > "light"
    pub fn effective_default(&self, system_pref: Option<&str>) -> &str;
}
```

**Validation Rules**:
- At least one theme must be defined
- If `default_theme` specified, it must exist in `themes`
- All themes must pass individual validation

### Theme (EXISTS - extend)

Already exists at `dampen-core/src/ir/theme.rs:12-40`. No structural changes needed.

```rust
/// Theme definition containing all visual properties
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Theme {
    pub name: String,
    pub palette: ThemePalette,
    pub typography: Typography,
    pub spacing: SpacingScale,
    pub base_styles: HashMap<String, StyleProperties>,
}
```

**Relationships**:
- Contains one `ThemePalette`
- Contains one `Typography`
- Contains one `SpacingScale`
- Contains zero or more `StyleProperties` (base widget styles)

### ThemePalette (EXISTS)

Already exists at `dampen-core/src/ir/theme.rs:44-57`. Maps to Iced's Palette with extensions.

```rust
/// Theme color palette
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ThemePalette {
    // Maps directly to Iced Palette
    pub primary: Color,      // → iced::theme::Palette::primary
    pub background: Color,   // → iced::theme::Palette::background
    pub text: Color,         // → iced::theme::Palette::text
    pub success: Color,      // → iced::theme::Palette::success
    pub warning: Color,      // → iced::theme::Palette::warning
    pub danger: Color,       // → iced::theme::Palette::danger
    
    // Dampen extensions (stored separately)
    pub secondary: Color,
    pub surface: Color,
    pub text_secondary: Color,
}
```

**Iced Mapping**:
```rust
impl ThemePalette {
    /// Convert to Iced's Palette (6 colors only)
    pub fn to_iced_palette(&self) -> iced::theme::Palette;
}
```

### Typography (EXISTS)

Already exists at `dampen-core/src/ir/theme.rs:59-80`. No changes needed.

### SpacingScale (EXISTS)

Already exists at `dampen-core/src/ir/theme.rs:118-133`. No changes needed.

### ThemeContext (NEW)

Runtime state for theme management. Lives in application memory.

```rust
/// Runtime theme context shared across all windows
/// Location: dampen-core/src/state/theme_context.rs (NEW FILE)
#[derive(Debug, Clone)]
pub struct ThemeContext {
    /// Currently active theme name
    active_theme: String,
    
    /// All loaded themes from theme.dampen
    themes: HashMap<String, Theme>,
    
    /// Detected system preference ("light" or "dark")
    system_preference: Option<String>,
    
    /// User-selected preference (persisted)
    user_preference: Option<String>,
    
    /// Cached Iced theme for rendering
    iced_theme: IcedTheme,
}

impl ThemeContext {
    /// Create from parsed ThemeDocument
    pub fn from_document(doc: ThemeDocument) -> Self;
    
    /// Get the currently active Theme
    pub fn active(&self) -> &Theme;
    
    /// Get the Iced theme for rendering
    pub fn iced_theme(&self) -> &IcedTheme;
    
    /// Switch to a different theme by name
    pub fn set_theme(&mut self, name: &str) -> Result<(), ThemeError>;
    
    /// Update from system preference change
    pub fn update_system_preference(&mut self, pref: &str);
    
    /// Reload themes from new document (hot-reload)
    pub fn reload(&mut self, doc: ThemeDocument);
}
```

**State Transitions**:
```
                     ┌──────────────┐
                     │   Initial    │
                     └──────┬───────┘
                            │ from_document()
                            ▼
┌───────────┐       ┌──────────────┐       ┌───────────┐
│  Reload   │◄──────│   Active     │──────►│  Switch   │
│  (parse)  │       │   (idle)     │       │  (name)   │
└───────────┘       └──────────────┘       └───────────┘
      │                    ▲                     │
      │                    │                     │
      └────────────────────┴─────────────────────┘
```

### Color (EXISTS)

Already exists at `dampen-core/src/ir/style.rs`. Used for all color values.

## XML Schema

### theme.dampen Structure

```xml
<?xml version="1.0" encoding="UTF-8" ?>
<dampen version="1.0">
    <!-- Theme definitions -->
    <themes>
        <theme name="light">
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
        
        <theme name="dark">
            <!-- Dark theme definition -->
        </theme>
    </themes>
    
    <!-- Default theme selection -->
    <default_theme name="light" />
    
    <!-- Optional: follow system preference -->
    <follow_system enabled="true" />
</dampen>
```

### Validation Rules

| Element | Required | Validation |
|---------|----------|------------|
| `<themes>` | Yes | At least one `<theme>` child |
| `<theme>` | Yes (1+) | Must have unique `name` attribute |
| `<palette>` | Yes | All 9 color attributes required |
| `<typography>` | No | Defaults to system values |
| `<spacing>` | No | Defaults to `unit="8"` |
| `<default_theme>` | No | If present, `name` must match a theme |
| `<follow_system>` | No | Defaults to `enabled="true"` |

### Error Messages

| Error Code | Condition | Message |
|------------|-----------|---------|
| `THEME_001` | No themes defined | "At least one theme must be defined in theme.dampen" |
| `THEME_002` | Invalid default | "Default theme '{name}' not found. Available: {list}" |
| `THEME_003` | Missing palette color | "Palette missing required color: {color}" |
| `THEME_004` | Invalid color value | "Invalid color '{value}' for {field}. Use hex (#RGB or #RRGGBB) or rgb(r,g,b)" |
| `THEME_005` | Duplicate theme name | "Duplicate theme name: '{name}'" |

## File Locations

| File | Purpose |
|------|---------|
| `dampen-core/src/ir/theme.rs` | Theme IR types (extend with `ThemeDocument`) |
| `dampen-core/src/parser/theme_parser.rs` | Theme XML parsing (extend for theme.dampen) |
| `dampen-core/src/state/theme_context.rs` | Runtime ThemeContext (NEW) |
| `dampen-iced/src/theme_adapter.rs` | Iced theme conversion (implement) |
| `dampen-dev/src/theme_loader.rs` | Theme file discovery and hot-reload (NEW) |

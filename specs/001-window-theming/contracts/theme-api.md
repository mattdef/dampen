# Theme API Contracts

**Feature**: 001-window-theming  
**Date**: 2026-01-16

This document defines the public API contracts for the theming system. These contracts serve as the basis for contract tests that must pass before implementation is considered complete.

---

## Contract 1: Theme Document Parsing

### Description
Parse a valid `theme.dampen` file into a `ThemeDocument` structure.

### Input
```xml
<?xml version="1.0" encoding="UTF-8" ?>
<dampen version="1.0">
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
        </theme>
    </themes>
    <default_theme name="light" />
</dampen>
```

### Expected Output
```rust
ThemeDocument {
    themes: {
        "light" => Theme {
            name: "light",
            palette: ThemePalette {
                primary: Color::from_hex("#3498db"),
                secondary: Color::from_hex("#2ecc71"),
                // ...
            },
            typography: Typography::default(),
            spacing: SpacingScale { unit: 8.0 },
        }
    },
    default_theme: Some("light"),
    follow_system: true,
}
```

### Contract Test
```rust
#[test]
fn contract_parse_valid_theme_document() {
    let xml = include_str!("fixtures/valid_theme.dampen");
    let result = parse_theme_document(xml);
    
    assert!(result.is_ok());
    let doc = result.unwrap();
    
    assert_eq!(doc.themes.len(), 1);
    assert!(doc.themes.contains_key("light"));
    assert_eq!(doc.default_theme, Some("light".to_string()));
}
```

---

## Contract 2: Theme Document Validation Errors

### Description
Invalid theme documents must produce specific, actionable error messages.

### Test Cases

| Input | Expected Error |
|-------|---------------|
| Empty `<themes>` | `THEME_001: At least one theme must be defined` |
| Missing palette color | `THEME_003: Palette missing required color: primary` |
| Invalid hex color | `THEME_004: Invalid color '#xyz' for primary` |
| Default theme not found | `THEME_002: Default theme 'missing' not found` |

### Contract Test
```rust
#[test]
fn contract_validation_missing_palette_color() {
    let xml = r#"
        <dampen version="1.0">
            <themes>
                <theme name="incomplete">
                    <palette primary="#3498db" />
                </theme>
            </themes>
        </dampen>
    "#;
    
    let result = parse_theme_document(xml);
    assert!(result.is_err());
    
    let err = result.unwrap_err();
    assert!(err.message.contains("missing required color"));
}
```

---

## Contract 3: Iced Theme Conversion

### Description
Convert Dampen `ThemePalette` to Iced's `Theme::custom()`.

### Input
```rust
ThemePalette {
    primary: Color::from_hex("#3498db"),
    secondary: Color::from_hex("#2ecc71"),
    success: Color::from_hex("#27ae60"),
    warning: Color::from_hex("#f39c12"),
    danger: Color::from_hex("#e74c3c"),
    background: Color::from_hex("#ecf0f1"),
    surface: Color::from_hex("#ffffff"),
    text: Color::from_hex("#2c3e50"),
    text_secondary: Color::from_hex("#7f8c8d"),
}
```

### Expected Output
```rust
iced::Theme::custom(
    "theme_name".to_string(),
    iced::theme::Palette {
        primary: iced::Color::from_rgb8(0x34, 0x98, 0xDB),
        success: iced::Color::from_rgb8(0x27, 0xAE, 0x60),
        warning: iced::Color::from_rgb8(0xF3, 0x9C, 0x12),
        danger: iced::Color::from_rgb8(0xE7, 0x4C, 0x3C),
        background: iced::Color::from_rgb8(0xEC, 0xF0, 0xF1),
        text: iced::Color::from_rgb8(0x2C, 0x3E, 0x50),
    }
)
```

### Contract Test
```rust
#[test]
fn contract_palette_to_iced_theme() {
    let palette = create_test_palette();
    let iced_theme = ThemeAdapter::to_iced_theme("test", &palette);
    
    // Verify it's a custom theme
    assert!(matches!(iced_theme, iced::Theme::Custom(_)));
    
    // Extract and verify palette
    let extracted = iced_theme.palette();
    assert_eq!(extracted.primary.r, 0.204); // #34 / 255
}
```

---

## Contract 4: Theme Context Creation

### Description
Create a `ThemeContext` from a `ThemeDocument` with correct default selection.

### Test Cases

| Document State | System Pref | Expected Active |
|----------------|-------------|-----------------|
| default="light" | None | "light" |
| default="light" | "dark" | "dark" (follow_system=true) |
| default="light", follow_system=false | "dark" | "light" |
| no default | "dark" | "dark" |
| no default | None | "light" (fallback) |

### Contract Test
```rust
#[test]
fn contract_theme_context_follows_system() {
    let doc = ThemeDocument {
        themes: create_light_dark_themes(),
        default_theme: Some("light".to_string()),
        follow_system: true,
    };
    
    let ctx = ThemeContext::from_document(doc);
    ctx.update_system_preference("dark");
    
    assert_eq!(ctx.active().name, "dark");
}
```

---

## Contract 5: Runtime Theme Switching

### Description
Switch themes at runtime via `ThemeContext::set_theme()`.

### Preconditions
- ThemeContext initialized with multiple themes

### Input
```rust
ctx.set_theme("dark")
```

### Expected Behavior
1. `active_theme` changes to "dark"
2. `iced_theme` is regenerated
3. Returns `Ok(())`

### Error Cases
| Input | Expected |
|-------|----------|
| `set_theme("nonexistent")` | `Err(ThemeError::NotFound("nonexistent"))` |

### Contract Test
```rust
#[test]
fn contract_theme_switching() {
    let mut ctx = create_test_context();
    
    assert_eq!(ctx.active().name, "light");
    
    let result = ctx.set_theme("dark");
    assert!(result.is_ok());
    assert_eq!(ctx.active().name, "dark");
    
    let result = ctx.set_theme("nonexistent");
    assert!(result.is_err());
}
```

---

## Contract 6: Theme File Discovery

### Description
Discover and load `src/ui/theme/theme.dampen` if it exists.

### Test Cases

| File State | Expected Result |
|------------|-----------------|
| File exists, valid | `Some(ThemeDocument)` |
| File exists, invalid | `Err(ParseError)` |
| File doesn't exist | `None` (use default) |
| Directory doesn't exist | `None` (use default) |

### Contract Test
```rust
#[test]
fn contract_theme_discovery_missing_file() {
    let temp_dir = tempfile::tempdir().unwrap();
    let result = discover_theme_file(temp_dir.path());
    
    assert!(result.is_none());
}

#[test]
fn contract_theme_discovery_valid_file() {
    let temp_dir = tempfile::tempdir().unwrap();
    let theme_dir = temp_dir.path().join("src/ui/theme");
    std::fs::create_dir_all(&theme_dir).unwrap();
    std::fs::write(
        theme_dir.join("theme.dampen"),
        VALID_THEME_XML
    ).unwrap();
    
    let result = discover_theme_file(temp_dir.path());
    assert!(result.is_some());
}
```

---

## Contract 7: Hot-Reload Theme Updates

### Description
Theme changes trigger UI updates without restart.

### Preconditions
- Application running in interpreted mode (`dampen run`)
- `src/ui/theme/theme.dampen` exists

### Input
Modify `theme.dampen` file contents

### Expected Behavior
1. File watcher detects change (within 100ms debounce)
2. Theme file is re-parsed
3. `ThemeContext::reload()` is called
4. All windows update to new theme (within 500ms total)

### Contract Test
```rust
#[test]
fn contract_hot_reload_theme_change() {
    let (tx, rx) = channel();
    let mut ctx = create_test_context();
    
    // Simulate file change
    let new_doc = parse_theme_document(MODIFIED_THEME_XML).unwrap();
    ctx.reload(new_doc);
    
    assert_eq!(ctx.active().palette.primary, Color::from_hex("#ff0000"));
}
```

---

## Contract 8: Backward Compatibility

### Description
Existing apps without `theme.dampen` continue working unchanged.

### Test Cases

| Scenario | Expected Behavior |
|----------|-------------------|
| No `src/ui/theme/` directory | App uses Iced default theme |
| Empty `src/ui/theme/` directory | App uses Iced default theme |
| Existing `.dampen` with inline themes | Inline themes still work |

### Contract Test
```rust
#[test]
fn contract_backward_compat_no_theme_file() {
    let temp_dir = tempfile::tempdir().unwrap();
    create_minimal_dampen_app(temp_dir.path());
    // No theme.dampen file
    
    let ctx = load_theme_context(temp_dir.path());
    
    // Should use Iced default
    assert!(matches!(ctx.iced_theme(), iced::Theme::Light));
}
```

---

## Contract 9: Codegen Theme Generation

### Description
Generate Rust code for theme in production builds.

### Input
```rust
ThemeDocument { /* ... */ }
```

### Expected Output
```rust
// Generated code
pub fn app_theme() -> iced::Theme {
    iced::Theme::custom(
        "light".to_string(),
        iced::theme::Palette {
            primary: iced::Color::from_rgb8(52, 152, 219),
            success: iced::Color::from_rgb8(39, 174, 96),
            warning: iced::Color::from_rgb8(243, 156, 18),
            danger: iced::Color::from_rgb8(231, 76, 60),
            background: iced::Color::from_rgb8(236, 240, 241),
            text: iced::Color::from_rgb8(44, 62, 80),
        }
    )
}
```

### Contract Test
```rust
#[test]
fn contract_codegen_theme_function() {
    let doc = create_test_theme_document();
    let generated = generate_theme_code(&doc).unwrap();
    
    assert!(generated.contains("pub fn app_theme()"));
    assert!(generated.contains("iced::Theme::custom"));
    
    // Verify it compiles
    let syntax = syn::parse_file(&generated);
    assert!(syntax.is_ok());
}
```

---

## Contract 10: XML Binding for Theme Switching

### Description
Support `theme="{model.theme}"` binding in window elements.

### Input XML
```xml
<window theme="{current_theme}">
    <!-- widgets -->
</window>
```

### Expected Behavior
- When `model.current_theme` changes, theme updates reactively
- Invalid theme names log warning and keep current theme

### Contract Test
```rust
#[test]
fn contract_theme_binding_reactive() {
    let xml = r#"<window theme="{current_theme}"><text value="Test" /></window>"#;
    let doc = parse(xml).unwrap();
    
    let mut model = TestModel { current_theme: "light".to_string() };
    let mut ctx = create_test_context();
    
    // Build initial view
    let _ = build_view(&doc, &model, &ctx);
    assert_eq!(ctx.active().name, "light");
    
    // Change model
    model.current_theme = "dark".to_string();
    
    // Rebuild view
    let _ = build_view(&doc, &model, &ctx);
    assert_eq!(ctx.active().name, "dark");
}
```

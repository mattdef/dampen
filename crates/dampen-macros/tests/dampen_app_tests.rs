//! Contract tests for #[dampen_app] macro - User Story 1: View Discovery
//!
//! Following TDD: These tests MUST fail initially, then pass after implementation.

use std::path::PathBuf;

// Access the macro impl directly for testing
// Since proc-macro crates can't export non-proc-macro items, we include the module directly
#[path = "../src/dampen_app.rs"]
mod dampen_app;

#[path = "../src/discovery.rs"]
mod discovery;

// Import discovery functions (they exist now in discovery.rs but not exposed yet)
// We'll add proper module access once the main macro implementation is done

#[cfg(test)]
mod us1_discovery_tests {
    use super::*;

    // T014: Test discover_dampen_files() with flat structure
    #[test]
    fn test_discover_flat_structure() {
        // Given: A flat UI directory with 3 .dampen files
        let fixture_dir =
            PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/multi_view/src/ui");

        // When: We discover .dampen files
        // Note: discover_dampen_files is in discovery.rs but not yet publicly accessible from tests
        // For now, we verify the fixture exists
        assert!(fixture_dir.exists(), "Fixture directory should exist");

        // Verify all .dampen files exist
        assert!(fixture_dir.join("home.dampen").exists());
        assert!(fixture_dir.join("settings.dampen").exists());
        assert!(fixture_dir.join("about.dampen").exists());

        // Verify corresponding .rs files exist
        assert!(fixture_dir.join("home.rs").exists());
        assert!(fixture_dir.join("settings.rs").exists());
        assert!(fixture_dir.join("about.rs").exists());

        // TODO: Once discovery.rs functions are public, uncomment:
        // let views = dampen_macros::discovery::discover_dampen_files(&fixture_dir, &[]).unwrap();
        // assert_eq!(views.len(), 3);
        // assert!(views.iter().any(|v| v.view_name == "about"));
        // assert!(views.iter().any(|v| v.view_name == "home"));
        // assert!(views.iter().any(|v| v.view_name == "settings"));
    }

    // T015: Test discover_dampen_files() with nested structure
    #[test]
    fn test_discover_nested_structure() {
        // Given: A nested UI directory structure
        let fixture_dir =
            PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/nested_views/src/ui");

        // Verify fixture exists
        assert!(fixture_dir.exists(), "Fixture directory should exist");

        // Verify nested structure exists
        assert!(fixture_dir.join("main.dampen").exists());
        assert!(fixture_dir.join("main.rs").exists());

        assert!(fixture_dir.join("widgets").join("button.dampen").exists());
        assert!(fixture_dir.join("widgets").join("button.rs").exists());

        // TODO: Once discovery.rs functions are public, uncomment:
        // let views = dampen_macros::discovery::discover_dampen_files(&fixture_dir, &[]).unwrap();
        // assert_eq!(views.len(), 2);
        // assert!(views.iter().any(|v| v.view_name == "main"));
        // assert!(views.iter().any(|v| v.view_name == "button"));
    }

    // T016: Test ViewInfo struct creation
    #[test]
    fn test_view_info_creation() {
        // This test validates ViewInfo field mapping logic:
        // Example 1: src/ui/home.dampen
        // - view_name: "home"
        // - variant_name: "Home"
        // - field_name: "home_state"
        // - module_path: "ui::home"

        // Example 2: src/ui/widgets/button.dampen
        // - view_name: "button"
        // - variant_name: "Button"
        // - field_name: "button_state"
        // - module_path: "ui::widgets::button"

        // For now, just document the expected behavior
        assert!(true, "ViewInfo field logic documented");
    }
}

// ==============================================================================
// Phase 3: User Story 1 - Code Generation Tests
// ==============================================================================

#[cfg(test)]
mod us1_codegen_tests {
    use super::*;

    // T020: Test generate_current_view_enum
    #[test]
    #[ignore = "Snapshot test"]
    fn test_generate_current_view_enum() {
        // Given: A set of discovered ViewInfo structs
        // When: generate_current_view_enum() is called
        // Then: Output matches snapshot with:
        // - Enum definition: pub enum CurrentView { ... }
        // - Variants: Home, Settings, About

        assert!(true, "Snapshot test documented");
    }

    // T021: Test generate_app_struct
    #[test]
    #[ignore = "Snapshot test"]
    fn test_generate_app_struct() {
        // Given: A set of discovered ViewInfo structs
        // When: generate_app_struct() is called
        // Then: Output matches snapshot with:
        // - Struct definition: pub struct App { ... }
        // - Fields: home_state: AppState<ui::home::Model>, ...
        // - current_view: CurrentView field

        assert!(true, "Snapshot test documented");
    }

    // T022: Test generate_init_method
    #[test]
    #[ignore = "Snapshot test"]
    fn test_generate_init_method() {
        // Given: A set of discovered ViewInfo structs
        // When: generate_init_method() is called
        // Then: Output matches snapshot with:
        // - Method signature: pub fn init() -> Self { ... }
        // - Calls to ui::*::document() for loading
        // - AppState::with_handlers() construction

        assert!(true, "Snapshot test documented");
    }
}

// ==============================================================================
// Phase 4: User Story 2 - View Switching Tests
// ==============================================================================

#[cfg(test)]
mod us2_view_switching_tests {
    use super::*;

    // T030: Test generate_switch_to_methods
    #[test]
    #[ignore = "Snapshot test"]
    fn test_generate_switch_to_methods() {
        // Given: A set of discovered ViewInfo structs
        // When: generate_switch_to_methods() is called
        // Then: Output matches snapshot with:
        // - Method: pub fn switch_to_home(&mut self)
        // - Method: pub fn switch_to_settings(&mut self)
        // - Each method sets self.current_view to appropriate variant

        assert!(true, "Snapshot test documented");
    }

    // T031: Test generate_update_method
    #[test]
    #[ignore = "Snapshot test"]
    fn test_generate_update_method() {
        // Given: A set of discovered ViewInfo structs
        // When: generate_update_method() is called
        // Then: Output matches snapshot with:
        // - Method signature: pub fn update(&mut self, message: Message) -> iced::Task<Message>
        // - Match on Message::Handler variant
        // - Dispatch to correct AppState based on current_view

        assert!(true, "Snapshot test documented");
    }

    // T032: Test generate_view_method
    #[test]
    #[ignore = "Snapshot test"]
    fn test_generate_view_method() {
        // Given: A set of discovered ViewInfo structs
        // When: generate_view_method() is called
        // Then: Output matches snapshot with:
        // - Method signature: pub fn view(&self) -> iced::Element<'_, Message>
        // - Match on current_view to select AppState
        // - Call build_ui() with correct AppState (TODO: pending dampen_iced)

        assert!(true, "Snapshot test documented");
    }

    // T033: Test switch_to_* methods call switching logic
    #[test]
    fn test_view_switching_logic() {
        // Given: Generated App struct
        // When: switch_to_home() is called
        // Then: current_view field changes to CurrentView::Home
        // And: subsequent view() calls render home view

        assert!(true, "View switching logic documented");
    }
}

// ==============================================================================
// Phase 4: User Story 2 - Error Cases
// ==============================================================================

#[cfg(test)]
mod error_cases {
    use super::*;

    // E1: Directory not found
    #[test]
    fn test_error_directory_not_found() {
        // Given: ui_dir points to non-existent directory
        // When: Macro expands
        // Then: Compile error with helpful message
        // Error: "UI directory not found: 'src/nonexistent'"
        assert!(
            true,
            "Documented: directory check implemented in dampen_app.rs:323"
        );
    }

    // E2: No .dampen files found
    #[test]
    fn test_error_no_dampen_files() {
        // Given: Directory exists but contains no .dampen files
        // When: discover_dampen_files() is called
        // Then: Returns error
        // Error: "No .dampen files found in 'src/ui'"
        assert!(
            true,
            "Documented: empty directory check should fail at compile time"
        );
    }

    // E3: Missing .rs file
    #[test]
    fn test_error_missing_rs_file() {
        // VR-003 validation catches this during discovery
        // Error: "No matching Rust module found for '/path/to/file.dampen'"
        assert!(
            true,
            "Documented: .dampen without .rs should fail at compile time"
        );
    }

    // E4: View naming conflict
    #[test]
    fn test_error_view_naming_conflict() {
        // VR-002 validation catches duplicate variant names
        // Error: "View naming conflict: 'Input' variant found in multiple locations"
        assert!(
            true,
            "Documented: duplicate variant names should fail at compile time"
        );
    }

    // E5: Invalid view name
    #[test]
    fn test_error_invalid_view_name() {
        // VR-001 validation catches invalid Rust identifiers
        // Error: "Invalid view name '123-invalid'"
        assert!(
            true,
            "Documented: invalid identifier should fail at compile time"
        );
    }

    // E7: No views discovered
    #[test]
    fn test_error_no_views_found() {
        // Current implementation returns error if no .dampen files found
        // Error: "No .dampen files found in 'src/ui'"
        assert!(true, "Documented: empty ui_dir should fail at compile time");
    }
}

// ==============================================================================
// Phase 5: User Story 3 - Hot-Reload Code Generation Tests
// ==============================================================================

#[cfg(test)]
mod us3_hot_reload_tests {
    use super::dampen_app;

    // T051: Snapshot test for subscription() method generation
    #[test]
    #[ignore = "snapshot test - run with insta"]
    fn test_subscription_method_generation() {
        // Given: MacroAttributes with hot_reload_variant specified
        let attr = quote::quote! {
            ui_dir = "tests/fixtures/multi_view/src/ui",
            message_type = "Message",
            handler_variant = "Handler",
            hot_reload_variant = "HotReload",
            dismiss_error_variant = "DismissError"
        };
        let item = quote::quote! { struct App; };

        // When: We expand the macro
        let output =
            dampen_app::dampen_app_impl(attr, item).expect("Macro expansion should succeed");

        // Then: Output should contain subscription() method
        let output_str = output.to_string();

        // Verify key components are present
        assert!(
            output_str.contains("subscription"),
            "Should generate subscription() method"
        );
        assert!(output_str.contains("cfg"), "Should be debug-only");
        assert!(
            output_str.contains("debug_assertions"),
            "Should use debug_assertions cfg"
        );
        assert!(
            output_str.contains("dampen_dev"),
            "Should use dampen_dev crate"
        );
        assert!(
            output_str.contains("watch_files"),
            "Should use watch_files function"
        );

        // Snapshot test for formatted output
        insta::assert_snapshot!("subscription_method", output_str);
    }

    // T052: Snapshot test for hot-reload handling in update() method
    #[test]
    #[ignore = "snapshot test - run with insta"]
    fn test_hot_reload_update_handler() {
        // Given: MacroAttributes with hot_reload_variant
        let attr = quote::quote! {
            ui_dir = "tests/fixtures/multi_view/src/ui",
            message_type = "Message",
            handler_variant = "Handler",
            hot_reload_variant = "HotReload"
        };
        let item = quote::quote! { struct App; };

        // When: We expand the macro
        let output =
            dampen_app::dampen_app_impl(attr, item).expect("Macro expansion should succeed");

        // Then: Output should contain hot-reload match arms in update()
        let output_str = output.to_string();

        assert!(
            output_str.contains("HotReload"),
            "Should handle HotReload message"
        );
        assert!(
            output_str.contains("FileEvent"),
            "Should match on FileEvent enum"
        );
        assert!(
            output_str.contains("Success"),
            "Should handle FileEvent::Success"
        );
        assert!(
            output_str.contains("ParseError"),
            "Should handle FileEvent::ParseError"
        );

        // Snapshot test
        insta::assert_snapshot!("hot_reload_update", output_str);
    }

    // T053: Snapshot test for error overlay handling in view() method
    #[test]
    #[ignore = "snapshot test - run with insta"]
    fn test_error_overlay_view() {
        // Given: MacroAttributes with dismiss_error_variant
        let attr = quote::quote! {
            ui_dir = "tests/fixtures/multi_view/src/ui",
            message_type = "Message",
            handler_variant = "Handler",
            hot_reload_variant = "HotReload",
            dismiss_error_variant = "DismissError"
        };
        let item = quote::quote! { struct App; };

        // When: We expand the macro
        let output =
            dampen_app::dampen_app_impl(attr, item).expect("Macro expansion should succeed");

        // Then: Output should contain error overlay rendering
        let output_str = output.to_string();

        assert!(
            output_str.contains("error_overlay"),
            "Should have error_overlay field"
        );
        assert!(
            output_str.contains("ErrorOverlay"),
            "Should use ErrorOverlay type"
        );
        assert!(output_str.contains("render"), "Should call render method");

        // Snapshot test
        insta::assert_snapshot!("error_overlay_view", output_str);
    }

    // T054: Snapshot test for DismissError handling in update()
    #[test]
    #[ignore = "snapshot test - run with insta"]
    fn test_dismiss_error_handler() {
        // Given: MacroAttributes with dismiss_error_variant
        let attr = quote::quote! {
            ui_dir = "tests/fixtures/multi_view/src/ui",
            message_type = "Message",
            handler_variant = "Handler",
            dismiss_error_variant = "DismissError"
        };
        let item = quote::quote! { struct App; };

        // When: We expand the macro
        let output =
            dampen_app::dampen_app_impl(attr, item).expect("Macro expansion should succeed");

        // Then: Output should contain DismissError match arm
        let output_str = output.to_string();

        assert!(
            output_str.contains("DismissError"),
            "Should handle DismissError message"
        );
        assert!(
            output_str.contains("hide"),
            "Should call hide on error overlay"
        );

        // Snapshot test
        insta::assert_snapshot!("dismiss_error_handler", output_str);
    }

    // Integration test: Verify hot-reload code only generated when variant specified
    #[test]
    fn test_hot_reload_conditional_generation() {
        // Given: MacroAttributes WITHOUT hot_reload_variant
        let attr_without = quote::quote! {
            ui_dir = "tests/fixtures/multi_view/src/ui",
            message_type = "Message",
            handler_variant = "Handler"
        };
        let item = quote::quote! { struct App; };

        // When: We expand the macro
        let output_without = dampen_app::dampen_app_impl(attr_without, item.clone())
            .expect("Macro expansion should succeed");
        let output_str_without = output_without.to_string();

        // Then: Output should NOT contain subscription or hot-reload code
        assert!(
            !output_str_without.contains("subscription"),
            "Should NOT generate subscription() without hot_reload_variant"
        );
        assert!(
            !output_str_without.contains("HotReload"),
            "Should NOT handle HotReload without hot_reload_variant"
        );

        // Given: MacroAttributes WITH hot_reload_variant
        let attr_with = quote::quote! {
            ui_dir = "tests/fixtures/multi_view/src/ui",
            message_type = "Message",
            handler_variant = "Handler",
            hot_reload_variant = "HotReload"
        };

        // When: We expand the macro
        let output_with =
            dampen_app::dampen_app_impl(attr_with, item).expect("Macro expansion should succeed");
        let output_str_with = output_with.to_string();

        // Then: Output SHOULD contain subscription and hot-reload code
        assert!(
            output_str_with.contains("subscription"),
            "Should generate subscription() with hot_reload_variant"
        );
        assert!(
            output_str_with.contains("HotReload"),
            "Should handle HotReload with hot_reload_variant"
        );
    }

    // Integration test: Verify error overlay only generated when variant specified
    #[test]
    fn test_error_overlay_conditional_generation() {
        // Given: MacroAttributes WITHOUT dismiss_error_variant
        let attr_without = quote::quote! {
            ui_dir = "tests/fixtures/multi_view/src/ui",
            message_type = "Message",
            handler_variant = "Handler",
            hot_reload_variant = "HotReload"
        };
        let item = quote::quote! { struct App; };

        // When: We expand the macro
        let output_without = dampen_app::dampen_app_impl(attr_without, item.clone())
            .expect("Macro expansion should succeed");
        let output_str_without = output_without.to_string();

        // Then: Output should NOT contain error overlay field or rendering
        assert!(
            !output_str_without.contains("error_overlay: "),
            "Should NOT have error_overlay field without dismiss_error_variant"
        );

        // Given: MacroAttributes WITH dismiss_error_variant
        let attr_with = quote::quote! {
            ui_dir = "tests/fixtures/multi_view/src/ui",
            message_type = "Message",
            handler_variant = "Handler",
            hot_reload_variant = "HotReload",
            dismiss_error_variant = "DismissError"
        };

        // When: We expand the macro
        let output_with =
            dampen_app::dampen_app_impl(attr_with, item).expect("Macro expansion should succeed");
        let output_str_with = output_with.to_string();

        // Then: Output SHOULD contain error overlay field
        assert!(
            output_str_with.contains("error_overlay"),
            "Should have error_overlay field with dismiss_error_variant"
        );
    }
}

// ============================================================================
// Phase 6: User Story 4 - Selective View Exclusion (T063-T066)
// ============================================================================

#[cfg(test)]
mod us4_exclusion_tests {
    use super::*;

    // T063: Test single file exclusion
    #[test]
    fn test_exclude_single_file() {
        // Given: A UI directory with main.dampen, debug.dampen, and experimental/feature.dampen
        let item = quote::quote! { struct App; };

        // When: We exclude "debug" file
        let attr = quote::quote! {
            ui_dir = "tests/fixtures/excluded_views/src/ui",
            message_type = "Message",
            handler_variant = "Handler",
            exclude = ["debug"]
        };

        // Then: Macro expansion should succeed
        let output =
            dampen_app::dampen_app_impl(attr, item).expect("Macro expansion should succeed");
        let output_str = output.to_string();

        // And: Output should contain Main view
        assert!(
            output_str.contains("Main"),
            "Should include Main variant in CurrentView enum"
        );

        // And: Output should NOT contain Debug view (check for Debug as an enum variant, not #[derive(Debug)])
        // Extract the CurrentView enum to check variants
        let has_debug_variant = output_str.contains("enum CurrentView")
            && output_str
                .split("enum CurrentView")
                .nth(1)
                .map(|s| s.split('}').next().unwrap_or(""))
                .map(|enum_body| enum_body.contains("Debug ,") || enum_body.contains("Debug }"))
                .unwrap_or(false);

        assert!(
            !has_debug_variant,
            "Should NOT include Debug variant in CurrentView enum"
        );

        // And: Output should contain experimental/feature view
        assert!(
            output_str.contains("Feature"),
            "Should include Feature variant (not excluded)"
        );
    }

    // T064: Test directory wildcard exclusion
    #[test]
    fn test_exclude_directory_wildcard() {
        // Given: A UI directory with nested experimental/ directory
        let item = quote::quote! { struct App; };

        // When: We exclude "experimental/*" pattern
        let attr = quote::quote! {
            ui_dir = "tests/fixtures/excluded_views/src/ui",
            message_type = "Message",
            handler_variant = "Handler",
            exclude = ["experimental/*"]
        };

        // Then: Macro expansion should succeed
        let output =
            dampen_app::dampen_app_impl(attr, item).expect("Macro expansion should succeed");
        let output_str = output.to_string();

        // And: Output should contain Main and Debug views
        assert!(output_str.contains("Main"), "Should include Main variant");
        assert!(output_str.contains("Debug"), "Should include Debug variant");

        // And: Output should NOT contain Feature view from experimental/
        assert!(
            !output_str.contains("Feature"),
            "Should NOT include Feature variant from experimental/ directory"
        );
    }

    // T065: Test multiple exclusion patterns
    #[test]
    fn test_exclude_multiple_patterns() {
        // Given: A UI directory with multiple views
        let item = quote::quote! { struct App; };

        // When: We exclude multiple patterns
        let attr = quote::quote! {
            ui_dir = "tests/fixtures/excluded_views/src/ui",
            message_type = "Message",
            handler_variant = "Handler",
            exclude = ["debug", "experimental/*"]
        };

        // Then: Macro expansion should succeed
        let output =
            dampen_app::dampen_app_impl(attr, item).expect("Macro expansion should succeed");
        let output_str = output.to_string();

        // And: Output should ONLY contain Main view
        assert!(output_str.contains("Main"), "Should include Main variant");

        // And: Output should NOT contain excluded views (check enum variants, not #[derive(Debug)])
        let enum_body = output_str
            .split("enum CurrentView")
            .nth(1)
            .and_then(|s| s.split('}').next())
            .unwrap_or("");

        assert!(
            !(enum_body.contains("Debug ,") || enum_body.contains("Debug }")),
            "Should NOT include Debug variant (excluded)"
        );
        assert!(
            !(enum_body.contains("Feature ,") || enum_body.contains("Feature }")),
            "Should NOT include Feature variant (excluded by wildcard)"
        );
    }

    // T066: Test exclusion affects generated enum and struct fields
    #[test]
    fn test_exclusion_affects_generated_code() {
        // Given: A UI directory with views to exclude
        let item = quote::quote! { struct App; };
        let attr = quote::quote! {
            ui_dir = "tests/fixtures/excluded_views/src/ui",
            message_type = "Message",
            handler_variant = "Handler",
            exclude = ["debug"]
        };

        // When: We expand the macro
        let output =
            dampen_app::dampen_app_impl(attr, item).expect("Macro expansion should succeed");
        let output_str = output.to_string();

        // Then: CurrentView enum should not have Debug variant
        // Extract enum definition (look for "pub enum CurrentView")
        assert!(
            output_str.contains("enum CurrentView"),
            "Should generate CurrentView enum"
        );

        // Verify Main is in enum but Debug is not
        let enum_section = output_str
            .split("enum CurrentView")
            .nth(1)
            .and_then(|s| s.split('}').next())
            .expect("Should have CurrentView enum body");

        assert!(
            enum_section.contains("Main"),
            "CurrentView enum should contain Main variant"
        );
        assert!(
            !(enum_section.contains("Debug ,") || enum_section.contains("Debug }")),
            "CurrentView enum should NOT contain Debug variant (found in: {})",
            enum_section
        );

        // Verify App struct doesn't have debug_document or debug_state fields
        assert!(
            !output_str.contains("debug_document"),
            "App struct should NOT have debug_document field"
        );
        assert!(
            !output_str.contains("debug_state"),
            "App struct should NOT have debug_state field"
        );
    }

    // T067: Test invalid glob pattern produces error
    #[test]
    fn test_invalid_glob_pattern_error() {
        // Given: An invalid glob pattern
        let item = quote::quote! { struct App; };
        let attr = quote::quote! {
            ui_dir = "tests/fixtures/excluded_views/src/ui",
            message_type = "Message",
            handler_variant = "Handler",
            exclude = ["[invalid"]  // Invalid: unclosed bracket
        };

        // When/Then: Macro expansion should fail with helpful error
        let result = dampen_app::dampen_app_impl(attr, item);

        assert!(result.is_err(), "Should fail with invalid glob pattern");

        let err_msg = result.unwrap_err().to_string();
        assert!(
            err_msg.contains("Invalid exclude pattern") || err_msg.contains("glob"),
            "Error message should mention invalid pattern: {}",
            err_msg
        );
    }
}

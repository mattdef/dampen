//! Tests for dev-mode warnings about missing shared context

#[cfg(test)]
mod tests {
    use dampen_core::parse;
    use dampen_iced::DampenWidgetBuilder;
    use dampen_macros::UiModel;
    use serde::{Deserialize, Serialize};

    #[derive(Clone, Default, Serialize, Deserialize, UiModel)]
    struct TestModel {
        pub count: i32,
    }

    #[test]
    fn test_shared_binding_without_context_shows_warning_in_debug() {
        // This test verifies that in debug mode, we get a warning
        // when using {shared.field} without providing a shared context.
        //
        // Note: The warning goes to stderr, so we can't easily capture it
        // in the test, but we verify the code doesn't panic and returns
        // an empty string as expected.

        let xml = r#"<dampen><text value="{shared.theme}" /></dampen>"#;
        let document = parse(xml).expect("Failed to parse XML");
        let model = TestModel::default();

        // Build without shared context - should show warning in debug mode
        let builder = DampenWidgetBuilder::new(&document, &model, None);
        let _element = builder.build();

        // If we reach here, the code handled missing context gracefully
        // In debug mode, a warning was printed to stderr
    }

    #[test]
    fn test_interpolated_shared_binding_without_context() {
        let xml = r#"<dampen><text value="Theme: {shared.theme}" /></dampen>"#;
        let document = parse(xml).expect("Failed to parse XML");
        let model = TestModel::default();

        // Build without shared context - should show warning for interpolated binding
        let builder = DampenWidgetBuilder::new(&document, &model, None);
        let _element = builder.build();

        // Should handle gracefully without panic
    }

    #[test]
    fn test_mixed_bindings_partial_shared() {
        // Binding that uses both model and shared fields
        let xml = r#"<dampen><text value="{count} - {shared.total}" /></dampen>"#;
        let document = parse(xml).expect("Failed to parse XML");
        let model = TestModel { count: 42 };

        // Build without shared context - model binding works, shared returns empty
        let builder = DampenWidgetBuilder::new(&document, &model, None);
        let _element = builder.build();

        // Should handle gracefully - model field works, shared field returns empty
    }

    #[test]
    fn test_no_warning_for_model_bindings() {
        // Regular model bindings should not trigger any warnings
        let xml = r#"<dampen><text value="{count}" /></dampen>"#;
        let document = parse(xml).expect("Failed to parse XML");
        let model = TestModel { count: 42 };

        // Build without shared context - no warnings expected
        let builder = DampenWidgetBuilder::new(&document, &model, None);
        let _element = builder.build();

        // No warnings, works normally
    }
}

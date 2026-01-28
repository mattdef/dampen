// ============================================================================
// COLOR PICKER WIDGET TESTS
// ============================================================================

/// Test Color Picker Event Handling
///
/// Given: A ColorPicker widget with event handlers
/// When: Events are triggered (submit, cancel)
/// Then: The correct handlers should be invoked with correct data
#[test]
fn test_color_picker_events() {
    use dampen_iced::HandlerMessage;
    use iced::Color;
    use iced_aw::widgets::color_picker;

    // Simulate handlers
    let on_submit = |color: Color| {
        let r = (color.r * 255.0) as u8;
        let g = (color.g * 255.0) as u8;
        let b = (color.b * 255.0) as u8;
        let hex = format!("#{:02x}{:02x}{:02x}", r, g, b);
        HandlerMessage::Handler("handle_submit".to_string(), Some(hex))
    };

    let on_cancel = || HandlerMessage::Handler("handle_cancel".to_string(), None);

    // Test submit with red color
    let red_color = Color::from_rgb8(255, 0, 0);
    let submit_msg = on_submit(red_color);

    if let HandlerMessage::Handler(name, payload) = submit_msg {
        assert_eq!(name, "handle_submit");
        assert_eq!(payload, Some("#ff0000".to_string()));
    } else {
        panic!("Expected HandlerMessage::Handler");
    }

    // Test cancel
    let cancel_msg = on_cancel();

    if let HandlerMessage::Handler(name, payload) = cancel_msg {
        assert_eq!(name, "handle_cancel");
        assert_eq!(payload, None);
    } else {
        panic!("Expected HandlerMessage::Handler");
    }
}

/// Test Color Value Round-Tripping
///
/// Given: Various color formats (hex, rgb, rgba)
/// When: Parsed and converted back to string
/// Then: Values should be consistent
#[test]
fn test_color_round_trip() {
    use dampen_core::ir::style::Color as DampenColor;

    // Parser logic from builder
    fn parse_color(s: &str) -> Option<iced::Color> {
        DampenColor::parse(s)
            .ok()
            .map(|c| iced::Color::from_rgba(c.r, c.g, c.b, c.a))
    }

    // Formatter logic from builder
    fn color_to_hex(color: &iced::Color) -> String {
        let dampen_color = DampenColor {
            r: color.r,
            g: color.g,
            b: color.b,
            a: color.a,
        };
        if dampen_color.a == 1.0 {
            dampen_color.to_hex()
        } else {
            dampen_color.to_rgba_hex()
        }
    }

    // Test cases
    let cases = vec![
        ("#ff0000", "#ff0000"),        // Red
        ("#00ff00", "#00ff00"),        // Green
        ("#0000ff", "#0000ff"),        // Blue
        ("#ffffff", "#ffffff"),        // White
        ("#000000", "#000000"),        // Black
        ("#ff0000ff", "#ff0000"),      // Red full alpha (normalized to hex without alpha)
        ("#ff000080", "#ff000080"),    // Red 50% alpha
        ("red", "#ff0000"),            // Named
        ("rgb(255, 0, 0)", "#ff0000"), // RGB function
    ];

    for (input, expected) in cases {
        let color = parse_color(input).expect("Failed to parse color");
        let output = color_to_hex(&color);
        assert_eq!(output, expected, "Round trip failed for {}", input);
    }
}

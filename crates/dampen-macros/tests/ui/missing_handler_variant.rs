// Test: Missing required attribute handler_variant
// Expected error: "missing required attribute 'handler_variant'"

use dampen_macros::dampen_app;

#[dampen_app(ui_dir = "tests/fixtures/multi_view/src/ui", message_type = "Message")]
struct App;

fn main() {}

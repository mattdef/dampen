// Test: Missing required attribute message_type
// Expected error: "missing required attribute 'message_type'"

use dampen_macros::dampen_app;

#[dampen_app(
    ui_dir = "tests/fixtures/multi_view/src/ui",
    handler_variant = "Handler"
)]
struct App;

fn main() {}

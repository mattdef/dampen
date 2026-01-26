// Test: No .dampen files found in ui_dir
// Expected error: "No views discovered" or similar

use dampen_macros::dampen_app;

#[dampen_app(
    ui_dir = "tests/fixtures/empty_ui_dir/src/ui",
    message_type = "Message",
    handler_variant = "Handler"
)]
struct App;

fn main() {}

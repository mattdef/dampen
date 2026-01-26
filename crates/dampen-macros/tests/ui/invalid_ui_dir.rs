// Test: Invalid ui_dir path (directory doesn't exist)
// Expected error: "UI directory not found"

use dampen_macros::dampen_app;

#[dampen_app(
    ui_dir = "tests/fixtures/nonexistent_directory",
    message_type = "Message",
    handler_variant = "Handler"
)]
struct App;

fn main() {}

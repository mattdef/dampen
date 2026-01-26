// Test: Invalid view name (starts with number)
// Expected error: "Invalid view name" / "must start with a letter or underscore"

use dampen_macros::dampen_app;

#[dampen_app(
    ui_dir = "tests/fixtures/invalid_view_name/src/ui",
    message_type = "Message",
    handler_variant = "Handler"
)]
struct App;

fn main() {}

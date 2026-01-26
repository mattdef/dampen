// Test: Invalid glob pattern in exclude parameter
// Expected error: "Invalid exclude pattern"

use dampen_macros::dampen_app;

#[dampen_app(
    ui_dir = "tests/fixtures/multi_view/src/ui",
    message_type = "Message",
    handler_variant = "Handler",
    exclude = ["[invalid"]  // Unclosed bracket is invalid glob syntax
)]
struct App;

fn main() {}

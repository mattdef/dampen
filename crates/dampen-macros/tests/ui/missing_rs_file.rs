// Test: .dampen file without corresponding .rs file
// Expected error: "No matching Rust module found"

use dampen_macros::dampen_app;

#[dampen_app(
    ui_dir = "tests/fixtures/missing_rs/src/ui",
    message_type = "Message",
    handler_variant = "Handler"
)]
struct App;

fn main() {}

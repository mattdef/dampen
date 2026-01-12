// Test: Naming conflict (two views with same name in different directories)
// Expected error: "View naming conflict" / "duplicate variant name"

use dampen_macros::dampen_app;

#[dampen_app(
    ui_dir = "tests/fixtures/naming_conflict/src/ui",
    message_type = "Message",
    handler_variant = "Handler"
)]
struct App;

fn main() {}

// Test: Missing required attribute ui_dir
// Expected error: "missing required attribute 'ui_dir'"

use dampen_macros::dampen_app;

#[dampen_app(message_type = "Message", handler_variant = "Handler")]
struct App;

fn main() {}

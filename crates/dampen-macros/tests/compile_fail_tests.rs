//! Compile-fail tests for #[dampen_app] macro error messages
//!
//! These tests use trybuild to verify that the macro produces clear,
//! actionable error messages when conventions are violated.
//!
//! Run with: cargo test -p dampen-macros --test compile_fail_tests

#[test]
fn ui_tests() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/ui/*.rs");
}

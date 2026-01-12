# Compile-Fail Tests for #[dampen_app] Macro

This directory contains compile-fail tests using the `trybuild` crate.

## Purpose

Tests that the `#[dampen_app]` macro produces **clear, actionable error messages** when:
- Required attributes are missing
- Invalid attribute values are provided
- File structure is incorrect
- Type constraints are violated

## Structure

```
ui/
├── err_missing_ui_dir.rs           # T072: No ui_dir attribute
├── err_invalid_message.rs          # T073: Invalid Message<M> type
├── err_missing_dampen_files.rs     # T074: No .dampen files found
├── err_duplicate_view_names.rs     # T075: Duplicate view names
├── err_invalid_glob_pattern.rs     # T076: Invalid exclude pattern
├── err_missing_rs_file.rs          # T077: .dampen without .rs file
├── err_invalid_handler_variant.rs  # T078: Invalid handler_variant value
└── *.stderr                        # Expected error output files
```

## Usage

Run with:
```bash
cargo test --test dampen_app_tests -- --nocapture
```

Trybuild will:
1. Attempt to compile each `*.rs` file
2. Expect compilation to **FAIL**
3. Compare error output with corresponding `*.stderr` file
4. Pass if errors match expected output

## Writing Tests

Each test file should:
1. Demonstrate a specific error case
2. Include comments explaining the error
3. Have a corresponding `.stderr` file with expected error text
4. Test that error messages include:
   - File paths
   - Suggestions for fixes
   - Relevant spans

## Example

```rust
// ui/err_missing_ui_dir.rs
use dampen_macros::dampen_app;

#[dampen_app] // ERROR: missing required ui_dir attribute
mod app {}
```

Expected output in `err_missing_ui_dir.stderr`:
```
error: missing required attribute `ui_dir`
  --> tests/ui/err_missing_ui_dir.rs:3:1
   |
3  | #[dampen_app]
   | ^^^^^^^^^^^^^
   |
help: add the ui_dir attribute pointing to your UI directory
   |
3  | #[dampen_app(ui_dir = "src/ui")]
   |
```

## Related Tasks

- T072-T078: Write 7 compile-fail tests (US5)
- T079: Add trybuild test runner in dampen_app_tests.rs
- T080: Update stderr files with actual error output
- T086: Verify all errors include paths + suggestions

## Resources

- [trybuild documentation](https://docs.rs/trybuild/)
- User Story 5: Clear Compile-Time Error Messages
- contracts/macro-api.md: Error case specifications

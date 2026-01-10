# Integration Tests

This directory contains end-to-end integration tests for the dual-mode architecture.

## Test Coverage

- Hot-reload functionality (file watching, state preservation)
- Code generation in production mode
- Mode parity (ensuring both modes produce identical behavior)
- Error handling and recovery scenarios

## Running Tests

```bash
# Run all integration tests
cargo test --test '*' --test-threads=1

# Run specific test
cargo test --test hot_reload_tests
```

## Adding New Tests

Integration tests should:
- Test complete user workflows
- Verify cross-crate interactions
- Cover both interpreted and codegen modes
- Include performance assertions where applicable

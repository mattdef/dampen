# Contract Tests for Dampen Features

This crate contains contract tests that verify behavioral contracts across entire features, ensuring that public APIs work correctly across different usage scenarios.

## What are Contract Tests?

Contract tests verify the **contracts** (behavioral guarantees) of public APIs. Unlike unit tests (which test individual functions) or integration tests (which test full workflows), contract tests focus on:

- **API behavior promises**: Does the API behave as documented?
- **Cross-component interactions**: Do components work correctly together?
- **Invariants**: Are guarantees maintained across different usage patterns?

## Test Organization

Tests are organized by feature specification:

```
tests/contract/
├── Cargo.toml
├── lib.rs
├── shared_state_contracts.rs    # 001-inter-window-communication
└── README.md
```

Each test file corresponds to a feature spec in `specs/` and implements the contract tests defined in that spec's `contracts/` directory.

## Shared State Contracts (001-inter-window-communication)

Tests in `shared_state_contracts.rs` verify:

### CT-001: SharedContext changes visible across clones (T028)
- Changes made through one clone are immediately visible in all other clones
- Multiple views can read shared state concurrently without blocking
- Thread-safe access via `Arc<RwLock<S>>`

**Tests:**
- `ct_001_shared_context_changes_visible_across_clones`
- `ct_001_multiple_clones_see_same_state`
- `ct_001_concurrent_reads_do_not_block`

### CT-002: Handler modifications visible in all views (T029)
- Handlers registered with `register_with_shared()` can modify shared state
- All views see changes immediately after handler execution
- State persists across view switches

**Tests:**
- `ct_002_handler_modifies_shared_all_views_see_change`
- `ct_002_multiple_handlers_modify_different_fields`
- `ct_002_handler_with_value_parameter`
- `ct_002_shared_state_persists_across_view_switches`

## Running Tests

```bash
# Run all contract tests
cargo test -p contract-tests

# Run specific test file
cargo test -p contract-tests --test shared_state_contracts

# Run specific test
cargo test -p contract-tests ct_001_shared_context_changes_visible_across_clones

# Run with output
cargo test -p contract-tests -- --nocapture
```

## When to Add Contract Tests

Add contract tests when:
1. A new feature spec defines contracts in `specs/{NNN}-{feature}/contracts/`
2. A public API makes behavioral promises that need verification
3. Cross-component interactions need guarantees

## Contract Test Naming Convention

```rust
// Format: ct_{contract_id}_{description}
#[test]
fn ct_001_shared_context_changes_visible_across_clones() { ... }

#[test]
fn ct_002_handler_modifies_shared_all_views_see_change() { ... }
```

## References

- Feature specs: `specs/001-inter-window-communication/`
- Contract definitions: `specs/001-inter-window-communication/contracts/`
- Task tracker: `specs/001-inter-window-communication/tasks.md`

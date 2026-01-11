# Tasks: XML Schema Version Parsing and Validation

**Feature**: 001-schema-version-validation  
**Date**: 2026-01-11  
**Spec**: [spec.md](./spec.md) | **Plan**: [plan.md](./plan.md)

## Task Legend

- **Priority**: [P1] Critical, [P2] Important, [P3] Nice-to-have
- **User Story**: [US1-US5] maps to spec.md user stories
- **Status**: [ ] Pending, [x] Complete

## Phase 1: Setup

- [x] T001 [P1] [Setup] Create test file `crates/dampen-core/tests/version_tests.rs` with module structure
- [x] T002 [P1] [Setup] Verify existing `SchemaVersion` struct in `crates/dampen-core/src/ir/mod.rs`
- [x] T003 [P1] [Setup] Verify existing `ParseErrorKind` enum in `crates/dampen-core/src/parser/error.rs`

## Phase 2: Foundational

- [x] T004 [P1] [US2] Add `UnsupportedVersion` variant to `ParseErrorKind` in `crates/dampen-core/src/parser/error.rs`
- [x] T005 [P1] [US2] Add `MAX_SUPPORTED_VERSION` constant to `crates/dampen-core/src/parser/mod.rs`
- [x] T006 [P1] [US2] Export `MAX_SUPPORTED_VERSION` from `crates/dampen-core/src/lib.rs` if needed

## Phase 3: US1 + US2 Implementation (P1)

### TDD: parse_version_string Tests

- [x] T007 [P1] [US1] Write test: `parse_valid_version_1_0` expects `Ok(SchemaVersion { major: 1, minor: 0 })`
- [x] T008 [P1] [US1] Write test: `parse_valid_version_0_1` expects `Ok(SchemaVersion { major: 0, minor: 1 })`
- [x] T009 [P1] [US1] Write test: `parse_valid_version_with_whitespace` for `" 1.0 "` → trimmed to 1.0
- [x] T010 [P1] [US1] Write test: `parse_valid_version_leading_zeros` for `"01.00"` → 1.0
- [x] T011 [P1] [US3] Write test: `parse_invalid_empty_string` expects error with "cannot be empty"
- [x] T012 [P1] [US3] Write test: `parse_invalid_single_number` for `"1"` expects format error
- [x] T013 [P1] [US3] Write test: `parse_invalid_triple_version` for `"1.0.0"` expects format error
- [x] T014 [P1] [US3] Write test: `parse_invalid_prefix` for `"v1.0"` expects format error
- [x] T015 [P1] [US3] Write test: `parse_invalid_non_numeric` for `"1.x"` expects format error
- [x] T016 [P1] [US3] Write test: `parse_invalid_negative` for `"-1.0"` expects format error

### Implementation: parse_version_string

- [x] T017 [P1] [US1] Implement `parse_version_string(version_str: &str, span: Span) -> Result<SchemaVersion, ParseError>` in `crates/dampen-core/src/parser/mod.rs`
- [x] T018 [P1] [US1] Verify all parse_version_string tests pass

### TDD: validate_version_supported Tests

- [x] T019 [P1] [US2] Write test: `validate_supported_version_1_0` expects `Ok(())`
- [x] T020 [P1] [US2] Write test: `validate_supported_version_0_9` expects `Ok(())`
- [x] T021 [P1] [US2] Write test: `validate_unsupported_version_1_1` expects `UnsupportedVersion` error
- [x] T022 [P1] [US2] Write test: `validate_unsupported_version_2_0` expects `UnsupportedVersion` error
- [x] T023 [P1] [US2] Write test: Error message includes declared version and max version

### Implementation: validate_version_supported

- [x] T024 [P1] [US2] Implement `validate_version_supported(version: &SchemaVersion, span: Span) -> Result<(), ParseError>` in `crates/dampen-core/src/parser/mod.rs`
- [x] T025 [P1] [US2] Verify all validate_version_supported tests pass

### Integration: Parser Updates

- [x] T026 [P1] [US1] Write test: `parse_document_with_version_1_0` expects version in DampenDocument
- [x] T027 [P1] [US1] Write test: `parse_document_without_version_defaults` expects version 1.0
- [x] T028 [P1] [US2] Write test: `parse_document_with_unsupported_version` expects error
- [x] T029 [P1] [US3] Write test: `parse_document_with_invalid_version_format` expects error
- [x] T030 [P1] [US1] Update `parse()` function in `crates/dampen-core/src/parser/mod.rs` to call `parse_version_string` and `validate_version_supported`
- [x] T031 [P1] [US1] Remove hardcoded version 1.0 defaults (lines ~75 and ~552 per research)
- [x] T032 [P1] [US1] Verify all parser integration tests pass

## Phase 4: US3 - Invalid Format Handling (P2)

- [x] T033 [P2] [US3] Write test: `parse_version_suffix` for `"1.0-beta"` expects format error
- [x] T034 [P2] [US3] Write test: `parse_version_text` for `"one.zero"` expects format error
- [x] T035 [P2] [US3] Verify error messages include the invalid input value
- [x] T036 [P2] [US3] Verify error messages include expected format suggestion

## Phase 5: US4 - File Updates (P2)

### Audit Existing Files

- [x] T037 [P2] [US4] List all `.dampen` files in `examples/` that need version attribute
- [x] T038 [P2] [US4] List all `.dampen` files in `crates/dampen-cli/templates/` that need version attribute
- [x] T039 [P2] [US4] List all `.dampen` files in `crates/dampen-core/tests/fixtures/` that need version attribute

### Update Example Files

- [x] T040 [P2] [US4] Add `version="1.0"` to `examples/hello-world/src/ui/window.dampen`
- [x] T041 [P2] [US4] Add `version="1.0"` to `examples/styling/src/ui/window.dampen`
- [x] T042 [P2] [US4] Add `version="1.0"` to `examples/settings/src/ui/*.dampen` files
- [x] T043 [P2] [US4] Add `version="1.0"` to `examples/todo-app/src/ui/window.dampen`
- [x] T044 [P2] [US4] Add `version="1.0"` to all `examples/widget-showcase/src/ui/*.dampen` files
- [x] T045 [P2] [US4] Verify `examples/counter/src/ui/window.dampen` already has version (per plan.md)

### Update Template Files

- [x] T046 [P2] [US4] Add `version="1.0"` to `crates/dampen-cli/templates/new/src/ui/window.dampen.template`
- [x] T047 [P2] [US4] Verify `dampen new` generates files with version attribute

### Update Test Fixtures

- [x] T048 [P2] [US4] Add `version="1.0"` to test fixture files in `crates/dampen-core/tests/fixtures/`
- [x] T049 [P2] [US4] Add `version="1.0"` to any inline test XML strings that should have explicit versions

### Verification

- [x] T050 [P2] [US4] Run `grep -r '<dampen>' examples/ crates/` to find any remaining files without version
- [x] T051 [P2] [US4] Run full test suite to verify all files parse correctly

## Phase 6: US5 - Widget Version Infrastructure (P3)

- [x] T052 [P3] [US5] Write test: `widget_kind_column_minimum_version` expects v1.0
- [x] T053 [P3] [US5] Write test: `widget_kind_radio_minimum_version` expects v1.0
- [x] T054 [P3] [US5] Write test: `widget_kind_canvas_minimum_version` expects v1.0
- [x] T055 [P3] [US5] Implement `WidgetKind::minimum_version(&self) -> SchemaVersion` in `crates/dampen-core/src/ir/node.rs`
- [x] T056 [P3] [US5] Add rustdoc comment explaining future enforcement plan
- [x] T057 [P3] [US5] Verify all widget minimum_version tests pass

## Phase 7: Polish

### Documentation

- [ ] T058 [P2] [Docs] Update `docs/XML_SCHEMA.md` with version attribute documentation
- [ ] T059 [P2] [Docs] Add version error examples to troubleshooting section
- [ ] T060 [P3] [Docs] Update `CHANGELOG.md` with version validation feature

### Final Verification

- [ ] T061 [P1] [QA] Run `cargo test --workspace` - all tests pass
- [ ] T062 [P1] [QA] Run `cargo clippy --workspace -- -D warnings` - no warnings
- [ ] T063 [P1] [QA] Run `cargo fmt --all -- --check` - properly formatted
- [ ] T064 [P1] [QA] Verify all success criteria from spec.md (SC-001 through SC-008)

---

## Summary

| Phase | Task Count | Priority Breakdown |
|-------|------------|-------------------|
| Phase 1: Setup | 3 | P1: 3 |
| Phase 2: Foundational | 3 | P1: 3 |
| Phase 3: US1 + US2 | 26 | P1: 26 |
| Phase 4: US3 | 4 | P2: 4 |
| Phase 5: US4 | 15 | P2: 15 |
| Phase 6: US5 | 6 | P3: 6 |
| Phase 7: Polish | 7 | P1: 4, P2: 2, P3: 1 |
| **Total** | **64** | P1: 36, P2: 21, P3: 7 |

## Dependency Graph

```
T001-T003 (Setup)
    │
    ▼
T004-T006 (Foundational: Error type, constant)
    │
    ├──────────────────┐
    ▼                  ▼
T007-T018           T019-T025
(parse_version)     (validate_version)
    │                  │
    └────────┬─────────┘
             ▼
        T026-T032
    (Parser integration)
             │
    ┌────────┼────────┐
    ▼        ▼        ▼
T033-T036  T037-T051  T052-T057
(US3)      (US4)      (US5)
    │        │          │
    └────────┴────┬─────┘
                  ▼
            T058-T064
            (Polish)
```

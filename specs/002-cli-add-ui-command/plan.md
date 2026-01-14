# Implementation Plan: CLI Add UI Command

**Branch**: `002-cli-add-ui-command` | **Date**: 2026-01-13 | **Spec**: [spec.md](spec.md)  
**Input**: Feature specification from `/specs/002-cli-add-ui-command/spec.md`

**Note**: This template is filled in by the `/speckit.plan` command. See `.specify/templates/commands/plan.md` for the execution workflow.

## Summary

Add a `dampen add --ui <window_name>` CLI command that generates UI window files (`.rs` and `.dampen`) based on the hello-world example template. The command validates project context, normalizes window names to snake_case, prevents file overwrites, and supports custom output paths via `--path` option. This feature reduces UI window scaffolding time from ~5 minutes (manual) to <10 seconds (automated).

## Technical Context

**Language/Version**: Rust Edition 2024, MSRV 1.85 (aligned with Dampen constitution)  
**Primary Dependencies**: `clap` 4.0+ (CLI parsing), `std::fs` (file operations), existing dampen-cli infrastructure  
**Storage**: File-based templates (`.rs.template` and `.dampen.template` files) stored in `crates/dampen-cli/templates/add/`  
**Testing**: `cargo test` with contract tests, integration tests, and unit tests for validation logic  
**Target Platform**: Linux/macOS/Windows (CLI tool, filesystem operations)  
**Project Type**: Single crate extension (dampen-cli) with new subcommand  
**Performance Goals**: Command execution <5 seconds, file generation <1 second  
**Constraints**: Must work within existing CLI structure, reuse patterns from `dampen new`, no breaking changes  
**Scale/Scope**: Single command, 2 templates, ~500 LOC including tests

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

### ✅ Declarative-First
**Status**: PASS (N/A for CLI feature)  
This feature generates declarative `.dampen` XML files but does not modify the declarative UI system itself.

### ✅ Type Safety Preservation
**Status**: PASS  
Generated templates include `#[derive(UiModel)]` and typed handler functions. No type erasure introduced.

### ✅ Production Mode
**Status**: PASS (N/A for CLI feature)  
This is a developer tool that generates code. Generated code follows production mode patterns (uses `#[dampen_ui]` macro for compile-time loading).

### ✅ Backend Abstraction
**Status**: PASS  
CLI tool operates at the project level, does not introduce backend dependencies. Generated templates reference `dampen-core` and `dampen-macros` appropriately.

### ✅ Test-First Development
**Status**: PASS  
Will follow TDD: contract tests for command execution → implementation → integration tests for file generation.

### Quality Gates
- **Tests**: All new code covered by unit + integration tests (target >90%)
- **Linting**: `cargo clippy --workspace -- -D warnings` (zero warnings)
- **Formatting**: `cargo fmt --all -- --check`
- **Documentation**: All public functions have rustdoc comments

**Constitution Compliance**: ✅ All principles satisfied, no violations to justify.

## Project Structure

### Documentation (this feature)

```text
specs/002-cli-add-ui-command/
├── spec.md              # Feature specification (completed)
├── plan.md              # This file (/speckit.plan command output)
├── research.md          # Phase 0 output (technical decisions)
├── data-model.md        # Phase 1 output (command structure)
├── quickstart.md        # Phase 1 output (usage examples)
├── contracts/           # Phase 1 output (templates, validation)
│   ├── window_rs_template.md
│   └── window_dampen_template.md
└── tasks.md             # Phase 2 output (/speckit.tasks command - NOT created by /speckit.plan)
```

### Source Code (repository root)

```text
crates/dampen-cli/
├── src/
│   ├── lib.rs                    # Add `Add(commands::AddArgs)` to Commands enum
│   ├── main.rs                   # (no changes)
│   └── commands/
│       ├── mod.rs                # Export add module
│       ├── add.rs                # NEW: Main add command implementation
│       ├── add/                  # NEW: Add command modules
│       │   ├── mod.rs            # Re-exports
│       │   ├── validation.rs    # Window name validation, project detection
│       │   ├── generation.rs    # Template processing and file creation
│       │   ├── templates.rs     # Template loading and placeholder replacement
│       │   └── errors.rs        # Custom error types
│       ├── new.rs                # (reference for patterns)
│       └── [other commands]
│
├── templates/
│   ├── new/                      # Existing templates
│   └── add/                      # NEW: Templates for add command
│       ├── window.rs.template    # Rust module template
│       └── window.dampen.template # XML UI template
│
└── tests/
    ├── cli_add_tests.rs          # NEW: Integration tests for add command
    └── [other test files]

tests/ (workspace root)
└── integration/
    └── cli_add_integration.rs    # NEW: End-to-end tests
```

**Structure Decision**: Extension of existing `dampen-cli` crate. Follows the pattern established by `dampen new` command with modular organization under `commands/add/` directory. Templates stored in `templates/add/` to mirror the `templates/new/` structure.

## Complexity Tracking

> **Fill ONLY if Constitution Check has violations that must be justified**

N/A - No constitution violations. This feature extends existing CLI infrastructure without introducing complexity.

## Phase 0: Research & Decisions

### Research Tasks

1. **Window Name Validation Patterns**
   - Research: Rust identifier rules (RustDoc, syn crate conventions)
   - Decision needed: How to handle CamelCase → snake_case conversion
   - Reference: Existing validation in `dampen new` command

2. **Template Engine Choice**
   - Research: Simple placeholder replacement vs. template engine (handlebars, tera)
   - Decision needed: Reuse pattern from `dampen new` (include_str! + replace) or introduce template engine
   - Constraint: Keep dependencies minimal (Constitution principle)

3. **Cargo.toml Parsing**
   - Research: Methods to detect Dampen project (cargo_metadata crate, manual parsing)
   - Decision needed: Lightweight validation approach
   - Requirement: Check for dampen-core dependency

4. **Path Normalization**
   - Research: Cross-platform path handling (std::path, pathdiff crate)
   - Decision needed: How to resolve relative paths, detect out-of-project paths
   - Requirement: Works on Windows, Linux, macOS

5. **Atomic File Creation**
   - Research: Strategies to prevent partial file creation on failure
   - Decision needed: Write to temp + rename, or transactional approach
   - Requirement: Edge case handling (disk full, permissions)

6. **Error Message Design**
   - Research: CLI UX best practices (clap examples, Cargo error messages)
   - Decision needed: Error message format, suggestion patterns
   - Requirement: Clear, actionable messages with suggestions

### Expected Outcomes

- `research.md` documenting all decisions above
- Technology choices aligned with existing codebase patterns
- No new major dependencies introduced

## Phase 1: Design & Contracts

### Data Model

Key entities (see `data-model.md` for details):

1. **AddCommand** - CLI argument structure
2. **WindowName** - Validated window identifier (snake_case)
3. **TargetPath** - Resolved filesystem path (within project)
4. **WindowTemplate** - Loaded template content with placeholders
5. **ValidationResult** - Project validation outcome

### Contracts

**Templates** (`/contracts/`):

1. `window_rs_template.md` - Rust module template contract
   - Placeholders: `{{WINDOW_NAME}}`, `{{WINDOW_NAME_SNAKE}}`, `{{WINDOW_NAME_PASCAL}}`
   - Required sections: Model struct, #[dampen_ui], create_app_state(), handler registry

2. `window_dampen_template.md` - XML UI template contract
   - Placeholders: `{{WINDOW_NAME_TITLE}}`
   - Required structure: Column layout, text, button, data binding example

**Validation Rules**:
- Window name must start with letter or underscore
- Window name contains only alphanumeric + underscores
- Path must be relative or within project directory
- Cargo.toml must exist in project root
- dampen-core must be in dependencies or dev-dependencies

### Quickstart

**Developer usage** (see `quickstart.md` for full examples):

```bash
# Basic usage
dampen add --ui settings

# Custom path
dampen add --ui new_order --path "src/ui/orders/"

# With various naming styles (all convert to snake_case)
dampen add --ui MyWindow      # Creates my_window.rs
dampen add --ui user_profile  # Creates user_profile.rs
```

## Phase 2: Task Breakdown

*This phase is handled by the separate `/speckit.tasks` command.*

Tasks will include:
- Setup: Create module structure, add templates
- Core: Implement validation, path resolution, template processing
- Integration: Wire command into CLI, add to Commands enum
- Testing: Contract tests, integration tests, error case tests
- Documentation: Update CLI docs, add examples

## Dependencies & Integration Points

### Reuses from Existing Codebase

1. **CLI Infrastructure** (`crates/dampen-cli/src/lib.rs`)
   - Commands enum pattern
   - Error handling conventions

2. **Template Pattern** (`crates/dampen-cli/src/commands/new.rs`)
   - `include_str!` + `.replace()` approach
   - File generation helpers
   - Directory creation logic

3. **Validation Patterns**
   - Similar to `validate_project_name` in new.rs
   - Error message formatting

### New Components

1. **AddArgs struct** - CLI arguments with clap derive
2. **Project validator** - Detects Dampen project via Cargo.toml
3. **Path normalizer** - Resolves custom paths, validates boundaries
4. **Template loader** - Reads and processes templates
5. **File generator** - Creates .rs and .dampen files atomically

## Risks & Mitigations

| Risk | Impact | Mitigation |
|------|--------|------------|
| File overwrite causes data loss | High | Check existence before write, clear error messages |
| Invalid paths crash command | Medium | Comprehensive path validation and normalization |
| Template placeholders incomplete | Medium | Contract tests verify all placeholders replaced |
| Cross-platform path issues | Medium | Use std::path consistently, test on Windows |
| Cargo.toml parsing fragility | Low | Use simple string matching, not full parsing |

## Success Metrics

From spec.md Success Criteria:

- **SC-001**: Command execution < 5 seconds ✓ (automated test)
- **SC-002**: Generated files compile 100% ✓ (integration test)
- **SC-003**: Generated XML validates 100% ✓ (integration test with `dampen check`)
- **SC-004**: Error messages clear 100% ✓ (manual review + user testing)
- **SC-005**: Custom paths work 100% ✓ (integration tests)
- **SC-006**: Prevents overwrites 100% ✓ (unit tests)
- **SC-007**: Detects non-Dampen projects 100% ✓ (unit tests)
- **SC-008**: Time reduction 5min → <10sec ✓ (manual benchmark)

## Next Steps

1. Run `/speckit.plan` to generate Phase 0 (research.md) and Phase 1 artifacts
2. Review and approve research decisions
3. Generate Phase 1 contracts and data model
4. Update agent context with new patterns
5. Run `/speckit.tasks` to create detailed task breakdown
6. Begin implementation following TDD principles

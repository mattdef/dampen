# Implementation Plan: Showcase Todo Application

**Branch**: `001-showcase-todo-app` | **Date**: 2026-01-14 | **Spec**: [spec.md](spec.md)  
**Input**: Feature specification from `/specs/001-showcase-todo-app/spec.md`

## Summary

Transform the existing `examples/todo-app` into a flagship showcase application demonstrating all Dampen framework capabilities: custom styling, theming, multi-window architecture with shared state, hot-reload development workflow, advanced data bindings, and transparent code generation. The application will serve as the primary reference implementation for developers evaluating Dampen, exhibiting production-quality visual design with modern UI patterns while maintaining code clarity for educational purposes.

**Technical Approach**: Enhance existing single-window todo-app by (1) implementing a comprehensive design system with light/dark themes, (2) adding a statistics window using SharedContext for inter-window communication, (3) enriching XML definitions with advanced binding patterns and conditional rendering, (4) documenting hot-reload workflow with error handling examples, and (5) providing introspection tools for examining generated code. All functionality will be achieved using pure Dampen constructs without custom Iced code.

## Technical Context

**Language/Version**: Rust Edition 2024, MSRV 1.85 (aligned with Dampen constitution)  
**Primary Dependencies**: 
- `dampen-core` (parser, AppState, binding system)
- `dampen-macros` (UiModel, dampen_ui, dampen_app macros)
- `dampen-iced` (Iced 0.14+ backend)
- `dampen-dev` (hot-reload FileWatcher) - dev-dependencies only
- `iced` 0.14+ (UI framework)
- `serde`, `serde_json` (state persistence)

**Storage**: File-based state persistence (`.dampen-state.json` for task data, theme preferences)  
**Testing**: 
- Contract tests: XML parsing → expected IR
- Integration tests: Full UI rendering, event dispatch, state updates
- Manual testing: Visual quality, theme switching, multi-window sync
- `cargo test --workspace` for framework tests
- Visual regression testing via screenshots (manual comparison)

**Target Platform**: Desktop (Windows, macOS, Linux) - Iced native windows  
**Project Type**: Single application example (within `examples/todo-app/`)  
**Performance Goals**: 
- Theme switching: <300ms visual transition
- Multi-window sync: <50ms state propagation
- Hot-reload: <1s XML-to-UI update
- Smooth scrolling with 500+ tasks at 60 FPS
- Binary size: <15MB release build

**Constraints**: 
- Must use only Dampen framework features (no raw Iced code in examples)
- Must compile with stable Rust (no nightly)
- Must demonstrate all 6 feature categories from spec
- Must remain understandable to Dampen beginners (avoid over-engineering)
- Code must pass `cargo clippy -- -D warnings`
- Must work across all Dampen-supported platforms

**Scale/Scope**: 
- Single example application (~1000 LOC Rust, ~500 LOC XML)
- 2 windows (main + statistics)
- ~20 UI components (buttons, inputs, lists, charts)
- 15+ handler functions
- 2 themes (light + dark)
- 10+ binding patterns demonstrated

## Constitution Check

*GATE: Must pass before Phase 0 research. Re-check after Phase 1 design.*

### I. Declarative-First ✅ **PASS**

**Requirement**: XML is the source of truth for UI structure. All UI definitions MUST be declared in `.dampen` XML files.

**Plan Compliance**: 
- All UI structure defined in `window.dampen` and `statistics.dampen` XML files
- Visual design (themes, styles, layouts) fully declarative
- No imperative widget construction in Rust code
- Handlers contain only business logic, not UI structure

**Verification**: Grep for raw Iced widget construction (should be zero instances in example code)

---

### II. Type Safety Preservation ✅ **PASS**

**Requirement**: No runtime type erasure for messages or state. All message types and model fields MUST be statically typed.

**Plan Compliance**:
- Model uses `#[derive(UiModel)]` for type-safe field access
- Handler registry validates function signatures at compile time
- All bindings resolve to concrete types (no `dyn Any` leakage to user code)
- `Message` enum variants are strongly typed (not stringly typed)

**Verification**: Run `cargo build --release` and verify zero type-related warnings

---

### III. Production Mode ✅ **PASS**

**Requirement**: Static code generation for deployments. Release builds MUST use codegen mode where XML is compiled to Rust code at build time.

**Plan Compliance**:
- `build.rs` compiles `.dampen` files to Rust during build phase
- No XML parsing in release binaries (verified via binary inspection)
- `#[dampen_ui]` macro embeds parsed Document as `static`
- Production binaries contain zero runtime XML interpretation

**Verification**: 
```bash
cargo build --release
strings target/release/todo-app | grep -i "xml" # Should find zero XML fragments
```

---

### IV. Backend Abstraction ✅ **PASS**

**Requirement**: The `dampen-core` crate MUST have no Iced dependency. Backend-specific code in `dampen-iced` only.

**Plan Compliance**:
- Example uses `dampen-iced` for Iced backend implementation
- Model definition uses only `dampen-core` traits (UiBindable)
- No direct `iced::` imports in model or handler logic
- Backend abstraction demonstrated by clear separation of concerns

**Verification**: Check imports in `src/ui/window.rs` - should only import `dampen_core`, not `iced`

---

### V. Test-First Development ⚠️ **PARTIAL COMPLIANCE - JUSTIFIED**

**Requirement**: Tests define contracts before implementation (TDD). Contract tests MUST be written first.

**Plan Compliance**:
- This is an *example application*, not a framework feature
- Framework features (SharedContext, theming, bindings) already have comprehensive tests
- Example demonstrates *usage* of tested features, not new framework code
- Manual testing checklist provided for visual quality validation

**Justification**: TDD applies to framework development (dampen-core, dampen-macros). Example applications serve as integration tests and documentation. Writing unit tests for example code would duplicate framework tests and obscure the educational purpose. Manual testing is appropriate for UI/UX validation.

**Verification**: Framework test suite passes (`cargo test --workspace`)

---

### Quality Gates ✅ **PASS**

**Plan Compliance**:
- All code passes `cargo clippy --workspace -- -D warnings`
- All code formatted with `cargo fmt --all`
- Public APIs (if any) documented with rustdoc
- Error messages provide actionable guidance
- Uses `Result<T, E>` for fallible operations (file I/O)

---

### Re-Evaluation After Phase 1

Will verify:
1. XML definitions use only documented Dampen elements
2. Generated code is readable and follows Rust conventions
3. No unsafe code introduced
4. Performance budgets met (measured in Phase 2 testing)

## Project Structure

### Documentation (this feature)

```text
specs/001-showcase-todo-app/
├── plan.md              # This file (/speckit.plan command output)
├── research.md          # Phase 0: Design system research, multi-window patterns
├── data-model.md        # Phase 1: Task, Theme, Statistics entities
├── quickstart.md        # Phase 1: Developer guide for running/modifying showcase
├── contracts/           # Phase 1: XML schema examples, handler contracts
│   ├── window-schema.md       # Main window XML structure
│   ├── statistics-schema.md   # Statistics window XML structure
│   └── theme-schema.md        # Theme definition format
└── tasks.md             # Phase 2: NOT created by /speckit.plan - use /speckit.tasks
```

### Source Code (repository root)

```text
examples/todo-app/
├── Cargo.toml                 # Project dependencies (dampen-*, iced, serde)
├── build.rs                   # Compile .dampen files to Rust
├── README.md                  # User-facing documentation, feature overview
├── src/
│   ├── main.rs                # App entry point with #[dampen_app] macro
│   └── ui/
│       ├── mod.rs             # Re-exports window and statistics modules
│       ├── window.rs          # Main window: Model, handlers, create_app_state
│       ├── window.dampen      # Main window XML: task list, add form, theme toggle
│       ├── statistics.rs      # Statistics window: Model, handlers, create_app_state
│       └── statistics.dampen  # Statistics window XML: charts, metrics display
├── assets/                    # Visual assets (icons, example images)
│   ├── priority-low.svg       # Low priority icon
│   ├── priority-medium.svg    # Medium priority icon
│   └── priority-high.svg      # High priority icon
└── tests/
    └── manual_checklist.md    # Manual testing scenarios for visual validation
```

**Structure Decision**: Single project structure (Option 1) - this is an example application, not a library. All code resides in `examples/todo-app/` following Dampen's example conventions. The existing structure is retained and extended with a new `statistics` module for the second window.

**Key Files**:
- `window.dampen`: Main window with comprehensive theme/style demonstrations
- `statistics.dampen`: Secondary window showcasing SharedContext usage
- `window.rs`: Main model with all task management logic
- `statistics.rs`: Statistics model (computed values derived from shared state)
- `main.rs`: Application orchestration using `#[dampen_app]` macro

## Complexity Tracking

> **Fill ONLY if Constitution Check has violations that must be justified**

| Violation | Why Needed | Simpler Alternative Rejected Because |
|-----------|------------|-------------------------------------|
| V. Test-First (partial) | Example application for demonstration/documentation | Unit tests would duplicate framework tests and obscure educational intent. Manual testing appropriate for UI/UX validation. |

**Additional Justification**: 
- The Dampen framework itself follows strict TDD (>90% coverage in dampen-core)
- Examples serve as *integration tests* for the framework
- Visual quality (primary success criterion) requires human evaluation
- Writing tests for example code creates maintenance burden without quality benefit
- Industry standard: UI frameworks provide tested examples, not test suites for examples

## Research Phase (Phase 0)

### Research Questions

1. **Design System Patterns**: What modern design systems provide good reference for color palettes, spacing, typography?
   - **Research Task**: Survey popular design systems (Material Design 3, Apple HIG, Fluent 2, Tailwind) for:
     - Color palette structure (light/dark variants, semantic colors)
     - Spacing scales (4px, 8px, 16px grids)
     - Typography hierarchies (font sizes, weights, line heights)
     - Animation timing (transition durations, easing functions)

2. **SharedContext Multi-Window Patterns**: How should statistics window subscribe to main window state changes?
   - **Research Task**: Review Dampen 0.2.4 SharedContext implementation:
     - Subscription mechanism for cross-window updates
     - Message routing between windows
     - State synchronization performance characteristics
     - Error handling when windows are closed/reopened

3. **Iced Theme Customization**: How to implement theme switching within Iced's theme system?
   - **Research Task**: Investigate Iced 0.14+ theming:
     - Built-in theme variants vs custom themes
     - Runtime theme switching patterns
     - CSS-like style cascade vs explicit styling
     - Performance implications of theme changes

4. **Hot-Reload Development Workflow**: What's the optimal developer experience for XML modifications?
   - **Research Task**: Document hot-reload capabilities:
     - FileWatcher setup and configuration
     - Error overlay implementation patterns
     - State preservation strategies during reload
     - Performance impact of file watching

5. **Visual Design Inspiration**: What visual patterns make todo apps feel "modern and professional"?
   - **Research Task**: Analyze modern todo applications (Todoist, Things 3, Microsoft To Do):
     - Layout patterns (card-based, list-based)
     - Micro-interactions (completion animations, add transitions)
     - Empty states and onboarding patterns
     - Accessibility patterns (contrast, focus indicators)

6. **Code Generation Inspection Tools**: How should developers inspect generated Rust code?
   - **Research Task**: Document existing tooling:
     - `dampen inspect` CLI command capabilities
     - Generated code location (target/debug/build)
     - Comment annotations in generated code
     - Comparison patterns (XML side-by-side with Rust output)

**Output**: `research.md` consolidating findings with design decisions and rationale

## Design Phase (Phase 1)

### Data Model (`data-model.md`)

Define entities and relationships:

1. **Task Entity**
   - Fields: id (unique), text (description), completed (boolean), created_at (timestamp), category (string), priority (enum)
   - Validation: text non-empty, id unique, created_at immutable
   - Relationships: belongs to Application State
   - Persistence: serialized to JSON

2. **Theme Entity**
   - Fields: variant (Light/Dark enum), palette (color map), typography (font settings), spacing (grid unit)
   - Validation: colors are valid hex codes, spacing > 0
   - Relationships: applied globally to all windows
   - Persistence: preference saved to state file

3. **Statistics Entity** (computed)
   - Fields: total_count, completed_count, pending_count, completion_percentage, recent_activity
   - Validation: counts non-negative, percentage 0-100
   - Relationships: derived from Task collection
   - Persistence: not persisted (recomputed on load)

4. **Application State** (root)
   - Fields: tasks (Vec<Task>), theme (Theme), window_states (HashMap<WindowId, bool>)
   - Shared State: tasks collection shared between windows via SharedContext
   - Lifecycle: initialized on startup, persisted on change

### Contracts (`contracts/`)

#### `window-schema.md`: Main Window XML Structure

```xml
<dampen version="1.0">
  <!-- Theme Definitions -->
  <themes>
    <theme name="light">
      <palette primary="..." secondary="..." ... />
      <typography font_family="..." font_size_base="..." ... />
      <spacing unit="8" />
    </theme>
    <theme name="dark">
      <!-- Dark theme variant -->
    </theme>
  </themes>

  <!-- Style Classes -->
  <styles>
    <style name="btn_primary">
      <base background="..." color="..." padding="..." border_radius="..." />
      <hover background="..." />
      <active background="..." />
    </style>
    <!-- More style classes -->
  </styles>

  <!-- Set Active Theme -->
  <global_theme name="light" binding="{theme_name}" />

  <!-- Main UI -->
  <scrollable>
    <container padding="20">
      <column spacing="20">
        <!-- Header with theme toggle -->
        <row spacing="15" align="center">
          <text value="Todo Showcase" size="32" weight="bold" />
          <space />
          <toggler label="Dark Mode" on_toggle="toggle_theme" active="{is_dark_mode}" />
        </row>

        <!-- Add Task Form -->
        <column spacing="10">
          <text value="Add New Task" size="20" weight="bold" />
          <row spacing="10">
            <text_input value="{new_task_text}" on_input="update_new_task" placeholder="What needs to be done?" />
            <button label="Add Task" on_click="add_task" enabled="{new_task_text.len() > 0}" class="btn_primary" />
          </row>
        </column>

        <!-- Task List with Conditional Rendering -->
        <column spacing="10">
          <text value="Your Tasks" size="20" weight="bold" />
          
          <!-- Empty state -->
          <text value="{if tasks.len() == 0 then 'No tasks yet! Add one above to get started.' else ''}" />

          <!-- Task items with <for> loop -->
          <for each="task" in="{tasks}">
            <row spacing="10" padding="10" class="task_item">
              <checkbox checked="{task.completed}" on_toggle="toggle_task:{task.id}" />
              <text value="{task.text}" class="{if task.completed then 'text_completed' else 'text_active'}" />
              <space />
              <button label="Delete" on_click="delete_task:{task.id}" class="btn_danger_small" />
            </row>
          </for>

          <!-- Progress Indicator -->
          <progress_bar min="0" max="100" value="{completion_percentage}" />
          <text value="{completed_count} of {total_count} tasks completed" />
        </column>

        <!-- Actions -->
        <row spacing="10">
          <button label="Open Statistics" on_click="open_statistics" class="btn_outlined" />
          <space />
          <button label="Clear Completed" on_click="clear_completed" enabled="{completed_count > 0}" />
        </row>
      </column>
    </container>
  </scrollable>
</dampen>
```

**Handler Contracts**:
- `toggle_theme: () -> ()` - Switches between light/dark theme
- `update_new_task: (String) -> ()` - Updates input field state
- `add_task: () -> ()` - Creates new task from input
- `toggle_task: (usize) -> ()` - Toggles task completion status
- `delete_task: (usize) -> ()` - Removes task from list
- `clear_completed: () -> ()` - Removes all completed tasks
- `open_statistics: () -> ()` - Opens statistics window (sends Command)

#### `statistics-schema.md`: Statistics Window XML Structure

```xml
<dampen version="1.0">
  <!-- Inherits theme from global state -->
  
  <scrollable>
    <container padding="20">
      <column spacing="20">
        <!-- Header -->
        <text value="Task Statistics" size="32" weight="bold" />

        <!-- Metrics Grid -->
        <grid columns="2" spacing="20">
          <!-- Total Tasks Card -->
          <column spacing="10" padding="20" class="card">
            <text value="Total Tasks" size="16" weight="bold" />
            <text value="{shared.total_count}" size="48" weight="bold" />
          </column>

          <!-- Completed Tasks Card -->
          <column spacing="10" padding="20" class="card">
            <text value="Completed" size="16" weight="bold" />
            <text value="{shared.completed_count}" size="48" weight="bold" class="text_success" />
          </column>

          <!-- Pending Tasks Card -->
          <column spacing="10" padding="20" class="card">
            <text value="Pending" size="16" weight="bold" />
            <text value="{shared.pending_count}" size="48" weight="bold" class="text_warning" />
          </column>

          <!-- Completion Rate Card -->
          <column spacing="10" padding="20" class="card">
            <text value="Completion Rate" size="16" weight="bold" />
            <text value="{shared.completion_percentage}%" size="48" weight="bold" class="text_primary" />
          </column>
        </grid>

        <!-- Visual Progress -->
        <column spacing="10">
          <text value="Overall Progress" size="20" weight="bold" />
          <progress_bar min="0" max="100" value="{shared.completion_percentage}" style="success" />
        </column>

        <!-- Empty State -->
        <text value="{if shared.total_count == 0 then 'No tasks to analyze. Add tasks in the main window!' else ''}" />

        <!-- Close Button -->
        <row spacing="10">
          <space />
          <button label="Close" on_click="close_window" class="btn_outlined" />
        </row>
      </column>
    </container>
  </scrollable>
</dampen>
```

**Shared State Bindings**:
- `{shared.total_count}` - Total task count (computed)
- `{shared.completed_count}` - Completed task count (computed)
- `{shared.pending_count}` - Pending task count (computed)
- `{shared.completion_percentage}` - Completion rate 0-100 (computed)

**Handler Contracts**:
- `close_window: () -> ()` - Closes statistics window

#### `theme-schema.md`: Theme Definition Format

```xml
<theme name="light">
  <!-- Color Palette -->
  <palette
    primary="#3498db"        <!-- Primary actions, links -->
    secondary="#2ecc71"      <!-- Secondary actions -->
    success="#27ae60"        <!-- Success states, completed items -->
    warning="#f39c12"        <!-- Warnings, pending items -->
    danger="#e74c3c"         <!-- Destructive actions, errors -->
    background="#ecf0f1"     <!-- Page background -->
    surface="#ffffff"        <!-- Card backgrounds -->
    text="#2c3e50"           <!-- Primary text -->
    text_secondary="#7f8c8d" <!-- Secondary text, labels -->
  />

  <!-- Typography -->
  <typography
    font_family="Inter, system-ui, sans-serif"
    font_size_base="16"      <!-- Base font size in px -->
    font_size_small="12"     <!-- Small text -->
    font_size_large="20"     <!-- Large text -->
    font_weight="normal"     <!-- Default weight -->
    line_height="1.5"        <!-- Default line height -->
  />

  <!-- Spacing System (8px grid) -->
  <spacing unit="8" />
</theme>
```

**Design Tokens**:
- Colors follow WCAG AA contrast requirements (4.5:1 for text)
- Spacing uses 8px base unit (multiples: 8, 16, 24, 32, 40)
- Typography uses system font stack fallback
- Theme variants share structure, differ only in color values

### Quickstart Guide (`quickstart.md`)

Developer guide covering:

1. **Running the Showcase**
   ```bash
   cd examples/todo-app
   cargo run
   # Or with hot-reload enabled (debug mode):
   RUST_LOG=debug cargo run
   ```

2. **Modifying UI**
   - Edit `src/ui/window.dampen` for main window
   - Edit `src/ui/statistics.dampen` for statistics window
   - Changes hot-reload in debug mode (<1s latency)
   - Invalid XML shows error overlay with fix suggestions

3. **Adding New Themes**
   - Define `<theme name="custom">` in XML
   - Set color palette, typography, spacing
   - Reference via `<global_theme name="custom" />`
   - Toggle programmatically via theme state binding

4. **Extending Handlers**
   - Add handler function in `window.rs` or `statistics.rs`
   - Register in `create_handler_registry()`
   - Reference in XML via `on_click="handler_name"`
   - Use parameter syntax for dynamic handlers: `on_click="delete:{item.id}"`

5. **Inspecting Generated Code**
   ```bash
   # View generated Rust code
   cargo build
   cat target/debug/build/todo-app-*/out/window.rs

   # Use Dampen CLI inspector
   dampen inspect src/ui/window.dampen
   ```

6. **Testing Changes**
   - Visual testing: Run app, verify appearance/interactions
   - State persistence: Close/reopen app, verify tasks retained
   - Theme consistency: Toggle theme, verify all elements update
   - Multi-window: Open statistics, verify real-time sync

### Agent Context Update

After completing Phase 1 artifacts, run:

```bash
.specify/scripts/bash/update-agent-context.sh opencode
```

This will update `AGENTS.md` with new showcase example reference without modifying existing framework documentation.

## Phase Breakdown

### Phase 0: Research & Decisions ✅ (To be completed by `/speckit.plan`)

**Deliverable**: `research.md`

**Activities**:
1. Research design systems for color/spacing/typography patterns
2. Document SharedContext multi-window usage from existing implementation
3. Review Iced 0.14+ theming capabilities
4. Document hot-reload workflow and error handling
5. Analyze modern todo app UX patterns
6. Document code inspection tooling

**Exit Criteria**: All "NEEDS CLARIFICATION" resolved, design decisions documented

### Phase 1: Design & Contracts ✅ (To be completed by `/speckit.plan`)

**Deliverable**: `data-model.md`, `contracts/*.md`, `quickstart.md`, updated `AGENTS.md`

**Activities**:
1. Define Task, Theme, Statistics data models
2. Create XML schema examples for both windows
3. Document handler contracts (signatures, behavior)
4. Write developer quickstart guide
5. Update agent context file

**Exit Criteria**: 
- All entities modeled with validation rules
- XML contracts specify element structure
- Handler signatures documented
- Quickstart enables any developer to run/modify showcase

### Phase 2: Task Breakdown (Use `/speckit.tasks` command - NOT created by this command)

**Note**: This phase is executed by the `/speckit.tasks` command, not `/speckit.plan`.

**Deliverable**: `tasks.md` with granular implementation tasks

**Activities** (performed by `/speckit.tasks`):
1. Break down implementation into atomic tasks
2. Assign priorities and dependencies
3. Estimate effort for each task
4. Create verification criteria

## Risk Assessment

### High Risk

**Risk**: Theme switching performance degrades with complex UI  
**Mitigation**: 
- Benchmark theme toggle on large task lists (500+ items)
- Profile rendering pipeline with Iced's built-in tools
- Optimize by caching themed styles if needed
- Success Criterion: <300ms measured via stopwatch in debug mode

**Risk**: Multi-window state synchronization has latency or race conditions  
**Mitigation**:
- Use existing SharedContext implementation (proven in 001-inter-window-communication)
- Add debug logging for message passing timing
- Test rapid updates (add/delete tasks while statistics open)
- Success Criterion: <50ms sync verified via debug timestamps

### Medium Risk

**Risk**: Hot-reload fails to preserve complex state (filters, editing mode)  
**Mitigation**:
- Document state preservation limitations in quickstart
- Test hot-reload with various state combinations
- Provide clear error messages if state is lost
- Success Criterion: Core state (tasks, theme) always preserved

**Risk**: Generated code is unreadable or contains unexpected patterns  
**Mitigation**:
- Manually inspect generated code during development
- Run `cargo clippy` on generated code (currently passing)
- Document any non-obvious code patterns in comments
- Success Criterion: Generated code passes clippy, human-reviewable

### Low Risk

**Risk**: Visual design doesn't meet "modern and professional" criterion  
**Mitigation**:
- Follow established design system patterns from research
- Gather feedback from 3+ developers during development
- Iterate on spacing, colors, typography based on feedback
- Success Criterion: 90%+ positive aesthetic feedback (per spec SC-005)

**Risk**: Binary size exceeds 15MB target  
**Mitigation**:
- Measure release build size early (`cargo build --release`)
- Strip symbols if needed (`strip target/release/todo-app`)
- Iced apps typically <10MB, target has margin
- Success Criterion: Release binary <15MB

## Implementation Strategy

### Incremental Enhancement Approach

Rather than rewriting from scratch, incrementally enhance existing todo-app:

**Phase A: Visual Design Foundation** (P1)
1. Enhance theme definitions (richer color palettes)
2. Add comprehensive style classes (button variants, cards)
3. Improve spacing and typography hierarchy
4. Add smooth animations (task add/remove, completion toggle)
5. Verify: Visual polish matches "modern and professional" criterion

**Phase B: Multi-Window Architecture** (P2)
1. Create `statistics.rs` and `statistics.dampen` modules
2. Implement SharedContext setup in `main.rs`
3. Add "Open Statistics" command and window spawning
4. Implement shared state bindings in statistics window
5. Verify: Real-time sync <50ms, theme consistency across windows

**Phase C: Advanced Bindings** (P2)
1. Add computed value bindings (completion percentage, filtered counts)
2. Implement conditional rendering (empty states, completion messages)
3. Add complex list rendering with dynamic styling
4. Verify: All binding patterns work correctly, update reactively

**Phase D: Hot-Reload Documentation** (P3)
1. Document hot-reload setup in quickstart
2. Add error handling examples (invalid XML recovery)
3. Test state preservation across various reload scenarios
4. Verify: Developers understand hot-reload workflow

**Phase E: Code Generation Transparency** (P3)
1. Add comments in XML referencing generated code locations
2. Document `dampen inspect` CLI usage in quickstart
3. Provide side-by-side comparison examples
4. Verify: Developers can inspect and understand generated code

### Testing Strategy

**Visual Testing** (Manual):
- Checklist in `tests/manual_checklist.md` covering:
  - Theme switching (all elements update, <300ms)
  - Task CRUD operations (add/complete/delete with animations)
  - Multi-window sync (statistics update on main window changes)
  - Hot-reload (XML changes reflect instantly)
  - Edge cases (empty list, 500+ tasks, rapid interactions)

**Framework Testing** (Automated):
- Existing `cargo test --workspace` validates framework features
- No new framework code added (only example usage)

**Performance Testing** (Manual):
- Theme toggle stopwatch timing
- Multi-window sync debug logging
- Large task list scrolling (500+ items)
- Hot-reload latency measurement

**Acceptance Testing**:
- Run through all 28 acceptance scenarios from spec
- Verify all 14 success criteria measurable
- Gather feedback from 3+ developers (informal survey)

## Dependencies & Prerequisites

### Internal Dependencies
- Dampen 0.2.4+ with SharedContext support (already complete)
- `#[dampen_app]` macro (already exists)
- `#[dampen_ui]` macro for auto-loading (already exists)
- File watcher for hot-reload (dampen-dev, already exists)

### External Dependencies
- Iced 0.14+ (currently used)
- serde, serde_json 1.0+ (currently used)
- No new external dependencies required

### Prerequisites
- Rust 1.85+ stable
- Understanding of existing todo-app structure
- Familiarity with Dampen XML syntax
- Basic Iced concepts (not required for XML authoring)

## Timeline Estimate

**Phase 0 (Research)**: 2 hours
- Design system survey: 30 min
- SharedContext review: 30 min
- Iced theming research: 30 min
- UX pattern analysis: 30 min

**Phase 1 (Design)**: 3 hours
- Data modeling: 1 hour
- XML contracts: 1 hour
- Quickstart guide: 1 hour

**Phase 2 (Tasks)**: 1 hour
- Task breakdown with `/speckit.tasks` command

**Phase 3 (Implementation)**: Estimated 12-16 hours
- Phase A (Visual): 4-5 hours
- Phase B (Multi-window): 3-4 hours
- Phase C (Bindings): 2-3 hours
- Phase D (Hot-reload docs): 1-2 hours
- Phase E (Code gen docs): 1-2 hours
- Testing & polish: 1-2 hours

**Total**: ~18-22 hours elapsed (split across multiple sessions)

## Success Metrics

From spec Success Criteria, we'll measure:

**User Experience** (Manual Testing):
- SC-001: Task operations <100ms (stopwatch)
- SC-002: Theme switching <300ms (stopwatch)
- SC-003: 500+ tasks smooth (add 500 tasks, test scrolling)
- SC-004: 95% comprehension (informal survey of 3+ devs)

**Visual Quality** (Manual Review):
- SC-005: 90% positive aesthetic feedback (survey)
- SC-006: WCAG AA contrast (color picker verification)
- SC-007: 60 FPS animations (visual inspection, Iced profiler)

**Technical Demonstrations** (Manual + Automated):
- SC-008: <50ms multi-window sync (debug logging)
- SC-009: <1s hot-reload (file save to UI update timing)
- SC-010: Readable generated code (manual review + clippy)
- SC-011: <15MB binary size (`ls -lh target/release/todo-app`)

**Developer Impact** (Survey):
- SC-012: Identify 5+ features in 10 minutes (task observation)
- SC-013: 80% "easy to understand" rating (survey)
- SC-014: 40% support question reduction (future metric, not measurable immediately)

## Completion Criteria

This feature is **complete** when:

1. ✅ All 6 user stories have acceptance scenarios verified
2. ✅ All 37 functional requirements implemented
3. ✅ All 14 success criteria measurable and targets met
4. ✅ Manual testing checklist 100% passed
5. ✅ Code passes `cargo clippy -- -D warnings`
6. ✅ README documents all features
7. ✅ Quickstart enables any developer to run/modify showcase
8. ✅ Visual quality receives 90%+ positive feedback from 3+ reviewers
9. ✅ Constitution re-check passes (all gates green)
10. ✅ Example serves as primary reference in Dampen documentation

## Next Steps

After this plan is complete:

1. **Execute Phase 0**: Research design systems, document findings in `research.md`
2. **Execute Phase 1**: Model entities, create XML contracts, write quickstart guide
3. **Run `/speckit.tasks`**: Generate granular task breakdown in `tasks.md`
4. **Begin Implementation**: Follow incremental enhancement phases A→E
5. **Continuous Testing**: Run manual checklist after each phase
6. **Gather Feedback**: Show to developers for comprehension/aesthetic validation
7. **Iterate & Polish**: Address feedback, refine visual design
8. **Document**: Update main README and framework docs with showcase reference

---

**Plan Status**: ✅ Ready for Phase 0 Research  
**Constitution Gates**: ✅ All passed (1 justified deviation)  
**Next Command**: Execute research tasks, generate `research.md`

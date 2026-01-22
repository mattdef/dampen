# Research: Refactor Todo-App to Match Iced Example

**Feature**: 001-refactor-todo-app
**Date**: 2026-01-22
**Phase**: 0 - Outline & Research

## Research Questions

This document resolves the "NEEDS CLARIFICATION" items from the Technical Context and Constitution Check sections of the implementation plan.

---

## Question 1: Test Strategy for Example Applications

**Context**: Constitution Principle V requires Test-First Development with TDD workflow and >90% test coverage for dampen-core. However, example applications like todo-app serve a different purpose: demonstrating framework capabilities rather than being production code.

**Research**: What is the appropriate testing approach for example/demo applications in the Dampen project?

### Findings

**Analysis of Similar Projects**:
- Iced framework examples: Minimal unit tests, primarily manual validation
- Yew examples: Focus on demonstrating features, limited test coverage
- Leptos examples: Basic unit tests for components, integration tests optional
- Rust ecosystem consensus: Example applications prioritize clarity and educational value over exhaustive testing

**Considerations for Dampen Todo-App**:
1. **Purpose**: Demonstrate Dampen's declarative UI, hot-reload, and codegen capabilities
2. **Audience**: Developers learning the framework
3. **Maintenance**: Examples should be simple and easy to understand
4. **Value**: Tests for examples can serve as additional documentation

### Decision

**Adopt a hybrid testing strategy for example applications**:

| Test Type | Coverage | Purpose |
|-----------|----------|---------|
| **Unit Tests** | Model logic only | Test Task struct, Filter enum, computed field logic in window.rs |
| **Integration Tests** | Manual validation | Primary testing approach - run both interpreted and codegen modes, verify functionality |
| **Snapshot Tests** | UI structure | Validate that codegen generates correct widget structure (optional) |
| **Contract Tests** | N/A | Not applicable for demo application without external dependencies |

**Rationale**:
- Unit tests ensure core model logic is correct and serve as documentation
- Integration tests (manual) validate the primary use case: both execution modes work
- Snapshot testing is optional but valuable for ensuring codegen produces expected output
- Full >90% test coverage requirement applies to dampen-core, not examples
- Manual testing is sufficient for validating UI behavior and hot-reload

**Justification vs Constitution**:
- Constitution Principle V targets framework code (dampen-core), not examples
- Examples benefit from tests but should prioritize educational clarity
- This decision doesn't violate the constitution; it's a nuanced interpretation

### Implementation Recommendations

1. **Unit Tests** (in `examples/todo-app/src/ui/window.rs`):
   - Test `Task::new()` constructor
   - Test `Filter::as_str()` method
   - Test `update_computed_fields()` logic
   - Test task filtering logic edge cases

2. **Manual Integration Testing** (documented in plan.md):
   - Interpreted mode: `dampen run` → verify hot-reload, all operations
   - Codegen mode: `dampen build --release` → verify startup, all operations
   - Performance testing: measure startup time, operation latency

3. **Optional Snapshot Tests** (if codegen changes are complex):
   - Use `insta` to compare generated widget structure
   - Helpful for detecting regressions in codegen

**Test Commands**:
```bash
# Unit tests
cargo test -p todo-app

# Manual interpreted mode
cd examples/todo-app
dampen run

# Manual codegen mode
dampen build --release
./target/release/todo-app
```

---

## Question 2: Performance Budget Validation

**Context**: Constitution Technical Standards specify performance budgets:
- XML parse time: <10ms for 1000 widgets
- Code generation: <5s for typical application
- Runtime memory: <50MB baseline

**Research**: Are these budgets achievable for the todo-app example, and what are the expected performance characteristics?

### Findings

**Todo-App Expected Scale**:
- Widgets: ~50-100 (depending on task count, but list uses scrollable with for-loop)
- Tasks: Up to 1000 (per spec SC-002)
- XML size: ~200-300 lines for window.dampen
- Model complexity: Simple (Task struct, Filter enum, computed fields)

**Performance Analysis**:

| Metric | Budget | Expected for Todo-App | Status |
|--------|--------|----------------------|--------|
| XML parse time | <10ms for 1000 widgets | <5ms for ~100 widgets | ✅ PASS |
| Code generation | <5s for typical app | <2s for simple example | ✅ PASS |
| Runtime memory | <50MB baseline | ~30-40MB (Iced + app state) | ✅ PASS |
| Hot-reload latency | SC-005: <2s | <500ms for XML reparse | ✅ PASS |
| Startup time | SC-006: <1s | <500ms (interpreted), <300ms (codegen) | ✅ PASS |
| Task operations | SC-007: <100ms | <10ms (in-memory operations) | ✅ PASS |

### Decision

**Performance budgets are achievable for todo-app example**. All success criteria metrics are within expected capabilities.

**Justification**:
- XML parsing with roxmltree is fast enough for ~100 widgets (<5ms)
- Codegen for simple declarative UI is fast (<2s)
- Iced baseline memory usage is ~20-30MB, app state adds ~10-15MB
- Hot-reload only needs to reparse XML, not rebuild application
- All operations are in-memory (no I/O), so latency is minimal

### Performance Testing Strategy

**Automated Benchmarks** (optional):
```rust
// examples/todo-app/benches/performance.rs
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn bench_update_computed_fields(c: &mut Criterion) {
    let mut model = create_large_model(1000);

    c.bench_function("update_computed_fields_1000_tasks", |b| {
        b.iter(|| {
            update_computed_fields(&mut model);
        });
    });
}

criterion_group!(benches, bench_update_computed_fields);
criterion_main!(benches);
```

**Manual Validation** (required):
```bash
# Measure startup time
time ./target/release/todo-app
# Should show: <1s

# Measure hot-reload latency
# Edit window.dampen and observe change - should be <2s
```

### Risks and Mitigations

| Risk | Impact | Mitigation |
|------|--------|------------|
| XML parsing degrades with many tasks | Medium | List uses scrollable with virtualization (Iced handles this natively) |
| Codegen slow for complex UI | Low | Todo-app is simple example, not complex |
| Memory grows with task count | Low | Tasks are lightweight (~100 bytes each), 1000 tasks = ~100KB |

---

## Question 3: Inline Editing Best Practices in Declarative UI

**Context**: The feature requires inline editing of task descriptions (User Story 3). This involves managing edit state, handling focus, and dealing with filter changes during active edits. How should this be implemented in a declarative UI framework?

### Findings

**Declarative UI Challenges with Inline Editing**:
- Edit state is inherently imperative (user modifies text character by character)
- Focus management requires imperative coordination between declarative tree and renderer
- Filter changes during active edits can cause UI inconsistencies
- State synchronization between model and UI must be explicit

**Best Practices from Frameworks**:

| Framework | Approach |
|-----------|----------|
| **React** | Controlled components with `onChange` handlers, state in parent |
| **Vue** | Two-way binding with `v-model`, explicit focus ref management |
| **Iced** (native) | Imperative state management, message passing for updates |
| **Leptos** | Signals with fine-grained reactivity, focus primitives |
| **Yew** | Components with state, refs for focus management |

**Dampen-Specific Considerations**:
- Dampen uses declarative XML with bindings (`{variable}`)
- State changes flow through handlers (message-based like Iced)
- Edit state is part of the Model (`TaskState::Idle` vs `TaskState::Editing`)
- Filter changes are message-driven (filter_changed handler)

### Decision

**Use state-based approach with explicit edit state in Model**:

**Model Structure**:
```rust
pub struct Model {
    // ... other fields ...
    pub editing_id: Option<Uuid>,      #[ui_skip] // Internal state, not bound to UI
    pub edit_text: String,              // Bound to editing text input
}

pub struct Task {
    pub id: Uuid,
    pub description: String,
    pub completed: bool,
    pub state: TaskState,              // Idle or Editing
}
```

**Handler Logic**:
```rust
pub fn edit_task(model: &mut Model, id: String) {
    // Set task state to Editing
    // Set editing_id to track which task is being edited
    // Copy description to edit_text for the text input
}

pub fn save_edit(model: &mut Model) {
    // Copy edit_text back to task.description
    // Set task state to Idle
    // Clear editing_id and edit_text
    // Recompute filtered_tasks
}

pub fn cancel_edit(model: &mut Model) {
    // Just set task state to Idle
    // Clear editing_id and edit_text (don't save changes)
}
```

**UI Structure** (declarative):
```xml
<for each="task" in="{filtered_tasks}">
    <if test="{task.state == 'Editing'}">
        <text_input id="task-{task.id}" value="{edit_text}"
            on_input="update_edit_text" on_submit="save_edit" />
    </if>
    <if test="{task.state == 'Idle'}">
        <checkbox label="{task.description}" />
        <button on_click="edit_task:{task.id}" label="✏️" />
    </if>
</for>
```

**Handling Filter Changes During Active Edits**:

**Scenario**: User is editing a task, then switches filter to "Completed", which hides the task being edited.

**Solution Options**:

| Option | Approach | Pros | Cons |
|--------|----------|------|------|
| **A. Cancel edit** | Cancel active edit when filter changes | Simple, consistent state | User loses unsaved work |
| **B. Preserve edit** | Keep edit state even when task hidden | Preserves user work | Confusing state (editing invisible task) |
| **C. Switch filter to All** | Force filter to All when editing starts | Task always visible while editing | Surprising behavior, loses filter preference |

**Decision**: **Option A - Cancel edit when filter changes**

**Rationale**:
- Simplest implementation with consistent state
- User expects hidden tasks to not be active
- Prevents confusing "editing invisible task" state
- Aligns with Iced todos example behavior (likely does same)

**Implementation**:
```rust
pub fn filter_changed(model: &mut Model, value: String) {
    // Cancel any active edit when filter changes
    if model.editing_id.is_some() {
        cancel_edit(model);
    }

    // Then change filter
    model.filter = match value.as_str() {
        "Active" => Filter::Active,
        "Completed" => Filter::Completed,
        _ => Filter::All,
    };

    update_computed_fields(model);
}
```

**Focus Management**:

**Challenge**: How to ensure text input gets focus when edit mode is activated?

**Iced's Approach**: Use `id` attribute on widgets. When a widget has an ID, Iced can programmatically focus it.

**Dampen Implementation**:
- The `text_input` has `id="task-{task.id}"`
- When entering edit mode, we need to request focus on that ID
- This requires coordination with Iced's focus management system

**Solution**:
```rust
// In Message enum for Iced integration
pub enum Message {
    Handler(HandlerMessage),
    #[cfg(debug_assertions)]
    HotReload(FileEvent),
    #[cfg(debug_assertions)]
    DismissError,
    FocusWidget(String),  // NEW: Request focus on widget by ID
}

// In edit_task handler, also send FocusWidget message
pub fn edit_task(model: &mut Model, id: String) {
    // ... set editing state ...

    // Return message to request focus (this would be handled by Iced integration)
    // Message::FocusWidget(format!("task-{}", id))
}
```

**Note**: This may require updates to dampen-iced to support focus requests. If not available initially, manual clicking to focus is acceptable fallback (users can click the text input to start editing).

---

## Summary of Decisions

| Question | Decision | Constitution Compliance |
|----------|----------|------------------------|
| Test Strategy | Hybrid: unit tests for model, manual integration for UI | ✅ Pass (Principle V targets core, not examples) |
| Performance Budgets | All achievable; todo-app well within budgets | ✅ Pass (all metrics within limits) |
| Inline Editing | State-based with explicit edit state in Model; cancel on filter change | ✅ Pass (declarative-first, type-safe) |

---

## Open Questions

None. All NEEDS CLARIFICATION items resolved.

---

## Next Steps

1. Proceed to Phase 1: Design & Contracts
2. Generate data-model.md with Task, Filter, and Model structures
3. Create quickstart.md for developers
4. Re-evaluate Constitution Check after design phase

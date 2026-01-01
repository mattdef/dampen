# Research: Gravity Framework Technical Decisions

**Feature**: 001-framework-technical-specs  
**Date**: 2025-12-30

## Overview

This document captures research findings and technology decisions for the Gravity declarative UI framework. Each decision includes rationale and alternatives considered.

---

## 1. XML Parser Library

### Decision: `roxmltree`

### Rationale
- Zero-copy parsing for performance
- Maintains DOM tree with full position information (line/column)
- No external dependencies (pure Rust)
- Well-maintained, used by major projects (resvg)
- Read-only tree fits our use case (parse once, generate IR)

### Alternatives Considered

| Library | Pros | Cons | Rejected Because |
|---------|------|------|------------------|
| `quick-xml` | Streaming, low memory | No DOM, requires manual tree building | More implementation work |
| `xml-rs` | SAX-style, mature | Slow, allocates heavily | Performance concerns |
| `xmltree` | Simple API | Less maintained, missing features | Position info incomplete |
| `serde-xml-rs` | serde integration | Doesn't preserve structure for IR | Loses attribute order, positions |

### Implementation Notes
- Use `roxmltree::Document::parse()` for initial parse
- Extract `Node::position()` for span information
- Consider caching parsed `Document` in dev mode

---

## 2. Expression Parser Approach

### Decision: Hand-written recursive descent parser

### Rationale
- Expression grammar is simple and well-defined
- Full control over error messages and recovery
- No external dependencies
- Easier to maintain than grammar file
- Position tracking integrates cleanly with XML spans

### Alternatives Considered

| Approach | Pros | Cons | Rejected Because |
|----------|------|------|------------------|
| `nom` | Powerful, composable | Learning curve, macro-heavy | Overkill for simple grammar |
| `pest` | PEG grammar file | Build-time dependency, less control | Error message customization harder |
| `lalrpop` | LR parser generator | Complex setup, grammar file | Too heavyweight |
| Regex-based | Simple | Fragile, poor errors | Not suitable for nested expressions |

### Expression Grammar (Simplified)

```text
expr       := field_access | method_call | conditional | literal
field_access := IDENT ('.' IDENT)*
method_call  := field_access '(' args? ')'
args         := expr (',' expr)*
conditional  := 'if' expr 'then' expr 'else' expr
literal      := STRING | NUMBER | BOOL
```

### Implementation Notes
- Tokenize first, then parse token stream
- Store AST nodes with spans for error reporting
- Limit recursion depth to prevent stack overflow

---

## 3. Code Generation Strategy

### Decision: Hybrid approach - build.rs for XML processing, proc-macro for Model/handler annotations

### Rationale
- **build.rs advantages**:
  - Can read XML files from disk
  - Generates complete Rust source files
  - Clear separation from user code
  - Easier to debug (output visible in `target/`)
  
- **Proc-macro advantages**:
  - `#[derive(UiModel)]` integrates naturally with Rust workflow
  - `#[ui_handler]` marks handlers in-place
  - IDE support for macro expansions

### Alternatives Considered

| Approach | Pros | Cons | Rejected Because |
|----------|------|------|------------------|
| Pure proc-macro | IDE integration | Can't read external files easily | XML file access requires tricks |
| Pure build.rs | Full file access | User must manually include generated code | Ergonomics worse for Model/handlers |
| Runtime compilation | Maximum flexibility | Slow, complex | Defeats purpose of production mode |

### Implementation Notes

**build.rs flow**:
```text
1. Scan for *.gravity files
2. Parse each to IR
3. Generate ui_generated.rs with view/update functions
4. Include via include!(concat!(env!("OUT_DIR"), "/ui_generated.rs"))
```

**Proc-macro flow**:
```text
1. #[derive(UiModel)] generates UiBindable impl
2. #[ui_handler] registers handler in compile-time map
3. Build.rs references these via trait bounds
```

---

## 4. File Watcher Library

### Decision: `notify` crate (version 6.x)

### Rationale
- Cross-platform (Windows, Linux, macOS)
- Active maintenance and wide adoption
- Configurable debouncing built-in
- Async support via `notify-debouncer-full`

### Alternatives Considered

| Library | Pros | Cons | Rejected Because |
|---------|------|------|------------------|
| `hotwatch` | Simple API | Less flexible, smaller community | notify is more robust |
| Raw inotify/FSEvents | Maximum control | Platform-specific code required | Maintenance burden |
| Polling | Works everywhere | Slow, CPU-intensive | Poor UX for hot-reload |

### Implementation Notes
- Use `RecommendedWatcher` for platform-optimal backend
- Debounce with 100ms delay to batch rapid saves
- Watch parent directory, filter for `.gravity` extension
- Handle `Remove` + `Create` as file replacement

---

## 5. State Serialization Format

### Decision: JSON via `serde_json`

### Rationale
- Human-readable for debugging
- Lenient deserialization (skip unknown fields)
- Universal support in Rust ecosystem
- Fast enough for hot-reload use case

### Alternatives Considered

| Format | Pros | Cons | Rejected Because |
|--------|------|------|------------------|
| `bincode` | Fast, compact | Binary, hard to debug | Hot-reload debugging needs visibility |
| `rmp` (MessagePack) | Fast, compact | Binary format | Same as bincode |
| `ron` | Rust-native, readable | Less ecosystem support | JSON more universal |
| `toml` | Very readable | Doesn't handle complex types well | Vec/nested structs awkward |

### Implementation Notes
- Use `#[serde(default)]` on Model fields for forward compatibility
- Store as `.gravity-state.json` in temp directory
- Include schema version for migration detection

---

## 6. Error Overlay Implementation

### Decision: Dedicated Iced widget rendered atop application

### Rationale
- Uses existing Iced infrastructure
- Styled consistently with application
- Can be dismissed or minimized
- No external window management

### Alternatives Considered

| Approach | Pros | Cons | Rejected Because |
|----------|------|------|------------------|
| Separate window | Clear separation | Platform window management | Complexity, focus issues |
| Terminal output only | Simple | Not visible during UI work | Poor UX |
| System notification | Non-intrusive | Limited content, disappears | Not suitable for error details |
| Replace entire UI | Simple implementation | Lose application context | Can't see partial updates |

### Implementation Notes
- Overlay as `Stack` layer over application content
- Red/orange styling for visibility
- Include: error message, source location, suggestion
- "Dismiss" button or auto-dismiss on successful reload

---

## 7. IR Serialization Format

### Decision: JSON with optional binary (bincode) for cache

### Rationale
- JSON for `gravity inspect` output (human-readable)
- Bincode for build cache (performance)
- Separation of concerns: inspection vs. caching

### Implementation Notes
- `serde` derive on all IR types
- Cache key: hash of XML file content
- Cache location: `target/gravity-cache/`
- Invalidate on Gravity version change

---

## 8. Iced Backend Trait Design

### Decision: Trait with associated types for widget output

### Rationale
- Associated types allow backend-specific widget types
- No dynamic dispatch overhead in generated code
- Clean separation of IR from rendering

### Trait Sketch

```rust
pub trait Backend {
    type Widget<'a>;
    type Message: Clone;
    
    fn text(&self, content: &str) -> Self::Widget<'_>;
    fn button(&self, label: &str, on_press: Option<Self::Message>) -> Self::Widget<'_>;
    fn column(&self, children: Vec<Self::Widget<'_>>) -> Self::Widget<'_>;
    // ... other widgets
}
```

### Implementation Notes
- `gravity-iced` implements `Backend` for Iced types
- Core crate defines trait, knows nothing about Iced
- Generic code generation uses trait bounds

---

## 9. Minimum Supported Rust Version (MSRV)

### Decision: Rust 1.75 (Edition 2021 initially, upgrade to 2024 when stable)

### Rationale
- 1.75 is current stable as of late 2024
- Edition 2024 will be Rust 1.82+ (expected early 2025)
- Start with 2021 for broader compatibility, upgrade when Edition 2024 stabilizes
- No nightly features in public API

### Implementation Notes
- Set `rust-version = "1.75"` in Cargo.toml
- CI tests on MSRV and stable
- Document upgrade path for Edition 2024

---

## 10. CLI Framework

### Decision: `clap` (version 4.x) with derive macros

### Rationale
- Industry standard for Rust CLIs
- Derive macros reduce boilerplate
- Shell completion generation
- Good documentation and ecosystem

### Alternatives Considered

| Library | Pros | Cons | Rejected Because |
|---------|------|------|------------------|
| `argh` | Minimal, fast compile | Less features | Completion, help formatting |
| `pico-args` | Zero dependencies | Manual parsing | Too low-level |
| `structopt` | Familiar | Merged into clap 3+ | Use clap directly |

### Implementation Notes
- Use `#[derive(Parser)]` for command structure
- Subcommands for `dev`, `build`, `check`, `inspect`
- `--config` flag for custom config file path

---

## Summary of Key Dependencies

| Purpose | Crate | Version |
|---------|-------|---------|
| XML Parsing | `roxmltree` | 0.19+ |
| File Watching | `notify` | 6.0+ |
| Serialization | `serde`, `serde_json` | 1.0+ |
| Proc Macros | `syn`, `quote`, `proc-macro2` | 2.0+ |
| CLI | `clap` | 4.0+ |
| UI Backend | `iced` | 0.14+ |
| Property Testing | `proptest` | 1.0+ |
| Snapshot Testing | `insta` | 1.0+ |

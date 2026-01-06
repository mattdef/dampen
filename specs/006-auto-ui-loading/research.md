# Research: Automatic UI File Loading with AppState Structure

**Feature**: 006-auto-ui-loading
**Date**: 2026-01-06

## Research Questions

### Q1: Auto-Loading Mechanism (Proc Macro vs Build.rs)

**Decision**: Hybrid approach - build.rs for discovery + procedural macro for code generation

**Rationale**:
1. **Transparency for common cases**: When `app.gravity.rs` exists, automatically load `app.gravity`
2. **Explicit override capability**: `#[gravity_ui]` attribute for special cases or manual control
3. **Build caching**: Properly integrates with Cargo's incremental compilation
4. **IDE support**: Proc macro provides better IDE integration for errors and completions

**Implementation approach**:
- Build.rs discovers `.gravity` files and generates file lists with `cargo:rerun-if-changed`
- Procedural macro `#[gravity_ui]` parses XML and generates widget code
- Auto-loading convention: `<filename>.gravity.rs` automatically loads `<filename>.gravity`

**Alternatives considered**:
- Pure build.rs: Less explicit, magic behavior can confuse users
- Pure procedural macro: Requires user to add attributes, more verbose for common cases
- `include_str!` pattern: Rejected as too manual (the problem we're solving)

---

### Q2: AppState Default Behavior (Optional Fields)

**Decision**: Generic struct with `PhantomData<M>` and named constructors

**Rationale**:
1. **Idiomatic**: Uses Default trait which is core Rust pattern
2. **Type-safe generics**: `PhantomData<M>` ensures model has correct type without runtime overhead
3. **Zero-cost**: No Option wrapper overhead in the hot path
4. **Discoverable**: Named constructors make intent clear

**API Design**:
```rust
pub struct AppState<M: UiBindable = ()> {
    pub document: GravityDocument,  // Required
    pub model: M,                    // Generic, defaults to ()
    pub handler_registry: HandlerRegistry, // Defaults to empty
    _marker: PhantomData<M>,
}

impl<M: UiBindable> AppState<M> {
    pub fn new(document: GravityDocument) -> Self
    where
        M: Default,
    {
        Self {
            document,
            model: M::default(),
            handler_registry: HandlerRegistry::default(),
            _marker: PhantomData,
        }
    }

    pub fn with_model(document: GravityDocument, model: M) -> Self {
        Self {
            document,
            model,
            handler_registry: HandlerRegistry::default(),
            _marker: PhantomData,
        }
    }

    pub fn with_handlers(document: GravityDocument, handler_registry: HandlerRegistry) -> Self
    where
        M: Default,
    {
        Self {
            document,
            model: M::default(),
            handler_registry,
            _marker: PhantomData,
        }
    }
}
```

**Usage patterns**:
```rust
// Basic (document only)
let state = AppState::new(document);

// With model only
let state = AppState::with_model(document, my_model);

// With handlers only
let state = AppState::with_handlers(document, handlers);

// Full control
let state = AppState {
    document,
    model: my_model,
    handler_registry: handlers,
    _marker: PhantomData,
};
```

**Alternatives considered**:
- Builder pattern: More boilerplate, overkill for simple struct
- Option<T> wrapper: Runtime overhead, less idiomatic
- Wrapper structs (SimpleAppState/FullAppState): Two types to maintain

---

### Q3: Error Handling Strategy

**Decision**: `syn::Error::to_compile_error()` with `proc-macro-error2` for multi-part diagnostics

**Rationale**:
1. **Integrates with existing pattern**: Extends Gravity's `ParseError`/`BindingError`
2. **Span information**: Provides line/column for easy debugging
3. **Actionable messages**: Include help and suggestions
4. **Graceful degradation**: `gravity check` for compile-time, overlay for runtime

**Implementation**:
```rust
// Extend existing ParseError with compile-time support
impl ParseError {
    pub fn to_compile_error(&self) -> proc_macro2::TokenStream {
        let msg = format!(
            "Gravity parsing error: {}\n  at {}:{}:{}",
            self.message,
            self.span.source.map_or("<unknown>", |s| s),
            self.span.line,
            self.span.column
        );
        quote::quote! { compile_error!(#msg); }
    }
}

// Macro error handling
use proc_macro_error2::*;

#[proc_macro]
pub fn gravity_ui(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as LitStr);
    let path = input.value();
    
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let abs_path = Path::new(&manifest_dir).join(&path);
    
    if !abs_path.exists() {
        emit_error!(
            input,
            "Gravity UI file not found: '{}'\n  = help: Check the path is correct",
            path
        );
    }
    
    // Continue with file processing...
}
```

**Error categories and handling**:

| Error Type | Compile-Time | Runtime |
|------------|--------------|---------|
| Missing file | Hard error (macro) | `gravity dev` overlay |
| Invalid XML | Hard error (macro) | Red error display |
| Unknown handler | `gravity check` error | Runtime click toast |

**Alternatives considered**:
- Only runtime errors: Rejected - violates type safety principle
- Only compile-time errors: Rejected - no graceful degradation
- Custom error types: Overkill - existing `ParseError` works well

---

## Technology Decisions Summary

| Decision | Choice | Rationale |
|----------|--------|-----------|
| Auto-loading mechanism | Hybrid (build.rs + proc macro) | Balances transparency and explicit control |
| AppState API | Generic struct + constructors | Idiomatic Rust, zero-cost, discoverable |
| Error handling | syn::Error + proc-macro-error2 | Integrates with existing pattern |
| File convention | `<filename>.gravity.rs` → `<filename>.gravity` | Simple, predictable, matches Rust conventions |

## Code References

- Existing `ParseError`: `gravity-core/src/parser/error.rs`
- Existing `HandlerRegistry`: `gravity-core/src/handler/mod.rs`
- Existing `UiBindable`: `gravity-core/src/binding/mod.rs`
- Existing proc macros: `gravity-macros/src/lib.rs`

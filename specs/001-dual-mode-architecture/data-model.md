# Data Model: Dual-Mode Architecture

**Feature**: 001-dual-mode-architecture  
**Date**: 2026-01-09

## Overview

This document defines the key data structures, state models, and configuration entities required for the dual-mode architecture feature. All models are designed to support both interpreted (development) and codegen (production) modes.

---

## 1. Core State Models

### 1.1 AppState (Existing - Extended)

**Location**: `dampen-core/src/state/mod.rs`

**Purpose**: Root application state container with model, UI document, and event handlers

**Structure**:
```rust
pub struct AppState<M: UiBindable> {
    pub document: DampenDocument,       // UI structure from XML
    pub model: M,                        // Application data model
    pub handler_registry: HandlerRegistry, // Event handler functions
    _marker: PhantomData<M>,
}
```

**New Methods** (to be added):
```rust
impl<M: UiBindable> AppState<M> {
    /// Hot-reload: update document while preserving model and handlers
    pub fn hot_reload(&mut self, new_document: DampenDocument) {
        self.document = new_document;
    }
    
    /// Create AppState with all components
    pub fn with_handlers(
        document: DampenDocument,
        handler_registry: HandlerRegistry,
    ) -> Self {
        Self {
            document,
            model: M::default(),
            handler_registry,
            _marker: PhantomData,
        }
    }
}
```

**Relationships**:
- Contains `DampenDocument` (1:1)
- Contains `M: UiBindable` (1:1)
- Contains `HandlerRegistry` (1:1)

---

## 2. Development Mode Models

### 2.1 HotReloadContext

**Location**: `dampen-dev/src/reload.rs` (NEW)

**Purpose**: Tracks hot-reload state and history for debugging

**Structure**:
```rust
pub struct HotReloadContext<M> {
    /// Last successful model snapshot (JSON)
    last_model_snapshot: Option<String>,
    
    /// Timestamp of last reload
    last_reload_timestamp: Instant,
    
    /// Reload count (for metrics)
    reload_count: usize,
    
    /// Current error state (if any)
    error: Option<ReloadError>,
    
    _marker: PhantomData<M>,
}
```

**State Transitions**:
```
Initial → FileChanged → Reloading → Success → Ready
                            ↓
                          Failed → ErrorShown → (retry)
```

**Methods**:
```rust
impl<M: Serialize + DeserializeOwned> HotReloadContext<M> {
    pub fn new() -> Self;
    pub fn snapshot_model(&mut self, model: &M) -> Result<()>;
    pub fn restore_model(&self) -> Result<M>;
    pub fn record_reload(&mut self, success: bool);
    pub fn last_reload_latency(&self) -> Duration;
}
```

### 2.2 FileEvent

**Location**: `dampen-dev/src/subscription.rs` (NEW)

**Purpose**: Domain event for file watcher subscription output

**Structure**:
```rust
pub enum FileEvent {
    /// File changed and parsed successfully
    Success {
        path: PathBuf,
        document: DampenDocument,
    },
    
    /// Parse error (XML syntax or validation)
    ParseError {
        path: PathBuf,
        error: ParseError,
        content: String,  // For error overlay display
    },
    
    /// File watcher error (permissions, deleted file, etc.)
    WatcherError {
        path: PathBuf,
        error: String,
    },
}
```

**Lifecycle**:
```
notify::Event → Filter(.dampen) → Parse XML → FileEvent
```

### 2.3 ErrorOverlay

**Location**: `dampen-dev/src/overlay.rs` (NEW)

**Purpose**: UI state for displaying parse errors during hot-reload

**Structure**:
```rust
pub struct ErrorOverlay {
    /// Parse error details
    pub error: Option<ParseError>,
    
    /// Whether overlay is visible
    pub visible: bool,
    
    /// Timestamp when error occurred
    pub timestamp: Instant,
}
```

**Fields**:
- `error.span.line`: Line number in XML
- `error.span.column`: Column number
- `error.kind`: Error type (syntax, validation, etc.)
- `error.message`: Human-readable error message

**UI Rendering**:
- Red background overlay
- Shows file path, line/column
- Displays error message with context
- "Dismiss" button (keeps old UI visible)

---

## 3. Production Mode Models

### 3.1 CodegenConfig

**Location**: `dampen-core/src/codegen/config.rs` (NEW)

**Purpose**: Configuration for code generation behavior

**Structure**:
```rust
pub struct CodegenConfig {
    /// Output directory for generated code
    pub output_dir: PathBuf,
    
    /// Whether to format generated code with prettyplease
    pub format_output: bool,
    
    /// Whether to validate generated code syntax
    pub validate_syntax: bool,
    
    /// Model type name
    pub model_type: String,
    
    /// Message enum name
    pub message_type: String,
}
```

**Validation Rules**:
- `output_dir` must be writable
- `model_type` must be valid Rust identifier
- `message_type` must be valid Rust identifier

### 3.2 GeneratedCode

**Location**: `dampen-core/src/codegen/mod.rs` (EXTEND)

**Purpose**: Container for generated Rust code with metadata

**Structure**:
```rust
pub struct GeneratedCode {
    /// Generated Rust source code
    pub code: String,
    
    /// Module path (e.g., "ui_window")
    pub module_name: String,
    
    /// Source .dampen file path
    pub source_file: PathBuf,
    
    /// Generated at timestamp
    pub timestamp: SystemTime,
    
    /// Validation status
    pub validated: bool,
}
```

**Methods**:
```rust
impl GeneratedCode {
    /// Validate syntax by parsing with syn
    pub fn validate(&mut self) -> Result<()>;
    
    /// Format code with prettyplease
    pub fn format(&mut self) -> Result<()>;
    
    /// Write to output directory
    pub fn write_to_file(&self, path: &Path) -> Result<()>;
}
```

---

## 4. File Watching Models

### 4.1 FileWatcherConfig

**Location**: `dampen-dev/src/watcher.rs` (NEW)

**Purpose**: Configuration for file watcher behavior

**Structure**:
```rust
pub struct FileWatcherConfig {
    /// Paths to watch (directories or specific files)
    pub watch_paths: Vec<PathBuf>,
    
    /// Debounce interval in milliseconds
    pub debounce_ms: u64,
    
    /// File extension filter (default: ".dampen")
    pub extension_filter: String,
    
    /// Whether to watch recursively
    pub recursive: bool,
}
```

**Default Values**:
```rust
impl Default for FileWatcherConfig {
    fn default() -> Self {
        Self {
            watch_paths: vec![PathBuf::from("src/ui")],
            debounce_ms: 100,
            extension_filter: ".dampen".to_string(),
            recursive: true,
        }
    }
}
```

### 4.2 FileWatcherState

**Location**: `dampen-dev/src/watcher.rs` (NEW)

**Purpose**: Runtime state of file watcher

**Structure**:
```rust
pub enum FileWatcherState {
    /// Watcher is initialized but not started
    Idle,
    
    /// Actively watching for changes
    Watching { paths: Vec<PathBuf> },
    
    /// Error state (watcher failed to initialize)
    Failed { error: String },
}
```

**State Transitions**:
```
Idle → init() → Watching
Idle → init() → Failed (if permissions error, etc.)
Watching → stop() → Idle
Watching → (error) → Failed
Failed → retry() → Watching (if error resolved)
```

---

## 5. Handler Models (Existing - Extended)

### 5.1 HandlerRegistry (Existing)

**Location**: `dampen-core/src/handler/mod.rs`

**Current Structure**:
```rust
pub struct HandlerRegistry {
    simple: HashMap<String, Arc<dyn Fn(&mut dyn Any) + Send + Sync>>,
    with_value: HashMap<String, Arc<dyn Fn(&mut dyn Any, Box<dyn Any>) + Send + Sync>>,
    with_command: HashMap<String, Arc<dyn Fn(&mut dyn Any) -> Box<dyn Any> + Send + Sync>>,
}
```

**Usage in Hot-Reload**:
- Completely replaced on each reload
- User's `create_handler_registry()` function called
- No state preservation needed (handlers are code, not data)

### 5.2 HandlerMetadata (NEW)

**Location**: `dampen-core/src/handler/mod.rs`

**Purpose**: Metadata for handler validation and debugging

**Structure**:
```rust
pub struct HandlerMetadata {
    /// Handler name (matches XML attribute value)
    pub name: String,
    
    /// Signature type
    pub signature_type: HandlerSignatureType,
    
    /// Source file where handler is defined
    pub source_file: Option<PathBuf>,
    
    /// Source line number
    pub source_line: Option<usize>,
}

pub enum HandlerSignatureType {
    Simple,              // fn(&mut Model)
    WithValue(TypeId),   // fn(&mut Model, T)
    WithCommand,         // fn(&mut Model) -> Command
}
```

**Usage**:
- Validation during hot-reload
- Error messages ("Handler 'foo' expects no parameters, but XML passes value")
- Developer tooling (jump to handler definition)

---

## 6. Validation and Error Models

### 6.1 ReloadResult

**Location**: `dampen-dev/src/reload.rs` (NEW)

**Purpose**: Result type for hot-reload attempts with detailed error information

**Structure**:
```rust
pub enum ReloadResult<M> {
    /// Reload succeeded
    Success(AppState<M>),
    
    /// XML parse error (reject reload)
    ParseError(ParseError),
    
    /// Schema validation error (reject reload)
    ValidationError(Vec<String>),
    
    /// Model deserialization failed, using default (accept reload with warning)
    StateRestoreWarning(AppState<M>, String),
}
```

**Error Handling Matrix**:
| Result Variant | UI Action | State Preservation |
|----------------|-----------|-------------------|
| `Success` | Apply reload | Model restored |
| `ParseError` | Show error overlay | Keep old state completely |
| `ValidationError` | Show error overlay | Keep old state completely |
| `StateRestoreWarning` | Apply reload + show warning | Use `M::default()` |

### 6.2 ParseError (Existing - Used)

**Location**: `dampen-core/src/parser/error.rs`

**Current Structure**:
```rust
pub struct ParseError {
    pub kind: ParseErrorKind,
    pub span: Span,
    pub message: String,
}

pub struct Span {
    pub line: usize,
    pub column: usize,
    pub offset: usize,
}

pub enum ParseErrorKind {
    XmlSyntax,
    UnknownWidget,
    InvalidAttribute,
    MissingRequiredAttribute,
    // ...
}
```

**Usage in Hot-Reload**:
- Displayed in error overlay
- Includes file location for quick navigation
- Actionable error messages

---

## 7. Configuration Models

### 7.1 DevelopmentModeConfig

**Location**: `dampen-dev/src/config.rs` (NEW)

**Purpose**: Global configuration for development mode features

**Structure**:
```rust
pub struct DevelopmentModeConfig {
    /// Enable hot-reload
    pub hot_reload_enabled: bool,
    
    /// File watcher configuration
    pub watcher: FileWatcherConfig,
    
    /// Whether to persist state to .dampen-state.json
    pub persist_state: bool,
    
    /// State file path
    pub state_file: PathBuf,
    
    /// Error overlay styling
    pub error_overlay_style: ErrorOverlayStyle,
}
```

**Loading**:
```rust
impl DevelopmentModeConfig {
    /// Load from .dampen-dev.toml if exists, otherwise use defaults
    pub fn load_or_default() -> Self;
    
    /// Validate configuration
    pub fn validate(&self) -> Result<()>;
}
```

### 7.2 ErrorOverlayStyle

**Location**: `dampen-dev/src/config.rs` (NEW)

**Purpose**: Customizable styling for error overlay

**Structure**:
```rust
pub struct ErrorOverlayStyle {
    pub background_color: Color,
    pub text_color: Color,
    pub accent_color: Color,
    pub font_size: u16,
    pub padding: u16,
}
```

**Default**:
- Red background (semi-transparent)
- White text
- Monospace font for code snippets

---

## 8. Data Flow Diagrams

### 8.1 Hot-Reload State Flow

```
User Edits .dampen File
         ↓
File System Event (notify)
         ↓
FileWatcherState: Watching
         ↓
Debounce (100ms)
         ↓
HotReloadContext::snapshot_model()
         ↓
Parse XML → DampenDocument
         ↓
   [Valid?] ━━ No ━━> ReloadResult::ParseError
         ↓ Yes                    ↓
Rebuild HandlerRegistry    ErrorOverlay::visible = true
         ↓                        ↓
HotReloadContext::restore_model()  Keep Old AppState
         ↓
   [Restore OK?] ━━ No ━━> ReloadResult::StateRestoreWarning
         ↓ Yes                     ↓
ReloadResult::Success        Use M::default()
         ↓                         ↓
Update AppState.document    Show Warning Banner
         ↓
FileWatcherState: Watching (ready for next change)
```

### 8.2 Codegen Pipeline Flow

```
build.rs Invocation
         ↓
Scan src/ui/ for .dampen files
         ↓
For each file:
    ↓
Parse XML → DampenDocument
    ↓
CodegenConfig loaded
    ↓
generate_application()
    ↓
GeneratedCode struct created
    ↓
GeneratedCode::validate()
    ↓
GeneratedCode::format() (prettyplease)
    ↓
GeneratedCode::write_to_file(OUT_DIR)
    ↓
cargo:rerun-if-changed emitted
    ↓
Build continues with generated code
```

---

## 9. Serialization Formats

### 9.1 Model State Snapshot (JSON)

**Format**: JSON via `serde_json`

**Example**:
```json
{
  "count": 42,
  "name": "Alice",
  "items": [
    { "id": 1, "title": "Buy milk", "done": false },
    { "id": 2, "title": "Write code", "done": true }
  ],
  "status": "Active"
}
```

**Properties**:
- Human-readable
- Version-tolerant (graceful degradation)
- Fast serialization/deserialization (<5ms)

### 9.2 Handler Manifest (TOML) - Optional

**Format**: TOML (future feature for build.rs handler discovery)

**Example**:
```toml
[increment]
params = []
command = false

[set_value]
params = ["String"]
command = false

[save]
params = []
command = true
```

**Usage**:
- Build-time handler signature discovery
- Alternative to parsing Rust source files

---

## 10. Validation Rules

### 10.1 AppState Constraints

- `model` must implement `UiBindable` (enforce `Serialize + Deserialize`)
- `document` must be valid (all widget names recognized)
- `handler_registry` must contain all handlers referenced in XML

### 10.2 HotReloadContext Constraints

- `last_model_snapshot` must be valid JSON
- `last_reload_timestamp` must not be in the future
- `error` must be cleared on successful reload

### 10.3 FileWatcherConfig Constraints

- `watch_paths` must exist and be readable
- `debounce_ms` must be in range [10, 5000]
- `extension_filter` must start with `.`

### 10.4 CodegenConfig Constraints

- `output_dir` must be writable
- `model_type` must match Rust identifier rules (`[A-Z][A-Za-z0-9_]*`)
- `message_type` must match Rust identifier rules

---

## 11. Performance Characteristics

| Model | Size | Serialization Time | Notes |
|-------|------|-------------------|-------|
| AppState<M> | Varies | N/A (not serialized) | Only model component serialized |
| Model (typical) | <1KB | <5ms | JSON serialization |
| DampenDocument | 1-10KB | <10ms | XML parsing |
| HandlerRegistry | <100 bytes | <1ms | Metadata only |
| FileEvent | <10KB | <1ms | Small enum |
| GeneratedCode | 10-100KB | <50ms | Code formatting overhead |

---

## 12. Entity Relationships

```
AppState<M>
    ├──> DampenDocument (1:1)
    ├──> M: UiBindable (1:1)
    └──> HandlerRegistry (1:1)
         └──> Vec<HandlerMetadata> (1:N)

HotReloadContext<M>
    ├──> Option<String> (model snapshot) (1:0..1)
    └──> Option<ReloadError> (1:0..1)

FileWatcher
    ├──> FileWatcherConfig (1:1)
    ├──> FileWatcherState (1:1)
    └──> Receiver<FileEvent> (1:1)

ErrorOverlay
    └──> Option<ParseError> (1:0..1)

GeneratedCode
    └──> CodegenConfig (N:1)
```

---

**Document Status**: ✅ Complete  
**All Models Defined**: Yes  
**Ready for Contract Generation**: Yes

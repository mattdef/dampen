# Data Model: Fix Code Quality Issues in dampen-dev

**Branch**: `001-fix-code-quality-issues`
**Date**: January 21, 2026
**Phase**: 1 - Design & Contracts

## Overview

This document describes the data structures affected by the code quality fixes. All entities are existing types being modified or enhanced, not new types being created.

---

## Entity: HotReloadContext

**Purpose**: Manages cache state and performance metrics for hot-reload operations

**File**: `crates/dampen-dev/src/reload.rs`

### Fields

| Field | Type | Description | Visibility |
|-------|------|-------------|-------------|
| `parse_cache` | `HashMap<u64, CacheEntry>` | Cache of parsed documents keyed by content hash | private |
| `max_cache_entries` | `usize` | Maximum number of entries before eviction | private |
| `cache_hits` | `AtomicUsize` | Count of successful cache hits | private |
| `cache_misses` | `AtomicUsize` | Count of cache misses | private |
| `last_model_snapshot` | `Option<String>` | Serialized model state for restoration | private |
| `_marker` | `PhantomData<M>` | Type marker for generic M | private |

### Methods (Modified)

| Method | Signature | Changes |
|---------|------------|----------|
| `new()` | `pub fn new() -> Self` | Initialize `cache_hits` and `cache_misses` to `AtomicUsize::new(0)` |
| `get_cached_document()` | `fn get_cached_document(&self, xml_source: &str) -> Option<Document>` | Increment `cache_hits` or `cache_misses` using `fetch_add(Ordering::Relaxed)` |
| `calculate_cache_hit_rate()` | `fn calculate_cache_hit_rate(&self) -> f64` | Implement actual calculation: `hits / (hits + misses)` with zero-division handling |

### Methods (New)

| Method | Signature | Purpose |
|---------|------------|---------|
| `compute_content_hash()` | `fn compute_content_hash(xml_source: &str) -> u64` | Helper to compute hash once, called by both get and cache operations |
| `get_or_cache_document()` | `fn get_or_cache_document<F>(&mut self, xml_source: &str, f: F) -> Document where F: FnOnce() -> Document` | Entry API pattern for single-lookup cache operations |

### State Transitions

The `HotReloadContext` has no formal state machine. It's a container with:

1. **Initialization**: Created with empty cache, zero counters
2. **Normal operation**: Cache hits/misses tracked incrementally
3. **Cache eviction**: When `parse_cache.len() >= max_cache_entries`, oldest entry removed
4. **No terminal states**: Context remains valid throughout application lifetime

### Validation Rules

1. `max_cache_entries` must be > 0 (enforced in constructor)
2. `cache_hit_rate()` must handle zero-division (return 0.0 if hits + misses = 0)
3. Cache keys are content hashes; collisions possible but unlikely (handled by HashMap)
4. All counter operations use `Ordering::Relaxed` for performance

### Relationships

- **Owned by**: `AppState<M>` (via composition)
- **Uses**: `CacheEntry` (internal struct), `Document` (from dampen-core)
- **Consumed by**: `attempt_hot_reload()`, `attempt_hot_reload_async()`

---

## Entity: ReloadPerformanceMetrics

**Purpose**: Exposes cache hit rate and other performance statistics to consumers

**File**: `crates/dampen-dev/src/reload.rs`

### Fields

| Field | Type | Description | Visibility |
|-------|------|-------------|-------------|
| `reload_count` | `usize` | Total number of reload attempts | public |
| `successful_reloads` | `usize` | Number of successful reloads | public |
| `failed_reloads` | `usize` | Number of failed reloads | public |
| `avg_reload_time_ms` | `f64` | Average reload time in milliseconds | public |

### Methods (Enhanced)

| Method | Signature | Changes |
|---------|------------|----------|
| `cache_hit_rate()` | `pub fn cache_hit_rate(&self) -> f64` | Delegate to `HotReloadContext::calculate_cache_hit_rate()` |

### Validation Rules

1. `avg_reload_time_ms` must be calculated only when `reload_count > 0`
2. `successful_reloads + failed_reloads` should equal `reload_count` (invariant)

### Relationships

- **Owned by**: `HotReloadContext` (via composition)
- **Computed from**: `HotReloadContext` counters and timers

---

## Entity: FileWatcherState

**Purpose**: Represents the runtime state of the file watcher

**File**: `crates/dampen-dev/src/watcher.rs`

### Variants

| Variant | Fields | Description | When Active |
|---------|---------|-------------|-------------|
| `Idle` | None | Watcher initialized but not started | After `FileWatcher::new()`, before `watch()` |
| `Watching` | `paths: Vec<PathBuf>` | Actively monitoring paths for changes | After successful `watch()` call |
| `Failed` | `error: String` | Error state if initialization fails | When watcher setup fails (e.g., permission denied, too many file descriptors) |

### State Machine

```text
    ┌─────┐
    │ Idle │
    └──┬──┘
       │ watch() succeeds
       ▼
   ┌─────────┐
   │ Watching │
   └────┬────┘
        │ fatal error
        ▼
   ┌─────────┐
   │ Failed  │────┐
   └─────────┘    │
        │          │ recover (unwatch + re-watch)
        │          └─────────────────────┐
        ▼                                ▼
   (exit application)                ┌─────┐
                                   │ Idle │
                                   └─────┘
```

### Transitions

| From | To | Trigger |
|------|-----|---------|
| `Idle` | `Watching` | `watch()` call succeeds |
| `Idle` | `Failed` | `watch()` call fails (permission denied, OS limit) |
| `Watching` | `Failed` | Fatal error during operation (filesystem unmounted, watcher killed) |
| `Failed` | `Idle` | Recovery (create new `FileWatcher` instance) |

### Validation Rules

1. State changes only via public methods (`watch()`, `unwatch()`)
2. `Failed` state contains error message explaining the failure
3. Recovery from `Failed` requires creating new instance (no direct transition to `Watching`)

### Relationships

- **Owned by**: `FileWatcher`
- **Exposed via**: `FileWatcher::state()` method

---

## Entity: FileWatcherError

**Purpose**: Enum of possible errors from file watching operations

**File**: `crates/dampen-dev/src/watcher.rs`

### Variants (After Fix)

| Variant | Field | Description | Usage |
|---------|-------|-------------|--------|
| `PermissionDenied` | `path: PathBuf` | Insufficient permissions to watch path | When `watch()` fails due to permissions |
| `WatchLimitExceeded` | None | OS file descriptor limit reached | When too many files watched |
| `InvalidPath` | `path: PathBuf` | Path doesn't exist or is invalid | When path validation fails |
| `IoError` | `error: std::io::Error` | Generic I/O error | For other filesystem errors |

### Variants Removed

| Variant | Reason for Removal |
|---------|-------------------|
| `FileDeleted` | Never returned by code; file deletion handled silently (normal operation in development) |

### Validation Rules

1. All variants must be actively used by implementation
2. Error messages must be actionable (include path or context)
3. Use `thiserror::Error` derive for error handling

### Relationships

- **Returned by**: `FileWatcher::watch()`, `handle_debounced_events()`
- **Consumed by**: Application error handling code

---

## Entity: CacheEntry

**Purpose**: Internal cache entry storing parsed document and metadata

**File**: `crates/dampen-dev/src/reload.rs`

### Fields

| Field | Type | Description | Visibility |
|-------|------|-------------|-------------|
| `document` | `Document` | Parsed document from dampen-core | private |
| `timestamp` | `Instant` | When entry was added to cache | private |

### Validation Rules

1. `timestamp` used for LRU eviction (evict oldest when cache full)
2. `document` is cloned when retrieved (cache holds reference, not ownership)

### Relationships

- **Stored in**: `HotReloadContext::parse_cache` (keyed by hash)
- **References**: `Document` from dampen-core

---

## Relationships Summary

```
AppState<M>
    ├── HotReloadContext<M>
    │   ├── HashMap<u64, CacheEntry>  (cache storage)
    │   ├── AtomicUsize cache_hits
    │   ├── AtomicUsize cache_misses
    │   └── ReloadPerformanceMetrics (computed from counters)
    └── M (user model)

FileWatcher
    ├── FileWatcherState (runtime state)
    └── FileWatcherError (error type)
        ├── PermissionDenied
        ├── WatchLimitExceeded
        ├── InvalidPath
        └── IoError
        └─> FileDeleted (REMOVED - never used)
```

---

## Data Flow

### Cache Lookup Flow

```
1. User triggers hot-reload
   ↓
2. attempt_hot_reload() called with XML source
   ↓
3. context.get_cached_document(xml_source)
   ├─ compute_content_hash(xml_source)
   ├─ hash lookup in parse_cache
   ├─ IF found:
   │   ├─ cache_hits.fetch_add(1, Relaxed)
   │   └─ return Some(document)
   └─ IF not found:
       ├─ cache_misses.fetch_add(1, Relaxed)
       └─ return None
```

### Cache Insert Flow

```
1. Cache miss occurred
   ↓
2. Parse XML → document
   ↓
3. context.cache_document(xml_source, document)
   ├─ compute_content_hash(xml_source)
   ├─ IF cache full:
   │   └─ evict_oldest_entry()
   └─ Insert CacheEntry { document, timestamp }
```

### Performance Metrics Flow

```
1. User requests metrics
   ↓
2. context.get_performance_metrics()
   ├─ ReloadPerformanceMetrics {
   │   reload_count,
   │   successful_reloads,
   │   failed_reloads,
   │   avg_reload_time_ms,
   │   cache_hit_rate: calculate_cache_hit_rate()
   │       ├─ hits = cache_hits.load(Relaxed)
   │       ├─ misses = cache_misses.load(Relaxed)
   │       ├─ total = hits + misses
   │       └─ IF total == 0: return 0.0
   │           ELSE: return hits / total
   │   }
   └─ Return metrics
```

---

## Invariants

1. **HotReloadContext**:
   - `cache_hits + cache_misses = total cache lookups performed`
   - `parse_cache.len() <= max_cache_entries`
   - `calculate_cache_hit_rate()` always returns value in [0.0, 1.0]

2. **FileWatcherState**:
   - Only one state active at a time
   - State transitions occur only via public API methods
   - `Failed` state contains non-empty error message

3. **FileWatcherError**:
   - All variants are actively used by implementation
   - Error messages are actionable (include paths or context)

---

## Thread Safety

### HotReloadContext

- **Atomic counters**: `cache_hits`, `cache_misses` are `AtomicUsize` - thread-safe
- **Cache access**: `parse_cache` requires `&mut self` for mutations - single-threaded only
- **Use in async**: Safe for read-only operations across async tasks

### FileWatcherState

- **Read-only**: State queried via `&self` - thread-safe for reads
- **Mutations**: Only via internal methods - not designed for concurrent access
- **Async context**: Typically owned by single async task, not shared

---

## Notes

1. All entities are existing types being modified, not new entities
2. Changes are backward compatible except `FileDeleted` removal (was never used)
3. No new API surface changes except removing unused error variant
4. All modifications preserve existing behavior except for fixes and enhancements

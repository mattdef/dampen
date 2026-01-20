# Migration Guide: CLI Command Changes

## Breaking Changes in v0.X.X

The `dampen` CLI commands have been updated to provide a more intuitive workflow
for application development. This guide helps you migrate existing projects.

---

## Summary of Changes

| Command | Old Behavior | New Behavior |
|----------|--------------|--------------|
| `dampen run` | Interpreted (debug) | Interpreted (debug) - **No change** |
| `dampen run --release` | Interpreted (release) | **Codegen (release)** - **Changed** |
| `dampen build` | Codegen (debug) | **Interpreted (debug)** - **Changed** |
| `dampen build --release` | - | **Codegen (release)** - **New** |
| `dampen release` | Codegen (release) | Alias for `build --release` - **No functional change** |
| `dampen release --interpreted` | Interpreted (release) | **Removed** |

---

## Migration Steps

### If you were using `dampen build` for development:

**Before:**
```bash
dampen build    # Used codegen mode
```

**After:**
```bash
dampen build    # Now uses interpreted mode (recommended for development)
```

**Impact**: No change needed - `dampen build` is now faster for development.

---

### If you were using `dampen run --release`:

**Before:**
```bash
dampen run --release    # Ran interpreted mode with release optimizations
```

**After:**
```bash
dampen run --release    # Now uses codegen mode
```

**Impact**: Your application will now have **zero runtime overhead**. This is usually
beneficial, but ensure your `build.rs` is properly configured.

---

### If you were using `dampen build` for production builds:

**Before:**
```bash
dampen build    # Built with codegen (debug)
```

**After:**
```bash
dampen build --release    # Build with codegen (release)
# or:
dampen release            # Alias for build --release
```

**Impact**: You now need to add `--release` for production builds. This provides
better performance.

---

### If you were using `dampen release --interpreted`:

**Before:**
```bash
dampen release --interpreted    # Built interpreted mode with release optimizations
```

**After:**
```bash
# Option 1: Use interpreted debug build
dampen build

# Option 2: Use codegen release build (recommended for production)
dampen build --release
# or:
dampen release
```

**Impact**: The `--interpreted` flag has been removed. Choose the appropriate
mode based on your needs.

---

## CI/CD Pipeline Updates

If you have CI/CD pipelines that use `dampen build` for production:

**Before:**
```yaml
- name: Build
  run: dampen build
```

**After:**
```yaml
- name: Build
  run: dampen build --release
  # or:
  # run: dampen release
```

---

## New Workflows

### Development Workflow (Recommended)

```bash
# 1. Develop with hot-reload
dampen run

# 2. Validate XML
dampen check

# 3. Run tests
dampen test

# 4. Build for production
dampen build --release
# or:
dampen release
```

### Production Workflow

```bash
# Build optimized binary with codegen
dampen build --release

# Run
./target/release/your-app
```

### Framework Development Workflow

```bash
# Test interpreted mode
cargo run -p example-name

# Test codegen mode
cargo build -p example-name --release --no-default-features --features codegen
./target/release/example-name
```

---

## Frequently Asked Questions

### Q: Why was `dampen build` changed from codegen to interpreted?

**A**: The new workflow aligns with common patterns:
- `run` = execute app
- `build` = compile for development (fast)
- `build --release` = compile for production (optimized)
- `release` = alias for `build --release`

### Q: Do I need to modify my `build.rs`?

**A**: Only if you're using codegen mode. Check that `build.rs` exists if you use
`--release` flag. For interpreted mode (default), `build.rs` is optional.

### Q: How do I know which mode is being used?

**A**: Use the verbose flag:
```bash
dampen build -v    # Shows: "cargo build --features interpreted"
dampen build --release -v    # Shows: "cargo build --release --no-default-features --features codegen"
```

### Q: Is hot-reload available in codegen mode?

**A**: No, codegen mode compiles XML at build time. For hot-reload development,
use interpreted mode (default): `dampen run` or `dampen build`.

---

## Additional Resources

- [USAGE.md](../USAGE.md) - Complete CLI command reference
- [AGENTS.md](../../AGENTS.md) - Framework development guide
- [QUICKSTART.md](../QUICKSTART.md) - Getting started guide

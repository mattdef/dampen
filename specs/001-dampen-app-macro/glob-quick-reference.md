# Quick Reference: Glob Patterns for #[dampen_app]

## TL;DR

**Decision**: Use `glob` crate (v0.3.3)

```toml
[dependencies]
glob = "0.3.3"
```

```rust
use glob::Pattern;

// Compile patterns once
let patterns: Vec<Pattern> = exclude_strs
    .iter()
    .map(|s| Pattern::new(s).unwrap())
    .collect();

// Match paths
let should_exclude = |path: &Path| {
    patterns.iter().any(|p| p.matches_path(path))
};
```

## Pattern Examples

| Pattern | Matches | Doesn't Match |
|---------|---------|---------------|
| `debug_view.dampen` | `debug_view.dampen` | `debug_view2.dampen` |
| `test_*.dampen` | `test_foo.dampen` | `foo_test.dampen` |
| `experimental/*` | `experimental/a.dampen`<br>`experimental/b/c.dampen` | `stable/a.dampen` |
| `tmp/**/*.dampen` | `tmp/a.dampen`<br>`tmp/b/c.dampen` | `tmp/readme.txt` |
| `{debug,test}_*` | `debug_foo.dampen`<br>`test_bar.dampen` | `prod_baz.dampen` |

## Performance

- **Compilation**: 14µs for 5 patterns
- **Matching**: < 1µs per path
- **Memory**: ~100 bytes per pattern
- **Dependencies**: Zero

## Why Not globset?

- 77x slower (1.08ms vs 14µs)
- 4 extra dependencies
- Overkill for our use case

## Why Not Hand-Rolled?

- Missing features (`**`, `[]`, `{}`)
- No error handling
- Maintenance burden

## Integration Checklist

- [ ] Add `glob = "0.3.3"` to Cargo.toml
- [ ] Parse patterns in attribute parser
- [ ] Strip `ui_dir` prefix before matching
- [ ] Handle pattern errors with clear messages
- [ ] Test with all pattern types
- [ ] Document supported patterns

## Common Pitfalls

1. **Absolute paths**: Match against relative paths (strip ui_dir prefix)
2. **Path separators**: Use forward slashes in patterns (`/` not `\`)
3. **Case sensitivity**: Follows OS defaults (sensitive on Unix)
4. **Empty patterns**: Reject empty strings as invalid
5. **Exclude all**: Warn on patterns like `*` or `**`

## See Also

- Full research: `specs/001-dampen-app-macro/research-glob-patterns.md`
- Test implementation: `crates/dampen-macros/tests/glob_pattern_research.rs`
- glob docs: https://docs.rs/glob/0.3.3/glob/

# Contributing to Dampen

Thank you for your interest in contributing to Dampen! This guide will help you get started with development, testing, and submitting changes.

## Table of Contents

- [Development Setup](#development-setup)
- [Code Style and Standards](#code-style-and-standards)
- [Testing](#testing)
  - [Unit Tests](#unit-tests)
  - [Integration Tests](#integration-tests)
  - [Visual Regression Tests](#visual-regression-tests)
- [Submitting Changes](#submitting-changes)

## Development Setup

### Prerequisites

- Rust 1.85 or later (Edition 2024)
- Git

### Clone and Build

```bash
git clone https://github.com/yourusername/dampen.git
cd dampen
cargo build --workspace
```

### Running Examples

```bash
cargo run -p counter
cargo run -p hello-world
cargo run -p todo-app
```

## Code Style and Standards

Please refer to [CLAUDE.md](./CLAUDE.md) for detailed code style guidelines. Key points:

- **Max line width**: 100 characters
- **No panics**: Use `Result<T, E>` for all fallible operations
- **No `.unwrap()` or `.expect()`**: Clippy will deny these
- **Document public APIs**: Use `///` doc comments
- **Format before committing**: Run `cargo fmt --all`

### Before Committing

Always run the following checks:

```bash
cargo fmt --all                          # Format code
cargo clippy --workspace -- -D warnings  # Lint
cargo test --workspace                   # Run all tests
```

## Testing

### Unit Tests

Unit tests are located in each crate's `tests/` directory or inline `#[cfg(test)]` modules.

```bash
# Run all tests
cargo test --workspace

# Run tests for a specific crate
cargo test -p dampen-core

# Run a single test
cargo test test_parse_valid_simple

# Run tests matching a pattern
cargo test parse_
```

### Integration Tests

Integration tests are in the workspace `tests/` directory.

```bash
cargo test -p integration-tests
cargo test -p contract-tests
```

### Visual Regression Tests

Visual regression tests ensure pixel-perfect consistency between Interpreted (dev) and Codegen (prod) modes.

#### What Are Visual Regression Tests?

Visual regression tests:
1. Render the same UI in both Interpreted and Codegen modes
2. Compare the rendered output pixel-by-pixel
3. Generate a diff image highlighting any differences
4. Fail if the difference exceeds a tolerance threshold (default: 0.001 or 0.1%)

#### Running Visual Tests

```bash
# Run all visual tests
cargo test -p dampen-visual-tests

# Run with verbose output
cargo test -p dampen-visual-tests -- --nocapture
```

#### Creating New Visual Test Cases

Visual test cases are XML files in `tests/visual/cases/`. Each test case should focus on a specific feature or widget.

**Example: `tests/visual/cases/button_with_padding.dampen`**

```xml
<dampen version="1.0">
  <Column>
    <Button text="Click Me" padding="20" align_x="center" />
  </Column>
</dampen>
```

#### Generating Baselines

After creating a new test case, generate the baseline images:

```bash
# Generate baselines for all test cases
bash scripts/generate_baselines.sh

# Generate baseline for a specific test
bash scripts/generate_baselines.sh tests/visual/cases/button_with_padding.dampen
```

Baselines are stored in `tests/visual/baselines/` and should be committed to Git.

#### Understanding Test Results

When a visual test fails:

1. **Check the diff image**: Located at `tests/visual/diffs/<test_name>_diff.png`
2. **Review the actual output**: Located at `tests/visual/actual/<test_name>_actual.png`
3. **Compare with baseline**: Located at `tests/visual/baselines/<test_name>_baseline.png`

The diff image highlights differences in red, making it easy to spot discrepancies.

#### When to Update Baselines

Update baselines when:
- You intentionally changed rendering behavior
- You added new layout features
- You fixed a rendering bug (after verifying the fix is correct)

**DO NOT** update baselines if:
- The diff shows an unintended regression
- You haven't investigated why the output changed

#### Visual Test Architecture

```
crates/dampen-visual-tests/
├── src/
│   ├── lib.rs        # Core types (VisualTestCase, VisualTestResult)
│   ├── compare.rs    # Image comparison logic
│   └── renderer.rs   # Headless rendering (future)
└── tests/
    └── integration_test.rs  # Test runner

tests/visual/
├── cases/            # Test XML files
├── baselines/        # Expected output (PNG)
├── actual/           # Test output (PNG, gitignored)
└── diffs/            # Diff images (PNG, gitignored)
```

#### Best Practices

1. **Keep test cases focused**: One feature or widget per test case
2. **Use descriptive names**: `text_with_color.dampen`, not `test1.dampen`
3. **Test edge cases**: Empty text, long text, nested containers, etc.
4. **Document complex cases**: Add comments in XML explaining what's being tested
5. **Review diffs carefully**: Always inspect the diff image before updating baselines

#### Continuous Integration

Visual regression tests run automatically on CI. If a test fails:

1. Download the diff artifact from the CI logs
2. Investigate the difference
3. Either fix the code or update the baseline (with justification)

## Submitting Changes

### Pull Request Process

1. **Create a feature branch**: `git checkout -b feature/my-feature`
2. **Make your changes**: Follow code style guidelines
3. **Add tests**: Ensure your changes are tested
4. **Run checks**: `cargo fmt`, `cargo clippy`, `cargo test`
5. **Update baselines if needed**: For visual changes, regenerate baselines
6. **Commit with clear messages**: Describe what and why
7. **Push and create PR**: Submit your pull request on GitHub

### Commit Message Format

```
<type>: <short summary>

<detailed description>

Fixes #<issue_number>
```

**Types**: `feat`, `fix`, `docs`, `test`, `refactor`, `chore`

**Example:**

```
feat: add password attribute support to TextInput

- Added `password` attribute to TextInput widget
- Masks input characters in both Interpreted and Codegen modes
- Updated visual tests with password field test case
- Added backward compatibility for legacy `secure` attribute

Fixes #123
```

### Code Review

All contributions require code review. Reviewers will check:

- Code quality and style adherence
- Test coverage
- Documentation completeness
- Visual regression test results

### Getting Help

- **Questions**: Open a GitHub Discussion
- **Bugs**: Open a GitHub Issue
- **Feature Requests**: Open a GitHub Issue with `[Feature Request]` prefix

Thank you for contributing to Dampen!

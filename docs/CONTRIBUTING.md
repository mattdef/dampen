# Contributing to Dampen UI Framework

Thank you for your interest in contributing to Dampen! We welcome contributions from the community and are excited to work with you.

## Table of Contents

- [Code of Conduct](#code-of-conduct)
- [Getting Started](#getting-started)
- [Development Workflow](#development-workflow)
- [Project Structure](#project-structure)
- [Coding Standards](#coding-standards)
- [Testing Requirements](#testing-requirements)
- [Submitting Changes](#submitting-changes)
- [Reporting Issues](#reporting-issues)
- [Feature Requests](#feature-requests)
- [Documentation](#documentation)
- [Community](#community)

## Code of Conduct

This project adheres to a code of conduct that all contributors are expected to follow. Please be respectful, inclusive, and professional in all interactions.

**Our Standards:**
- Be welcoming and inclusive
- Respect differing viewpoints and experiences
- Accept constructive criticism gracefully
- Focus on what is best for the community
- Show empathy towards other community members

## Getting Started

### Prerequisites

- Rust 1.85 or higher (stable channel)
- Git
- A code editor with Rust support

### Setting Up Your Development Environment

1. **Fork the repository** on GitHub

2. **Clone your fork:**
   ```bash
   git clone https://github.com/mattdef/dampen.git
   cd dampen
   ```

3. **Add the upstream remote:**
   ```bash
   git remote add upstream https://github.com/mattdef/dampen.git
   ```

4. **Build the project:**
   ```bash
   cargo build --workspace
   ```

5. **Run the tests:**
   ```bash
   cargo test --workspace
   ```

6. **Try the examples:**
   ```bash
   cargo run -p hello-world
   cargo run -p counter
   cargo run -p todo-app
   ```

## Development Workflow

### Creating a Feature Branch

Always create a new branch for your work:

```bash
git checkout -b feature/your-feature-name
```

Branch naming conventions:
- `feature/description` - New features
- `fix/description` - Bug fixes
- `docs/description` - Documentation updates
- `refactor/description` - Code refactoring
- `test/description` - Test improvements

### Keeping Your Fork Up-to-Date

```bash
git fetch upstream
git checkout master
git merge upstream/master
git push origin master
```

### Working on Your Changes

1. Make your changes in logical, atomic commits
2. Write or update tests as needed
3. Ensure all tests pass
4. Update documentation if you change public APIs
5. Run linting and formatting tools

## Project Structure

```
dampen/
├── crates/
│   ├── dampen-core/       # Core parser, IR, traits (no Iced dependency)
│   ├── dampen-macros/     # Procedural macros
│   ├── dampen-iced/       # Iced backend implementation
│   └── dampen-cli/        # Developer CLI tools
├── examples/              # Example applications
├── specs/                 # Technical specifications
└── docs/                  # Documentation
```

## Coding Standards

### Rust Conventions

- **Edition:** Rust 2024
- **MSRV:** 1.85 stable (no nightly features in public API)
- **Formatting:** Use `rustfmt` with default settings
- **Linting:** Code must pass `clippy` with `-D warnings`

### Code Style

```bash
# Format your code
cargo fmt --all

# Check formatting (used in CI)
cargo fmt --all -- --check

# Run clippy
cargo clippy --workspace -- -D warnings
```

### Naming Conventions

- **Crates:** `dampen-{module}` (kebab-case)
- **Types:** `PascalCase` (e.g., `WidgetNode`, `BindingExpr`)
- **Functions:** `snake_case` (e.g., `parse_xml`, `evaluate_binding`)
- **Constants:** `SCREAMING_SNAKE_CASE`
- **Modules:** `snake_case` matching file names

### Documentation

All public items must have rustdoc comments:

```rust
/// Parses an XML document into a DampenDocument.
///
/// # Arguments
///
/// * `xml` - The XML string to parse
///
/// # Errors
///
/// Returns `ParseError` if the XML is malformed or contains invalid widgets.
///
/// # Examples
///
/// ```
/// use dampen_core::parse_xml;
///
/// let xml = r#"<dampen><text value="Hello" /></dampen>"#;
/// let doc = parse_xml(xml)?;
/// ```
pub fn parse_xml(xml: &str) -> Result<DampenDocument, ParseError> {
    // implementation
}
```

### Error Handling

- Use `Result<T, E>` for fallible operations
- Create custom error types with `thiserror`
- Include span information (line/column) in parse errors
- Provide actionable error messages with fix suggestions

Example:

```rust
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ParseError {
    #[error("Unknown widget '{name}' at {span}")]
    UnknownWidget { name: String, span: Span },
    
    #[error("Missing required attribute '{attr}' on <{widget}> at {span}")]
    MissingAttribute { widget: String, attr: String, span: Span },
}
```

## Testing Requirements

**Test-Driven Development (TDD) is mandatory** for Dampen. Tests must be written before or alongside implementation.

### Running Tests

```bash
# Run all tests
cargo test --workspace

# Run tests for a specific crate
cargo test -p dampen-core
cargo test -p dampen-macros

# Run a specific test
cargo test test_name

# Run tests with output
cargo test -- --nocapture
```

### Test Types

1. **Unit Tests:** Test individual functions and modules
2. **Integration Tests:** Test public APIs and end-to-end workflows
3. **Contract Tests:** Verify XML parsing produces expected IR
4. **Property Tests:** Use `proptest` for edge cases
5. **Snapshot Tests:** Use `insta` for code generation verification

### Test Coverage Goals

- Core parser: >90% coverage
- Public APIs: 100% coverage
- Critical paths: 100% coverage

### Example Test

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_text_widget() {
        let xml = r#"<text value="Hello" size="24" />"#;
        let result = parse_widget(xml).unwrap();
        
        assert_eq!(result.kind, WidgetKind::Text);
        assert_eq!(result.attrs.get("value"), Some(&"Hello".to_string()));
        assert_eq!(result.attrs.get("size"), Some(&"24".to_string()));
    }

    #[test]
    fn test_missing_required_attribute() {
        let xml = r#"<button />"#;
        let result = parse_widget(xml);
        
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), ParseError::MissingAttribute { .. }));
    }
}
```

## Submitting Changes

### Commit Message Guidelines

Follow the [Conventional Commits](https://www.conventionalcommits.org/) specification:

```
<type>(<scope>): <description>

[optional body]

[optional footer]
```

**Types:**
- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation changes
- `style`: Formatting changes (not code style)
- `refactor`: Code refactoring
- `test`: Adding or updating tests
- `chore`: Maintenance tasks

**Examples:**

```
feat(core): add radio widget support

Implements radio button parsing, rendering, and event handling.
Includes single-selection behavior and disabled state support.

Closes #123
```

```
fix(parser): handle malformed XML attributes

Previously, the parser would panic on certain malformed attributes.
Now returns a proper ParseError with span information.
```

### Pull Request Process

1. **Update your branch** with the latest from upstream:
   ```bash
   git fetch upstream
   git rebase upstream/master
   ```

2. **Ensure all checks pass:**
   ```bash
   cargo test --workspace
   cargo clippy --workspace -- -D warnings
   cargo fmt --all -- --check
   ```

3. **Push to your fork:**
   ```bash
   git push origin feature/your-feature-name
   ```

4. **Open a Pull Request** on GitHub:
   - Provide a clear title and description
   - Reference any related issues
   - Describe what changed and why
   - Include screenshots for UI changes
   - List any breaking changes

5. **Respond to feedback:**
   - Address review comments promptly
   - Push additional commits to your branch
   - Request re-review when ready

### PR Checklist

- [ ] All tests pass locally
- [ ] Code follows project style guidelines
- [ ] Clippy produces no warnings
- [ ] Code is properly formatted with rustfmt
- [ ] New public APIs have rustdoc comments
- [ ] Tests added/updated for new functionality
- [ ] Documentation updated if needed
- [ ] CHANGELOG.md updated (for significant changes)
- [ ] No breaking changes (or clearly documented)

## Reporting Issues

### Before Submitting an Issue

1. **Search existing issues** to avoid duplicates
2. **Try the latest version** to see if the issue is fixed
3. **Gather information:**
   - Dampen version
   - Rust version (`rustc --version`)
   - Operating system
   - Minimal reproduction case

### Issue Template

```markdown
**Describe the bug**
A clear and concise description of what the bug is.

**To Reproduce**
Steps to reproduce the behavior:
1. Create XML file with '...'
2. Run command '...'
3. See error

**Expected behavior**
A clear description of what you expected to happen.

**Environment:**
- Dampen version: [e.g., 0.1.0]
- Rust version: [e.g., 1.85.0]
- OS: [e.g., Ubuntu 22.04]

**Additional context**
Add any other context about the problem here.
```

## Feature Requests

We welcome feature requests! Please:

1. **Check existing feature requests** first
2. **Describe the use case** clearly
3. **Explain the expected behavior**
4. **Consider alternatives** you've thought about
5. **Be open to discussion** about implementation

### Feature Request Template

```markdown
**Is your feature request related to a problem?**
A clear description of what the problem is. Ex. I'm always frustrated when [...]

**Describe the solution you'd like**
A clear and concise description of what you want to happen.

**Describe alternatives you've considered**
Any alternative solutions or features you've considered.

**Additional context**
Add any other context or screenshots about the feature request here.
```

## Documentation

### Types of Documentation

1. **Code Documentation:** Rustdoc comments on public items
2. **Examples:** Working example applications in `examples/`
3. **Guides:** Tutorials and how-to guides in `docs/`
4. **Specifications:** Technical specs in `specs/`

### Writing Good Documentation

- Use clear, concise language
- Provide code examples
- Explain the "why" not just the "what"
- Keep documentation up-to-date with code changes
- Link to related documentation

### Building Documentation Locally

```bash
# Generate and open documentation
cargo doc --workspace --no-deps --open
```

## Community

### Getting Help

- **GitHub Discussions:** Ask questions and discuss ideas
- **GitHub Issues:** Report bugs and request features
- **Documentation:** Check the docs first

### Recognition

Contributors will be:
- Listed in CONTRIBUTORS.md
- Credited in release notes for significant contributions
- Acknowledged in commit messages and PRs

## License

By contributing to Dampen, you agree that your contributions will be licensed under the same license as the project (MIT/Apache-2.0 dual license).

---

Thank you for contributing to Dampen! Your efforts help make this project better for everyone.

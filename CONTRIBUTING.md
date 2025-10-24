# Contributing to Luau Obfuscator

Thank you for your interest in contributing! This document provides guidelines for contributing to the project.

## Table of Contents

- [Code of Conduct](#code-of-conduct)
- [Getting Started](#getting-started)
- [Development Workflow](#development-workflow)
- [Coding Standards](#coding-standards)
- [Testing Guidelines](#testing-guidelines)
- [Pull Request Process](#pull-request-process)
- [Issue Guidelines](#issue-guidelines)

---

## Code of Conduct

### Our Pledge

We pledge to make participation in our project a harassment-free experience for everyone, regardless of:
- Age, body size, disability
- Ethnicity, sex characteristics
- Gender identity and expression
- Level of experience, education
- Nationality, personal appearance
- Race, religion, sexual orientation

### Our Standards

**Positive behavior includes**:
- Using welcoming and inclusive language
- Respecting differing viewpoints and experiences
- Gracefully accepting constructive criticism
- Focusing on what's best for the community
- Showing empathy towards others

**Unacceptable behavior includes**:
- Trolling, insulting/derogatory comments, personal attacks
- Public or private harassment
- Publishing others' private information
- Other conduct inappropriate in a professional setting

### Enforcement

Instances of abusive, harassing, or otherwise unacceptable behavior may be reported to the project maintainers at conduct@example.com. All complaints will be reviewed and investigated promptly and fairly.

---

## Getting Started

### Prerequisites

- **Rust**: 1.70 or higher
  ```bash
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  rustup update
  ```

- **Git**: For version control
  ```bash
  git --version  # Should be 2.0+
  ```

### Fork and Clone

1. **Fork the repository** on GitHub

2. **Clone your fork**:
   ```bash
   git clone https://github.com/YOUR_USERNAME/luau-obfuscator.git
   cd luau-obfuscator
   ```

3. **Add upstream remote**:
   ```bash
   git remote add upstream https://github.com/danila-permogorskii/luau-obfuscator.git
   git fetch upstream
   ```

### Build the Project

```bash
# Debug build (fast compilation)
cargo build

# Release build (optimized)
cargo build --release

# Run tests
cargo test

# Run clippy (linter)
cargo clippy

# Format code
cargo fmt
```

---

## Development Workflow

### 1. Create a Branch

```bash
# Create feature branch
git checkout -b feature/your-feature-name

# Or bug fix branch
git checkout -b fix/issue-123
```

**Branch naming conventions**:
- `feature/` - New features
- `fix/` - Bug fixes
- `docs/` - Documentation updates
- `refactor/` - Code refactoring
- `test/` - Test additions/improvements

### 2. Make Changes

- Write code following our [coding standards](#coding-standards)
- Add tests for new functionality
- Update documentation as needed
- Run tests frequently: `cargo test`

### 3. Commit Changes

**Commit message format**:
```
<type>(<scope>): <subject>

<body>

<footer>
```

**Types**:
- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation changes
- `style`: Code style (formatting, no logic change)
- `refactor`: Code refactoring
- `test`: Test additions/improvements
- `chore`: Maintenance tasks

**Example**:
```bash
git commit -m "feat(obfuscation): add control flow flattening

Implement state machine-based control flow flattening for Premium tier.
Reduces code readability by 70% according to benchmarks.

Closes #123"
```

### 4. Sync with Upstream

```bash
# Fetch latest changes
git fetch upstream

# Rebase your branch
git rebase upstream/master

# If conflicts, resolve them
git add <resolved-files>
git rebase --continue
```

### 5. Push Changes

```bash
git push origin feature/your-feature-name
```

### 6. Create Pull Request

See [Pull Request Process](#pull-request-process) below.

---

## Coding Standards

### Rust Style Guide

We follow the [Rust Style Guide](https://rust-lang.github.io/api-guidelines/) with these additions:

#### 1. Formatting

**Always run** `cargo fmt` before committing:
```bash
cargo fmt --all
```

**EditorConfig** (for consistent formatting across editors):
```ini
# .editorconfig
root = true

[*]
charset = utf-8
end_of_line = lf
insert_final_newline = true
trim_trailing_whitespace = true

[*.rs]
indent_style = space
indent_size = 4
```

#### 2. Naming Conventions

```rust
// Types: PascalCase
struct MyStruct { }
enum MyEnum { }

// Functions, variables: snake_case
fn my_function() { }
let my_variable = 42;

// Constants: SCREAMING_SNAKE_CASE
const MAX_SIZE: usize = 1024;

// Lifetime parameters: short, descriptive
fn foo<'a, 'b>(x: &'a str, y: &'b str) { }
```

#### 3. Documentation

**Module documentation**:
```rust
//! Module for parsing Luau source code.
//!
//! This module provides a wrapper around the `full_moon` crate
//! with Roblox-specific handling.

use full_moon::parse;

// ...
```

**Function documentation**:
```rust
/// Parses a Luau source string into an AST.
///
/// # Arguments
///
/// * `source` - Luau source code as a string
///
/// # Returns
///
/// * `Ok(Ast)` - Parsed abstract syntax tree
/// * `Err(ParseError)` - Syntax error with location
///
/// # Examples
///
/// ```
/// let ast = parse("local x = 42")?;
/// assert!(ast.statements().len() == 1);
/// ```
pub fn parse(source: &str) -> Result<Ast, ParseError> {
    // ...
}
```

#### 4. Error Handling

**Use `Result` and `?` operator**:
```rust
// ✅ Good
fn read_file(path: &Path) -> Result<String> {
    let content = fs::read_to_string(path)?;
    Ok(content)
}

// ❌ Bad (unwrap)
fn read_file(path: &Path) -> String {
    fs::read_to_string(path).unwrap()
}
```

**Custom error types** (use `thiserror`):
```rust
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ObfuscationError {
    #[error("Failed to parse script: {0}")]
    ParseError(String),
    
    #[error("Encryption failed: {0}")]
    CryptoError(#[from] ring::error::Unspecified),
}
```

#### 5. Testing

**Unit tests in same file**:
```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_parse_simple() {
        let ast = parse("local x = 42").unwrap();
        assert_eq!(ast.statements().len(), 1);
    }
    
    #[test]
    fn test_parse_invalid() {
        let result = parse("local x =");  // Missing value
        assert!(result.is_err());
    }
}
```

**Integration tests** in `tests/` directory:
```rust
// tests/integration/obfuscation.rs
use luau_obfuscator::*;

#[test]
fn test_end_to_end_obfuscation() {
    let input = include_str!("../fixtures/sample.lua");
    let config = ObfuscationConfig::default();
    
    let output = obfuscate(input, &config).unwrap();
    
    assert!(output.contains("decrypt_string"));
    assert!(!output.contains("sensitive_data"));
}
```

#### 6. Performance

**Use `#[inline]` for hot paths**:
```rust
#[inline]
pub fn xor_bytes(a: &[u8], b: &[u8]) -> Vec<u8> {
    a.iter().zip(b).map(|(x, y)| x ^ y).collect()
}
```

**Avoid unnecessary allocations**:
```rust
// ❌ Bad (allocates new String)
fn get_name(&self) -> String {
    self.name.clone()
}

// ✅ Good (borrows)
fn get_name(&self) -> &str {
    &self.name
}
```

---

## Testing Guidelines

### Test Coverage Goals

- **Core modules**: >90% coverage
- **CLI**: >80% coverage
- **Utilities**: >70% coverage

### Running Tests

```bash
# All tests
cargo test

# Specific test
cargo test test_name

# With output
cargo test -- --nocapture

# Integration tests only
cargo test --test '*'

# Benchmarks
cargo bench
```

### Writing Good Tests

**1. Descriptive names**:
```rust
#[test]
fn test_parse_returns_error_on_invalid_syntax() {
    // ...
}
```

**2. Arrange-Act-Assert pattern**:
```rust
#[test]
fn test_encryption() {
    // Arrange
    let encryptor = Aes256Gcm::new(key);
    let plaintext = b"secret";
    
    // Act
    let encrypted = encryptor.encrypt(plaintext).unwrap();
    let decrypted = encryptor.decrypt(&encrypted).unwrap();
    
    // Assert
    assert_eq!(decrypted, plaintext);
}
```

**3. Test edge cases**:
```rust
#[test]
fn test_parse_empty_string() {
    assert!(parse("").is_err());
}

#[test]
fn test_parse_very_large_file() {
    let input = "local x = 1\n".repeat(100_000);
    assert!(parse(&input).is_ok());
}
```

**4. Use fixtures for complex inputs**:
```rust
#[test]
fn test_obfuscate_real_script() {
    let input = include_str!("../fixtures/admin_commands.lua");
    let output = obfuscate(input, &Config::default()).unwrap();
    // Assertions...
}
```

---

## Pull Request Process

### Before Submitting

**Checklist**:
- [ ] Code follows style guidelines (`cargo fmt`, `cargo clippy`)
- [ ] Tests pass (`cargo test`)
- [ ] New tests added for new functionality
- [ ] Documentation updated
- [ ] CHANGELOG.md updated (if user-facing changes)
- [ ] Commit messages follow convention
- [ ] Branch is rebased on latest `master`

### PR Template

When creating a PR, use this template:

```markdown
## Description

Brief description of changes.

## Motivation and Context

Why is this change needed? What problem does it solve?

Closes #123

## How Has This Been Tested?

- [ ] Unit tests
- [ ] Integration tests
- [ ] Manual testing (describe steps)

## Types of Changes

- [ ] Bug fix (non-breaking change which fixes an issue)
- [ ] New feature (non-breaking change which adds functionality)
- [ ] Breaking change (fix or feature that changes existing functionality)
- [ ] Documentation update

## Checklist

- [ ] My code follows the code style of this project
- [ ] I have updated the documentation accordingly
- [ ] I have added tests to cover my changes
- [ ] All new and existing tests passed
```

### Review Process

1. **Automated checks**: CI runs tests, linting, formatting
2. **Code review**: Maintainers review code and provide feedback
3. **Address feedback**: Make requested changes
4. **Approval**: Requires 1 maintainer approval
5. **Merge**: Maintainer merges PR

**Review timeline**:
- **Small PRs** (<100 lines): 1-3 days
- **Medium PRs** (100-500 lines): 3-7 days
- **Large PRs** (>500 lines): 7-14 days

---

## Issue Guidelines

### Before Filing an Issue

1. **Search existing issues**: Check if already reported
2. **Check documentation**: Problem may be user error
3. **Try latest version**: Bug may be already fixed

### Bug Reports

Use this template:

```markdown
**Environment**:
- OS: [e.g., macOS 14.0]
- Rust version: [e.g., 1.70.0]
- Luau Obfuscator version: [e.g., 0.1.0]

**Description**:
Clear description of the bug.

**Steps to Reproduce**:
1. Step 1
2. Step 2
3. See error

**Expected Behavior**:
What should happen.

**Actual Behavior**:
What actually happens.

**Logs**:
```
Paste relevant logs here (use RUST_LOG=debug)
```

**Additional Context**:
Any other relevant information.
```

### Feature Requests

Use this template:

```markdown
**Is your feature request related to a problem?**
Clear description of the problem. Ex. "I'm frustrated when..."

**Describe the solution you'd like**
Clear description of what you want to happen.

**Describe alternatives you've considered**
Other solutions or features you've considered.

**Additional context**
Any other context or screenshots.

**Are you willing to implement this?**
- [ ] Yes, I can submit a PR
- [ ] No, requesting someone else implements it
```

---

## Development Resources

### Documentation

- [User Guide](docs/USER_GUIDE.md)
- [Developer Guide](docs/DEVELOPER_GUIDE.md)
- [Architecture Documentation](docs/ARCHITECTURE.md)
- [API Integration Guide](docs/API_INTEGRATION.md)

### Useful Links

- [Rust Book](https://doc.rust-lang.org/book/)
- [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- [full_moon documentation](https://docs.rs/full_moon/)
- [ring cryptography](https://docs.rs/ring/)
- [Luau documentation](https://luau-lang.org/)

### Community

- **Discord**: https://discord.gg/example
- **GitHub Discussions**: https://github.com/danila-permogorskii/luau-obfuscator/discussions

---

## Release Process

(For maintainers only)

### Version Numbering

We follow [Semantic Versioning](https://semver.org/):

- **MAJOR**: Breaking changes
- **MINOR**: New features (backwards compatible)
- **PATCH**: Bug fixes (backwards compatible)

### Release Checklist

1. Update version in `Cargo.toml`
2. Update `CHANGELOG.md`
3. Run `cargo test` and `cargo clippy`
4. Create git tag: `git tag v1.0.0`
5. Push tag: `git push origin v1.0.0`
6. GitHub Actions creates release automatically
7. Publish to crates.io: `cargo publish`

---

## Recognition

### Hall of Fame

Top contributors will be recognized in:
- README.md
- Release notes
- Annual contributor report

### Rewards

- **First-time contributors**: Welcome message and guidance
- **Regular contributors**: Discord role and early access to features
- **Major contributors**: Free API subscription upgrade

---

## Questions?

If you have questions about contributing:

- **GitHub Discussions**: https://github.com/danila-permogorskii/luau-obfuscator/discussions
- **Discord**: https://discord.gg/example
- **Email**: contribute@example.com

Thank you for contributing! 🚀

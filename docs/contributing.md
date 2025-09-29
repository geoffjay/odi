# Contributing to ODI

Thank you for your interest in contributing to ODI! This document provides guidelines and information for contributors.

## Table of Contents

- [Code of Conduct](#code-of-conduct)
- [Getting Started](#getting-started)
- [How to Contribute](#how-to-contribute)
- [Development Process](#development-process)
- [Testing Guidelines](#testing-guidelines)
- [Documentation](#documentation)
- [Community](#community)

## Code of Conduct

### Our Pledge

We are committed to making participation in ODI a harassment-free experience for everyone, regardless of age, body size, disability, ethnicity, sex characteristics, gender identity and expression, level of experience, education, socio-economic status, nationality, personal appearance, race, religion, or sexual identity and orientation.

### Our Standards

Examples of behavior that contributes to creating a positive environment include:

- Using welcoming and inclusive language
- Being respectful of differing viewpoints and experiences
- Gracefully accepting constructive criticism
- Focusing on what is best for the community
- Showing empathy towards other community members

### Enforcement

Instances of abusive, harassing, or otherwise unacceptable behavior may be reported by contacting the project team at conduct@odi-project.org. All complaints will be reviewed and investigated promptly and fairly.

## Getting Started

### Prerequisites

Before contributing, ensure you have:

- **Rust 1.75+** installed via [rustup](https://rustup.rs/)
- **Git** for version control
- Familiarity with Rust development practices
- Basic understanding of distributed systems concepts

### Setting Up Your Development Environment

1. **Fork the repository** on GitHub
2. **Clone your fork:**
   ```bash
   git clone https://github.com/your-username/odi.git
   cd odi
   ```
3. **Add upstream remote:**
   ```bash
   git remote add upstream https://github.com/original-org/odi.git
   ```
4. **Install development tools:**
   ```bash
   rustup component add rustfmt clippy
   cargo install cargo-tarpaulin  # For code coverage
   ```
5. **Build and test:**
   ```bash
   cargo build --workspace
   cargo test --workspace
   ```

## How to Contribute

### Types of Contributions

We welcome various types of contributions:

#### 🐛 Bug Reports
- Use the bug report template
- Include steps to reproduce
- Provide system information
- Search existing issues first

#### ✨ Feature Requests  
- Use the feature request template
- Explain the problem you're solving
- Describe your proposed solution
- Consider backward compatibility

#### 📖 Documentation
- Fix typos and grammar
- Improve clarity and examples
- Add missing documentation
- Translate to other languages

#### 🔧 Code Contributions
- Bug fixes
- New features
- Performance improvements
- Refactoring for maintainability

#### 🧪 Testing
- Add test cases for uncovered scenarios
- Improve test reliability
- Add integration tests
- Performance benchmarks

### Finding Issues to Work On

Good first issues for new contributors:

- Look for `good first issue` label
- Check `help wanted` label
- Browse `documentation` issues
- Review `bug` reports with clear reproduction steps

For experienced contributors:
- `enhancement` requests
- `performance` improvements  
- `architecture` discussions

## Development Process

### Branching Strategy

1. **Create a feature branch:**
   ```bash
   git checkout -b feature/your-feature-name
   ```
2. **Keep branches focused** on a single feature or fix
3. **Use descriptive branch names:**
   - `feature/advanced-search`
   - `fix/sync-conflict-resolution`
   - `docs/api-examples`

### Commit Guidelines

Follow [Conventional Commits](https://www.conventionalcommits.org/):

```
type(scope): description

[optional body]

[optional footer(s)]
```

**Types:**
- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation only changes
- `style`: Formatting, missing semicolons, etc.
- `refactor`: Code change that neither fixes a bug nor adds a feature
- `perf`: Performance improvements
- `test`: Adding missing tests or correcting existing tests
- `chore`: Changes to build process or auxiliary tools

**Examples:**
```
feat(sync): add automatic conflict resolution

Implement configurable strategies for resolving conflicts automatically
during synchronization operations.

Closes #123

fix(storage): handle malformed configuration files

Previously, malformed TOML files would cause a panic. Now they
are handled gracefully with a descriptive error message.

Fixes #456
```

### Pull Request Process

1. **Ensure your branch is up-to-date:**
   ```bash
   git fetch upstream
   git rebase upstream/main
   ```

2. **Run the full test suite:**
   ```bash
   cargo test --workspace
   cargo clippy --workspace -- -D warnings
   cargo fmt --all --check
   ```

3. **Create a pull request:**
   - Use the PR template
   - Write a clear description
   - Link related issues
   - Include screenshots for UI changes

4. **Respond to feedback:**
   - Address review comments promptly
   - Push additional commits as needed
   - Keep the conversation constructive

5. **Final steps:**
   - Squash commits if requested
   - Ensure CI passes
   - Wait for maintainer approval

### Code Review Guidelines

#### For Contributors
- **Be responsive** to feedback
- **Ask questions** if something is unclear
- **Explain your approach** for complex changes
- **Test thoroughly** before requesting review

#### For Reviewers
- **Be constructive** and helpful
- **Focus on the code**, not the person
- **Suggest improvements** with examples
- **Acknowledge good practices**

Example review comments:
```
👍 Good: "Nice use of the builder pattern here!"

💡 Suggestion: "Consider using `Result<T>` instead of `Option<T>` 
to provide more context about failures."

❓ Question: "Could you explain why this approach was chosen 
over using the existing `Storage` trait?"

🐛 Issue: "This could panic if the vector is empty. Consider 
using `first()` instead of indexing."
```

## Testing Guidelines

### Test Categories

#### Unit Tests
- Test individual functions and methods
- Located in `#[cfg(test)]` modules
- Fast execution, no external dependencies

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_issue_title_validation() {
        assert!(Issue::validate_title("Valid title"));
        assert!(!Issue::validate_title(""));
        assert!(!Issue::validate_title(&"x".repeat(101)));
    }
}
```

#### Integration Tests
- Test interactions between components
- Located in `tests/integration/`
- May use external resources (files, network)

```rust
#[tokio::test]
async fn test_issue_lifecycle() {
    let workspace = setup_test_workspace().await;
    
    let issue_id = workspace.create_issue(sample_issue()).await.unwrap();
    let issue = workspace.get_issue(issue_id).await.unwrap();
    
    assert_eq!(issue.status, IssueStatus::Open);
}
```

#### Contract Tests
- Validate interfaces between crates
- Located in `tests/contract/`
- Ensure API compatibility

### Test Quality Standards

- **Test names should be descriptive:**
  ```rust
  #[test]
  fn test_issue_status_transition_from_open_to_in_progress() {
      // ...
  }
  ```

- **Use the AAA pattern (Arrange, Act, Assert):**
  ```rust
  #[test]
  fn test_issue_assignment() {
      // Arrange
      let mut issue = Issue::new("Test".to_string(), "author".to_string());
      let assignee = "user123".to_string();
      
      // Act  
      issue.add_assignee(assignee.clone());
      
      // Assert
      assert!(issue.assignees.contains(&assignee));
  }
  ```

- **Test edge cases and error conditions:**
  ```rust
  #[test]
  fn test_invalid_status_transition() {
      let mut issue = Issue::new("Test".to_string(), "author".to_string());
      
      // Can't go directly from Open to Resolved
      assert!(issue.update_status(IssueStatus::Resolved).is_err());
  }
  ```

### Running Tests

```bash
# All tests
cargo test --workspace

# Specific test
cargo test test_issue_creation

# With output
cargo test -- --nocapture

# Integration tests only  
cargo test --test integration

# With coverage
cargo tarpaulin --workspace
```

### Test Data and Fixtures

Use the `tests/fixtures/` directory for shared test data:

```rust
// tests/fixtures/mod.rs
pub fn sample_issue() -> Issue {
    Issue::new("Sample issue".to_string(), "test_user".to_string())
}

pub fn sample_workspace() -> Workspace {
    Workspace::new_temp().unwrap()
}
```

## Documentation

### Types of Documentation

#### API Documentation
- Use rustdoc comments (`///`)
- Include examples
- Document all public items

```rust
/// Create a new issue with the specified title and author.
///
/// # Arguments
/// * `title` - Issue title (must be 1-100 characters)
/// * `author` - User ID of the issue creator
///
/// # Returns
/// A new `Issue` instance with default values for optional fields
///
/// # Examples
/// ```
/// use odi_core::Issue;
/// 
/// let issue = Issue::new("Fix login bug".to_string(), "alice".to_string());
/// assert_eq!(issue.status, IssueStatus::Open);
/// ```
///
/// # Panics
/// Panics if the title is empty or longer than 100 characters.
pub fn new(title: String, author: UserId) -> Self {
    assert!(!title.is_empty() && title.len() <= 100, "Invalid title length");
    // ... implementation
}
```

#### User Guides
- Located in `docs/`
- Written in Markdown
- Include practical examples
- Cover common use cases

#### Code Comments
- Explain **why**, not what
- Document complex algorithms
- Note important invariants

```rust
// We use a B-tree map here because we need ordered iteration
// for consistent synchronization across remotes
let mut sorted_issues = BTreeMap::new();

// SAFETY: This is safe because we've verified the index bounds above
unsafe { issues.get_unchecked(idx) }
```

### Documentation Standards

- **Use clear, simple language**
- **Include practical examples**
- **Keep documentation up-to-date** with code changes
- **Link to related documentation**
- **Use consistent terminology**

### Building Documentation

```bash
# Generate and open API docs
cargo doc --workspace --open

# Check for broken links
cargo doc --workspace --no-deps

# Test code examples in docs
cargo test --doc
```

## Community

### Communication Channels

- **GitHub Issues**: Bug reports, feature requests, discussions
- **GitHub Discussions**: General questions, ideas, showcase
- **Discord**: Real-time chat with maintainers and community
- **Email**: Security issues and private matters

### Getting Help

- **Check existing documentation** first
- **Search GitHub issues** for similar problems
- **Ask in GitHub Discussions** for general questions
- **Join our Discord** for real-time help
- **Be patient and respectful** when asking for help

### Community Guidelines

- **Be welcoming** to newcomers
- **Help others** when you can
- **Share your experiences** and learnings
- **Respect different perspectives** and approaches
- **Follow the code of conduct** in all interactions

## Recognition

We appreciate all contributions and recognize them in several ways:

### Contributors File
All contributors are listed in `CONTRIBUTORS.md` with their contributions.

### Release Notes
Significant contributions are mentioned in release notes and changelogs.

### GitHub Recognition
- Contributors get mentioned in PR descriptions and commits
- Regular contributors may be invited as collaborators
- Outstanding contributors may join the core team

## Licensing

By contributing to ODI, you agree that your contributions will be licensed under the same license as the project (MIT License). 

### Developer Certificate of Origin

By making a contribution to this project, I certify that:

1. The contribution was created in whole or in part by me and I have the right to submit it under the open source license indicated in the file; or

2. The contribution is based upon previous work that, to the best of my knowledge, is covered under an appropriate open source license and I have the right under that license to submit that work with modifications, whether created in whole or in part by me, under the same open source license (unless I am permitted to submit under a different license), as indicated in the file; or

3. The contribution was provided directly to me by some other person who certified (1), (2) or (3) and I have not modified it.

## Questions?

If you have questions about contributing:

1. **Read through this guide** completely
2. **Check the [Development Guide](development.md)** for technical details
3. **Look at existing issues and PRs** for examples
4. **Ask in GitHub Discussions** if still unclear
5. **Contact maintainers** directly for sensitive matters

Thank you for contributing to ODI! 🚀
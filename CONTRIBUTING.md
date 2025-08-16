# Contributing to Feytra

> Help us revolutionize database schema intelligence!

Thank you for your interest in contributing to Feytra! We're building the future of database schema understanding with AI, and we welcome contributions from developers, database experts, and AI enthusiasts.

## 🎯 What We're Building

Feytra is an AI-powered database schema intelligence tool that runs as a sidecar next to your database. Think of it as "Cursor but for databases" - you can ask natural language questions about your schema and get intelligent, contextual answers.

## 🤝 How to Contribute

### Types of Contributions We Welcome

- 🐛 **Bug Reports**: Help us identify and fix issues
- 💡 **Feature Requests**: Suggest new capabilities and improvements
- 🔧 **Code Contributions**: Implement features, fix bugs, improve performance
- 📚 **Documentation**: Improve docs, add examples, clarify concepts
- 🧪 **Testing**: Write tests, improve test coverage
- 🌍 **Localization**: Help translate documentation and UI
- 🎨 **UI/UX**: Improve the CLI interface and user experience
- 🔍 **Database Support**: Add support for new database types

### Before You Start

1. **Read the Vision**: Understand our [README.md](README.md) and [ROADMAP.md](ROADMAP.md)
2. **Check Issues**: Look at existing [issues](https://github.com/RikhiSingh/feytra/issues)
3. **Join the Community**: Participate in discussions and share your ideas

## 🚀 Development Setup

### Prerequisites

- **Rust 1.78+**: [Install via rustup](https://rustup.rs/)
- **PostgreSQL**: For testing and development
- **Git**: For version control

### Local Development

```bash
# Clone the repository
git clone https://github.com/RikhiSingh/feytra.git
cd feytra

# Install dependencies
cargo build

# Run tests
cargo test

# Start development
cargo run -p sidecar -- chat
```

### Environment Setup

Create a `.env` file for development:

```env
# Database Configuration
DATABASE_URL=postgresql://admin:admin@localhost:5432/feytra

# AI Configuration (Optional)
OPENAI_API_KEY=your_openai_api_key
LLM_PROVIDER=openai
OPENAI_MODEL=gpt-4o-mini
OPENAI_EMBED_MODEL=text-embedding-3-small

# Feytra Configuration
SIDECAR_DB=./data/sidecar.sqlite
EMBED_PROVIDER=openai
EMBED_DIM=768
VECTOR_BACKEND=hnsw
RUST_LOG=info,sidecar=debug
```

## 📋 Development Workflow

### 1. Fork and Clone

```bash
# Fork the repository on GitHub
# Then clone your fork
git clone https://github.com/RikhiSingh/feytra.git
cd feytra

# Add the upstream repository
git remote add upstream https://github.com/RikhiSingh/feytra.git
```

### 2. Create a Feature Branch

```bash
# Create a new branch for your feature
git checkout -b feature/your-feature-name

# Or for bug fixes
git checkout -b fix/your-bug-description
```

### 3. Make Your Changes

Follow our coding standards and make your changes:

```bash
# Make your changes
# Test your changes
cargo test

# Format your code
cargo fmt

# Check for linting issues
cargo clippy --all -- -D warnings
```

### 4. Test Your Changes

```bash
# Run all tests
cargo test --all

# Run specific tests
cargo test -p feytra-core
cargo test -p feytra-api

# Run integration tests
cargo test --test integration

# Test the CLI
cargo run -p sidecar -- chat
```

### 5. Commit Your Changes

```bash
# Add your changes
git add .

# Commit with a descriptive message
git commit -m "feat: add support for MySQL database connector

- Implement MySQL catalog provider
- Add MySQL-specific DDL canonicalization
- Update documentation with MySQL examples

Closes #123"
```

### 6. Push and Create a Pull Request

```bash
# Push to your fork
git push origin feature/your-feature-name

# Create a pull request on GitHub
```

## 📝 Coding Standards

### Rust Code Style

We follow Rust best practices and use standard tooling:

- **Formatting**: Use `cargo fmt` for consistent code formatting
- **Linting**: Use `cargo clippy` for code quality checks
- **Documentation**: Document all public APIs with Rustdoc comments
- **Error Handling**: Use `anyhow` for error handling, provide meaningful error messages
- **Testing**: Write unit tests for all new functionality

### Code Organization

- **One export per file**: Keep files focused and single-purpose
- **Clear naming**: Use descriptive names for functions, variables, and types
- **Separation of concerns**: Keep business logic separate from infrastructure
- **Type safety**: Leverage Rust's type system for safety and clarity

### Example Code Style

```rust
/// Fetches schema objects from the database catalog.
///
/// This function connects to the database and retrieves all schema objects
/// including tables, views, functions, and indexes.
///
/// # Arguments
/// * `config` - Database configuration containing connection details
///
/// # Returns
/// * `Result<Vec<SchemaObject>>` - List of schema objects or error
///
/// # Errors
/// * Database connection failures
/// * Permission errors
/// * Invalid configuration
pub async fn fetch_schema_objects(config: &DatabaseConfig) -> anyhow::Result<Vec<SchemaObject>> {
    let pool = PgPool::connect(&config.connection_string).await?;
    let provider = PgCatalogProvider::new(pool);
    let objects = provider.snapshot().await?;

    Ok(objects)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_fetch_schema_objects() {
        // Arrange
        let config = DatabaseConfig::test_config();

        // Act
        let result = fetch_schema_objects(&config).await;

        // Assert
        assert!(result.is_ok());
        let objects = result.unwrap();
        assert!(!objects.is_empty());
    }
}
```

## 🧪 Testing Guidelines

### Test Structure

Follow the **Arrange-Act-Assert** pattern:

```rust
#[test]
fn test_feature_name() {
    // Arrange - Set up test data and conditions
    let input = "test data";
    let expected = "expected result";

    // Act - Execute the function being tested
    let actual = function_under_test(input);

    // Assert - Verify the results
    assert_eq!(actual, expected);
}
```

### Test Naming

Use descriptive test names that explain what is being tested:

```rust
// Good
#[test]
fn test_fetch_schema_objects_returns_empty_list_when_database_is_empty() {}

// Bad
#[test]
fn test_fetch() {}
```

### Test Coverage

- **Unit tests**: Test individual functions and methods
- **Integration tests**: Test component interactions
- **End-to-end tests**: Test complete user workflows
- **Performance tests**: Test performance characteristics

## 📚 Documentation Standards

### Code Documentation

- **Public APIs**: Document all public functions, structs, and traits
- **Examples**: Include usage examples in documentation
- **Error cases**: Document possible error conditions
- **Performance notes**: Mention performance characteristics when relevant

### User Documentation

- **Clear examples**: Provide practical, working examples
- **Step-by-step guides**: Break down complex processes
- **Troubleshooting**: Include common issues and solutions
- **Screenshots**: Add visual aids where helpful

## 🐛 Bug Reports

### Before Reporting

1. **Check existing issues**: Search for similar problems
2. **Reproduce the issue**: Ensure you can consistently reproduce it
3. **Check your environment**: Verify your setup matches requirements

### Bug Report Template

```markdown
## Bug Description

Brief description of the issue.

## Steps to Reproduce

1. Step 1
2. Step 2
3. Step 3

## Expected Behavior

What you expected to happen.

## Actual Behavior

What actually happened.

## Environment

- OS: [e.g., Windows 11, macOS 14.0, Ubuntu 22.04]
- Rust version: [e.g., 1.78.0]
- PostgreSQL version: [e.g., 15.0]
- Feytra version: [e.g., v0.1.0]

## Additional Information

- Error messages
- Logs
- Screenshots
- Related issues
```

## 💡 Feature Requests

### Before Requesting

1. **Check the roadmap**: See if it's already planned
2. **Search discussions**: Look for similar requests
3. **Think about use cases**: Consider how it fits our vision

### Feature Request Template

```markdown
## Feature Description

Brief description of the feature.

## Use Case

Why is this feature needed? What problem does it solve?

## Proposed Solution

How should this feature work?

## Alternatives Considered

What other approaches were considered?

## Additional Context

- Related features
- Implementation ideas
- Mockups or examples
```

## 🔄 Pull Request Process

### Before Submitting

1. **Test thoroughly**: Ensure all tests pass
2. **Update documentation**: Add or update relevant docs
3. **Check formatting**: Run `cargo fmt` and `cargo clippy`
4. **Write clear commit messages**: Use conventional commit format

### Pull Request Template

```markdown
## Description

Brief description of changes.

## Type of Change

- [ ] Bug fix (non-breaking change which fixes an issue)
- [ ] New feature (non-breaking change which adds functionality)
- [ ] Breaking change (fix or feature that would cause existing functionality to not work as expected)
- [ ] Documentation update

## Testing

- [ ] Unit tests pass
- [ ] Integration tests pass
- [ ] Manual testing completed
- [ ] Performance impact assessed

## Checklist

- [ ] Code follows the project's style guidelines
- [ ] Self-review of code completed
- [ ] Code is commented, particularly in hard-to-understand areas
- [ ] Documentation is updated
- [ ] Changes generate no new warnings
- [ ] Tests are added that prove the fix is effective or that the feature works

## Related Issues

Closes #123
Related to #456
```

## 🏷️ Commit Message Format

We use [Conventional Commits](https://www.conventionalcommits.org/) format:

```
<type>[optional scope]: <description>

[optional body]

[optional footer(s)]
```

### Types

- **feat**: A new feature
- **fix**: A bug fix
- **docs**: Documentation only changes
- **style**: Changes that do not affect the meaning of the code
- **refactor**: A code change that neither fixes a bug nor adds a feature
- **perf**: A code change that improves performance
- **test**: Adding missing tests or correcting existing tests
- **chore**: Changes to the build process or auxiliary tools

### Examples

```
feat: add MySQL database connector

- Implement MySQL catalog provider
- Add MySQL-specific DDL canonicalization
- Update documentation with MySQL examples

Closes #123
```

```
fix(core): resolve memory leak in vector search

The HNSW index was not properly cleaning up old embeddings,
causing memory usage to grow over time.

Fixes #456
```

## 🎯 Areas for Contribution

### High Priority

- **Database Connectors**: MySQL, SQLite, Oracle support
- **Performance Optimization**: Faster vector search and response times
- **Error Handling**: Better error messages and recovery
- **Testing**: Improve test coverage and add integration tests

### Medium Priority

- **CLI Enhancements**: Better user experience and more commands
- **Documentation**: Tutorials, examples, and best practices
- **Monitoring**: Health checks and metrics
- **Deployment**: Docker, Kubernetes, and cloud deployment guides

### Low Priority

- **UI/UX**: Web interface and visualizations
- **Integrations**: IDE plugins and third-party integrations
- **Advanced Features**: Schema visualization and advanced analytics

## 🤝 Community Guidelines

### Code of Conduct

We are committed to providing a welcoming and inclusive environment for all contributors. Please:

- **Be respectful**: Treat everyone with respect and kindness
- **Be constructive**: Provide helpful, constructive feedback
- **Be inclusive**: Welcome contributors from all backgrounds
- **Be patient**: Remember that everyone is learning and growing

### Communication

- **GitHub Issues**: For bug reports and feature requests
- **GitHub Discussions**: For questions, ideas, and general discussion
- **Pull Requests**: For code contributions
- **Documentation**: For improving docs and examples

## 🏆 Recognition

We appreciate all contributions! Contributors will be:

- **Listed in CONTRIBUTORS.md**: For significant contributions
- **Mentioned in release notes**: For major features and fixes
- **Invited to join the team**: For consistent, high-quality contributions

## 📞 Getting Help

- **Issues**: Search existing [issues](https://github.com/RikhiSingh/feytra/issues)
- **Community**: Join our community channels

## 🎉 Thank You!

Thank you for contributing to Feytra! Your contributions help us build the future of database schema intelligence and make databases as easy to understand as code.

---

**Together, we're building "Cursor for databases"! 🚀**

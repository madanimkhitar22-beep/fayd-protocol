# 🤝 Contributing to Fayd

Thank you for your interest in contributing to Fayd Protocol! This document provides guidelines and instructions for contributions.

---

## 📋 Table of Contents

- [Code of Conduct](#code-of-conduct)
- [Getting Started](#getting-started)
- [How to Contribute](#how-to-contribute)
- [Development Setup](#development-setup)
- [Pull Request Process](#pull-request-process)
- [Community](#community)

---

## 📜 Code of Conduct

Please read and follow our [Code of Conduct](CODE_OF_CONDUCT.md) to help us maintain a respectful, inclusive, and welcoming community.

---

## 🚀 Getting Started

### 1. Fork and Clone

```bash
# Fork the repository on GitHub, then clone your fork
git clone https://github.com/YOUR_USERNAME/fayd-protocol.git
cd fayd-protocol
```

### 2. Build

```bash
cd fayd-node
cargo build
```

### 3. Run Tests

```bash
cargo test
```

---

## 🙋 How to Contribute

### 🐛 Reporting Bugs

1. Check if the bug is already reported in [Issues](../../issues).
2. If not, create a new issue with:
   - Clear and descriptive title
   - Steps to reproduce
   - Expected vs actual behavior
   - Environment details (OS, Rust version, node version)

### 💡 Suggesting Features

1. Open an issue with a clear description of the feature.
2. Explain the use case and benefits.
3. Discuss with the community before implementing significant changes.

### 🔧 Submitting Code

1. Create a feature branch:
   ```bash
   git checkout -b feat/your-feature
   ```
2. Make your changes.
3. Add or update tests as needed.
4. Ensure all tests pass:
   ```bash
   cargo test
   ```
5. Commit with clear, conventional messages:
   ```bash
   git commit -m "feat: add resource monitoring to node"
   ```
6. Push and open a Pull Request.

### 📚 Improving Documentation

Documentation contributions are highly valued! You can:
- Fix typos and errors
- Add examples
- Improve clarity and structure
- Translate content to other languages

---

## 🛠️ Development Setup

### Prerequisites

- Rust 1.70 or later
- Git
- Basic familiarity with command line

### Project Structure

```
fayd-protocol/
├── fayd-node/     # Node implementation (Rust)
├── docs/          # Documentation
├── examples/      # Usage examples
└── ...
```

### Common Commands

```bash
# Build the node
cargo build

# Build release version
cargo build --release

# Run tests
cargo test

# Run the node
cargo run --release

# Format code
cargo fmt

# Check for linting issues
cargo clippy
```

---

## ✅ Pull Request Checklist

Before submitting a PR, ensure:

- [ ] Tests pass (`cargo test`)
- [ ] Code is formatted (`cargo fmt`)
- [ ] No clippy warnings (`cargo clippy`)
- [ ] Documentation is updated if needed
- [ ] Changes are minimal and focused
- [ ] Commit messages are clear and descriptive
- [ ] Related issues are referenced

---

## 💬 Community

- **Discussions**: Use GitHub Issues for technical discussions.
- **Questions**: Open an issue with `question` label.
- **Feedback**: We welcome constructive feedback on all aspects of the project.

---

## 🌟 Recognition

All contributors will be acknowledged. We value every contribution, whether it's code, documentation, bug reports, or community support.

---

**Thank you for contributing to Fayd Protocol!** 🌊

Together, we're building decentralized compute infrastructure for everyone.
```

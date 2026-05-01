# 🌊 Fayd Protocol

**Decentralized Compute Protocol for Verifiable AI Infrastructure**

[![Rust CI](https://github.com/madanimkhitar22-beep/fayd-protocol/actions/workflows/rust.yml/badge.svg)](https://github.com/madanimkhitar22-beep/fayd-protocol/actions/workflows/rust.yml)
[![License](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/Rust-1.70+-orange.svg)](https://www.rust-lang.org/)
[![Status](https://img.shields.io/badge/Status-Phase%200%20%7C%20Foundation-yellow.svg)](ROADMAP.md)
[![Contributions](https://img.shields.io/badge/Contributions-Welcome-brightgreen.svg)](CONTRIBUTING.md)

> **Turning idle devices into verifiable compute infrastructure.**
> 
> Mobile-first • Deterministic • Auditable • Open

[Manifesto](MANIFESTO.md) • [Architecture](docs/architecture.md) • [Roadmap](ROADMAP.md) • [Contributing](CONTRIBUTING.md)

</div>

---

## 📌 Overview

Fayd is an open protocol that enables any device — from phones to servers — to contribute idle compute to a decentralized network. It provides a foundation for verifiable, distributed AI workloads without centralized infrastructure.

### Core Properties

| Property | Description |
|----------|-------------|
| **Deterministic** | Tasks produce identical results across all nodes |
| **Verifiable** | Cryptographic proof of execution integrity |
| **Sandboxed** | WebAssembly isolation with resource bounds |
| **Accessible** | Runs on ARM/x86, Linux/Android/macOS/Windows |
| **Minimal** | <50MB binary, <100MB memory footprint |

---

## 🏗️ Architecture

```
┌─────────────────────────────────────────────────────────────────┐
│                           Fayd Network                                      │
│                                                                              │
│       ┌──────────┐     ┌──────────┐     ┌──────────┐                 │
│        │   Node   │ ◄──► │   Node   │ ◄──► │  Node    │                   │
│        │ (Phone)  │       │ (Laptop) │       │ (Server) │                 │
│       └────┬─────┘     └────┬─────┘     └────┬─────┘                  │
│              │                 │                │                          │
│              └──────────────┼──────────────┘                           │
│                                │                                           │
│                     ┌────────▼───────┐                                   │
│                       │   Discovery    │                                   │
│                       │     (DHT)      │                                    │
│                     └─────────────────┘                                 │
│                                                                             │
└─────────────────────────────────────────────────────────────────┘

┌─────────────────────────────────────────────────────────────────┐
│                                 Node Internals                              │
│                                                                              │
│  ┌─────────────────────────────────────────────────────────┐     │
│  │                           Fayd Node Runtime                        │   │
│  │                                                                     │   │
│  │        ┌──────────┐  ┌──────────┐  ┌─────────────────────┐   │   │
│  │        │  CLI       │  │ Config    │   │  Resource Monitor     │   │   │
│  │        └──────────┘  └──────────┘  └─────────────────────┘   │   │
│  │                                                                     │   │
│  │       ┌─────────────────────────────────────────────────┐   │   │
│  │       │                      Task Engine                        │   │   │
│  │       │      ┌──────────┐  ┌──────────┐  ┌──────────────┐ │   │   │
│  │       │        │ Scheduler│   │ Executor │     │  Verifier  │  │   │   │
│  │              └──────────┘  └──────────┘  └──────────────┘ │   │   │
│  │       └─────────────────────────────────────────────────┘  │   │
│  │                                                                      │   │
│  │      ┌─────────────────────────────────────────────────┐   │   │
│  │      │       Wasm Sandbox                                        │   │   │
│  │      │  • Isolated Execution                                    │   │   │
│  │      │  • Resource Bounds                                        │   │   │
│  │      │  • Deterministic Runtime                                 │   │   │
│  │      └─────────────────────────────────────────────────┘    │   │
│  └─────────────────────────────────────────────────────────┘    │
└─────────────────────────────────────────────────────────────────┘
```

See [Architecture Docs](docs/architecture.md) for detailed specifications.

---

## 🚀 Quick Start

### Prerequisites

- **Rust** 1.70 or later ([Install](https://www.rust-lang.org/tools/install))
- **Git** ([Install](https://git-scm.com/downloads))

### Build & Run

```bash
# Clone repository
git clone https://github.com/madanimkhitar22-beep/fayd-protocol.git
cd fayd-protocol/fayd-node

# Build release binary
cargo build --release

# Run node
./target/release/fayd-node
```

### Run with Options

```bash
# Custom configuration
cargo run --release -- \
    --name research-node \
    --max-cpu 40 \
    --max-memory 512
```

### Expected Output

```
🌊 Fayd Node Starting
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
Node ID     : fx_01a2b3c4d5e6f789
Name        : research-node
Version     : 0.1.0
Status      : idle
Max CPU     : 40%
Max Memory  : 512 MB
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
✓ Connected to Fayd test mesh
✓ Ready to receive tasks
```

---

## 🛠️ Technical Stack

| Layer | Technology | Rationale |
|-------|------------|-----------|
| **Language** | Rust | Memory safety, zero-cost abstractions |
| **Runtime** | WebAssembly | Portable sandboxing, multi-language |
| **Async** | Tokio | Battle-tested async runtime |
| **Crypto** | BLAKE3, Ed25519 | Fast hashing, secure signatures |
| **CLI** | clap | Industry-standard argument parsing |
| **Logging** | tracing | Structured, filterable logs |
| **Serialization** | serde | Efficient, type-safe serialization |
| **CI** | GitHub Actions | Automated quality gates |

---

## 📚 Documentation

| Document | Description |
|----------|-------------|
| [MANIFESTO](MANIFESTO.md) | Core beliefs, principles, and scope |
| [Architecture](docs/architecture.md) | Technical design, components, threat model |
| [Vision](docs/vision.md) | Problem statement and success metrics |
| [ROADMAP](ROADMAP.md) | Development phases and milestones |
| [FAQ](docs/faq.md) | Frequently asked questions |
| [CONTRIBUTING](CONTRIBUTING.md) | Contribution guidelines and setup |
| [Examples](examples/hello-node.md) | Usage examples and platform guides |

---

## 🤝 Contributing

Fayd is built by a global community of engineers and researchers.

### Good Starting Points

- Review [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines
- Check issues labeled [`good-first-issue`](https://github.com/madanimkhitar22-beep/fayd-protocol/issues?q=is:issue+is:open+label:good-first-issue)
- Join technical discussions in GitHub Issues

### Development Workflow

```bash
# Clone and build
git clone https://github.com/madanimkhitar22-beep/fayd-protocol.git
cd fayd-protocol/fayd-node
cargo build

# Run tests
cargo test

# Format and lint
cargo fmt
cargo clippy
```

---

## 📜 License

Fayd Protocol is open source under the [Apache 2.0 License](LICENSE).

---

## ⚠️ Status & Disclaimer

**Current Phase**: Phase 0 — Foundation

This repository is in **early development**. The protocol is not yet ready for production workloads. Features, APIs, and specifications may evolve based on community feedback and technical validation.

See [ROADMAP.md](ROADMAP.md) for development timeline.

---

<div align="center">

**Compute should be accessible, verifiable, and distributed.**

🌊 *From abundance, for everyone.*

</div>

# 👋 Hello Node Example

This example demonstrates how to build and run a Fayd node.

---

## 📋 Prerequisites

Before you begin, ensure you have the following installed:

- **Rust 1.70** or later ([Install Rust](https://www.rust-lang.org/tools/install))
- **Git** ([Install Git](https://git-scm.com/downloads))
- A terminal or command prompt

---

## 🚀 Quick Start

### 1. Clone the Repository

```bash
git clone https://github.com/madanimkhitar22-beep/fayd-protocol.git
cd fayd-protocol
```

### 2. Build the Node

```bash
cd fayd-node
cargo build --release
```

This will create an optimized binary in `target/release/fayd-node`.

### 3. Run the Node

```bash
cargo run --release
```

Or run the binary directly:

```bash
./target/release/fayd-node
```

---

## 📤 Expected Output

When the node starts successfully, you should see:

```
🌊 Fayd Node Starting
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
Node ID     : fx_01a2b3c4d5e6f789
Name        : anonymous
Version     : 0.1.0
Status      : idle
Max CPU     : 50%
Max Memory  : 512 MB
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
Connecting to Fayd test mesh...
✓ Connected to Fayd test mesh
✓ Ready to receive tasks

Press Ctrl+C to stop the node.
```

---

## ⚙️ Customization Options

### Set a Custom Name

Give your node a recognizable name:

```bash
cargo run --release -- --name my-phone-node
```

### Limit CPU Usage

Restrict the node to use only a portion of your CPU:

```bash
cargo run --release -- --max-cpu 30
```

### Limit Memory Usage

Set maximum memory allocation in MB:

```bash
cargo run --release -- --max-memory 256
```

### Enable Verbose Output

See detailed logs for debugging:

```bash
cargo run --release -- --verbose
```

### Combine Options

```bash
cargo run --release -- --name research-node --max-cpu 40 --max-memory 512
```

---

## 📱 Running on Mobile (Termux)

Fayd is designed to run on mobile devices. Here's how to run it on Android using Termux:

### 1. Install Termux

Download Termux from [F-Droid](https://f-droid.org/en/packages/com.termux/).

### 2. Install Dependencies

```bash
pkg update
pkg install rust git
```

### 3. Clone and Build

```bash
git clone https://github.com/madanimkhitar22-beep/fayd-protocol.git
cd fayd-protocol/fayd-node
cargo build --release
```

### 4. Run

```bash
cargo run --release -- --name termux-node --max-cpu 25
```

> 💡 **Tip**: Use lower CPU limits on mobile devices to preserve battery life.

---

## 🔍 Verifying the Build

### Check Binary Size

Fayd node is optimized for small size:

```bash
ls -lh target/release/fayd-node
```

Expected: Less than 50MB (varies by platform).

### Run Tests

```bash
cargo test
```

### Check for Linting Issues

```bash
cargo clippy
```

### Format Code

```bash
cargo fmt
```

---

## 🛑 Stopping the Node

To stop the node, press `Ctrl+C` in the terminal.

The node will:
- Stop accepting new tasks
- Complete any in-progress work (in future phases)
- Disconnect from the network gracefully
- Save state if needed (in future phases)

---

## 🐛 Troubleshooting

### Build Fails

- Ensure Rust is up to date: `rustup update`
- Check you're in the `fayd-node` directory
- Clear build cache: `cargo clean && cargo build`

### Node Won't Start

- Check for port conflicts (in future phases)
- Verify system meets minimum requirements
- Run with `--verbose` for detailed logs

### High Resource Usage

- Reduce `--max-cpu` value
- Reduce `--max-memory` value
- Ensure no other heavy processes are running

---

## 📚 Next Steps

- Read the [Architecture](../docs/architecture.md) to understand the protocol design.
- Check the [ROADMAP](../ROADMAP.md) for upcoming features.
- Explore [CONTRIBUTING](../CONTRIBUTING.md) to learn how to contribute.
- Open an [Issue](https://github.com/madanimkhitar22-beep/fayd-protocol/issues) for questions or feedback.

---

## 🤝 Need Help?

- Open an issue with your question.
- Include your OS, Rust version, and error messages.
- We're here to help!

---

<div align="center">

**Welcome to Fayd. Thank you for contributing to decentralized compute.** 🌊

</div>

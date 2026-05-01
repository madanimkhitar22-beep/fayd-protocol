# ❓ Fayd FAQ

Frequently asked questions about Fayd Protocol.

---

## 📋 Table of Contents

- [General](#general)
- [Technical](#technical)
- [Contributing](#contributing)
- [Security & Privacy](#security--privacy)
- [Future](#future)

---

## 🌍 General

### What is Fayd?

Fayd is an open protocol for decentralized, verifiable compute. It enables any device — from phones to servers — to contribute idle compute resources to a distributed network.

### Why "Fayd"?

"Fayd" (فيض) is an Arabic word meaning "abundance" or "overflow". It represents the abundant idle compute resources available worldwide, waiting to be utilized. The name reflects our belief that compute scarcity is a misallocation problem, not a resource problem.

### Is Fayd a cryptocurrency or token project?

**No.** Fayd is an infrastructure protocol focused on compute, verification, and coordination. We do not issue tokens, coins, or speculative assets.

While applications built on Fayd may implement their own incentive mechanisms, the protocol itself is agnostic to incentives and focused purely on technical infrastructure.

### What problems does Fayd solve?

| Problem | Fayd's Approach |
|---------|-----------------|
| Compute centralization | Distributed network of diverse devices |
| High infrastructure costs | Utilize idle resources at lower cost |
| Access inequality | Open to anyone with a capable device |
| Resource waste | Harness idle capacity worldwide |
| Trust in decentralized compute | Cryptographic verification and auditing |

### Who is Fayd for?

- **Contributors**: Anyone with a device who wants to contribute idle compute.
- **Consumers**: Researchers, developers, and organizations needing affordable compute.
- **Builders**: Engineers creating tools, SDKs, and applications on Fayd.
- **Advocates**: Educators and community members spreading awareness.

### How is Fayd different from other decentralized compute projects?
| Aspect | Fayd's Approach |
|--------|-----------------|
| Scope | Infrastructure protocol, not application or marketplace |
| Devices | Mobile-first; supports phones, laptops, servers, IoT |
| Verification | Multi-layer: deterministic, replication, ZK proofs |
| Incentives | Protocol-agnostic; no native token |
| Philosophy | Minimalism, auditability, accessibility |
| Governance | Community-driven, transparent decision-making |

---

## ⚙️ Technical

### What are the system requirements?

Fayd is designed to run on minimal hardware:

| Requirement | Minimum |
|-------------|---------|
| CPU | ARM or x86_64 |
| RAM | 512MB |
| Storage | 100MB free space |
| Network | Internet connection |
| OS | Linux, Android, macOS, Windows |

### What programming languages are supported?

- **Node**: Written in Rust for safety and performance.
- **Tasks**: Compiled to WebAssembly, supporting Rust, C, C++, Go, Zig, and more.
- **SDKs**: Planned for multiple languages in Phase 4.

### How does verification work?

Fayd uses multiple verification mechanisms:

| Mechanism | Description |
|-----------|-------------|
| Deterministic Execution | Tasks produce identical results on all nodes |
| Replication | Critical tasks executed by multiple nodes for comparison |
| Challenge-Response | Random audits challenge nodes to prove correct execution |
| Zero-Knowledge Proofs | (Phase 3) Cryptographic proof without revealing details |

Consumers choose verification level based on their trust requirements.

### What types of workloads can run on Fayd?

Fayd targets **deterministic, verifiable workloads**:

| Supported | Not Supported (Yet) ||-----------|---------------------|
| AI inference | Real-time streaming |
| Batch processing | Non-deterministic tasks |
| Distributed training shards | Interactive workloads |
| Scientific computation | GPU-only workloads (planned) |
| Data transformation | Stateful services |

### How is task sandboxing implemented?

Tasks execute in a **WebAssembly sandbox** with:

- ❌ No filesystem access
- ❌ No network access (unless explicitly allowed)
- ❌ No host system calls
- ✅ Deterministic execution
- ✅ Resource limits enforced (CPU, memory, time)

This ensures tasks cannot harm the host device or access sensitive data.

### Will Fayd support GPU acceleration?

GPU support is **planned for a future phase** after core protocol stability. Initial releases focus on CPU-based workloads to maximize device accessibility.

### How does Fayd handle network connectivity issues?

- Nodes can reconnect without losing identity.
- Tasks have deadlines and can be reassigned.
- Results are cached locally until transmission succeeds.
- Protocol tolerates intermittent connectivity.

---

## 🤝 Contributing

### How can I contribute?

We welcome contributions of all kinds:

- **Code**: Implement features, fix bugs, improve performance.
- **Documentation**: Fix errors, add examples, improve clarity.
- **Testing**: Report bugs, verify fixes, test on diverse devices.
- **Community**: Help others, spread awareness, organize events.
- **Research**: Propose verification mechanisms, analyze security.

See [CONTRIBUTING.md](../CONTRIBUTING.md) for detailed guidelines.

### I found a bug. What should I do?

1. Check existing [Issues](https://github.com/madanimkhitar22-beep/fayd-protocol/issues) to see if it's already reported.
2. If not, open a new issue with:   - Clear description
   - Steps to reproduce
   - Expected vs actual behavior
   - Environment details (OS, Rust version, device)

### Can I suggest new features?

Yes! Open an issue with:
- Description of the feature
- Use case and benefits
- Implementation considerations (if any)

Discuss with the community before implementing significant changes.

### Do you accept translations?

Yes! Documentation translations are welcome. Open an issue to propose a new language, and we'll coordinate the effort.

---

## 🔒 Security & Privacy

### Is my device safe when running a Fayd node?

Yes. The node is designed with security as a priority:

- Tasks run in isolated WebAssembly sandbox.
- No access to host filesystem or sensitive data.
- Resource limits prevent abuse.
- Node operator controls maximum resource usage.
- Open source code can be audited by anyone.

### Can tasks access my personal data?

**No.** Tasks execute in a sandbox with:
- No filesystem access
- No access to environment variables
- No access to device identifiers
- No network access (unless explicitly configured)

### How is my privacy protected as a contributor?

- Node identity is pseudonymous (cryptographic ID).
- No personal information is required.
- Resource advertisements reveal only compute capabilities.
- Audit records contain only task metadata, not contributor identity.

### How is consumer data protected?

- Task inputs can be encrypted.- Zero-knowledge verification (Phase 3) allows verification without revealing inputs.
- Consumers control data retention and deletion.

### Has Fayd been security audited?

Security audits are planned for **Phase 3**. Until then, the protocol is in early development and should not be used for production workloads.

---

## 🔮 Future

### What's the timeline for development?

See [ROADMAP.md](../ROADMAP.md) for detailed phases and targets.

| Phase | Focus | Target |
|-------|-------|--------|
| Phase 0 | Foundation | Q2 2026 |
| Phase 1 | Local Execution | Q3 2026 |
| Phase 2 | Distributed Coordination | Q4 2026 |
| Phase 3 | Verification & Security | Q1 2027 |
| Phase 4 | Open Ecosystem | Q2 2027 |

### Will Fayd have a governance model?

A community-driven governance model will be proposed in **Phase 4**. We prioritize transparency, inclusivity, and technical merit in decision-making.

### How can organizations get involved?

- Test Fayd for your compute workloads.
- Sponsor development through grants.
- Contribute engineering resources.
- Partner on ecosystem integrations.

Open an issue to start a conversation.

### Where can I learn more?

| Resource | Link |
|----------|------|
| Manifesto | [MANIFESTO.md](../MANIFESTO.md) |
| Architecture | [architecture.md](architecture.md) |
| Vision | [vision.md](vision.md) |
| Roadmap | [../ROADMAP.md](../ROADMAP.md) |
| Contributing | [../CONTRIBUTING.md](../CONTRIBUTING.md) |
| Repository | [github.com/madanimkhitar22-beep/fayd-protocol](https://github.com/madanimkhitar22-beep/fayd-protocol) |

---

## 💬 Still Have Questions?
- Open an issue with your question.
- Start a discussion in GitHub Discussions (when enabled).
- Review existing issues for related topics.

---

<div align="center">

**Clarity builds trust. Questions build understanding.** 🌊

</div>

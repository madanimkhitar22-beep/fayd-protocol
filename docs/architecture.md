# 🏗️ Fayd Architecture

This document describes the technical architecture of Fayd Protocol. It serves as the foundational reference for implementation and contributions.

---

## 📋 Table of Contents

- [Overview](#overview)
- [Design Principles](#design-principles)
- [Core Components](#core-components)
- [Task Model](#task-model)
- [Verification Model](#verification-model)
- [Network Topology](#network-topology)
- [Security Model](#security-model)
- [Technical Stack](#technical-stack)
- [What Fayd Does Not Do](#what-fayd-does-not-do)
- [Future Directions](#future-directions)

---

## 🔍 Overview

Fayd is a decentralized protocol for distributed compute. It enables any device — from phones to servers — to contribute idle compute resources to a verifiable network.

The architecture prioritizes:

1. **Simplicity**: Minimal components with clear responsibilities.
2. **Verifiability**: Every computation can be independently verified.
3. **Accessibility**: Runs on resource-constrained devices.
4. **Security**: Sandboxed execution with cryptographic guarantees.
5. **Privacy**: Contributors and consumers retain data control.

---

## 🧭 Design Principles

| Principle | Implication |
|-----------|-------------|
| **Determinism over prediction** | Tasks must produce identical results across all nodes |
| **Verification over trust** | No node is trusted; all execution is verified |
| **Boundaries over scale** | Clear limits on task scope and resource usage |
| **Auditability over convenience** | Every action leaves a verifiable trail |
| **Minimalism over features** | Each component does one thing well |

---

## 🧱 Core Components

### 1. Fayd Node

The node is the fundamental unit of the network, running on contributor devices.

**Responsibilities**:
- Advertise available compute resources (CPU, memory, battery).
- Receive and execute tasks within sandboxed environment.
- Generate execution proofs and audit metadata.
- Return results to consumers or aggregators.

**Design Constraints**:
| Constraint | Target |
|------------|--------|
| Binary size | < 50MB |
| Memory footprint | < 100MB idle |
| Battery impact | < 3% when active |
| Storage | < 100MB |
| Platforms | ARM/x86_64, Linux/Android/macOS/Windows |

**Node States**:
```
Offline → Starting → Idle → Busy → Verifying → Idle
```

### 2. Task

A task is a unit of compute work submitted to the network.

**Properties**:
- **Deterministic**: Same input always produces same output.
- **Sandboxed**: Executes in isolated WebAssembly environment.
- **Bounded**: Has defined resource limits (CPU, memory, time).
- **Verifiable**: Result can be cryptographically verified.
- **Serializable**: Can be encoded, transmitted, and reconstructed.

**Task Structure**:
```
Task {
    id: Hash,
    spec: TaskSpec,
    input: Data,
    constraints: ResourceBounds,
    deadline: Timestamp,
}
```

**Task Lifecycle**:
```
Submitted → Validated → Routed → Executed → Verified → Completed
```

### 3. Verification Engine

Ensures compute integrity without trusting individual nodes.

**Mechanisms**:

| Mechanism | Description | Phase |
|-----------|-------------|-------|
| Deterministic Execution | Tasks produce identical results on all nodes | Phase 1 |
| Replication | Critical tasks executed by multiple nodes for comparison | Phase 2 |
| Challenge-Response | Random audits challenge nodes to prove correct execution | Phase 3 |
| Zero-Knowledge Proofs | Cryptographic proof of execution without revealing details | Phase 3 |

### 4. Scheduler

Coordinates task distribution across the network.

**Responsibilities**:
- Match tasks to capable nodes based on requirements.
- Balance load across available resources.
- Handle failures and reassignments.
- Optimize for latency, cost, and verification needs.

**Scheduling Factors**:
- Node capability (CPU, memory, architecture)
- Node reputation and reliability
- Geographic distribution (for replication)
- Resource availability and constraints

---

## 📦 Task Model

### Task Types

| Type | Description | Use Cases |
|------|-------------|-----------|
| **Inference** | Run model inference on input data | LLM inference, classification |
| **Batch Compute** | Deterministic batch processing | Data transformation, hashing |
| **Training Shard** | Distributed training subtask | Federated learning, gradient computation |
| **Verification** | Verify another node's execution | Cross-checking, audits |

### Resource Bounds

Every task must declare resource limits:

```
ResourceBounds {
    max_cpu_cores: u32,
    max_memory_mb: u64,
    max_duration_sec: u64,
    requires_gpu: bool,
}
```

Nodes reject tasks exceeding their capabilities.

---

## 🔐 Verification Model

### Verification Levels

| Level | Guarantee | Cost |
|-------|-----------|------|
| **None** | No verification; trusted execution | Lowest |
| **Replication** | Result matches across N nodes | Medium |
| **Challenge** | Random audits with penalties | Medium-High |
| **ZK Proof** | Cryptographic proof of correctness | Highest |

Consumers choose verification level based on trust requirements.

### Audit Trail

Every task execution produces an audit record:

```
AuditRecord {
    task_id: Hash,
    node_id: Hash,
    input_hash: Hash,
    output_hash: Hash,
    execution_time: Duration,
    resource_usage: ResourceMetrics,
    proof: VerificationProof,
    timestamp: Timestamp,
}
```

Audit records are immutable and independently verifiable.

---


## 🌐 Network Topology

Fayd uses a hybrid peer-to-peer topology:

```
┌─────────────────────────────────────────────────────────┐
│                    Fayd Network                                  │
│                                                                    │
│  ┌──────────┐    ┌──────────┐    ┌──────────┐              │
│  │  Node    │◄──►│  Node    │◄──►│  Node    │                   │
│  │ (Phone)  │    │ (Laptop) │    │ (Server) │                   │
│  └──────────┘    └──────────┘    └──────────┘               │
│       ▲               ▲               ▲                        │
│       │               │               │                           │
│       └───────────────┼───────────────┘                    │
│                       │                                           │
│              ┌────────▼────────┐                             │
│              │   Discovery     │                                  │
│              │     Layer       │                                 │
│              │     (DHT)       │                                  │
│              └─────────────────┘                              │
│                                                                    │
└─────────────────────────────────────────────────────────┘
```

**Layers**:

| Layer | Protocol | Purpose |
|-------|----------|---------|
| Discovery | libp2p Kademlia DHT | Node discovery and routing |
| Transport | QUIC/TCP | Reliable peer communication |
| Coordination | Custom | Task routing and aggregation |
| Verification | Custom | Proof exchange and validation |

---

## 🛡️ Security Model

### Threat Model

| Threat | Mitigation |
|--------|------------|
| Malicious node returns wrong result | Verification, replication, reputation |
| Node executes harmful code | WebAssembly sandboxing, resource limits |
| Sybil attack (fake nodes) | Reputation system, proof-of-work for registration |
| Data leakage | Encrypted inputs, zero-knowledge verification |
| Denial of service | Rate limiting, reputation penalties |
| Replay attack | Nonce-based task IDs, timestamp validation |

### Sandboxing

Tasks execute in WebAssembly sandbox with:

- ❌ No filesystem access
- ❌ No network access (unless explicitly allowed)
- ❌ No host system calls
- ✅ Deterministic execution
- ✅ Resource limits enforced

---

## 🛠️ Technical Stack

| Layer | Technology | Rationale |
|-------|------------|-----------|
| Language | Rust | Memory safety, performance, no GC |
| Runtime | WebAssembly | Portable, sandboxed, multi-language |
| Networking | libp2p | Battle-tested P2P primitives |
| Cryptography | BLAKE3, Ed25519 | Fast, secure, widely supported |
| Serialization | serde, MessagePack | Efficient, schema-less |
| Build | Cargo | Standard Rust tooling |

---

## ❌ What Fayd Does Not Do

| Area | Fayd's Position |
|------|-----------------|
| Storage | Fayd is compute-only; integrate with storage protocols (IPFS, Arweave) |
| Token/Currency | Fayd is infrastructure; incentives are implementation detail |
| General Computing | Fayd targets verifiable, deterministic workloads |
| Real-time | Fayd is for batch and async workloads |
| GPU-Only | Fayd supports CPU-first; GPU support planned for future |
| Centralized Orchestration | Fayd is fully decentralized; no central coordinator |

---

## 🔮 Future Directions

### Near-Term (Phases 1-2)

- WebAssembly task execution
- Local verification primitives
- Peer discovery and routing
- Basic reputation system

### Mid-Term (Phases 3-4)

- Zero-knowledge proof integration
- GPU acceleration support
- Mobile-optimized inference
- Cross-platform SDKs

### Long-Term

- Formal verification of core protocol
- Federated learning support
- Energy-aware scheduling
- Institutional adoption pathways

---

## 📚 References

- [MANIFESTO](../MANIFESTO.md) — Core beliefs and principles
- [ROADMAP](../ROADMAP.md) — Development phases
- [Vision](vision.md) — The why behind Fayd
- [FAQ](faq.md) — Frequently asked questions

---

<div align="center">

**Architecture is the foundation. Clarity is the guide.** 🌊

</div>

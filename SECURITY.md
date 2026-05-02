# 🔒 Security Policy

## 📌 Reporting a Vulnerability

Security is a top priority for Fayd Protocol. If you discover a security vulnerability, please report it responsibly to help us protect the network and its users.

### ✅ How to Report

**Do NOT open a public issue** for security vulnerabilities.

Instead, choose one of the following private channels:

1. **GitHub Security Advisory**
   - Go to the [Security tab](https://github.com/madanimkhitar22-beep/fayd-protocol/security)
   - Click "Report a vulnerability"
   - Submit your report privately

2. **Email**
   - Send details to: `security@fayd.sh` (placeholder — update when domain is ready)
   - Use subject: `[SECURITY] Brief description of vulnerability`

### 📋 What to Include

Please provide as much detail as possible:

- Description of the vulnerability
- Steps to reproduce
- Potential impact
- Affected versions/components
- Suggested mitigation (if any)
- Your contact information for follow-up

---

## ⏱️ Response Timeline

| Phase | Timeline |
|-------|----------|
| **Acknowledgment** | Within 48 hours |
| **Initial Assessment** | Within 5 days |
| **Fix Development** | Within 14 days (depending on severity) |
| **Public Disclosure** | After fix is released and users have upgraded |

We will keep you informed throughout the process.

---

## 🛡️ Scope

### In Scope
| Component | Description |
|-----------|-------------|
| `fayd-node` | Node runtime and CLI |
| Task Execution | Wasm sandboxing, resource isolation |
| Verification | Proof generation, audit mechanisms |
| Cryptography | Key management, signatures, hashing |
| Network | Peer discovery, communication protocols |

### Out of Scope

| Area | Reason |
|------|--------|
| Third-party dependencies | Report to upstream maintainers |
| Social engineering | Outside protocol scope |
| Physical attacks | Assumes secure host environment |
| Denial of service (non-protocol) | Resource limits are configurable |

---

## 🏆 Recognition

We appreciate and acknowledge security researchers who help improve Fayd:

- Contributors will be credited in security advisories (with permission).
- Significant findings may be highlighted in release notes.
- We value responsible disclosure and collaborative improvement.

---

## 📜 Best Practices for Users

### Running a Node

- Keep your node updated to the latest version.
- Run with minimal privileges.
- Monitor resource usage and logs.
- Use firewall rules to restrict unnecessary access.

### Submitting Tasks

- Verify task inputs and outputs.
- Use appropriate verification levels for your trust requirements.
- Encrypt sensitive data before submission.

---

## 🔗 Related Documents

- [Architecture](docs/architecture.md) — Security model and threat analysis- [FAQ](docs/faq.md) — Security and privacy questions
- [ROADMAP](ROADMAP.md) — Security milestones

---

<div align="center">

**Security is a process, not a product. Thank you for helping keep Fayd secure.** 🛡️

</div>

# ottyp

A highly **Work In Progress**, zero-telemetry CLI for managing and generating Time-Based One-Time Passwords (TOTP). Built in Rust, ottyp delegates secret storage to your OS's native secure enclave (macOS Keychain, Linux Secret Service/KWallet, Windows Credential Manager) and executes token generation entirely in-memory.

## Features & Security

- **OS Keyring Integration**: No plaintext secrets on disk; leverages native secure storage.
- **In-Memory Cryptography**: Tokens are computed instantly and discarded; secrets never touch swap or logs.
- **Algorithm Agnostic**: Full support for SHA1, SHA256, and SHA512.
- **Configurable Constraints**: Granular control over TOTP step sizes (e.g., 30s/60s) and token lengths.
- **Workflow Optimized**: Automatically provisions generated tokens to the system clipboard.
- **Terminal Native**: Dependency-free UI using structured, responsive terminal tables.

## Installation

Requires the **[cargo](https://rustup.rs/)** toolchain.

```bash
git clone https://github.com/daglaroglou/ottyp.git && cd ottyp
cargo install --path .
```

## CLI Reference

| Command | Syntax | Description |
| :--- | :--- | :--- |
| **Add** | `ottyp add <name> <secret> <alg> <digits> <step>` | Provisions a new TOTP secret (e.g. `SHA256 6 30`) |
| **Get** | `ottyp get <name>` | Evaluates token and copies directly to clipboard |
| **List** | `ottyp list` | Renders a tabulated view of all provisioned accounts |
| **Remove** | `ottyp rm <name>` | Permanently purges the secret from the OS keyring |

*Example Provisioning:*
```bash
ottyp add AWS-Prod JBSWY3DPEHPK3PXP SHA256 6 30
```

## Roadmap

- [ ] **Core**: Interactive TUI provisioning, QR code decoding via image processing, fuzzy finding for fast retrieval.
  - [ ] General UI polish
- [ ] **Interoperability**: Standardized `otty://` URI import/export
  - [ ] Steam Guard 5-char alphanumeric support.
  - [ ] HOTP support
- [ ] **Online Backup**: Save encrypted TOTP keys into cloud services
  1. [ ]  Google Drive
  2. [ ] OneDrive
  3. [ ] TeraBox
  4. [ ] Mega
  5. [ ] Custom FTP server 

## License

MIT License. See `LICENSE` for details.

[![No AI](https://custom-icon-badges.demolab.com/badge/No%20AI-2f2f2f?logo=non-ai&logoColor=white)](#)
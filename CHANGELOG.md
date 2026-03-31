# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/).

## [0.4.0] - 2026-03-31

### Added

- **xorion-sdk v0.4.0** — Multi-chain HD wallet SDK
  - BIP-39 mnemonic generation and recovery
  - BIP-44 hierarchical deterministic key derivation
  - Ethereum address derivation (secp256k1 + Keccak-256)
  - Solana address derivation (Ed25519)
  - Async JSON-RPC providers for Ethereum and Solana
  - ABI encoder/decoder for smart contract interaction
  - ERC-20 and ERC-721 token interfaces
  - Uniswap V2 router integration
  - WalletClient unified kernel API

- **xorion-core v0.1.0** — WASM dApp Runtime
  - Wasmtime 19 sandboxed execution engine
  - Permission-based sandbox (network, filesystem, wallet)
  - WalletBridge host functions for WASM guests
  - IPFS-based dApp loader

- **xorion-zk v0.1.0** — zk-SNARKs Privacy Layer
  - Groth16 proving system on BN254 curve (arkworks)
  - Private transaction circuit (balance >= amount)
  - Age verification circuit (age >= threshold)
  - Balance range proof circuit
  - Proof serialization and caching with TTL

- **xorion-ipfs v0.1.0** — IPFS Storage
  - IPFS HTTP API client (add, cat, pin)
  - AES-256-GCM client-side encryption with Argon2id key derivation
  - Virtual filesystem over IPFS CIDs
  - Pin management and disk-backed LRU cache

- **xorion-governance v0.1.0** — DAO Governance
  - Proposal lifecycle state machine (Pending -> Active -> Succeeded/Defeated -> Queued -> Executed)
  - Token-weighted voting (For/Against/Abstain) with quorum
  - Voting power delegation with circular delegation prevention
  - Treasury management with spending limits
  - Timelock execution delay enforcement
  - OpenZeppelin Governor ABI encoding

- **xorion-scheme v0.1.0** — Redox OS `wallet:/` scheme daemon
- **xorion-gui v0.1.0** — Dioxus desktop wallet GUI
- Cross-crate integration tests (14 tests)
- GitHub Actions CI/CD (test, docs, clippy, multi-platform release)

### Crates Published

| Crate | Version | crates.io |
|-------|---------|----------|
| xorion-sdk | 0.4.0 | [link](https://crates.io/crates/xorion-sdk) |
| xorion-core | 0.1.0 | [link](https://crates.io/crates/xorion-core) |
| xorion-governance | 0.1.0 | [link](https://crates.io/crates/xorion-governance) |
| xorion-ipfs | 0.1.0 | [link](https://crates.io/crates/xorion-ipfs) |
| xorion-zk | 0.1.0 | [link](https://crates.io/crates/xorion-zk) |

### Stats

- 229 tests passing across all crates
- 10 development phases complete
- 7 workspace crates

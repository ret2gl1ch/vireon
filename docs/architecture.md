Vireon — Architecture Overview

Purpose

Vireon is a modular, secure, and extensible biometric authentication system written in idiomatic Rust 2024. It provides authentication via facial recognition and integrates with system-level authentication mechanisms (PAM), while exposing modern APIs (REST/gRPC/WebSocket) for third-party applications.

Core Design Principles

Modularity: Every responsibility is encapsulated in a dedicated crate.

Security: Encryption at rest, sandboxed model execution, safe error handling.

Extensibility: Pluggable architecture for models, engines, storage backends.

Idiomatic Rust: Traits, lifetimes, ownership, async-await, clear error handling.

Open Source Ready: CI/CD, good documentation, contribution guide, test coverage.

Crate Structure

1. vireon-core

Core logic and types: facial vectors, verification pipeline, algorithm traits.

2. vireon-storage

Encrypted storage abstraction layer for biometric vectors and metadata.

3. vireon-engine

Sandboxed execution of biometric models. Executes detection and recognition.

4. vireon-pam

PAM integration crate to enable login via facial recognition.

5. vireon-daemon

Background service that manages devices, model orchestration, and local API access.

6. vireon-gateway

Public-facing REST, gRPC, and WebSocket API gateway.

7. vireon-cli

Administrative tool for system administrators and developers.

8. vireon-common

Shared types, traits, error definitions, logging setup.


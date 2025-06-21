# Vireon Architecture Overview

Vireon is modular Face ID system for Linux written in Rust.
Its goals are:
- Privacy-preserving biometric auth;
- Secure storage of face embedding;
- Extensibility via modules;
- PAM integration for login and sudo;
- Fill local processing, no cloud;

## Modules
- vireon-core: Shared traits and types
- vireon-engine: Face embedding generation
- vireon-storage: Encrypted vector storage
- vireon-daemon: Auth daemon (IPC interface)
- vireon-cli: User interface CLI 
- vireon-pam: PAM auth module

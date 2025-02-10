# zero-knowledge-proof

Smart Contract Development with RISC Zero Integration

![lock2.jpg](./lock2.jpg)

## Overview

This project lets you create and verify zero-knowledge proofs using both blockchain and regular web technologies.

TODO: Write a comprehensive overview of the project.

## References

- [RISC Zero Ethereum](https://github.com/risc0/risc0-ethereum) - RISC Zero's Ethereum integration
- [RISC Zero API Documentation](https://dev.risczero.com/api) - Official RISC Zero API docs
- [Alloy](https://github.com/alloy-rs) - Rust Ethereum development framework
- [Anvil](https://book.getfoundry.sh/anvil/) - Local Ethereum node for development and testing

## Documentation

- [Local Setup](./INSTALLATION.md) - How to setup the project
- [Contracts](./CONTRACTS.md) - The smart contracts that run on the blockchain.
- [Rust API Documentation](./API.md) - API Documentation
- [Lint, Test, & Build](./BUILD.md) - How to lint, run unit tests, and build the project
- [Deployment & Monitoring](./DEPLOYMENT.md) - How to deploy and monitor the project
- [Functional Tests](./TESTING.md) - How to run the functional tests once deployed

## Project Tree

```bash
/
├── circuit/
│ ├── src/
│ │ ├── auth.rs # The Rust code for handling authentication.
│ │ ├── contracts.rs # The Rust code for managing smart contract interactions and deployments.
│ │ ├── main.rs # Defines the Web Server, logging, CORS, etc.
│ │ ├── routes.rs # Defines the routing logic for the web server, mapping endpoints to their respective handlers.
│ │ ├── files.rs # Defines the logic for handling common file operations.
│ │ ├── security.rs # Implements verification key management.
│ │ ├── telemetry.rs # Implements OpenTelemetry for tracing and monitoring.
│ │ ├── proof.rs # Contains the logic for generating and verifying zero-knowledge proofs.
│ ├── Cargo.toml
│ ├── foundry.toml 
│ ├── ZkManager.sol # The smart contract for managing the zero-knowledge proofs
│ ├── ZkVerifier.sol # The smart contract for verifying the zero-knowledge proofs
│ ├── Dockerfile # Dockerfile for the Rust API
├── infra/
│ ├── grafana/ # Grafana configuration for monitoring
│ ├── prometheus.yml # Prometheus configuration for monitoring
│ ├── start-anvil.sh # Script to start the local Anvil blockchain
│ ├── Dockerfile # Dockerfile for the Anvil container
```

![lock1.jpg](./lock1.jpg)

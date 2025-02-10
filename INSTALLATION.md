# Installation Guide

## Prerequisites

- [RISC Zero CLI](https://docs.risc0.com/risc0-cli/installation/)
- [Docker](https://docs.docker.com/get-docker/)

## Installation

1. Clone the repository:

```bash
git clone https://github.com/MartinCastroAlvarez/zero-knowledge-proof.git
cd zero-knowledge-proof
```

2. Start all the services using Docker Compose:

This will start a local Ethereum network using Anvil, the Rust API server, and Graphana.

```bash
docker compose up -d
```

3. Validate that the services are running:

```bash
docker compose ps
```

4. Verify that the API is running:

```bash
curl http://localhost:3030/health
```

5. Verify that Graphana is running:

```bash
open http://localhost:3000
```

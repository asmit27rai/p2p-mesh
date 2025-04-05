# 🌐 P2P Mesh Network Simulation

A Rust implementation of a decentralized peer-to-peer (P2P) network with simulated share propagation across nodes.

[![Rust](https://img.shields.io/badge/Rust-1.70%2B-orange?logo=rust)](https://www.rust-lang.org/)
[![Tokio](https://img.shields.io/badge/Tokio-1.0-blue?logo=tokio)](https://tokio.rs/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

## 📌 Repository Description
**Simulates a 10-node P2P mesh network** where nodes periodically generate and propagate "shares" (messages) to all connected peers with configurable latency. Built with Rust's async runtime (Tokio) for high-performance networking.

Key Features:
- Full mesh topology (every node connects to all others)
- Configurable message latency simulation
- Structured logging with `tracing`
- Dockerized for easy deployment

## 🚀 Quick Start

### Prerequisites
- Docker ([Install Guide](https://docs.docker.com/get-docker/))
- Rust (optional, for local development)

### Running with Docker
```bash
# Build and run the simulation
docker build -t p2p-mesh .
docker run -it --rm p2p-mesh
```

### Example Output
```bash
Node 0 listening on 127.0.0.1:8000
Node 1 listening on 127.0.0.1:8001
...
Node 3 received share from Node 7: "Share from Node 7"
```

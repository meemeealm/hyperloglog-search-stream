
# HLL Search Stream Engine 🚀

A hyper-fast, memory-efficient, and concurrent data engineering pipeline built in Rust to track real-time unique search query metrics using the **HyperLogLog (HLL)** algorithm.

This system is decoupled using an event-driven architecture, processing millions of incoming logs asynchronously while protecting user data privacy by design.

---

## 1. System Architecture

Unlike traditional analytics setups that rely on massive memory-hogging `HashSets` or database lookups, this engine processes data through a decoupled streaming pipeline.

### Key Engineering Features:
* **Decoupled I/O (Producer-Consumer):** The web layer (`axum`) drops payloads into an asynchronous `tokio::sync::mpsc` channel and returns an instant response, eliminating ingestion bottlenecks.
* **Fixed-Memory Footprint:** Uses a register size of $b=12$ ($4,096$ buckets), restricting memory consumption to a flat $4\text{ KB}$ per unique search term—regardless of whether it tracks 100 or 100,000,000 unique users.
* **GDPR Compliant by Design:** Raw user identities are instantly converted into 32-bit hashes via `MurmurHash3` to count leading-zero streaks. Raw `user_id` values are immediately discarded from memory.

---

## 2. Project Layout

The system is highly modularized to respect clean domain boundaries:

```text
├── Cargo.toml
├── .gitignore
├── src/
│   ├── main.rs          # HTTP Orchestrator, Routing, & API endpoints
│   ├── pipeline.rs      # Stream Logistics, Queue Channels, & Background Worker
│   └── hll.rs           # Core Mathematical Logic & Cardinality Estimation Engine
└── stress_test.sh       # Concurrency and High-Throughput Validation Script
```
---

## Execution

1. Clone the repository and navigate to the project directory.
2. Boot the web service and processing engine:
```text
cargo run
```

The server will spin up on http://127.0.0.1:3000


## For Stress Test

```text
chmod +x stress_test.sh   
./stress_test.sh
```

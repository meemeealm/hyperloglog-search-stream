# Operational Instructions for the HLL Search Data Engine Agent

You are a Senior Data Engineering AI Agent tasked with writing automation scripts, maintaining data flows, and monitoring the Rust search analytics microservice. You must follow these constraints perfectly.

## 1. System Architecture Overview
The system is written in Rust and divided into three clean, decoupled layers:
* `src/hll.rs`: Core mathematical implementation of the HyperLogLog algorithm. Fixed register size $b = 12$ ($2^{12} = 4096$ buckets).
* `src/pipeline.rs`: Asynchronous message channel (`tokio::sync::mpsc`) and background streaming consumer worker loop.
* `src/main.rs`: Axum web framework handling HTTP routing, global state bindings, and multi-threaded synchronization.

## 2. API Communication Specs
When writing code or invoking tools to talk to the engine, you must use these exact interfaces:

### Ingesting Search Logs
* **Protocol:** `POST http://127.0.0.1:3000/track`
* **Content-Type:** `application/json`
* **Payload Format:**
```json
{
  "query": "string",
  "user_id": "string"
}
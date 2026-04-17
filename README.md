# Fetis 🚧 (Under Active Development)

**Fetis** is a blazing-fast, asynchronous, in-memory key-value data store built from scratch in Rust.

Currently in active development, this project serves as a lightweight, high-performance alternative for caching and shared state management in distributed applications and multiplayer game backends.

## ⚠️ Project Status: Work in Progress

Fetis is currently undergoing a rapid development sprint. The core networking loop and execution engine are being actively built. It is not yet ready for production use.

## 🛠️ Tech Stack

* **Language:** Rust
* **Runtime:** Tokio

## 📝 Planned MVP Operations (The Contract)

Fetis communicates over raw TCP. The initial milestone will support the following commands:

| Command | Description | Example |
| :--- | :--- | :--- |
| `PING` | Health check. Server responds with `PONG`. | `PING\r\n` |
| `SET` | Stores a string value under a specific key. | `SET username Countless\r\n` |
| `GET` | Retrieves the value associated with a key. | `GET username\r\n` |
| `DEL` | Deletes a key-value pair from memory. | `DEL username\r\n` |

## 🚀 Getting Started (Coming Soon)

Build and run instructions will be provided once the MVP command parser and memory hashmap are fully integrated.

---
*Built with precision for high-concurrency systems.*

<div align="center">
  
# ⚡ FETIS
  
  **A blazing-fast, asynchronous, in-memory key-value data store built from scratch in Rust.**

  [![Rust](https://img.shields.io/badge/rust-safe%20%26%20fast-orange.svg?style=for-the-badge&logo=rust)](https://www.rust-lang.org/)
  [![Tokio](https://img.shields.io/badge/tokio-async%20I%2FO-black.svg?style=for-the-badge)](https://tokio.rs)
  [![Status](https://img.shields.io/badge/status-MVP%20Operational-brightgreen.svg?style=for-the-badge)](#-project-status-mvp-operational)
  [![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg?style=for-the-badge)](LICENSE)

  <br>
</div>

Designed as a lightweight, high-performance alternative for caching and shared state management, **Fetis** is built to handle massive concurrency for distributed applications and multiplayer game backends.

## 🚀 Project Status: MVP Operational

The core engine for Fetis is fully functional. It successfully handles highly concurrent TCP connections, custom text-protocol parsing, and thread-safe memory mutation.

## 🛠️ Architectural Stack

* **Language:** Rust (Strictly safe, memory-leak-free concurrency)
* **Networking:** Tokio (Work-stealing async I/O via `epoll`)
* **Storage Engine:** `Arc<RwLock<HashMap>>` (Permits infinite simultaneous readers and safely isolated writes)

## 📝 Supported Commands (The Contract)

Fetis communicates over raw TCP. The engine currently supports the following operations:

| Command | Description | Example |
| :--- | :--- | :--- |
| `PING` | Health check. Server responds with `PONG`. | `PING\r\n` |
| `SET` | Stores a string value under a specific key. | `SET username Countless\r\n` |
| `GET` | Retrieves the value associated with a key. | `GET username\r\n` |
| `GET all` | *Diagnostic:* Dumps the entire server state. | `GET all\r\n` |
| `DEL` | Deletes a key-value pair from memory. | `DEL username\r\n` |

## 💻 Getting Started

### 1. Start the Server

Clone the repository and run the server using Cargo. By default, Fetis binds to `127.0.0.1:8080`.

```bash
cargo run
```

*Optional: You can provide a custom port as a command-line argument:*

```bash
cargo run 9000
```

### 2. Connect the Client

Because Fetis uses a custom raw text protocol, you can interact with it using Netcat (`nc`).

*Note: Use the `-C` flag (OpenBSD netcat) to ensure your terminal sends the required `\r\n` line endings.*

```bash
nc -C 127.0.0.1 8080
```

### 3. Execute Commands

Once connected, send commands directly to the engine:

```text
SET studio Countless
Done

GET studio
Countless

PING
PONG
```

---

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

---
<div align="center">
  <i>Built with ❤ and precision for high-concurrency systems by Pratyoosh.</i>
</div>

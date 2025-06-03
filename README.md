# Izoli

A lightweight Linux container/sandbox implementation written in Rust.

## What is Izoli?

Izoli creates isolated execution environments using Linux namespaces and cgroups v2. It's designed for process sandboxing and resource management.

## Features

- Process isolation (PID, UTS, IPC, Mount namespaces)
- Resource limits (CPU, memory, process count)
- Filesystem isolation with chroot
- Optional network isolation

## Requirements

- Linux with cgroups v2 support
- Root privileges
- Rust 1.70+ (for building)

## Installation

```bash
git clone <repository-url>
cd izoli
cargo build --release
```

## Usage

### CLI
```bash
sudo ./target/release/izoli
```

This creates an isolated bash shell with:
- 1GB memory limit
- CPU limited to one core
- Maximum 10 processes
- Read-only access to system directories

## License

MIT License - see [LICENSE](LICENSE) file.

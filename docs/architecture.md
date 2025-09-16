# Project Architecture: Port Scanner

## Project Definition

This project is a multi-threaded command-line port scanner written in Rust. Its goal is to efficiently scan a range of TCP and UDP ports on a given target (IP address or URL) to determine which ports are open. The tool is designed for speed, usability, and extensibility, making it suitable for network diagnostics and security assessments.

## Components and Modules

- **src/main.rs**: Entry point and CLI interface. Handles argument parsing using the `clap` crate and delegates scanning logic to the library module.
- **src/lib.rs**: Core library containing:
  - `parse_ports`: Parses user-supplied port ranges and lists into a vector of port numbers.
  - `scan_ports`: Asynchronously scans the specified ports using a configurable number of threads, reporting open TCP and UDP ports.
  - Unit tests for port parsing logic.

This separation allows for easy testing and future extension. The CLI is kept minimal, while the scanning logic is reusable and testable.

## Usage

Build and run the scanner from the project directory:

```bash
cargo run -- <TARGET> -p <PORTS> -t <THREADS>
```

- `<TARGET>`: IP address or hostname to scan (e.g., `127.0.0.1` or `example.com`)
- `<PORTS>`: Comma-separated list and/or ranges (e.g., `22,80-85,443`)
- `<THREADS>`: Maximum number of concurrent threads (e.g., `50`)

**Example:**

```bash
cargo run -- 127.0.0.1 -p 22,80,443 -t 50
```

This scans ports 22, 80, and 443 on localhost using up to 50 threads.

To run unit tests:

```bash
cargo test
```

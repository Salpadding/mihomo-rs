# mihomo-rs

A lightweight proxy server implementation written in Rust, providing SOCKS5 proxy functionality with high performance and reliability.

## Overview

mihomo-rs is a Rust-based proxy server that currently supports SOCKS5 protocol. It's designed to be efficient, safe, and easy to use, leveraging Rust's memory safety guarantees and async/await capabilities through Tokio.

## Features

- **SOCKS5 Protocol Support**: Full implementation of SOCKS5 proxy protocol
- **Asynchronous I/O**: Built on top of Tokio for high-performance async operations
- **Direct Connection Adapter**: Supports direct connections to target servers
- **TCP Connection Handling**: Efficient TCP connection management
- **Metadata Management**: Comprehensive connection metadata tracking
- **Modular Architecture**: Clean separation of concerns with adapters, listeners, and tunnels

## Prerequisites

- Rust 1.91.0 or higher
- Cargo (comes with Rust)

## Installation

### From Source

Clone the repository and build from source:

```bash
git clone https://github.com/Salpadding/mihomo-rs.git
cd mihomo-rs
cargo build --release
```

The compiled binary will be available at `target/release/mihomo-rs`.

## Usage

### Running the Server

Start the SOCKS5 proxy server:

```bash
cargo run
```

By default, the server listens on `0.0.0.0:7892`.

### Testing the Proxy

You can test the proxy using curl with SOCKS5 support:

```bash
curl --socks5 127.0.0.1:7892 https://www.baidu.com
```

Or use the provided Makefile:

```bash
make curl
```

## Project Structure

```
mihomo-rs/
├── src/
│   ├── adapter/          # Proxy adapters (Direct, etc.)
│   │   └── outbound/     # Outbound connection adapters
│   ├── component/        # Core components
│   │   └── dialer/       # Connection dialing logic
│   ├── constant/         # Constants and traits
│   │   ├── context.rs    # Connection context
│   │   └── metadata.rs   # Connection metadata
│   ├── listener/         # Inbound listeners
│   │   ├── inbound/      # Inbound protocol implementations
│   │   ├── socks/        # SOCKS5 server implementation
│   │   └── config/       # Configuration management
│   ├── tunnel/           # Tunnel logic for handling connections
│   ├── errors.rs         # Error definitions
│   └── main.rs           # Application entry point
├── Cargo.toml            # Project dependencies
├── LICENSE               # MIT License
└── Makefile             # Build and test commands
```

## Architecture

The project follows a modular architecture:

- **Listeners**: Handle incoming connections (currently SOCKS5)
- **Tunnel**: Core routing logic that connects inbound and outbound connections
- **Adapters**: Outbound connection handlers (e.g., Direct adapter)
- **Components**: Reusable components like dialers
- **Constants**: Shared types and traits

## Dependencies

Key dependencies include:

- `tokio`: Async runtime
- `fast-socks5`: SOCKS5 protocol implementation
- `async-trait`: Async trait support
- `anyhow`: Error handling
- `serde`: Serialization/deserialization
- `derive_builder`: Builder pattern for structs

## Development

### Building

```bash
cargo build
```

### Running in Development Mode

```bash
cargo run
```

### Running Tests

```bash
cargo test
```

## Configuration

Currently, the server is configured to:
- Listen on all interfaces (`0.0.0.0`)
- Use port `7892`

Future versions will support configuration files for more flexible setup.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Author

Copyright (c) 2025 zhuyingjie

## Roadmap

Planned features and improvements:

- [ ] Configuration file support
- [ ] Additional proxy protocols (HTTP, HTTPS)
- [ ] UDP support
- [ ] Rule-based routing
- [ ] Performance optimizations
- [ ] Comprehensive test coverage
- [ ] Documentation improvements

## Acknowledgments

- Built with Rust and Tokio
- Inspired by the Mihomo project

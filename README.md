# File Server

A lightweight HTTP file server written in Rust that serves the current directory with automatic content-type detection.

## Usage

```bash
cargo build --release
./target/release/server
```

Opens `http://127.0.0.1:8080` serving your current directory.

## License

MIT

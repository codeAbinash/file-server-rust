# 🚀 File Server

A lightweight, fast, and beautiful HTTP file server written in Rust. Perfect for local development, file sharing, and quick prototyping.

## ✨ Features

- 🎯 **Simple & Fast** - Zero configuration, just run and serve
- 📁 **Directory Browsing** - Beautiful, modern UI for browsing directories
- 🎨 **Responsive Design** - Works seamlessly on desktop and mobile
- 🔍 **Auto Content-Type** - Automatically detects MIME types for various file formats
- ⚡ **Performance Tracking** - Shows file size and time taken to serve each file
- 🛠️ **CLI Options** - Built-in `--help` and `--version` commands
- 🧩 **Modular Architecture** - Clean, maintainable codebase

## 📦 Installation

### From Source

```bash
# Clone the repository
git clone https://github.com/codeAbinash/server.git
cd server

# Build in release mode
cargo build --release

# The binary will be at ./target/release/server
```

### Quick Build

```bash
cargo build --release
```

## 🎯 Usage

### Start the Server

Simply run the server in any directory:

```bash
./target/release/server
```

Or on Windows:
```powershell
.\target\release\server.exe
```

The server will start at `http://127.28.29.30:3132` and serve the current directory.

### Command-Line Options

```bash
# Show help information
server --help
server -h

# Show version
server --version
server -v
```

### Example Output

```
🚀 File Server Started!
📁 Serving directory: /home/user/projects
🌐 Server running at: http://127.28.29.30:3132
Press Ctrl+C to stop

📥 GET /document.pdf
   ✓ Served 2.45 MB in 12.34ms
📥 GET /images/photo.jpg
   ✓ Served 847.32 KB in 5.67ms
```

## 📂 Project Structure

```
src/
├── main.rs           # Entry point and request handling
├── cli.rs            # Command-line argument parsing
├── server.rs         # File and directory serving logic
├── html.rs           # HTML generation for directory listings
├── content_type.rs   # MIME type detection
└── utils.rs          # Utility functions
```

### Module Breakdown

- **`main.rs`** - Application entry point, coordinates all modules
- **`cli.rs`** - Handles `--help` and `--version` commands
- **`server.rs`** - Core serving logic with performance tracking
- **`html.rs`** - Generates beautiful directory listing pages
- **`content_type.rs`** - Detects MIME types for 15+ file formats
- **`utils.rs`** - Path decoding and size formatting utilities

## 🎨 Features in Detail

### Directory Browsing

When you navigate to a directory, you get a beautiful, modern interface with:
- Color-coded files and folders with icons
- Sortable file list
- File size information
- Parent directory navigation
- Responsive design for all screen sizes

### Supported File Types

The server automatically detects and serves the correct content type for:
- **Web**: HTML, CSS, JavaScript, JSON, XML
- **Images**: PNG, JPEG, GIF, SVG
- **Documents**: PDF, TXT
- **Media**: MP4, MP3
- **Archives**: ZIP
- And more with fallback to `application/octet-stream`

### Performance Tracking

Every file served shows:
- ✓ Success indicator
- File size in human-readable format (B, KB, MB, GB)
- Time taken to read and serve the file

## 🛠️ Development

### Build for Development

```bash
cargo build
cargo run
```

### Build for Production

```bash
cargo build --release
```

The release build is optimized for:
- Minimal binary size
- Maximum performance
- Link-time optimization (LTO)

### Install Script

On Windows, you can use the provided installation script:

```cmd
scripts\install-server.cmd
```

## 📝 Configuration

The server runs on `127.28.29.30:3132` by default. To change this, modify the `HOST` constant in `src/main.rs`:

```rust
static HOST: &str = "127.28.29.30:3132";
```

## 🤝 Contributing

Contributions are welcome! The modular architecture makes it easy to:
- Add new file type support in `content_type.rs`
- Enhance the UI in `html.rs`
- Add new CLI options in `cli.rs`
- Improve serving logic in `server.rs`

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

Built with:
- [tiny_http](https://github.com/tiny-http/tiny-http) - Minimal HTTP server library
- [urlencoding](https://github.com/kornelski/rust-urlencoding) - URL encoding/decoding
- ❤️ Rust

## 📊 Why This Server?

- **Zero Dependencies** (runtime) - Just a single binary
- **Lightweight** - Optimized release build
- **Beautiful UI** - Not your typical directory listing
- **Fast** - Rust performance with async capabilities
- **Simple** - No configuration files, just run it
- **Open Source** - MIT licensed, modify as you need

---

Made with ❤️ using Rust

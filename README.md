# HTTP Server

A single-threaded HTTP/1.1 server written from scratch in Rust, with no external dependencies. It serves static files from a configurable public directory and implements HTTP protocol parsing manually.

## Features

- HTTP/1.1 request parsing (method, path, query string, headers)
- Static file serving (HTML, CSS, JSON) from a `public/` directory
- Directory traversal attack prevention
- Configurable public path via environment variable
- Dedicated error types for parsing failures (`ParseError`)
- Query string parsing with multi-value support per key

## Project Structure

```
src/
├── main.rs                  # Entry point, server startup
├── server.rs                # TCP server and Handler trait
├── website_handler.rs       # Static file handler
└── http/
    ├── mod.rs               # Module exports
    ├── method.rs            # Method enum (GET, POST, PUT, ...)
    ├── request.rs           # HTTP request parsing
    ├── response.rs          # HTTP response building
    ├── status_code.rs       # StatusCode enum with reason phrases
    ├── query_string.rs      # Query string parsing
    └── headers.rs           # HTTP headers parsing
public/
├── index.html
├── style.css
└── informations.json
```

## Requirements

Rust and Cargo installed. Installation guide: https://www.rust-lang.org/learn/get-started

## Getting Started

```bash
git clone https://github.com/federicobaldini/http-server
cd http-server
cargo run
```

The server starts on `127.0.0.1:5000`.

## Configuration

| Environment Variable | Description | Default |
|---|---|---|
| `PUBLIC_PATH` | Absolute path to the static files directory | `./public` |
| `HOST` | IP address the server binds to | `127.0.0.1` |
| `PORT` | TCP port the server listens on | `5000` |

Example:

```bash
PUBLIC_PATH=/var/www/html HOST=0.0.0.0 PORT=8080 cargo run
```

## Known Limitations

- Single-threaded: handles one connection at a time
- Fixed 2048-byte read buffer
- No keep-alive support (connection closes after each response)
- Request body not supported (GET only)
- No HTTPS support

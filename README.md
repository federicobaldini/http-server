# HTTP Server

A single-threaded HTTP/1.1 server written from scratch in Rust. It serves static files from a configurable public directory and implements HTTP protocol parsing manually.

## Features

- HTTP/1.1 request parsing (method, path, query string, headers, body)
- Request body parsed from `Content-Length` + `Content-Type`; represented as `Text` or `Binary`
- Static file serving (HTML, CSS, JSON, images, …) from a `public/` directory
- Auto-detected `Content-Type` header based on file extension
- `Content-Length` header included in every response
- `405 Method Not Allowed` for non-GET requests; `404` for missing files
- Custom `404.html` page served when present in `public/`
- Directory traversal attack prevention
- Dynamic read buffer: headers up to 8 KB, body up to 1 MB
- 5-second read timeout per connection
- Configurable bind address and port via environment variables
- Centralised error handling via `thiserror`
- Query string parsing with multi-value support per key

## Project Structure

```
src/
├── main.rs                  # Entry point, server startup
├── config.rs                # Config struct (env vars + .env file)
├── server.rs                # TCP server and Handler trait
├── website_handler.rs       # Static file handler
└── http/
    ├── mod.rs               # Module exports
    ├── error.rs             # ParseError (thiserror)
    ├── method.rs            # Method enum (GET, POST, PUT, …)
    ├── request.rs           # HTTP request parsing
    ├── request_body.rs      # RequestBody enum (Text / Binary)
    ├── response.rs          # HTTP response building
    ├── status_code.rs       # StatusCode enum with reason phrases
    ├── query_string.rs      # Query string parsing
    └── headers.rs           # HTTP headers parsing
public/
├── index.html
├── 404.html
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

The server starts on `127.0.0.1:5000` by default.

## Configuration

| Environment Variable | Description | Default |
|---|---|---|
| `PUBLIC_PATH` | Absolute path to the static files directory | `./public` |
| `HOST` | IP address the server binds to | `127.0.0.1` |
| `PORT` | TCP port the server listens on | `5000` |
| `MAX_HEADER_SIZE` | Max request header bytes | `8192` |
| `MAX_BODY_SIZE` | Max request body bytes | `1048576` |
| `READ_TIMEOUT_SECS` | Idle connection timeout in seconds | `5` |

All variables can also be set in a `.env` file at the project root (loaded automatically via `dotenvy`).

Example:

```bash
PUBLIC_PATH=/var/www/html HOST=0.0.0.0 PORT=8080 cargo run
```

## Known Limitations

- Single-threaded: handles one connection at a time
- No keep-alive support (connection closes after each response)
- No HTTPS support

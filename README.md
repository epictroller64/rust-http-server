# Rust TCP Server

A lightweight, multi-threaded HTTP server written in Rust. This server supports handling HTTP requests with custom handlers, serving static files, and JSON responses.

## Features

- Multi-threaded request handling using a thread pool
- Support for GET and POST requests
- JSON response handling
- Static file serving with automatic content-type detection
- Custom request handlers
- HTTP/1.1 compliant responses

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
rust-tcp-server = "0.1.0"
```

## Usage

Here's a basic example of how to use the server:

```rust
use rust_tcp_server::server::{handlers::Response, server::Server};

fn main() {
    let mut server = Server::new(8080); // Create server on port 8080

    // Handle JSON response
    server.get("/json", |_| {
        let mut response = Response::new(200);
        response.with_json(serde_json::json!({
            "message": "Hello, world!",
        }));
        response
    });

    // Handle text response
    server.get("/text", |req| {
        let mut response = Response::new(200);
        response.with_text("Hello, world!");
        response
    });

    // Serve static files
    server.get("/file", |_| {
        let mut response = Response::new(200);
        response.with_file("path/to/your/file.html");
        response
    });

    // Start the server
    if let Err(e) = server.start_server() {
        eprintln!("Server error: {}", e);
    }
}
```

## Response Types

The server supports several types of responses:

### JSON Response
```rust
response.with_json(serde_json::json!({
    "key": "value"
}));
```

### Text Response
```rust
response.with_text("Hello, world!");
```

### File Response
```rust
response.with_file("path/to/file.html");
```

The server automatically detects the content type based on the file extension:
- HTML files: `text/html`
- CSS files: `text/css`
- JavaScript files: `application/javascript`
- Images: `image/png`, `image/jpeg`, `image/gif`, `image/svg+xml`, `image/x-icon`
- Other files: `application/octet-stream`

## Request Handling

Each request handler receives a `Request` struct containing:
- `method`: The HTTP method (GET, POST, etc.)
- `path`: The request path
- `version`: The HTTP version
- `headers`: A HashMap of request headers
- `body`: The request body as a Vec<u8>

## Thread Pool

The server uses a thread pool to handle multiple requests concurrently. By default, it creates 4 worker threads, but you can modify this in the `ThreadPool::new()` call.

## Error Handling

The server includes basic error handling:
- 404 responses for non-existent routes
- Error logging for connection issues
- Graceful handling of malformed requests

## Dependencies

- `chrono`: For date/time handling in HTTP headers
- `serde_json`: For JSON response handling

## License

This project is licensed under the MIT License - see the LICENSE file for details.
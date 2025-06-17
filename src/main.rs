use std::collections::HashMap;

use rust_tcp_server::server::{handlers::Response, server::Server};

// Working example on how to use the library
fn main() {
    let mut server = Server::new(1000);

    server.get("/hello2", |req| {
        // Parse body bytes to string if needed
        let body = String::from_utf8(req.body.to_vec()).unwrap();
        println!("Body: {}", body);
        let mut response = Response::new(200, "Hi".to_string());
        response.with_header("Content-Type", "text/plain");
        response.with_headers(HashMap::from([
            ("Server".to_string(), "Rust".to_string()),
            (
                "Date".to_string(),
                "Tue, 17 Jun 2025 12:00:00 GMT".to_string(),
            ),
        ]));
        response
    });

    if let Err(e) = server.start_server() {
        eprintln!("Server error: {}", e);
    }
}

use rust_tcp_server::server::{handlers::Response, server::Server};

fn main() {
    let mut server = Server::new(1000);

    server.get("/hello", |req| {
        let body = String::from_utf8(req.body.to_vec()).unwrap();
        let response = format!("Received: {}", body);
        Response {
            response_code: 200,
            body: response,
        }
    });

    if let Err(e) = server.start_server() {
        eprintln!("Server error: {}", e);
    }
}

use std::{
    collections::HashMap,
    io::{BufRead, BufReader, Read, Write},
    net::{TcpListener, TcpStream},
};

use crate::server::handlers::{Handler, Request, Response};

pub struct Server {
    handlers: HashMap<String, Handler>,
    port: u16,
}

impl Server {
    pub fn new(port: u16) -> Self {
        Self {
            handlers: HashMap::new(),
            port,
        }
    }

    pub fn get(&mut self, path: &str, handler: fn(Request) -> Response) {
        self.handlers.insert(
            path.to_string(),
            Handler {
                path: path.to_string(),
                method: "GET".to_string(),
                handler,
            },
        );
    }

    pub fn post(&mut self, path: &str, handler: fn(Request) -> Response) {
        self.handlers.insert(
            path.to_string(),
            Handler {
                path: path.to_string(),
                method: "POST".to_string(),
                handler,
            },
        );
    }

    pub fn start_server(&self) -> Result<(), Box<dyn std::error::Error>> {
        let listener = TcpListener::bind(format!("127.0.0.1:{}", self.port))?;
        println!("Server is running on port {}", self.port);
        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    handle_client(stream, &self.handlers);
                }
                Err(e) => {
                    eprintln!("Error accepting connection: {}", e);
                }
            }
        }
        Ok(())
    }
}

fn handle_client(stream: TcpStream, handlers: &HashMap<String, Handler>) {
    let mut stream = stream;
    let request = match parse_request_bytes(&mut stream) {
        Ok(req) => req,
        Err(e) => {
            eprintln!("Error parsing request: {}", e);
            return;
        }
    };

    println!("Request: {:?}", request);

    // Find the appropriate handler
    if let Some(handler) = handlers.get(&request.path) {
        let response = (handler.handler)(request);
        let response_str = format!(
            "HTTP/1.1 {} OK\r\nContent-Length: {}\r\n\r\n{}",
            response.response_code,
            response.body.len(),
            response.body
        );
        if let Err(e) = stream.write_all(response_str.as_bytes()) {
            eprintln!("Error writing response: {}", e);
        }
    } else {
        let not_found = "HTTP/1.1 404 Not Found\r\nContent-Length: 13\r\n\r\n404 Not Found";
        if let Err(e) = stream.write_all(not_found.as_bytes()) {
            eprintln!("Error writing response: {}", e);
        }
    }
}

fn parse_request_bytes(stream: &mut TcpStream) -> Result<Request, Box<dyn std::error::Error>> {
    let mut reader = BufReader::new(stream);
    let mut headers: HashMap<String, String> = HashMap::new();

    // Read the first line
    let mut first_line = String::new();
    if reader.read_line(&mut first_line)? == 0 {
        return Err("Connection closed".into());
    }
    let (m, p, v) = parse_method_path_version(first_line.trim());

    // Read headers
    loop {
        let mut line = String::new();
        if reader.read_line(&mut line)? == 0 {
            return Err("Connection closed".into());
        }

        let line = line.trim();
        if line.is_empty() {
            break;
        }

        let (key, value) = parse_header(line);
        headers.insert(key.to_lowercase(), value);
    }

    // Read body if present
    let content_length = headers
        .get("content-length")
        .and_then(|v| v.trim().parse().ok())
        .unwrap_or(0);

    let mut body = Vec::with_capacity(content_length);
    if content_length > 0 {
        let mut buffer = vec![0u8; content_length];
        let bytes_read = reader.read(&mut buffer)?;
        body = buffer[..bytes_read].to_vec();
    }

    Ok(Request {
        method: m,
        path: p,
        version: v,
        headers,
        body,
    })
}

fn parse_header(line: &str) -> (String, String) {
    let parts = line.splitn(2, ": ").collect::<Vec<&str>>();
    if parts.len() == 2 {
        (parts[0].to_string(), parts[1].to_string())
    } else {
        (parts[0].to_string(), String::new())
    }
}

fn parse_method_path_version(line: &str) -> (String, String, String) {
    let parts = line.split_whitespace().collect::<Vec<&str>>();
    if parts.len() >= 3 {
        (
            parts[0].to_string(),
            parts[1].to_string(),
            parts[2].to_string(),
        )
    } else {
        (String::new(), String::new(), String::new())
    }
}

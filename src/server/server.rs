use std::{
    collections::HashMap,
    io::{Read, Write},
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

fn handle_client(mut stream: TcpStream, handlers: &HashMap<String, Handler>) {
    let mut buffer = [0; 1024];
    loop {
        match stream.read(&mut buffer) {
            Ok(size) => {
                if size == 0 {
                    println!("Client disconnected");
                    return;
                }
                let data = String::from_utf8_lossy(&buffer[..size]);
                parse_request(&data);
            }
            Err(e) => {
                eprintln!("Error reading from client: {}", e);
            }
        }
    }
}

fn parse_request(request: &str) -> Request {
    let lines = request.split("\r\n").collect::<Vec<&str>>();
    let (m, p, v) = parse_method_path_version(lines[0]);
    let request = Request {
        method: m,
        path: p,
        version: v,
        headers: HashMap::new(),
        body: [0; 1024],
    };
    request
}

fn parse_method_path_version(line: &str) -> (String, String, String) {
    let parts = line.split(" ").collect::<Vec<&str>>();
    (
        parts[0].to_string(),
        parts[1].to_string(),
        parts[2].to_string(),
    )
}

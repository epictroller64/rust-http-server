use std::collections::HashMap;

pub struct Handler {
    pub path: String,
    pub method: String,
    pub handler: fn(Request) -> Response,
}

#[derive(Debug)]
pub struct Request {
    pub path: String,
    pub method: String,
    pub version: String,
    pub headers: HashMap<String, String>,
    pub body: Vec<u8>,
}

pub struct Response {
    pub response_code: u16,
    pub body: String,
    pub headers: HashMap<String, String>,
}

impl Response {
    pub fn new(response_code: u16, body: String) -> Self {
        Self {
            response_code,
            body,
            headers: HashMap::new(),
        }
    }

    pub fn with_header(&mut self, key: &str, value: &str) {
        self.headers.insert(key.to_string(), value.to_string());
    }

    pub fn with_headers(&mut self, headers: HashMap<String, String>) {
        self.headers.extend(headers);
    }
}

impl Handler {}

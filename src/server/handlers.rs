use std::collections::HashMap;

pub struct Handler {
    pub path: String,
    pub method: String,
    pub handler: fn(Request) -> Response,
}

pub struct Request {
    pub path: String,
    pub method: String,
    pub version: String,
    pub headers: HashMap<String, String>,
    pub body: [u8; 1024],
}

pub struct Response {
    pub response_code: u16,
    pub body: String,
}

impl Handler {}

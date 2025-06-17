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
    pub fn new(response_code: u16) -> Self {
        Self {
            response_code,
            headers: HashMap::from([("Server".to_string(), "Rust".to_string())]),
            body: String::new(),
        }
    }

    pub fn with_json(&mut self, json: serde_json::Value) {
        self.body = json.to_string();
        self.with_header("Content-Type", "application/json");
    }

    pub fn with_text(&mut self, text: &str) {
        self.body = text.to_string();
        self.with_header("Content-Type", "text/plain");
    }

    pub fn with_header(&mut self, key: &str, value: &str) {
        self.headers.insert(key.to_string(), value.to_string());
    }

    pub fn with_headers(&mut self, headers: HashMap<String, String>) {
        self.headers.extend(headers);
    }

    pub fn date_header(&mut self) {
        self.with_header(
            "Date",
            chrono::Utc::now()
                .format("%a, %d %b %Y %H:%M:%S GMT")
                .to_string()
                .as_str(),
        );
    }
}

impl Handler {}

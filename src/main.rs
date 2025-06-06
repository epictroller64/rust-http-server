mod server;

fn main() {
    if let Err(e) = server::start_server() {
        eprintln!("Server error: {}", e);
    }
}

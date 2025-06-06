use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
};

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    loop {
        match stream.read(&mut buffer) {
            Ok(size) => {
                if size == 0 {
                    println!("Client disconnected");
                    return;
                }
                let data = String::from_utf8_lossy(&buffer[..size]);
                let response = format!("Received: {}", data);
                println!("Received data: {}", data);
                stream.write_all(response.as_bytes()).unwrap();
            }
            Err(e) => {
                eprintln!("Error reading from client: {}", e);
            }
        }
    }
}

pub fn start_server() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("127.0.0.1:7000")?;
    println!("Server is running on port 7000");
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                handle_client(stream);
            }
            Err(e) => {
                eprintln!("Error accepting connection: {}", e);
            }
        }
    }
    Ok(())
}

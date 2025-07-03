use std::fs;
use std::io::*;
use std::net::*; // tcplistner //prelude 
fn main() {
    let listner = TcpListener::bind("127.0.0.1:3000").unwrap();

    for stream in listner.incoming() {
        let stream = stream.unwrap();
        println!("Connection established!");
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();
    println!("request {}", String::from_utf8_lossy(&buffer[..]));

    let contents = fs::read_to_string(".././index.html").unwrap();
    let response = format!(
        "HTTP/1.1 200 OK\r\n Content-Length: {}\r\n\r\n{}",
        contents.len(),
        contents
    );

    stream.write(response.as_bytes()).unwrap();

    stream.flush().unwrap();
}


use std::fs;
use std::net::{TcpListener, TcpStream};
use std::io::prelude::*;
use std::env;

fn main() {
	let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
	for stream in listener.incoming() {
		let stream = stream.unwrap();
        println!("connection established!");
        handle_connections(stream);
    }
    
}


fn handle_connections(mut stream: TcpStream) {
	// create buffer to read/write requests
	let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";
    let (status_line, filename) = if buffer.starts_with(get) {
    	("HTTP/1.1 200 OK\r\n\r\n", "index.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "error.html")
    };

    let contents = fs::read_to_string(filename).unwrap();
    let response = format!("{}{}", status_line, contents);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();

}
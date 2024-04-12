use std::{io::{Read, Write}, net::TcpListener};

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(mut _stream) => {
                println!("accepted new connection");
                let mut buffer = [0; 10];
                let incoming_data = _stream.read(&mut buffer);
                let response = "HTTP/1.1 200 OK\r\n\r\n";
                let num_bytes_written = _stream.write(response.as_bytes());
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}

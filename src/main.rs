use std::{io::{Read, Write}, net::TcpListener};

use itertools::Itertools;

fn extract_path(data: String) -> String {
    let first_line = data.split("\r\n").next().unwrap();
    let path_vec = first_line.split(" ").collect_vec();
    let path = path_vec.get(1).unwrap();
    String::from(*path)
}

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(mut _stream) => {
                println!("accepted new connection");
                let mut buffer = [0; 120];
                let _num_incoming_bytes = _stream.read(&mut buffer);
                let incoming_data = String::from_utf8(Vec::from(buffer)).unwrap();
                
                let path = extract_path(incoming_data);
                
                if path == "/" {
                    let ok = "HTTP/1.1 200 OK\r\n\r\n";
                    let _num_bytes_written = _stream.write(ok.as_bytes());
                } else {
                    let not_found = "HTTP/1.1 404 NOT FOUND\r\n\r\n";
                    let _ = _stream.write(not_found.as_bytes());
                }

            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}

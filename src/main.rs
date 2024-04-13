use std::{
    io::{Read, Write},
    net::TcpListener,
};

use itertools::Itertools;

fn extract_random_string(data: String) -> String {
    let first_line = data.split("\r\n").next().unwrap();
    let path_vec = first_line.split(" ").collect_vec();
    let path = path_vec.get(1).unwrap();
    let random_string = path.split("/echo/").last().unwrap();
    String::from(random_string)
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

                let random_string = extract_random_string(incoming_data);

                let base_response = format!("HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}\r\n\r\n", random_string.len(), random_string);
                println!("Sending: \n{}", base_response);
                let _ = _stream.write(base_response.as_bytes());
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}

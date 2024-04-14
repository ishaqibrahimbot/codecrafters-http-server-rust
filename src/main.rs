use std::{
    collections::HashMap,
    io::{Read, Write},
    net::TcpListener,
};

use itertools::Itertools;

#[derive(PartialEq)]
enum Status {
    TwoHundred,
    FourZeroFour,
}

struct Request {
    method: String,
    path: String,
    headers: HashMap<String, String>,
}

impl Request {
    fn parse_from(raw_data: String) -> Request {
        let request_lines = raw_data.split("\r\n").collect_vec();
        let first_line = *request_lines.get(0).unwrap();
        let path_vec = first_line.split(" ").collect_vec();
        let path = String::from(*path_vec.get(1).unwrap());
        let method = String::from(*path_vec.get(0).unwrap());

        let mut headers: HashMap<String, String> = HashMap::new();

        for line in &request_lines[1..] {
            if *line != "\r\n" {
                let parts = line.split(": ").collect_vec();
                let key = String::from(*parts.get(0).unwrap());
                let value = String::from(*parts.get(1).unwrap());

                headers.insert(key, value);
            }
        }

        Request {
            method,
            path,
            headers,
        }
    }
}

struct Response {
    status: Status,
    headers: HashMap<String, String>,
    body: Option<String>,
}

impl Response {
    fn new(status: Status, body: Option<String>, headers: HashMap<String, String>) -> Response {
        Response {
            status,
            body,
            headers,
        }
    }

    fn format(&self) -> String {
        if self.status == Status::FourZeroFour {
            return String::from("HTTP/1.1 404 NOT FOUND\r\n\r\n");
        } else {
            let mut response = String::from("HTTP/1.1 200 OK\r\n");
            if self.headers.len() > 0 {
                let headers_string = self
                    .headers
                    .clone()
                    .into_iter()
                    .map(|(x, y)| format!("{}: {}", x, y))
                    .collect_vec()
                    .join("\r\n");

                response = response + &headers_string.clone() + "\r\n";
            }

            if let Some(body_text) = &self.body {
                response = response + "\r\n" + &body_text.clone() + "\r\n";
            }

            response = response + "\r\n";

            response
        }
    }

    fn add_header(&mut self, key: String, value: String) {
        self.headers.insert(key, value);
    }
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

                let request = Request::parse_from(incoming_data);

                let mut response_headers: HashMap<String, String> = HashMap::new();

                if request.path == "/" {
                    let response = Response::new(Status::TwoHundred, None, response_headers);
                    let _ = _stream.write(response.format().as_bytes());
                } else if request.path.starts_with("/echo") {
                    let random_string = request.path.split("/echo/").last().unwrap();
                    response_headers.insert("Content-Type".to_string(), "text/plain".to_string());
                    response_headers.insert(
                        "Content-Length".to_string(),
                        random_string.len().to_string(),
                    );
                    let response = Response::new(
                        Status::TwoHundred,
                        Some(String::from(random_string)),
                        response_headers,
                    );

                    let _ = _stream.write(response.format().as_bytes());
                } else if request.path.starts_with("/user-agent") {
                    let user_agent = request.headers.get("User-Agent").unwrap();
                    response_headers.insert("Content-Type".to_string(), "text/plain".to_string());
                    response_headers
                        .insert("Content-Length".to_string(), user_agent.len().to_string());

                    let response = Response::new(
                        Status::TwoHundred,
                        Some(user_agent.clone()),
                        response_headers,
                    );

                    let _ = _stream.write(response.format().as_bytes());
                } else {
                    let response = Response::new(Status::FourZeroFour, None, response_headers);
                    let _ = _stream.write(response.format().as_bytes());
                }
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}

use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    let (status_line, body) = if request_line == "GET / HTTP/1.1" {
        ("HTTP/1.1 200 OK", "Hello, We are DevOps Hobbies :)")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "Please send a GET request to the '/' path, to reach out the DevOps Hobbies")
    };

    let response = format!("{status_line}\r\n\r\n{body}\r\n");
    stream.write_all(response.as_bytes()).unwrap();
}

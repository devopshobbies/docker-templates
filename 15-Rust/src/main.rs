use std::{
    env,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        panic!("not enough arguments");
    } else if args.len() > 2 {
        panic!("more arguments");
    }

    let port_string = &args[1];
    let port = match port_string.parse::<i32>() {
        Ok(n) => n,
        Err(_) => {
            panic!("invalid port number");
        },
    };

    let address = format!("0.0.0.0:{port}");
    let listener = TcpListener::bind(address).unwrap();

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

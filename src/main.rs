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


    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|line| {
            println!("hi");
            return line.unwrap();
        })
        .take_while(|line| {
            println!("hello");
            return !line.is_empty();
        })
        .collect();

    let status_line = "HTTP/1.1 200 OK";
    let contents = std::fs::read_to_string("src/templates/index.html").unwrap_or_else(|err| {
        println!("Error reading file: {}", err);
        std::process::exit(500);});
    let length = contents.len();        
    let response =
        format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();
    println!("Request: {:#?}", http_request);
}

use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};
use hello::ThreadPool;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }
}



fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();
    let response: String;
    if request_line == "GET / HTTP/1.1" {
        let status_line = "HTTP/1.1 200 OK";
        let contents = std::fs::read_to_string("src/templates/index.html").unwrap();
        let length = contents.len();

        response = format!(
            "{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}"
        );

        stream.write_all(response.as_bytes()).unwrap();
    } else {
        let status_line = "HTTP/1.1 404 NOT FOUND";
        let content = "404 NOT FOUND";
        let length =content.len();

        response = format!(
            "{status_line}\r\nContent-Length: {length}\r\n\r\n{content}"
        );

        stream.write_all(response.as_bytes()).unwrap();
    }
}
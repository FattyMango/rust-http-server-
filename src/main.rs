use std::net::TcpListener;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:80").unwrap_or_else(|err| {
        eprintln!("Failed to bind to port 80: {}", err);
        std::process::exit(1);
    });

    for stream in listener.incoming() {
        let _stream = stream.unwrap();

        println!("Connection established!");
    }
}
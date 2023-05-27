use std::fs;
use std::io::prelude::*;
use std::io::BufReader;
use std::net::{TcpListener, TcpStream};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8000").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_request(stream);
    }
}

fn handle_request(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let request = buf_reader.lines().next().unwrap().unwrap();
    let response: String;

    println!("Request: {:#?}", &request);

    let (status, page) = match request.as_str() {
        "GET / HTTP/1.1" => ("HTTP/1.1 200", "index.html"),
        "GET /about HTTP/1.1" => ("HTTP/1.1 200", "about.html"),
        _ => ("HTTP/1.1 404", "404.html"),
    };

    let content = get_file(&format!("www/{}", page));

    response = format_response(status, &content);

    stream.write_all(response.as_bytes()).unwrap();
}

fn get_file(file_path: &str) -> String {
    fs::read_to_string(file_path).expect("File not found!")
}

fn format_response(status: &str, content: &str) -> String {
    let length = content.len();
    format!("{status}\r\nContent-Length: {length}\r\n\r\n{content}")
}

use std::net::TcpListener;
use std::net::TcpStream;
use std::io::prelude::*;
use std::fs;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();
    let get = b"GET";

    for x in buffer {
        println!("{} ", x);
    }

    let byte_stream_string = std::str::from_utf8(&buffer).unwrap().split("\r\n").collect::<Vec<&str>>();
    let string = format!("{}", byte_stream_string.get(0).unwrap());
    let asd = string.split_whitespace().collect::<Vec<&str>>();

    let (status_line, filename) = if buffer.starts_with(get) {

        ("HTTP/1.1 200 OK", "{\"status\": \"200, message\": \"OK\", \"content\":".to_owned()+ asd.get(1).unwrap() + " }")
    } else {
        ("HTTP/1.1 200 OK", "{\"status\": \"200, message\": \"OK\", \"content\":".to_owned()+ asd.get(1).unwrap() + " }")
    };

    let contents = filename; //fs::read_to_string(filename).unwrap();
    let response = format!("{}\r\nContent-Length: {}\r\n\r\n{}", status_line, contents.len(), contents);
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();

    println!("Request {}", String::from_utf8_lossy(&buffer[..]));
}
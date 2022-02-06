use std::net::TcpListener;
use std::net::TcpStream;
use std::io::prelude::*;

/// Look up our server port number in PORT, for compatibility with Heroku.
fn get_server_port() -> u16 {
    env::var("PORT").ok().and_then(|p| p.parse().ok()).unwrap_or(8080)
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:".to_owned() + get_server_port()).unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();
    let get = b"GET";
    let post = b"POST";

    for x in buffer {
        println!("{} ", x);
    }

    let byte_stream_string = std::str::from_utf8(&buffer).unwrap().split("\r\n").collect::<Vec<&str>>();
    let string = format!("{}", byte_stream_string.get(0).unwrap());
    let asd = string.split_whitespace().collect::<Vec<&str>>();

    let (status_line, filename) = if buffer.starts_with(get) {
        let x3 = asd.get(1).unwrap();
        ("HTTP/1.1 200 OK", "{\"status\": \"200\", \"message\": \"OK\", \"content\":".to_owned()+ x3 + " }")
    } else if buffer.starts_with(post) {
        let x2 = byte_stream_string.get(byte_stream_string.len() - 1).unwrap().trim_matches(char::from(0));
        ("HTTP/1.1 201 CREATED", "{\"status\": \"201\", \"message\": \"CREATED\", \"content\": \"".to_owned()+ x2 + "\" }")
    } else {
        let x1 = byte_stream_string.get(byte_stream_string.len() - 1).unwrap();
        ("HTTP/1.1 418 I'M A TEAPOT", "{\"status\": \"418, message\": \"I'M A TEAPOT\", \"content\":".to_owned()+ x1 + " }")
    };

    let contents = filename; //fs::read_to_string(filename).unwrap();
    let response = format!("{}\r\nContent-Length: {}\r\n\r\n{}", status_line, contents.len(), contents);
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();

    println!("Request {}", String::from_utf8_lossy(&buffer[..]));
}
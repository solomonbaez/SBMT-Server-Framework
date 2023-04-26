
use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

fn main() {
    
    // listen for TCP connections at addr
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    
    // process connections
    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    // parse requests
    if request_line == "GET / HTTP/1.1" {
        let status_line = "HTTP/1.1 200 OK";
        let contents = fs::read_to_string("hello.html").unwrap();
        let length = contents.len();

        let response = format!(
            "{}\r\nContent-Length: {}\r\n\r\n{}", 
            status_line, length, contents
        );

        // send bytes directly down the connection
        stream.write_all(response.as_bytes()).unwrap();

    } else {
        let status_line = "HTTP/1.1 404 NOT FOUND";
        let contents = fs::read_to_string("404.html").unwrap();
        let length = contents.len();

        let response = format!(
            "{}\r\nContent-Length: {}\r\n\r\n{}", 
            status_line, length, contents
        );
    
        stream.write_all(response.as_bytes()).unwrap();
    }
}
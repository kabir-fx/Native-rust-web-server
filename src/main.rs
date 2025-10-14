use std::{
    io::{BufRead, BufReader},
    net::{TcpListener, TcpStream},
};

fn main() {
    // Binding a TCP listener to port 7878
    let listner = TcpListener::bind("127.0.0.1:7878").unwrap();

    // Iterating over all the TCP streams hitting our IP.
    for stream in listner.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(stream: TcpStream) {
    // Creating an instance of BufReader to using the TcpStream passed in the function to exctract the information from the request.
    let buf_reader = BufReader::new(&stream);

    // Collecting the lines of request that the client sends to the server in a vector.
    let http_req: Vec<String> = buf_reader
        .lines()
        // Since lines() returns an interator of Result<String, Error> - we are mapping  each corresponding string to get its String value.
        .map(|result| result.unwrap())

        // The client signals the end of an HTTP request by sending two newline characters in a row, so to get one request from the stream, we take lines until we get a line that is the empty string.
        .take_while(|line| !line.is_empty())
        
        // Collect into a vector
        .collect();

    println!("Request: {http_req:#>?}");
}

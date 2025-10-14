use std::{
    fs,
    io::{BufRead, BufReader, Write},
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

fn handle_connection(mut stream: TcpStream) {
    // Creating an instance of BufReader to using the TcpStream passed in the function to exctract the information from the request.
    let buf_reader = BufReader::new(&stream);

    /*
        Collecting the lines of request that the client sends to the server in a vector.

        HTTP is a text-based protocol, and a request takes this format:

            Method Request-URI HTTP-Version CRLF
            headers CRLF
            message-body

        Thus since the request and headers are separated by /r/n (new line) we will be breaking and collecting these into a vector until we reach we a new line.
    */

    let http_req: Vec<String> = buf_reader
        .lines()
        // Since lines() returns an interator of Result<String, Error> - we are mapping  each corresponding string to get its String value.
        .map(|result| result.unwrap())
        // The client signals the end of an HTTP request by sending two newline characters in a row, so to get one request from the stream, we take lines until we get a line that is the empty string.
        .take_while(|line| !line.is_empty())
        // Collect into a vector
        .collect();

    println!("Request: {http_req:#>?}");

    /*
        Response Format:

            HTTP-Version Status-Code Reason-Phrase CRLF
            headers CRLF
            message-body

        So, we are currently writing a success response with a 200 status code - no header or body.
    */
    let status_line: &str = "HTTP/1.1 200 OK";
    let contents = fs::read_to_string("hello.html").unwrap();

    // To ensure a valid HTTP response, we add the Content-Length header which is set to the size of our response body, in this case the size of hello.html
    let length = contents.len();

    // Building the success response
    let success_response_data =
        format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    // Writing the response as bytes back to stream to return to the client
    stream.write_all(success_response_data.as_bytes()).unwrap();
}

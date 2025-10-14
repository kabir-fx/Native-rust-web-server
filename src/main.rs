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
        HTTP is a text-based protocol, and a request takes this format:

        Method Request-URI HTTP-Version CRLF
        headers CRLF
        message-body

        Here, we are reading the first line of the reaquest - send into the stream to determine whether the request is GET and is for root (/).
    */
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    println!("Request: {request_line}");

    if request_line == "GET / HTTP/1.1" {
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
    } else {
        // If the request for wither not GET or for ROOT we send a different page indicating the error.
        let status_line = "HTTP/1.1 404 NOT FOUND";
        let contents = fs::read_to_string("404.html").unwrap();
        let length = contents.len();

        let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

        stream.write_all(response.as_bytes()).unwrap();
    }
}

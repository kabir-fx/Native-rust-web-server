use web_server::ThreadPool;

use std::{
    fs,
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};

use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

fn main() {
    // Binding a TCP listener to port 7878
    let listner = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);

    // Create a flag to signal shutdown
    let shutdown = Arc::new(AtomicBool::new(false));
    let shutdown_clone = Arc::clone(&shutdown);

    // Set up signal handler for graceful shutdown
    ctrlc::set_handler(move || {
        println!("Received shutdown signal...");
        shutdown_clone.store(true, Ordering::SeqCst);
    })
    .expect("Error setting Ctrl+C handler");

    // Make listener non-blocking so we can check shutdown flag
    listner
        .set_nonblocking(true)
        .expect("Failed to set non-blocking");

    println!("Server started. Press Ctrl+C to shutdown gracefully.");

    loop {
        // Check if we should shutdown
        if shutdown.load(Ordering::SeqCst) {
            println!("Shutting down server...");
            break;
        }

        // Try to accept a connection
        match listner.accept() {
            Ok((stream, _)) => {
                pool.execute(|| handle_connection(stream));
            }
            Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                // No connection available, sleep briefly
                thread::sleep(Duration::from_millis(10));
                continue;
            }
            Err(e) => {
                eprintln!("Error accepting connection: {}", e);
            }
        }
    }

    // ThreadPool will be dropped here, showing shutdown messages
    println!("Server shutdown complete.");
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

    /*
        Response Format:

            HTTP-Version Status-Code Reason-Phrase CRLF
            headers CRLF
            message-body

        If the request matches our desired input we send then send the corresponding HTML and status code
    */
    let (status_line, filename) = match &request_line[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "hello.html"),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "hello.html")
        }
        _ => ("HTTP/1.1 404 NOT FOUND", "404.html"),
    };

    // Reading the contents of the corresponding HTML file
    let contents = fs::read_to_string(filename).unwrap();

    // To ensure a valid HTTP response, we add the Content-Length header which is set to the size of our response body, equal to the size of corresponding HTML file.
    let length = contents.len();

    // Building the response to send to the client
    let response_data = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    // Writing the response as bytes back to stream to return to the client
    stream.write_all(response_data.as_bytes()).unwrap();
}

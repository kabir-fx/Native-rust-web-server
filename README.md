# Rust Web Server

A simple, multi-threaded HTTP web server built in Rust from scratch, featuring a custom thread pool implementation for handling concurrent connections.

## Features

- **Multi-threaded Architecture**: Custom thread pool implementation for efficient concurrent request handling
- **Graceful Shutdown**: Clean shutdown handling with Ctrl+C signal interception
- **Simple Routing**: Basic HTTP GET request routing with support for custom endpoints
- **Static File Serving**: Serves HTML content with proper HTTP headers
- **Performance Testing**: Includes a `/sleep` endpoint to simulate slow responses for load testing
- **Error Handling**: Proper 404 responses for unmatched routes

## Installation

### Prerequisites

- Rust (latest stable version recommended)
- Cargo (comes with Rust)

### Building

Clone the repository and build the project:

```bash
git clone <repository-url>
cd rust-web-server
cargo build --release
```

## Usage

### Running the Server

Start the server with:

```bash
cargo run
```

The server will start on `http://127.0.0.1:7878` and display:

```
Server started. Press Ctrl+C to shutdown gracefully.
```

### Testing the Server

Once running, you can test the server with curl or a web browser:

```bash
# Root endpoint - returns hello message
curl http://127.0.0.1:7878/

# Sleep endpoint - simulates 5-second delay (good for testing concurrency)
curl http://127.0.0.1:7878/sleep

# Any other endpoint - returns 404 error
curl http://127.0.0.1:7878/unknown-page
```

### Shutting Down

Press `Ctrl+C` to gracefully shutdown the server. The server will:
1. Stop accepting new connections
2. Wait for all worker threads to complete their current tasks
3. Display shutdown messages for each worker
4. Exit cleanly

## API Endpoints

| Endpoint | Method | Description | Response |
|----------|--------|-------------|----------|
| `/` | GET | Root endpoint | Serves `hello.html` with welcome message |
| `/sleep` | GET | Slow response simulation | Serves `hello.html` after 5-second delay |
| `/*` | GET | Any other path | Serves `404.html` with error message |

## Architecture

### Thread Pool Implementation

The server uses a custom thread pool (`ThreadPool`) for handling concurrent connections:

- **Workers**: Individual threads that execute tasks
- **Channels**: MPSC (Multi-Producer, Single-Consumer) channels for task distribution
- **Jobs**: Closures wrapped in `Box<dyn FnOnce() + Send + 'static>` for thread-safe execution

### Key Components

#### `ThreadPool`
- Manages a pool of worker threads
- Distributes incoming connections across available workers
- Handles graceful shutdown by consuming the sender channel

#### `Worker`
- Represents an individual thread in the pool
- Each worker has a unique ID and runs in a loop waiting for jobs
- Automatically shuts down when the channel is closed

#### Connection Handling
- Non-blocking TCP listener for efficient polling
- Each connection is handled in a separate worker thread
- Proper HTTP/1.1 response formatting with Content-Length headers

### Request Flow

1. **Listener**: TCP listener accepts incoming connections
2. **Thread Pool**: New connections are assigned to available worker threads
3. **Request Parsing**: Worker reads HTTP request line
4. **Routing**: Matches request against defined routes
5. **Response**: Serves appropriate HTML file with HTTP headers
6. **Cleanup**: Connection is closed after response

## Dependencies

- **`ctrlc`**: Cross-platform signal handling for graceful shutdown

## Development

### Project Structure

```
src/
├── lib.rs          # Thread pool implementation
└── main.rs         # Server main loop and connection handling

Static files:
├── hello.html      # Welcome page
└── 404.html        # Error page
```

### Building for Development

```bash
cargo build
```

### Running Tests

```bash
cargo test
```

### Code Quality

```bash
# Check for issues
cargo check

# Format code
cargo fmt

# Lint code
cargo clippy
```

## Performance Considerations

- **Thread Pool Size**: Currently configured with 4 worker threads
- **Non-blocking I/O**: Server uses non-blocking TCP listener to avoid blocking on accept()
- **Connection Handling**: Each connection gets its own thread from the pool
- **Memory Management**: Efficient use of Arc and Mutex for shared state

## Security Notes

This is a basic HTTP server implementation intended for learning purposes. For production use, consider:

- HTTPS/TLS support
- Request validation and sanitization
- Rate limiting
- Authentication and authorization
- Proper error handling and logging
- Security headers (CSP, HSTS, etc.)

## Learning Outcomes

This project demonstrates:

- Low-level TCP networking in Rust
- Multi-threading and concurrency patterns
- Custom data structure implementation (ThreadPool)
- HTTP protocol basics
- Graceful shutdown patterns
- Error handling in concurrent systems
- Cargo workspace organization

## Contributing

Feel free to submit issues and enhancement requests!

## License

This project is open source and available under the [MIT License](LICENSE).

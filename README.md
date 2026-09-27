# RustNetcode

A multithreaded TCP/HTTP server built from scratch in Rust.

## Features

* TCP server using `std::net`
* Custom thread pool with worker threads
* Concurrent client handling
* Basic HTTP request parsing
* HTTP response handling
* `Content-Length` headers
* Error handling for invalid requests and resources
* Unit and integration tests

## Project Goals

This project is mainly for learning systems and network programming by implementing the underlying components rather than relying on existing server frameworks.

## Running

```bash
cargo run
```

The server runs locally at:

```text
http://127.0.0.1:6767/
```

## Testing

```bash
cargo test
```

To see test output:

```bash
cargo test -- --nocapture
```

## Tech

* Rust
* TCP/IP
* HTTP
* Multithreading
* Channels
* `Arc` / `Mutex`
* Unit & integration testing

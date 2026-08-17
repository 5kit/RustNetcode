use std::{
    format, fs,
    io::{BufReader, prelude::*},
    net::{TcpListener, TcpStream},
    println, thread,
    time::Duration,
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:6767").unwrap();
    println!("Starting Server...");

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        thread::spawn(|| {
            handle_connection(stream);
        });
    }

    println!("Closing Server.");
}

fn handle_connection(mut stream: TcpStream) {
    let reader = BufReader::new(&stream);
    let request_line = reader.lines().next().unwrap().unwrap();

    let (status, file) = match &request_line[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "index.html"),
        "GET /wait HTTP/1.1" => {
            thread::sleep(Duration::from_secs(10));
            ("HTTP/1.1 200 OK", "index.html")
        }
        _ => ("HTTP/1.1 404 NOT Found", "404.html"),
    };

    let contents = fs::read_to_string(file).unwrap();
    let length = contents.len();

    let response = format!("{status}\r\nContent-Length: {length}\r\n\r\n{contents}");
    stream.write_all(response.as_bytes()).unwrap();
}

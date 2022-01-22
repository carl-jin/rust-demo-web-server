use std::fs;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use web_server::ThreadPool;
use std::thread;
use std::time::Duration;

fn main() {
    let pool = ThreadPool::new(4);
    let listener = TcpListener::bind("127.0.0.1:9999").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        pool.execute(move ||{
            //  todo 为什么这里不调用内部的 handle_connection 又可以完成这个 并发了
            // println!("before sleep in bibao");
            // thread::sleep(Duration::from_secs(140));
            // println!("after sleep in bibao");
            handle_connection(stream)
        })
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";

    // println!("before sleep");
    // thread::sleep(Duration::from_secs(40));
    // println!("after sleep");

    let (start_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK", "index.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();
    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        start_line,
        contents.len(),
        contents
    );


    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();

    println!("Request: {}", response);
}

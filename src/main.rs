use std::{
    fs,
    io::prelude::*,
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};
use helloServer::ThreadPool;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:5000").unwrap();

    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        // println!("Connection established!");

        //  handle_connection(stream);
        
       // thread::spawn(|| {
       //     handle_connection(stream);
       // });

        pool.execute(|| {
            handle_connection(stream);
        });
    }

    println!("Shutting down...")
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    // let http_req: Vec<_> = buf_reader
    //     .lines()
    //     .map(|result| result.unwrap())
    //     .take_while(|line| !line.is_empty())
    //     .collect();

    // println!("Request: {:#?}", http_req);
    
    // let request_line = buf_reader.lines().next().unwrap().unwrap();
//
//    if request_line == "GET / HTTP/1.1"{
//        let status_line = "HTTP/1.1 200 OK";
//        let contents = fs::read_to_string("hello.html").unwrap();
//        let length = contents.len();
//        let response = format!(
//            "{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}"
//        );
//
//        stream.write_all(response.as_bytes()).unwrap();
//    }
//    else {
//        let status_line = "HTTP/1.1 404 NOT FOUND";
//        let contents = fs::read_to_string("404.html").unwrap();
//        let length = contents.len();
//        let response = format!(
//            "{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}"
//        );
//
//        stream.write_all(response.as_bytes()).unwrap();
//    }
    
//    let (status_line, filename) = if request_line == "GET / HTTP/1.1" {
//        ("HTTP/1.1 200 OK", "hello.html")
//    }
//    else {
//        ("HTTP/1.1 404 NOT FOUND", "404.html")
//    };

    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK", "hello.html")
    }
    else if buffer.starts_with(sleep) {
        thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1 200 OK", "hello.html")
    }
    else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();

    let response = format!(
            "{}\r\nContent-Length: {}\r\n\r\n{}",
            status_line,
            contents.len(),
            contents
        );
    
    stream.write_all(response.as_bytes()).unwrap();
    stream.flush().unwrap();

}

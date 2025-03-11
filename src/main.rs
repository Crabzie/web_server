use core::panic;
use dotenv::dotenv;
use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};
use web_handler::ThreadPool;

fn handle_connection(mut stream: TcpStream) {
    let www_path = std::env::var("www_path").expect("www_path Must be set");
    let buf_reader = BufReader::new(&stream);
    let request_lines: Vec<_> = buf_reader
        .lines()
        .map(|result| match result {
            Ok(result) => result,
            Err(e) => panic!("Problem parsing the data: {e:?}"),
        })
        .take_while(|line| !line.is_empty())
        .collect();
    let page_request = &request_lines[0];
    let (status_line, filename) = match &page_request[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", format!("{}hello.html", www_path)),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", format!("{}hello.html", www_path))
        }
        _ => ("HTTP/1.1 404 NOT FOUND", format!("{}404.html", www_path)),
    };
    let content = fs::read_to_string(filename).unwrap();
    let length = content.len();
    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{content}");
    match stream.write_all(response.as_bytes()) {
        Ok(_) => println!("Reply sent successfully"),
        Err(e) => panic!("Problem sending a reply: {e:?}"),
    };
}

fn main() {
    dotenv().ok();
    let listener = TcpListener::bind("localhost:7878")
        .unwrap_or_else(|error| panic!("Problem binding TcpListener port: {error:?}"));
    let pool = ThreadPool::build(4)
        .unwrap_or_else(|error| panic!("Problem creating thread pool: {error:?}"));

    for stream in listener.incoming() {
        match stream {
            Ok(s) => pool.execute(|| handle_connection(s)),
            Err(e) => println!("{e:?}"),
        }
    }
}

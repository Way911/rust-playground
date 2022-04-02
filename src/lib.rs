mod util;
use std::{
    fs,
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};
use util::threadpool::ThreadPool;

fn handle_connection(mut stream: TcpStream) {
    let mut data = [0; 1024];

    // read up to 10 bytes
    // let n = f.read(&mut buffer[..])?;

    // println!("The bytes: {:?}", &buffer[..n]);
    println!("Connection established!");
    stream.read(&mut data).unwrap();

    // let size = stream.read(&mut buffer).unwrap();
    // println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    let (status_line, body) = if data.starts_with(get) {
        (
            "HTTP/1.1 200 OK\r\n\r\n",
            fs::read_to_string("static/index.html").unwrap(),
        )
    } else if data.starts_with(sleep) {
        thread::sleep(Duration::from_secs(10));
        (
            "HTTP/1.1 200 OK\r\n\r\n",
            fs::read_to_string("static/index.html").unwrap(),
        )
    } else {
        (
            "HTTP/1.1 404 NOT FOUND\r\n\r\n",
            fs::read_to_string("static/404.html").unwrap(),
        )
    };

    let response = format!("{status_line}{body}");
    // println!("{}", response);
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

pub fn run() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let parl = thread::available_parallelism().unwrap().get();
    let pool = ThreadPool::new(parl);
    for stream in listener.incoming().take(3) {
        let stream = stream.unwrap();
        // println!("Connection established!");
        pool.execute(|| {
            handle_connection(stream);
        });
    }
}

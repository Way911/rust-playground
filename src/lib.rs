use std::thread;
use std::time::Duration;

use async_std::prelude::*;

use async_std::net::TcpListener;
use async_std::net::TcpStream;
use futures::stream::StreamExt;

use async_std::fs;
use async_std::task::spawn;
use rand::thread_rng;
use rand::Rng;

async fn handle_connection(mut stream: TcpStream) {
    let mut data = [0; 1024];

    // read up to 10 bytes
    // let n = f.read(&mut buffer[..])?;

    // println!("The bytes: {:?}", &buffer[..n]);
    println!("Connection established!");
    stream.read(&mut data).await.unwrap();

    // let size = stream.read(&mut buffer).unwrap();
    // println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";
    let expensive_cpu = b"GET /expensive_cpu HTTP/1.1\r\n";

    let (status_line, body) = if data.starts_with(get) {
        (
            "HTTP/1.1 200 OK\r\n\r\n",
            fs::read_to_string("static/index.html").await.unwrap(),
        )
    } else if data.starts_with(sleep) {
        thread::sleep(Duration::from_secs(10));
        (
            "HTTP/1.1 200 OK\r\n\r\n",
            fs::read_to_string("static/index.html").await.unwrap(),
        )
    } else if data.starts_with(expensive_cpu) {
        loop {
            let num: u32 = thread_rng().gen_range(0..1000000000);
            if num == 0 {
                break;
            }
        }
        (
            "HTTP/1.1 200 OK\r\n\r\n",
            fs::read_to_string("static/index.html").await.unwrap(),
        )
    } else {
        (
            "HTTP/1.1 404 NOT FOUND\r\n\r\n",
            fs::read_to_string("static/404.html").await.unwrap(),
        )
    };

    let response = format!("{status_line}{body}");
    // println!("{}", response);
    stream.write(response.as_bytes()).await.unwrap();
    stream.flush().await.unwrap();
}

pub async fn run() {
    let listener = TcpListener::bind("127.0.0.1:7878").await.unwrap();
    listener
        .incoming()
        .for_each_concurrent(/* limit */ None, |stream| async move {
            let stream = stream.unwrap();
            spawn(handle_connection(stream));
        })
        .await;
}

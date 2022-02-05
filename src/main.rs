use async_std::net::TcpListener;
use async_std::prelude::*;
use async_std::task;
use async_std::task::spawn;
use futures::stream::StreamExt;
use std::fs;
use async_std::io::{Read,Write};
use std::time::Duration;

mod mock_tcp;

#[async_std::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:8000").await.unwrap();

    listener
        .incoming()
        .for_each_concurrent(5, |tcpstream| async move {
            let tcpstream = tcpstream.unwrap();
            spawn(handle_connection(tcpstream));
        })
        .await;
}

async fn handle_connection(mut stream: impl Read + Write + Unpin) {
    println!("handling connectioness");
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).await.unwrap();

    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else if buffer.starts_with(sleep) {
        task::sleep(Duration::from_secs(5)).await;
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
    };
    let contents = fs::read_to_string(filename).unwrap();

    let response = format!("{status_line}{contents}");
    stream.write(response.as_bytes());
    stream.flush();
    println!("not sleeping connectioness");
}

#[async_std::test]
async fn test_handle_connection() {
    let input_bytes = b"GET / HTTP/1.1\r\n";
    let mut contents = vec![0u8; 1024];
    contents[..input_bytes.len()].clone_from_slice(input_bytes);
    let mut stream = mock_tcp::MockTcpStream {
        read_data: contents,
        write_data: Vec::new(),
    };

    handle_connection(&mut stream).await;
    let mut buf = [0u8; 1024];
    stream.read(&mut buf).await.unwrap();

    let expected_contents = fs::read_to_string("hello.html").unwrap();

    let expected_response = format!("HTTP/1.1 200 OK\r\n\r\n{}", expected_contents);
    
    assert!(stream.write_data.starts_with(expected_response.as_bytes()));
}

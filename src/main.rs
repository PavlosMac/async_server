use async_std::io::{Read, Write};
use async_std::net::TcpListener;
use async_std::prelude::*;
use async_std::task;
use async_std::task::spawn;
use futures::stream::StreamExt;
use std::fs;
use std::time::Duration;

mod mock_tcp;

#[async_std::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:8000").await.unwrap();

    let mut count = 0;

    listener
        .incoming()
        .for_each_concurrent(5, |tcpstream| async move {
            count = count + 1;
            let tcpstream = tcpstream.unwrap();
            // spawn(handle_connection(tcpstream, 0));
            spawn(handle_connection_testing(tcpstream, count));
        })
        .await;
}

async fn handle_connection_testing(mut stream: impl Read + Write + Unpin, count: i64) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).await.unwrap();

    if (count % 10) == 0 {
        println!("Adding delay. Count: {}", count);
        task::sleep(Duration::from_secs(2)).await;
    }
    let header = "
    HTTP/1.0 200 OK
    Connection: keep-alive
    Content-Length: 174
    Content-Type: text/html; charset=utf-8
        ";
    let contents = fs::read_to_string("hello.html").unwrap();

    let response = format!("{}\r\n\r\n{}", header, contents);

    stream.write(response.as_bytes()).await.unwrap();
    stream.flush().await.unwrap();
    println!("not sleeping connectioness");
}

async fn handle_connection(mut stream: impl Read + Write + Unpin, count: i64) {
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

    stream.write(response.as_bytes()).await.unwrap();
    stream.flush().await.unwrap();
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

    handle_connection(&mut stream, 0).await;

    let mut buf = [0u8; 1024];
    stream.read(&mut buf).await.unwrap();

    let expected_contents = fs::read_to_string("hello.html");

    let expected_response = format!(
        "HTTP/1.1 200 OK\r\n\r\n{}",
        expected_contents.unwrap_or("not there".to_owned())
    );
    let x = expected_response.as_bytes();

    assert!(stream.write_data.starts_with(x));
}

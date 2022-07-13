use std::io::prelude::*;
use std::fs;
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::time::Duration;

use mts_rw::ThreadPool;

fn handle_connection(mut stream: TcpStream )
{
    let mut buf = [0; 1024];
    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    stream.read(&mut buf).unwrap();
    println!("Request: {}", String::from_utf8_lossy(&buf[..]));
    
    let (status, filename) = if buf.starts_with(get)
    {
        ("HTTP/1.1 200 OK", "hello.html")
    }
    
    else if buf.starts_with(sleep)
    {
        thread::sleep(Duration::from_secs(10));
        ("HTTP/1.1 200 OK", "time_out.html")
    }
    
    else
    {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };
    
    let contents = fs::read_to_string(filename).unwrap();
    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status,
        contents.len(),
        contents
    );
    
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
    // println!("**Response**: {}", response);
}

fn main()
{
    let connection_listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    
    let pool = ThreadPool::new(5);
    println!("*****Server Waiting for Connections*****");

    for connections in connection_listener.incoming()
    {
       let stream = connections.unwrap();
       
    //    thread::spawn(|| {
    //        println!("\n*****Connection Established*****");
    //        handle_connection(stream);
    //    });
        pool.execute(|| {
            handle_connection(stream);
        });
    }

}
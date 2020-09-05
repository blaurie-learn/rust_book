use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::fs;

use std::thread;
use std::time::Duration;

use hello::ThreadPool;


fn main() {
    //recall that unwrap is a function on Result<T, E>
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }
}

//Internally, TcpStream updates itself to monitor what to send and what it'll need to send next
//time we read. so it needs to be mut.
fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    //a little refactoring:
    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else if buffer.starts_with(sleep) {

        //without some kind of ability to run asynchronously, this code will block
        //every other incoming connection for 5 seconds!
        //Thread pools can be a good solution for this.
        thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();

    let response = format!("{}{}", status_line, contents);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

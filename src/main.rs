use std::net::TcpListener;

// https://doc.rust-lang.org/book/ch20-01-single-threaded.html

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    // incoming: returns an iterator that gives a sequence of streams
    //           of type TcpStream. A single stream represents an open
    //           connection. 
    // connection: name for the full request and response processes
    //             - client connects, server responds, server closes conn
    for stream in listener.incoming() {
        let stream = stream.unwrap();

        println!("Connection established!");
    }
}
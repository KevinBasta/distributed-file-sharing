use std::{io::Write, net::TcpStream};



fn connectToPeer() {
    let mut stream = TcpStream::connect("127.0.0.1:8080").unwrap();

    let result = stream.write(b"Hello socket :)").unwrap();
    println!("{result}");
}

fn main() {
    println!("Hello world");

    connectToPeer();
}
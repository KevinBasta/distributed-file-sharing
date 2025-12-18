use std::{io::{Read, Write}, net::{TcpListener, TcpStream}, thread};



fn protocol(stream: &mut TcpStream) {
    println!("HIHIHI Stream :)!");

    let mut buffer: [u8; 128] = [0; 128]; 
    let result = stream.read(&mut buffer).unwrap();
    
    for byte in buffer.iter() {
        print!("{byte}");
    }
}

fn acceptPeer() {
    let listener = TcpListener::bind("0.0.0.0:9090").unwrap();
    let addr = listener.local_addr().unwrap();
    println!("{addr}");

    // spawn a detatched thread for each incoming connection
    for stream in listener.incoming() {
        println!("accepted");
        thread::spawn(move || protocol(&mut stream.unwrap()));
    }
}

fn requestPeer() {
    let mut stream = TcpStream::connect("127.0.0.1:7070").unwrap();

    let result = stream.write(b"").unwrap();
    println!("{result}");
}

fn main() {
    println!("Hello world");

    acceptPeer();
    //requestPeer();
}
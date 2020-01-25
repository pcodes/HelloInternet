/*
Code modified from https://riptutorial.com/rust/example/4404/a-simple-tcp-client-and-server-application--echo
*/

use std::thread;
use std::env;
use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::str::{from_utf8};

fn handle_client(mut stream: TcpStream) {
    let response = "Goodbye in Rust!";
    let mut data = [0 as u8; 50];
    let size = stream.read(&mut data).unwrap();
    println!("Received from client: {}", from_utf8(&data[0..size]).unwrap());
    stream.write(response.as_bytes()).unwrap();
}

fn server() {
    let listener = TcpListener::bind("0.0.0.0:8080").unwrap();
    println!("Server listening on port 8080");
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New Connection: {}", stream.peer_addr().unwrap());
                thread::spawn(move || {
                    handle_client(stream)
                });
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }
    
    drop(listener);
}

fn client(){
    match TcpStream::connect("localhost:8080") {
        Ok(mut stream) => {
            println!("Sending \"Hello in Rust!\"");

            let msg = b"Hello in Rust!";

            stream.write(msg).unwrap();

            let mut data = [0 as u8; 50]; // using 50 byte buffer
            match stream.read(&mut data) {
                Ok(size) => {
                    let text = from_utf8(&data[0..size]).unwrap();
                    println!("Received: {}", text);
                    //if &data == msg {
                    //    println!("Reply is ok!");
                    //} else {
                    //    let text = from_utf8(&data).unwrap();
                    //    println!("Unexpected reply: {}", text);
                    //}
                },
                Err(e) => {
                    println!("Failed to receive data: {}", e);
                }
            }
        },
        Err(e) => {
            println!("Failed to connect: {}", e);
        }
    }
    println!("Terminated.");
}

fn main() {
    let is_server = env::args().any(|arg| arg == "--server");
    let is_client = env::args().any(|arg| arg == "--client");

    if is_server && !is_client {
        server();
    } else if !is_server && is_client {
        client();
    } else {
        println!("Please pass in either --client or --server");
    }
}

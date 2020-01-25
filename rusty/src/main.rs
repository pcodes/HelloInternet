/*
Code modified from https://riptutorial.com/rust/example/4404/a-simple-tcp-client-and-server-application--echo
*/

use std::thread;
use std::env;
use std::net::{TcpListener, TcpStream, Shutdown};
use std::io::{Read, Write};
use std::str::from_utf8;

fn handle_client(mut stream: TcpStream) {
    let mut data = [0 as u8; 50];
    while match stream.read(&mut data) {
        Ok(size) => {
            stream.write(&data[0..size]).unwrap();
            true
        },
        Err(_) => {
            println!("An error occured, terminating connection with {}", stream.peer_addr().unwrap());
            stream.shutdown(Shutdown::Both).unwrap();
            false
        }
    } {}
}

fn server() {
    let listener = TcpListener::bind("0.0.0.0:8080").unwrap();
    println!("Server listening on port 80");
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
    match TcpStream::connect("3.86.163.129:8080") {
        Ok(mut stream) => {
            println!("Successfully connected to server in port 3333");

            let msg = b"Hello!";

            stream.write(msg).unwrap();
            println!("Sent Hello, awaiting reply...");

            let mut data = [0 as u8; 6]; // using 6 byte buffer
            match stream.read_exact(&mut data) {
                Ok(_) => {
                    if &data == msg {
                        println!("Reply is ok!");
                    } else {
                        let text = from_utf8(&data).unwrap();
                        println!("Unexpected reply: {}", text);
                    }
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

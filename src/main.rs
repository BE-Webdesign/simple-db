extern crate pest;
#[macro_use]
extern crate pest_derive;

use std::io::Read;
use std::io::Write;
use std::net::{TcpListener, TcpStream};
use std::thread;

use simpledb::GREETING;

use pest::Parser;

#[derive(Parser)]
#[grammar = "csv.pest"]
pub struct CSVParser;

fn handle_client(mut stream: TcpStream) {
    // read 20 bytes at a time from stream echoing back to stream
    loop {
        let mut read = [0; 1028];
        match stream.read(&mut read) {
            Ok(n) => {
                if n == 0 {
                    // connection was closed
                    break;
                }
                stream.write(GREETING.as_bytes()).unwrap();
                stream.write(&read[0..n]).unwrap();
            }
            Err(err) => {
                panic!("{}", err);
            }
        }
    }
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080")?;

    // accept connections and process them serially
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(move || {
                    handle_client(stream);
                });
            }
            Err(_) => {
                println!("Error");
            }
        }
    }
    Ok(())
}

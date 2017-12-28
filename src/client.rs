use std::net::TcpListener;
use std::thread;
use std::io::{Read, Write};
use std::io;

pub fn server_start() -> io::Result<()> {
    let listen = TcpListener::bind("127.0.0.1:8080")?;

    for stream in listen.incoming() {
        let mut stream = match stream {
            Ok(stream) => stream,
            Err(err) => {
                println!("An error occurred while accepting a connection: {}", err);
                continue;
            }
        };

        let _ = thread::spawn(move || -> io::Result<()> {
            loop {
                let mut b = [0; 1024];
                let n = stream.read(&mut b)?;
                if n == 0 {
                    return Ok(());
                } else {
                    stream.write(&b[0..n])?;
                }
            }
        });
    }
    Ok(())
}

fn main() {
    match server_start() {
        Ok(()) => (),
        Err(err) => println!("{:?}", err),
    }
}

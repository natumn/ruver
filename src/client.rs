extern crate hyper;
extern crate futures;
extern crate tokio_core;

use std::net::TcpListener;
use std::thread;
use std::io::{self, Read, Write};

use futures::{Future, Stream};
use hyper::Client;
use tokio_core::reactor::Core;

/*
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
*/

fn main() {
    let mut core = Core::new()?;
    let client = Client::new(&core.handle());
    let uri = "http://httpbin.org/ip".parse()?;
    let work = client.get(uri).and_then(|res| {
        println!("Response: {}", res.status());
        res.body().for_each(|chunk| {
            io::stdout().write_all(&chunk).map_err(From::from)
        })
    });
    core.run(work)?;
}

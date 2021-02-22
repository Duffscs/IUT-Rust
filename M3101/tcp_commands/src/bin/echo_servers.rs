
use std::io;
use std::net::{ TcpListener};
use std::io::{Result};
use std::thread;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Options {
    port : String
}

fn main()  -> std::io::Result<()> {
    let options = Options::from_args();
    let listener = TcpListener::bind(format!("localhost:{}",options.port))?;
    for stream in listener.incoming(){
        let mut stream1 = stream.expect("stream").try_clone() ?;
        let mut clone = stream1.try_clone() ?;
        thread::spawn(move||->Result<()> {
            io::copy(&mut stream1,&mut clone)?;
            Ok(())
        });
    }
    Ok(())
}
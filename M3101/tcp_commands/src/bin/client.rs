
use std::io;
use std::net::TcpStream;
use std::io::{stdin,stdout,Result};
use structopt::StructOpt;
use std::thread;


#[derive(Debug, StructOpt)]
struct Options {
    host : String,
    port : String
}

fn main()  -> io::Result<()> {
    let options = Options::from_args();
    let host = format!("{}:{}",options.host,options.port);
    let mut stream = TcpStream::connect(host).expect("connection au serveur");
    let mut clone = stream.try_clone() ?;
    thread::spawn(move||->Result<()> {
        io::copy(&mut clone,&mut stdout())?;
        Ok(())
    });
    io::copy(&mut stdin(),&mut stream)?;
    Ok(())
}
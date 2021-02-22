use std::io;
use std::net::{TcpStream, TcpListener};
use std::io::{stdin,stdout,Result};
use std::thread;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Options {
    subCmdHost : String,
    port : u16
}

fn forward_stdin_stdout(mut stream : TcpStream)->Result<()>{
    let mut clone = stream.try_clone() ?;
    thread::spawn(move||->Result<()> {
        io::copy(&mut clone,&mut stdout())?;
        Ok(())
    });
    io::copy(&mut stdin(),&mut stream)?;
    Ok(())
}

fn connect(host : &str, port : u16)->Result<()>{
    let host = format!("{}:{}",host,port);
    let mut stream = TcpStream::connect(host).expect("connection au serveur");
    forward_stdin_stdout(stream) ?;
    Ok(())
}

fn listen(port : u16)-> Result<()>{
    let listener = TcpListener::bind(format!("localhost:{}",port))?;
    for stream in listener.incoming(){
        let mut stream1 = stream.expect("stream").try_clone() ?;
        let mut clone = stream1.try_clone() ?;
        let mut stdin = stdin();
        thread::spawn(move||->Result<()> {
            io::copy(&mut stdin,&mut clone)?;
            Ok(())
        });
        thread::spawn(move||->Result<()> {
            io::copy(&mut stream1,&mut stdout())?;
            Ok(())
        });
    }
    Ok(())
}

fn main() ->Result<()>{
    let options = Options::from_args();
    if options.subCmdHost == "l" {
        listen(options.port);
    }else{
        connect(&options.subCmdHost[..],options.port);
    }
    Ok(())
}
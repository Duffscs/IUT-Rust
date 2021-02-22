use std::time::Instant;

use indicatif::{ProgressIterator};
use structopt::StructOpt;

use port_scanner;
use std::sync::mpsc;
use std::thread;

#[derive(Debug, StructOpt)]
struct Options {
    host: String,
    min_port: u16,
    max_port: u16,
}

fn main() {
    let instant = Instant::now();
    let options = Options::from_args();
    let (sender, receiver) = mpsc::channel();
    (options.min_port..=options.max_port).into_iter().progress()
        .for_each(|port| {
            let sender = sender.clone();
            let host = options.host.clone();
            thread::spawn(move || {
                if port_scanner::is_open(&host, port) {
                    sender.send(port).unwrap();
                }
            });
        });
    drop(sender);
    let open_ports: Vec<u16> = receiver.into_iter().collect();
    println!("Ports ouverts: {:?}", open_ports);
    println!("{:?}", instant.elapsed());
}
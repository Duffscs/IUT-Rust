use std::time::Instant;

use indicatif::{ProgressIterator, ParallelProgressIterator};
use structopt::StructOpt;
use rayon::prelude::*;

use port_scanner;

#[derive(Debug, StructOpt)]
struct Options {
    host : String,
    min_port: u16,
    max_port: u16,
}

fn main(){
    let instant = Instant::now();
    let options = Options::from_args();
    let open_ports: Vec<u16> = (options.min_port..=options.max_port)
        .into_par_iter().progress()
        .filter(|port| port_scanner::is_open(&options.host, *port))
        .collect();
    println!("Ports ouverts: {:?}", open_ports);
    println!("{:?}", instant.elapsed());
}
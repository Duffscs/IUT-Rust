use std::io::{self, BufRead, Read};
use structopt::StructOpt;
use std::collections::VecDeque;
use std::ops::Add;

#[derive(StructOpt)]
struct Options {
    #[structopt()]
    stop: usize
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Lecture de stdin");
    let mut mot:Vec<&str> =input.lines().collect();
    mot.reverse();
    mot.iter().for_each(| e| {
        println!("{}",e);
    })
}

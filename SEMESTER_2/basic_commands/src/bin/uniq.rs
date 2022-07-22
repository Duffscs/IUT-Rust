use std::io::{self, Read};
use structopt::StructOpt;
use std::collections::VecDeque;
use std::ops::Add;

fn main(){
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expected("input");

    let mut lines: Vec<&str> = input.lines().collect();
    lines.dedup();
    for line in lines.iter(){
        println!("{}", line);
    }
}

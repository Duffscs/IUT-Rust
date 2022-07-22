use std::io::{self, BufRead};
use structopt::StructOpt;
use std::collections::VecDeque;
use std::ops::Add;

#[derive(StructOpt)]
struct Options {
    #[structopt()]
    stop: usize
}

fn main() {
    io::stdin().lock().lines()
        .map(|line|
            line.expect("line")
        ).for_each(|line| {
            println!("{}", reverse_string(line));
        });
}

fn reverse_string(string : String) -> String{
    let mut vec: VecDeque<char> = VecDeque::new();
    string.chars().into_iter()
        .for_each(|e| vec.push_front(e));
    return vec.iter().collect();
}

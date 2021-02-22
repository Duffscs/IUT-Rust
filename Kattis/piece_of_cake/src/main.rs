use std::io;
use std::io::Read;

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Lecture de stdin");
    let mut words = input.split_whitespace();
    let length : f64 = words.next().expect("str").parse().expect("u64");
    let mut cut1 : f64 = words.next().expect("str").parse().expect("u64");
    let mut cut2 : f64 = words.next().expect("str").parse().expect("u64");
    if cut1 < (length / 2.0) {
        cut1 = length - cut1;
    }
    if cut2 < (length / 2.0) {
        cut2 = length - cut2 ;
    }
    println!("{}", cut1 * cut2 * 4.0);
}
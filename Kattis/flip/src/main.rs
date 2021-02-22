use std::io;
use std::io::Read;

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Lecture de stdin");
    let mut words = input.split_whitespace();
    let n1 : i64 = words.next().expect("n1").chars().rev().collect::<String>().parse().expect("i64");
    let n2 : i64 = words.next().expect("n1").chars().rev().collect::<String>().parse().expect("i64");
    println!("{}",n1.max(n2));
}
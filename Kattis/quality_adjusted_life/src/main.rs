use std::io;
use std::io::Read;

fn main()
{
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Lecture de stdin");
    let moy: f64 = input.lines().skip(1).map(|e| {
        let mut numbers = e.split_whitespace();
        let nb1: f64 = numbers.next().expect("str").parse().expect("f64");
        let nb2: f64 = numbers.next().expect("str").parse().expect("f64");
        nb1 * nb2
    }).sum();
    println!("{:0.3}", moy);
}
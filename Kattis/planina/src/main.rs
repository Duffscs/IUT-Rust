use std::io;
use std::io::Read;

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Lecture de stdin");
    let ith : u64 = input.lines().next().expect("str").parse().expect("u64");
    let nb_square: u64 = 1 << ith;
    let nb_point : u64 = (nb_square + 1).pow(2);
    println!("{}", nb_point);
}
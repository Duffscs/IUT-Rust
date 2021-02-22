use std::io;
use std::io::Read;

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Lecture de stdin");
    let mut words = input.split_whitespace().map(|e| e.parse::<i64>().expect("i64"));
    println!("{} {} {} {} {} {}", 1 - words.next().unwrap(), 1 - words.next().unwrap(), 2 - words.next().unwrap(), 2 - words.next().unwrap(), 2 - words.next().unwrap(), 8 - words.next().unwrap());
}
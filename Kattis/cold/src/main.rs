use std::io;
use std::io::Read;

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Lecture de stdin");
    let nb = input.split_whitespace()
        .skip(1)
        .map(|e| e.parse::<i32>().expect("Entier"))
        .filter(|e| *e < 0)
        .count();
    println!("{}",nb);
}

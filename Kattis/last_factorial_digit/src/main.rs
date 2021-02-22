use std::io;
use std::io::Read;

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Lecture de stdin");
    input.lines().skip(1).for_each(|e| {
        let nb: u64 = e.parse().expect("u64");
        println!("{}", factorial(nb) % 10);
    });
}

fn factorial(n: u64) -> u64 {
    if n == 1 { return 1; }
    return n * factorial(n - 1);
}
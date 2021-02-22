use std::io;
use std::io::Read;


fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Lecture de stdin");
    let mut words = input.split_whitespace();
    let number : i64 = words.next().expect("str").parse().expect("i64");
    let faktor : i64 = words.next().expect("str").parse().expect("i64");

    println!("{}", number * (faktor-1) + 1 );
}

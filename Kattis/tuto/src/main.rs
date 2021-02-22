use std::io;
use std::io::Read;

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Lecture de stdin");
    let mut words = input.split_whitespace();
    let r1 = words.next().expect("r1");
    println!("{}",r1);

}

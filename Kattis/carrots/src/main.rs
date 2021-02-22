use std::io;
use std::io::Read;

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Lecture de stdin");
    let mut words = input.split_whitespace();
    let _n = words.next().expect("r2").parse::<i32>().expect("Entier");
    let p = words.next().expect("r2").parse::<i32>().expect("Entier");
    println!("{}",p);
   
}
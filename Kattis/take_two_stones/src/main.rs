use std::io;
use std::io::Read;

fn main()
{
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Lecture de stdin");
    let nb: u64 = input.lines().next().expect("str").parse().expect("");
    if nb % 2 == 1 {
        println!("Alice");
    }else {
        println!("Bob");
    }
}
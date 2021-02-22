use std::io;
use std::io::Read;

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Lecture de stdin");
    let word = input.lines().next().expect("str").to_string();
    if word.contains("ss") {
        println!("hiss");
    }else {
        println!("no hiss");
    }
}

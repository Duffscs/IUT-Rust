use std::io;
use std::io::Read;
use std::collections::{VecDeque};
use std::str::Chars;


fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Lecture de stdin");
    let date = input.lines().next().expect("str");
    if date == "OCT 31" || date=="DEC 25" {
        println!("yup");
    }else {
        println!("nope");
    }
}

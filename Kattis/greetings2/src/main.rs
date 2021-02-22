use std::io;
use std::io::Read;
use std::collections::VecDeque;

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Lecture de stdin");
    let mut vec = input.lines().next().expect("str").chars().collect::<VecDeque<char>>();
    vec.pop_front();
    vec.pop_back();
    let e = vec.iter().collect::<String>();
    println!("h{}{}y",e,e);
}
use std::io;
use std::io::Read;
use std::collections::HashSet;

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Lecture de stdin");
    let words = input.split_whitespace();
    let mut hash: HashSet<i32> = HashSet::new();
    words.skip(1)
        .map(|e| e.parse::<i32>().expect("i32"))
        .collect::<Vec<i32>>()
        .chunks(2)
        .for_each(|e| {
            hash.extend((e[0]..(e[1] + 1)));
        });
    println!("{}", hash.len());
}